-- 创建风控日志表
CREATE TABLE abuse_logs (
  id CHAR(36) PRIMARY KEY,
  event_type VARCHAR(64) NOT NULL,
  work_id VARCHAR(64),
  ip_hash VARCHAR(128),
  reason TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_created (created_at DESC),
  INDEX idx_event_type (event_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
