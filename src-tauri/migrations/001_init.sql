-- 环境配置表
CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    "group" TEXT NOT NULL DEFAULT '',
    remark TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'stopped' CHECK(status IN ('stopped', 'running')),
    fingerprint TEXT NOT NULL, -- JSON
    proxy TEXT, -- JSON, nullable
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_profiles_status ON profiles(status);
CREATE INDEX IF NOT EXISTS idx_profiles_group ON profiles("group");
CREATE INDEX IF NOT EXISTS idx_profiles_updated_at ON profiles(updated_at DESC);

-- 应用设置表
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 插入默认设置
INSERT OR IGNORE INTO settings (key, value) VALUES 
    ('kernel_path', ''),
    ('user_data_dir', ''),
    ('default_proxy', '');
