-- 添加偏好设置表
CREATE TABLE IF NOT EXISTS profile_preferences (
    profile_id TEXT PRIMARY KEY NOT NULL,
    preferences TEXT NOT NULL, -- JSON
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

-- 为性能优化添加索引
CREATE INDEX IF NOT EXISTS idx_profile_preferences_profile_id ON profile_preferences(profile_id);