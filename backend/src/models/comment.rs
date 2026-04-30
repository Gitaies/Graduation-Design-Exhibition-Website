use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: String,
    pub public_id: String,
    pub work_id: String,
    pub visitor_name: String,
    pub visitor_fingerprint_hash: String,
    pub ip_hash: String,
    pub content: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub visitor_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<Comment> for CommentResponse {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.public_id,
            visitor_name: comment.visitor_name,
            content: comment.content,
            created_at: comment.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    #[allow(dead_code)]
    pub client_ts: i64,
    #[serde(default)]
    pub website: String, // 蜜罐字段
}

#[derive(Debug, Deserialize)]
pub struct ListCommentsQuery {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}
