-- Migration 007: Add Extensions table
-- 扩展管理系统 - 采用共享目录模式
-- 设计参考: 统一完整开发方案-合并版/第一部分-系统架构/01-整体架构设计.md

-- ============================================
-- 1. Extensions 表（扩展列表）
-- ============================================
-- 扩展文件存放在全局共享目录: {app_data_dir}/Extensions/{extension_id}/
CREATE TABLE IF NOT EXISTS extensions (
    id TEXT PRIMARY KEY,                                   -- UUID
    name TEXT NOT NULL,                                    -- 扩展名称
    extension_id TEXT NOT NULL UNIQUE,                     -- Chrome扩展ID (如 nkbihfbeogaeaoehlefnkodbefgpgknn)
    version TEXT NOT NULL DEFAULT '',                      -- 版本号
    category TEXT NOT NULL DEFAULT '',                     -- 分类 (web3/adblock/productivity/social/developer)
    description TEXT NOT NULL DEFAULT '',                  -- 描述
    icon TEXT NOT NULL DEFAULT '',                         -- 图标 (Material Icon名 或 base64)
    source TEXT NOT NULL DEFAULT 'local' CHECK(source IN ('local', 'store', 'upload')),  -- 来源
    file_path TEXT NOT NULL,                               -- 扩展文件夹路径 (相对于 Extensions 目录)
    manifest_json TEXT NOT NULL DEFAULT '{}',              -- manifest.json 完整内容 (JSON)
    enabled INTEGER NOT NULL DEFAULT 1,                    -- 全局启用状态 (0/1)
    created_at TEXT NOT NULL,                              -- 创建时间 (RFC3339)
    updated_at TEXT NOT NULL                               -- 更新时间 (RFC3339)
);

-- ============================================
-- 2. Profile-Extension 关联表（多对多）
-- ============================================
-- 记录每个 Profile 启用了哪些扩展
CREATE TABLE IF NOT EXISTS profile_extensions (
    profile_id TEXT NOT NULL,                              -- Profile ID
    extension_id TEXT NOT NULL,                            -- 扩展ID (对应 extensions.id)
    enabled INTEGER NOT NULL DEFAULT 1,                    -- 在此 Profile 中是否启用
    created_at TEXT NOT NULL,                              -- 创建时间
    PRIMARY KEY (profile_id, extension_id),
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY (extension_id) REFERENCES extensions(id) ON DELETE CASCADE
);

-- ============================================
-- 3. 索引
-- ============================================
CREATE INDEX IF NOT EXISTS idx_extensions_extension_id ON extensions(extension_id);
CREATE INDEX IF NOT EXISTS idx_extensions_category ON extensions(category);
CREATE INDEX IF NOT EXISTS idx_extensions_enabled ON extensions(enabled);
CREATE INDEX IF NOT EXISTS idx_profile_extensions_profile ON profile_extensions(profile_id);
CREATE INDEX IF NOT EXISTS idx_profile_extensions_extension ON profile_extensions(extension_id);

-- ============================================
-- 4. 预置常用扩展元数据（可选，仅记录信息不实际安装）
-- ============================================
-- 这些是推荐扩展，用户点击"安装"后才实际下载
INSERT OR IGNORE INTO extensions (id, name, extension_id, version, category, description, icon, source, file_path, enabled, created_at, updated_at)
VALUES 
    ('preset-metamask', 'MetaMask', 'nkbihfbeogaeaoehlefnkodbefgpgknn', '', 'web3', '以太坊区块链交互的标准浏览器扩展', 'account_balance_wallet', 'store', '', 0, datetime('now'), datetime('now')),
    ('preset-ublock', 'uBlock Origin', 'cjpalhdlnbpafiamejdnhcphjbkeiagm', '', 'adblock', '高效的广告拦截器', 'block', 'store', '', 0, datetime('now'), datetime('now')),
    ('preset-translate', 'Google Translate', 'aapbdbdomjkkjkaonfhkkikfgjllcleb', '', 'productivity', '浏览网页时查看翻译', 'translate', 'store', '', 0, datetime('now'), datetime('now'));
