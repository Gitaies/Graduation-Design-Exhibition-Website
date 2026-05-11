use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::json;

use crate::models::work::{ListWorksQuery, Work, WorkListResponse, WorkWithStats};
use crate::AppState;

pub async fn list_works(
    State(state): State<AppState>,
    Query(params): Query<ListWorksQuery>,
) -> Json<serde_json::Value> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * page_size;

    // 使用参数绑定防止 SQL 注入
    let works_result = if let Some(major) = &params.major {
        sqlx::query_as::<_, Work>(
            "SELECT * FROM works WHERE major_code = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(major)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.db)
        .await
    } else {
        sqlx::query_as::<_, Work>(
            "SELECT * FROM works ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.db)
        .await
    };

    match works_result {
        Ok(works) => {
            // 计数查询也使用参数绑定
            let total: i64 = if let Some(major) = &params.major {
                sqlx::query_scalar("SELECT COUNT(*) FROM works WHERE major_code = ?")
                    .bind(major)
                    .fetch_one(&state.db)
                    .await
                    .unwrap_or(0)
            } else {
                sqlx::query_scalar("SELECT COUNT(*) FROM works")
                    .fetch_one(&state.db)
                    .await
                    .unwrap_or(0)
            };

            let items: Vec<WorkWithStats> = works
                .into_iter()
                .map(|work| WorkWithStats {
                    work,
                    like_count: 0,
                    comment_count: 0,
                })
                .collect();

            Json(json!({
                "code": 0,
                "message": "ok",
                "data": WorkListResponse {
                    items,
                    page,
                    page_size,
                    total,
                    total_pages: ((total as f64) / (page_size as f64)).ceil() as i32,
                }
            }))
        }
        Err(e) => {
            tracing::error!("Failed to fetch works: {}", e);
            Json(json!({
                "code": 50000,
                "message": "Failed to fetch works",
                "data": null
            }))
        }
    }
}

pub async fn get_work(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    match sqlx::query_as::<_, Work>("SELECT * FROM works WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await
    {
        Ok(work) => Json(json!({
            "code": 0,
            "message": "ok",
            "data": work
        })),
        Err(sqlx::Error::RowNotFound) => Json(json!({
            "code": 40400,
            "message": "Work not found",
            "data": null
        })),
        Err(e) => {
            tracing::error!("Failed to fetch work: {}", e);
            Json(json!({
                "code": 50000,
                "message": "Internal server error",
                "data": null
            }))
        }
    }
}
