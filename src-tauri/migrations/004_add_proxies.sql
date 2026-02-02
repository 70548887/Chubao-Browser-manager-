-- Migration 004: Add Proxies table
-- 将代理从 settings JSON 升级为独立数据库表

-- ============================================
-- 1. Proxies 表（代理列表）
-- ============================================
CREATE TABLE IF NOT EXISTS proxies (
    id TEXT PRIMARY KEY,                              -- UUID
    name TEXT NOT NULL,                               -- 代理名称
    type TEXT NOT NULL CHECK(type IN ('http', 'https', 'socks5', 'direct')),  -- 代理类型
    source TEXT NOT NULL DEFAULT 'custom',            -- 来源：custom/imported
    tag TEXT NOT NULL DEFAULT '',                     -- 标签/分类
    host TEXT NOT NULL,                               -- 主机地址
    port TEXT NOT NULL,                               -- 端口
    username TEXT,                                    -- 用户名（可选）
    password TEXT,                                    -- 密码（可选）
    ip_address TEXT,                                  -- 出口 IP（检测后填充）
    location TEXT,                                    -- 地理位置（检测后填充）
    used_count INTEGER NOT NULL DEFAULT 0,            -- 使用次数
    auto_check INTEGER NOT NULL DEFAULT 0,            -- 是否自动检测（0/1）
    expire_at TEXT,                                   -- 过期时间（RFC3339）
    bind_window TEXT,                                 -- 绑定窗口 ID
    remark TEXT NOT NULL DEFAULT '',                  -- 备注
    status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'expired', 'error')),  -- 状态
    created_at TEXT NOT NULL,                         -- 创建时间（RFC3339）
    updated_at TEXT NOT NULL                          -- 更新时间（RFC3339）
);

-- ============================================
-- 2. 索引
-- ============================================
CREATE INDEX IF NOT EXISTS idx_proxies_status ON proxies(status);
CREATE INDEX IF NOT EXISTS idx_proxies_type ON proxies(type);
CREATE INDEX IF NOT EXISTS idx_proxies_tag ON proxies(tag);
CREATE INDEX IF NOT EXISTS idx_proxies_updated_at ON proxies(updated_at);

-- ============================================
-- 3. 从 settings JSON 迁移数据（如果存在）
-- ============================================
-- 注意：这部分需要在 Rust 代码中实现，SQL 无法直接解析 JSON 数组
-- 迁移逻辑将在 ProxyService 初始化时执行
