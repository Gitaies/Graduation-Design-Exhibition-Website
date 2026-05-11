use redis::AsyncCommands;
use uuid::Uuid;
use crate::error::AppError;

/// Redis 限流检查（滑动窗口）
pub async fn check_rate_limit(
    redis_client: &redis::Client,
    key: &str,
    limit: u32,
    window_seconds: u64,
) -> Result<bool, AppError> {
    let mut conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| AppError::InternalError)?;

    // 使用毫秒精度避免同一秒内的请求被覆盖
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // 使用 ZSET 实现滑动窗口
    let window_start = now - (window_seconds * 1000);

    // 删除窗口外的旧记录
    let _: () = conn
        .zrembyscore(key, 0, window_start as f64)
        .await
        .map_err(|_| AppError::InternalError)?;

    // 获取当前窗口内的请求数
    let count: u32 = conn
        .zcard(key)
        .await
        .map_err(|_| AppError::InternalError)?;

    if count >= limit {
        return Ok(false); // 超过限流
    }

    // 添加当前请求 - 使用唯一 member 避免覆盖
    let member = format!("{}:{}", now, Uuid::new_v4());
    let _: () = conn
        .zadd(key, now as f64, member)
        .await
        .map_err(|_| AppError::InternalError)?;

    // 设置过期时间（秒）
    let _: () = conn
        .expire(key, window_seconds as i64)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(true) // 通过限流检查
}

/// 点赞限流：同一指纹对同一作品，10 秒内最多操作 1 次
pub async fn check_like_rate_limit(
    redis_client: &redis::Client,
    fingerprint: &str,
    work_id: &str,
) -> Result<bool, AppError> {
    let key = format!("rate_limit:like:{}:{}", work_id, fingerprint);
    check_rate_limit(redis_client, &key, 1, 10).await
}

/// 评论限流：同一指纹，30 秒内最多评论 1 次
pub async fn check_comment_rate_limit(
    redis_client: &redis::Client,
    fingerprint: &str,
) -> Result<bool, AppError> {
    let key = format!("rate_limit:comment:{}", fingerprint);
    check_rate_limit(redis_client, &key, 1, 30).await
}

/// IP 限流：同一 IP，1 分钟内最多 20 次请求
pub async fn check_ip_rate_limit(
    redis_client: &redis::Client,
    ip: &str,
) -> Result<bool, AppError> {
    let key = format!("rate_limit:ip:{}", ip);
    check_rate_limit(redis_client, &key, 20, 60).await
}
