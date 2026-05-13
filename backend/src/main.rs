use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

mod config;
mod db;
mod error;
mod response;
mod routes;
mod models;
mod services;
mod utils;

use config::Config;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    // 初始化数据库连接池
    let db_pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // 初始化 Redis 连接（启动时建立，后续请求复用，避免每次请求重建连接）
    let redis_client = redis::Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");
    let redis_conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to create Redis connection");

    // 创建应用状态
    let app_state = AppState {
        db: db_pool,
        redis: redis_conn,
        config: config.clone(),
    };

    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        .route("/api/health", get(routes::health::health_check))
        .route("/api/works", get(routes::works::list_works))
        .route("/api/works/:id", get(routes::works::get_work))
        .route("/api/interactions/summary", get(routes::interactions::get_summary))
        .route("/api/works/:work_id/interaction", get(routes::interactions::get_work_interaction))
        .route("/api/works/:work_id/like/toggle", post(routes::likes::toggle_like))
        .route("/api/works/:work_id/comments", get(routes::comments::get_comments))
        .route("/api/works/:work_id/comments", post(routes::comments::create_comment))
        .route("/api/rankings/likes", get(routes::rankings::get_like_ranking))
        .layer(cors)
        .with_state(app_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub redis: redis::aio::MultiplexedConnection,
    pub config: Config,
}
