use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Work {
    pub id: String,
    pub title: String,
    pub major_code: String,
    pub major_name: String,
    pub theme: String,
    #[sqlx(json)]
    pub tags: Vec<String>,
    #[sqlx(json)]
    pub authors: Vec<String>,
    pub teacher: String,
    pub poster_url: Option<String>,
    pub video_url: Option<String>,
    pub duration: Option<String>,
    pub introduction: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ListWorksQuery {
    pub major: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct WorkListResponse {
    pub items: Vec<WorkWithStats>,
    pub page: i32,
    pub page_size: i32,
    pub total: i64,
    pub total_pages: i32,
}

#[derive(Debug, Serialize)]
pub struct WorkWithStats {
    #[serde(flatten)]
    pub work: Work,
    pub like_count: i32,
    pub comment_count: i32,
}
