use sha2::{Sha256, Digest};

/// 生成游客指纹哈希
/// 组合：work_id + visitor_id + ip + server_salt
pub fn generate_fingerprint(
    work_id: &str,
    visitor_id: &str,
    ip: &str,
    server_salt: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(work_id.as_bytes());
    hasher.update(visitor_id.as_bytes());
    hasher.update(ip.as_bytes());
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
