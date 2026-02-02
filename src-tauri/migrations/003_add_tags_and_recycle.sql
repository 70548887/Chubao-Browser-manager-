-- Migration 003: Add Tags and RecycleBin support

-- ============================================
-- 1. Tags 表（标签管理）
-- ============================================
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,              -- UUID
    name TEXT NOT NULL UNIQUE,        -- 标签名（唯一）
    sort INTEGER NOT NULL DEFAULT 0,  -- 排序
    created_at TEXT NOT NULL,         -- RFC3339
    updated_at TEXT NOT NULL          -- RFC3339
);

CREATE INDEX IF NOT EXISTS idx_tags_sort ON tags(sort);

-- ============================================
-- 2. Profile-Tags 关联表（多对多）
-- ============================================
CREATE TABLE IF NOT EXISTS profile_tags (
    profile_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL,         -- RFC3339
    PRIMARY KEY (profile_id, tag_id),
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_profile_tags_tag_id ON profile_tags(tag_id);
CREATE INDEX IF NOT EXISTS idx_profile_tags_profile_id ON profile_tags(profile_id);

-- ============================================
-- 3. RecycleBin 支持（软删除）
-- ============================================
-- 为 profiles 表添加 deleted_at 字段
-- SQLite 不支持 IF NOT EXISTS for ALTER TABLE，需要检查
-- 如果字段已存在，这条语句会失败但不影响后续操作
ALTER TABLE profiles ADD COLUMN deleted_at TEXT;

-- 为 deleted_at 创建索引（用于快速查询回收站）
CREATE INDEX IF NOT EXISTS idx_profiles_deleted_at ON profiles(deleted_at);

-- ============================================
-- 4. Proxy Templates 初始化
-- ============================================
-- 在 settings 表中初始化代理模板配置
INSERT OR IGNORE INTO settings (key, value) 
VALUES ('proxy_templates', '[]');

-- ============================================
-- 5. 数据完整性检查
-- ============================================
-- 确保所有现有 profiles 的 deleted_at 为 NULL（未删除状态）
-- 这条语句是幂等的，可以安全重复执行
UPDATE profiles SET deleted_at = NULL WHERE deleted_at IS NULL;
