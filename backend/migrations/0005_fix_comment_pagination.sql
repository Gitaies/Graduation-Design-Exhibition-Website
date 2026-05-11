-- 0005_fix_comment_pagination.sql
-- 添加复合索引支持稳定的游标分页

-- 添加复合索引 (work_id, status, created_at DESC, id DESC)
-- 这样可以确保同一秒内的多条评论有稳定的排序
ALTER TABLE comments
ADD INDEX idx_work_status_created_id (work_id, status, created_at DESC, id DESC);

-- 可选：删除旧索引以节省空间（如果确认新索引覆盖了旧索引的查询场景）
-- ALTER TABLE comments DROP INDEX idx_work_created;
