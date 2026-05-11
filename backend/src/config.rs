#[derive(Clone)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub server_salt: String,
    pub cors_allowed_origins: Vec<String>,
    pub trust_proxy: bool,
    pub cos_base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            app_host: std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("APP_PORT must be a valid port number"),
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            server_salt: std::env::var("SERVER_SALT")
                .expect("SERVER_SALT must be set"),
            cors_allowed_origins: std::env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            trust_proxy: std::env::var("TRUST_PROXY")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            cos_base_url: std::env::var("COS_BASE_URL")
                .unwrap_or_else(|_| "https://whcm-1353140174.cos.ap-nanjing.myqcloud.com".to_string()),
        }
    }
}
