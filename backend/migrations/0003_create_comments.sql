-- 创建评论表
CREATE TABLE comments (
  id CHAR(36) PRIMARY KEY,
  public_id VARCHAR(64) NOT NULL UNIQUE,
  work_id VARCHAR(64) NOT NULL,
  visitor_name VARCHAR(64) NOT NULL,
  visitor_fingerprint_hash VARCHAR(128) NOT NULL,
  ip_hash VARCHAR(128) NOT NULL,
  content TEXT NOT NULL,
  status VARCHAR(20) NOT NULL DEFAULT 'visible',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_work_created (work_id, created_at DESC),
  INDEX idx_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
