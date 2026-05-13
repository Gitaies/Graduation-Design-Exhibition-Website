use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
}
