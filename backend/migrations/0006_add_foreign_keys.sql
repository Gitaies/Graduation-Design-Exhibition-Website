-- 0006_add_foreign_keys.sql
-- 添加外键约束保证数据完整性

-- 为 likes 表添加外键
ALTER TABLE likes
ADD CONSTRAINT fk_likes_work_id
FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE;

-- 为 comments 表添加外键
ALTER TABLE comments
ADD CONSTRAINT fk_comments_work_id
FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE;

-- 为 abuse_logs 表添加外键
ALTER TABLE abuse_logs
ADD CONSTRAINT fk_abuse_logs_work_id
FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE;
