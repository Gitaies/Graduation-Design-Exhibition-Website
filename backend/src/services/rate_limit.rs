use uuid::Uuid;
use crate::error::AppError;

/// Redis 限流检查（滑动窗口）
/// 使用 pipeline 将 4 条命令合并为 1 次网络往返
pub async fn check_rate_limit(
    redis: &redis::aio::MultiplexedConnection,
    key: &str,
    limit: u32,
    window_seconds: u64,
) -> Result<bool, AppError> {
    let mut conn = redis.clone();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let window_start = now - (window_seconds * 1000);
    let member = format!("{}:{}", now, Uuid::new_v4());

    let (_, count, _, _): ((), u32, (), ()) = redis::pipe()
        .zrembyscore(key, 0, window_start as f64)
        .zcard(key)
        .zadd(key, now as f64, &member)
        .expire(key, window_seconds as i64)
        .query_async(&mut conn)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(count < limit)
}

/// 点赞限流：同一指纹对同一作品，10 秒内最多操作 1 次
pub async fn check_like_rate_limit(
    redis: &redis::aio::MultiplexedConnection,
    fingerprint: &str,
    work_id: &str,
) -> Result<bool, AppError> {
    let key = format!("rate_limit:like:{}:{}", work_id, fingerprint);
    check_rate_limit(redis, &key, 1, 10).await
}

/// 评论限流：同一指纹，30 秒内最多评论 1 次
pub async fn check_comment_rate_limit(
    redis: &redis::aio::MultiplexedConnection,
    fingerprint: &str,
) -> Result<bool, AppError> {
    let key = format!("rate_limit:comment:{}", fingerprint);
    check_rate_limit(redis, &key, 1, 30).await
}

/// IP 限流：同一 IP，1 分钟内最多 20 次请求
pub async fn check_ip_rate_limit(
    redis: &redis::aio::MultiplexedConnection,
    ip: &str,
) -> Result<bool, AppError> {
    let key = format!("rate_limit:ip:{}", ip);
    check_rate_limit(redis, &key, 20, 60).await
}
