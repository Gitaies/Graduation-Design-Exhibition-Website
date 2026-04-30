use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RankingItem {
    pub work_id: String,
    pub title: String,
    pub major_code: String,
    pub major_name: String,
    pub poster_url: Option<String>,
    pub like_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct RankingQuery {
    pub range: Option<String>, // all, today, week
    pub limit: Option<i32>,
}
