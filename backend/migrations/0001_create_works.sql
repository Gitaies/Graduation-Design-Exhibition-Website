-- 创建作品表（存储完整作品信息）
CREATE TABLE works (
  id VARCHAR(64) PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  major_code VARCHAR(64) NOT NULL,
  major_name VARCHAR(100) NOT NULL,
  theme VARCHAR(100) NOT NULL,
  tags JSON NOT NULL,
  authors JSON NOT NULL,
  teacher VARCHAR(100) NOT NULL,
  poster_url VARCHAR(500),
  video_url VARCHAR(500),
  duration VARCHAR(20),
  introduction TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX idx_major (major_code),
  INDEX idx_created (created_at DESC)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
