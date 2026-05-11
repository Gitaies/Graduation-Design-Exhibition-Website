-- 统一字符集与校对集为 utf8mb4_unicode_ci，避免 JOIN 时出现
-- "Illegal mix of collations" 错误。
-- 背景：MySQL 8.0 默认 collation 为 utf8mb4_0900_ai_ci，旧表/已有数据
-- 可能为 utf8mb4_unicode_ci，导致 works 与 likes/comments 之间的等值比较失败。

ALTER DATABASE graduation_exhibition CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

ALTER TABLE works
  CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

ALTER TABLE likes
  CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

ALTER TABLE comments
  CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

ALTER TABLE abuse_logs
  CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
