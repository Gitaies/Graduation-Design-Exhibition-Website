use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::comment::{Comment, CommentResponse, CreateCommentRequest, ListCommentsQuery};
use crate::services::{fingerprint, rate_limit, sanitize};
use crate::utils::{ip, random_name};
use crate::AppState;

pub async fn get_comments(
    State(state): State<AppState>,
    Path(work_id): Path<String>,
    Query(params): Query<ListCommentsQuery>,
) -> Json<serde_json::Value> {
    let limit = params.limit.unwrap_or(20).clamp(1, 50);

    let mut query = String::from(
        "SELECT * FROM comments WHERE work_id = ? AND status = 'visible'"
    );

    if let Some(_cursor) = &params.cursor {
        query.push_str(" AND created_at < (SELECT created_at FROM comments WHERE public_id = ?)");
    }

    query.push_str(" ORDER BY created_at DESC LIMIT ?");

    let comments: Vec<Comment> = if let Some(cursor) = &params.cursor {
        sqlx::query_as(&query)
            .bind(&work_id)
            .bind(cursor)
            .bind(limit)
            .fetch_all(&state.db)
            .await
            .unwrap_or_default()
    } else {
        sqlx::query_as(&query)
            .bind(&work_id)
            .bind(limit)
            .fetch_all(&state.db)
            .await
            .unwrap_or_default()
    };

    let has_more = comments.len() >= limit as usize;
    let next_cursor = comments.last().map(|c| c.public_id.clone());

    let items: Vec<CommentResponse> = comments.into_iter().map(|c| c.into()).collect();

    Json(json!({
        "code": 0,
        "message": "ok",
        "data": {
            "items": items,
            "next_cursor": next_cursor,
            "has_more": has_more
        }
    }))
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path(work_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<CreateCommentRequest>,
) -> Json<serde_json::Value> {
    // 蜜罐字段检测
    if !payload.website.is_empty() {
        return Json(json!({
            "code": 40003,
            "message": "Invalid request",
            "data": null
        }));
    }

    // 验证评论内容
    if let Err(msg) = sanitize::validate_comment(&payload.content) {
        return Json(json!({
            "code": 40000,
            "message": msg,
            "data": null
        }));
    }

    // 简单敏感词检测
    if sanitize::contains_sensitive_words(&payload.content) {
        return Json(json!({
            "code": 40003,
            "message": "评论内容包含敏感词",
            "data": null
        }));
    }

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

    // 检查指纹限流（30秒/1次）
    if !rate_limit::check_comment_rate_limit(&state.redis, &fingerprint_hash)
        .await
        .unwrap_or(false)
    {
        return Json(json!({
            "code": 40001,
            "message": "评论太频繁，请稍后再试",
            "data": null
        }));
    }

    // 生成随机游客昵称
    let visitor_name = random_name::generate_visitor_name();

    // HTML 转义
    let sanitized_content = sanitize::sanitize_html(&payload.content);

    // 插入数据库
    let comment_id = Uuid::new_v4().to_string();
    let public_id = format!("cmt_{}", Uuid::new_v4().simple());

    let result = sqlx::query(
        "INSERT INTO comments (id, public_id, work_id, visitor_name, visitor_fingerprint_hash, ip_hash, content, status)
         VALUES (?, ?, ?, ?, ?, ?, ?, 'visible')"
    )
    .bind(&comment_id)
    .bind(&public_id)
    .bind(&work_id)
    .bind(&visitor_name)
    .bind(&fingerprint_hash)
    .bind(&ip_hash)
    .bind(&sanitized_content)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            Json(json!({
                "code": 0,
                "message": "ok",
                "data": {
                    "id": public_id,
                    "visitor_name": visitor_name,
                    "content": sanitized_content,
                    "created_at": chrono::Utc::now().to_rfc3339()
                }
            }))
        }
        Err(e) => {
            tracing::error!("Failed to create comment: {}", e);
            Json(json!({
                "code": 50000,
                "message": "评论提交失败",
                "data": null
            }))
        }
    }
}
