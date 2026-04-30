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
    // 获取 visitor_id 和 IP
    let visitor_id = headers
        .get("X-Visitor-Id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let ip_addr = ip::extract_ip(&headers)
        .map(|ip| ip::ip_to_string(&ip))
        .unwrap_or_else(|| "unknown".to_string());

    // 生成指纹
    let fingerprint_hash = fingerprint::generate_fingerprint(
        &work_id,
        visitor_id,
        &ip_addr,
        &state.config.server_salt,
    );

    let ip_hash = fingerprint::hash_ip(&ip_addr, &state.config.server_salt);

    // 检查 IP 限流（60秒/20次）
    if !rate_limit::check_ip_rate_limit(&state.redis, &ip_addr)
        .await
        .unwrap_or(false)
    {
        return Json(json!({
            "code": 40001,
            "message": "操作太频繁，请稍后再试",
            "data": null
        }));
    }

    // 检查指纹限流（10秒/1次）
    if !rate_limit::check_like_rate_limit(&state.redis, &fingerprint_hash, &work_id)
        .await
        .unwrap_or(false)
    {
        return Json(json!({
            "code": 40001,
            "message": "操作太频繁，请稍后再试",
            "data": null
        }));
    }

    // 查询是否已点赞
    let existing: Option<(String, bool)> = sqlx::query_as(
        "SELECT id, is_active FROM likes WHERE work_id = ? AND visitor_fingerprint_hash = ?"
    )
    .bind(&work_id)
    .bind(&fingerprint_hash)
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None);

    let liked: bool;

    if let Some((like_id, is_active)) = existing {
        // 已存在记录，切换状态
        liked = !is_active;
        let _ = sqlx::query("UPDATE likes SET is_active = ?, updated_at = NOW() WHERE id = ?")
            .bind(liked)
            .bind(&like_id)
            .execute(&state.db)
            .await;
    } else {
        // 新增点赞记录
        liked = true;
        let like_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO likes (id, work_id, visitor_fingerprint_hash, ip_hash, is_active)
             VALUES (?, ?, ?, ?, TRUE)"
        )
        .bind(&like_id)
        .bind(&work_id)
        .bind(&fingerprint_hash)
        .bind(&ip_hash)
        .execute(&state.db)
        .await;
    }

    // 查询最新点赞数
    let like_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM likes WHERE work_id = ? AND is_active = TRUE"
    )
    .bind(&work_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

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
