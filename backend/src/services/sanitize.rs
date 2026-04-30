/// HTML 转义，防止 XSS 攻击
pub fn sanitize_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('/', "&#x2F;")
}

/// 验证评论内容
pub fn validate_comment(content: &str) -> Result<(), String> {
    let trimmed = content.trim();

    if trimmed.is_empty() {
        return Err("评论内容不能为空".to_string());
    }

    if trimmed.len() < 2 {
        return Err("评论内容至少需要 2 个字符".to_string());
    }

    if trimmed.len() > 500 {
        return Err("评论内容不能超过 500 个字符".to_string());
    }

    Ok(())
}

/// 简单的敏感词检测（降级方案）
pub fn contains_sensitive_words(content: &str) -> bool {
    let sensitive_words = [
        "fuck", "shit", "damn", "傻逼", "操", "妈的", "草泥马",
        "垃圾", "sb", "cnm", "nmsl",
    ];

    let lower_content = content.to_lowercase();

    for word in &sensitive_words {
        if lower_content.contains(word) {
            return true;
        }
    }

    false
}
