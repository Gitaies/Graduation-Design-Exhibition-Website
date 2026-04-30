use axum::http::HeaderMap;
use std::net::IpAddr;

/// 从请求头中提取真实 IP 地址
pub fn extract_ip(headers: &HeaderMap) -> Option<IpAddr> {
    // 优先从代理头获取
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            // X-Forwarded-For 可能包含多个 IP，取第一个
            if let Some(first_ip) = value.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }
    }

    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            if let Ok(ip) = value.parse::<IpAddr>() {
                return Some(ip);
            }
        }
    }

    None
}

/// 将 IP 地址转换为字符串
pub fn ip_to_string(ip: &IpAddr) -> String {
    ip.to_string()
}
