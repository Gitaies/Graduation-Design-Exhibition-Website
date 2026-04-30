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

    let mut query = String::from("SELECT * FROM works WHERE 1=1");

    if let Some(major) = &params.major {
        query.push_str(&format!(" AND major_code = '{}'", major));
    }

    query.push_str(" ORDER BY created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", page_size, offset));

    match sqlx::query_as::<_, Work>(&query)
        .fetch_all(&state.db)
        .await
    {
        Ok(works) => {
            let count_query = if let Some(major) = &params.major {
                format!("SELECT COUNT(*) as count FROM works WHERE major_code = '{}'", major)
            } else {
                "SELECT COUNT(*) as count FROM works".to_string()
            };

            let total: i64 = sqlx::query_scalar(&count_query)
                .fetch_one(&state.db)
                .await
                .unwrap_or(0);

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
