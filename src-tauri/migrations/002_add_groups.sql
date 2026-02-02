-- 创建分组表
CREATE TABLE IF NOT EXISTS groups (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    sort INTEGER NOT NULL DEFAULT 0,
    permission TEXT NOT NULL DEFAULT 'editable' CHECK(permission IN ('editable', 'readonly')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_groups_sort ON groups(sort ASC);
CREATE INDEX IF NOT EXISTS idx_groups_name ON groups(name);

-- 插入默认分组
INSERT OR IGNORE INTO groups (id, name, sort, permission) VALUES 
    ('default', '默认分组', 0, 'readonly');

-- 迁移现有 profiles 的 group 字段
-- 1. 为每个唯一的 group 名称创建对应的 group 记录
INSERT OR IGNORE INTO groups (id, name, sort, permission)
SELECT 
    lower(replace(replace("group", ' ', '_'), '-', '_')) as id,
    "group" as name,
    1 as sort,
    'editable' as permission
FROM profiles
WHERE "group" != '' AND "group" IS NOT NULL
GROUP BY "group";

-- 2. 添加新的 group_id 列到 profiles 表
ALTER TABLE profiles ADD COLUMN group_id TEXT;

-- 3. 迁移数据：将 group 名称映射为 group_id
UPDATE profiles
SET group_id = (
    SELECT id FROM groups WHERE groups.name = profiles."group"
)
WHERE "group" != '' AND "group" IS NOT NULL;

-- 4. 对于空 group 或未匹配的，设置为默认分组
UPDATE profiles
SET group_id = 'default'
WHERE group_id IS NULL OR group_id = '';

-- 5. 添加外键约束（通过重建表实现，因为 SQLite 不支持直接添加外键）
CREATE TABLE profiles_new (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    "group" TEXT NOT NULL DEFAULT '', -- 保留旧字段以兼容
    group_id TEXT NOT NULL DEFAULT 'default',
    remark TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'stopped' CHECK(status IN ('stopped', 'running')),
    fingerprint TEXT NOT NULL,
    proxy TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE RESTRICT
);

-- 复制数据
INSERT INTO profiles_new SELECT 
    id, name, "group", group_id, remark, status, fingerprint, proxy, created_at, updated_at
FROM profiles;

-- 删除旧表
DROP TABLE profiles;

-- 重命名新表
ALTER TABLE profiles_new RENAME TO profiles;

-- 重建索引
CREATE INDEX IF NOT EXISTS idx_profiles_status ON profiles(status);
CREATE INDEX IF NOT EXISTS idx_profiles_group ON profiles("group");
CREATE INDEX IF NOT EXISTS idx_profiles_group_id ON profiles(group_id);
CREATE INDEX IF NOT EXISTS idx_profiles_updated_at ON profiles(updated_at DESC);
