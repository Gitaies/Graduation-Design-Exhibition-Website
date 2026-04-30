use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::services::fingerprint;
use crate::utils::ip;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct SummaryQuery {
    work_ids: String, // 逗号分隔的作品 ID
}

pub async fn get_summary(
    State(state): State<AppState>,
    Query(params): Query<SummaryQuery>,
) -> Json<serde_json::Value> {
    let work_ids: Vec<&str> = params.work_ids.split(',').collect();

    if work_ids.is_empty() {
        return Json(json!({
            "code": 0,
            "message": "ok",
            "data": []
        }));
    }

    let mut results = Vec::new();

    for work_id in work_ids {
        let work_id = work_id.trim();

        // 查询点赞数
        let like_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM likes WHERE work_id = ? AND is_active = TRUE"
        )
        .bind(work_id)
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);

        // 查询评论数
        let comment_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM comments WHERE work_id = ? AND status = 'visible'"
        )
        .bind(work_id)
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);

        results.push(json!({
            "work_id": work_id,
            "like_count": like_count,
            "comment_count": comment_count
        }));
    }

    Json(json!({
        "code": 0,
        "message": "ok",
        "data": results
    }))
}

pub async fn get_work_interaction(
    State(state): State<AppState>,
    Path(work_id): Path<String>,
    headers: HeaderMap,
) -> Json<serde_json::Value> {
    // 查询点赞数
    let like_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM likes WHERE work_id = ? AND is_active = TRUE"
    )
    .bind(&work_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    // 查询评论数
    let comment_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM comments WHERE work_id = ? AND status = 'visible'"
    )
    .bind(&work_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    // 检查当前用户是否点赞
    let visitor_id = headers
        .get("X-Visitor-Id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let ip_addr = ip::extract_ip(&headers)
        .map(|ip| ip::ip_to_string(&ip))
        .unwrap_or_else(|| "unknown".to_string());

    let fingerprint_hash = fingerprint::generate_fingerprint(
        &work_id,
        visitor_id,
        &ip_addr,
        &state.config.server_salt,
    );

    let liked_by_me: bool = sqlx::query_scalar(
        "SELECT is_active FROM likes WHERE work_id = ? AND visitor_fingerprint_hash = ?"
    )
    .bind(&work_id)
    .bind(&fingerprint_hash)
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None)
    .unwrap_or(false);

    Json(json!({
        "code": 0,
        "message": "ok",
        "data": {
            "work_id": work_id,
            "like_count": like_count,
            "comment_count": comment_count,
            "liked_by_me": liked_by_me
        }
    }))
}
