use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::json;

use crate::models::ranking::{RankingItem, RankingQuery};
use crate::AppState;

pub async fn get_like_ranking(
    State(state): State<AppState>,
    Query(params): Query<RankingQuery>,
) -> Json<serde_json::Value> {
    let limit = params.limit.unwrap_or(10).clamp(1, 50);
    let range = params.range.unwrap_or_else(|| "all".to_string());

    let mut query = String::from(
        "SELECT w.id as work_id, w.title, w.major_code, w.major_name, w.poster_url,
         COUNT(l.id) as like_count
         FROM works w
         LEFT JOIN likes l ON w.id = l.work_id AND l.is_active = TRUE"
    );

    // 根据时间范围过滤
    match range.as_str() {
        "today" => {
            query.push_str(" AND l.created_at >= CURDATE()");
        }
        "week" => {
            query.push_str(" AND l.created_at >= DATE_SUB(CURDATE(), INTERVAL 7 DAY)");
        }
        _ => {} // "all" 不过滤
    }

    query.push_str(" GROUP BY w.id, w.title, w.major_code, w.major_name, w.poster_url");
    query.push_str(" ORDER BY like_count DESC, w.created_at DESC");
    query.push_str(&format!(" LIMIT {}", limit));

    let items: Vec<RankingItem> = sqlx::query_as(&query)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    Json(json!({
        "code": 0,
        "message": "ok",
        "data": {
            "items": items,
            "range": range,
            "limit": limit
        }
    }))
}
