use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::services::{fingerprint, rate_limit};
use crate::utils::ip;
use crate::AppState;

#[derive(Debug, serde::Deserialize)]
pub struct ToggleLikeRequest {
    #[allow(dead_code)]
    client_ts: i64,
}

pub async fn toggle_like(
    State(state): State<AppState>,
    Path(work_id): Path<String>,
    headers: HeaderMap,
    Json(_payload): Json<ToggleLikeRequest>,
) -> Json<serde_json::Value> {
    // 获取 visitor_id 和 IP（同步，无阻塞）
    let visitor_id = headers
        .get("X-Visitor-Id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let ip_addr = ip::extract_ip(&headers)
        .map(|ip| ip::ip_to_string(&ip))
        .unwrap_or_else(|| "unknown".to_string());

    // 生成指纹（同步）
    let fingerprint_hash = fingerprint::generate_fingerprint(
        &work_id,
        visitor_id,
        &ip_addr,
        &state.config.server_salt,
    );

    let ip_hash = fingerprint::hash_ip(&ip_addr, &state.config.server_salt);

    // 并行执行两项 Redis 限流检查，fail-open
    let (ip_result, like_result) = tokio::join!(
        rate_limit::check_ip_rate_limit(&state.redis, &ip_addr),
        rate_limit::check_like_rate_limit(&state.redis, &fingerprint_hash, &work_id),
    );

    let ip_allowed = ip_result.unwrap_or_else(|e| {
        tracing::warn!("IP 限流检查失败，放行: {:?}", e);
        true
    });
    if !ip_allowed {
        return Json(json!({
            "code": 40001,
            "message": "操作太频繁，请稍后再试",
            "data": null
        }));
    }

    let like_allowed = like_result.unwrap_or_else(|e| {
        tracing::warn!("点赞限流检查失败，放行: {:?}", e);
        true
    });
    if !like_allowed {
        return Json(json!({
            "code": 40001,
            "message": "操作太频繁，请稍后再试",
            "data": null
        }));
    }

    // 原子化切换点赞状态（INSERT ... ON DUPLICATE KEY UPDATE）
    let like_id = Uuid::new_v4().to_string();
    if let Err(e) = sqlx::query(
        "INSERT INTO likes (id, work_id, visitor_fingerprint_hash, ip_hash, is_active)
         VALUES (?, ?, ?, ?, TRUE)
         ON DUPLICATE KEY UPDATE is_active = NOT is_active, updated_at = NOW()"
    )
    .bind(&like_id)
    .bind(&work_id)
    .bind(&fingerprint_hash)
    .bind(&ip_hash)
    .execute(&state.db)
    .await
    {
        tracing::error!("点赞操作数据库失败: {:?}", e);
        return Json(json!({
            "code": 50000,
            "message": "操作失败，请稍后重试",
            "data": null
        }));
    }

    // 并行查询最终状态和点赞数（两个独立查询）
    let (liked_result, like_count_result): (Result<bool, sqlx::Error>, Result<i64, sqlx::Error>) = tokio::join!(
        sqlx::query_scalar(
            "SELECT is_active FROM likes WHERE work_id = ? AND visitor_fingerprint_hash = ?"
        )
        .bind(&work_id)
        .bind(&fingerprint_hash)
        .fetch_one(&state.db),
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM likes WHERE work_id = ? AND is_active = TRUE"
        )
        .bind(&work_id)
        .fetch_one(&state.db),
    );

    let liked = liked_result.unwrap_or(true);
    let like_count = like_count_result.unwrap_or(0);

    Json(json!({
        "code": 0,
        "message": "ok",
        "data": {
            "work_id": work_id,
            "liked": liked,
            "like_count": like_count
        }
    }))
}
