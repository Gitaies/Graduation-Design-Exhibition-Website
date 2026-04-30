use axum::Json;
use serde_json::json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "code": 0,
        "message": "ok",
        "data": {
            "status": "ok"
        }
    }))
}
