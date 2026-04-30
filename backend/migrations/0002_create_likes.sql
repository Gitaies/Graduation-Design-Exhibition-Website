-- 创建点赞表
CREATE TABLE likes (
  id CHAR(36) PRIMARY KEY,
  work_id VARCHAR(64) NOT NULL,
  visitor_fingerprint_hash VARCHAR(128) NOT NULL,
  ip_hash VARCHAR(128) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uk_work_visitor (work_id, visitor_fingerprint_hash),
  INDEX idx_work_active (work_id, is_active),
  INDEX idx_updated (updated_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
