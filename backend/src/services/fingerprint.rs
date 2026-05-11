use chrono::Local;
use sha2::{Digest, Sha256};

/// 生成游客指纹哈希（按天隔离）
/// 组合：work_id + visitor_id + ip + date + server_salt
/// 日期参与哈希，确保每天凌晨后同一用户可重新点赞
pub fn generate_fingerprint(
    work_id: &str,
    visitor_id: &str,
    ip: &str,
    server_salt: &str,
) -> String {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut hasher = Sha256::new();
    hasher.update(work_id.as_bytes());
    hasher.update(visitor_id.as_bytes());
    hasher.update(ip.as_bytes());
    hasher.update(today.as_bytes());
    hasher.update(server_salt.as_bytes());

    let result = hasher.finalize();
    hex::encode(result)
}

/// 生成 IP 哈希
pub fn hash_ip(ip: &str, server_salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    hasher.update(server_salt.as_bytes());

    let result = hasher.finalize();
    hex::encode(result)
}
