/**
 * API 后台服务配置
 * 集中管理所有后台接口地址
 */

// 环境配置
const ENV = import.meta.env.MODE || 'development'

// API 基础地址配置
const API_BASE_URLS = {
    development: 'http://localhost:8081',      // 🔧 开发环境 Go 服务器地址
    production: 'http://96.126.191.43',        // 🔧 生产环境 Go 服务器地址
    staging: 'https://api-staging.qutab.cn',   // 🔧 测试环境 Go 服务器地址
} as const

// 获取当前环境的 API 基础地址
export const API_BASE_URL = API_BASE_URLS[ENV as keyof typeof API_BASE_URLS] || API_BASE_URLS.development

/**
 * API 端点配置
 */
export const ApiEndpoints = {
    // ==================== 用户认证相关 ====================
    AUTH: {
        // 用户登录
        LOGIN: `${API_BASE_URL}/api/v1/auth/login`,
        // 用户注册
        REGISTER: `${API_BASE_URL}/api/v1/auth/register`,
        // 用户登出
        LOGOUT: `${API_BASE_URL}/api/v1/auth/logout`,
        // 刷新 Token
        REFRESH_TOKEN: `${API_BASE_URL}/api/v1/auth/refresh`,
        // 获取用户信息
        USER_INFO: `${API_BASE_URL}/api/v1/auth/user`,
        // 修改密码
        CHANGE_PASSWORD: `${API_BASE_URL}/api/v1/auth/password`,
        // 找回密码
        RESET_PASSWORD: `${API_BASE_URL}/api/v1/auth/reset-password`,
    },

    // ==================== 应用更新相关 ====================
    UPDATE: {
        // 检查启动器更新
        CHECK_LAUNCHER: `${API_BASE_URL}/api/v1/updates/launcher`,
        // 检查内核更新
        CHECK_KERNEL: `${API_BASE_URL}/api/v1/updates/kernel`,
        // 获取更新日志
        CHANGELOG: `${API_BASE_URL}/api/v1/updates/changelog`,
    },

    // ==================== 内核下载相关 ====================
    KERNEL: {
        // 获取内核下载信息
        DOWNLOAD_INFO: `${API_BASE_URL}/api/v1/kernel/download-info`,
        // 获取内核版本列表
        VERSION_LIST: `${API_BASE_URL}/api/v1/kernel/versions`,
        // 报告内核安装状态
        REPORT_INSTALL: `${API_BASE_URL}/api/v1/kernel/install-report`,
    },

    // ==================== 消息通知相关 ====================
    NOTIFICATION: {
        // 获取消息列表
        LIST: `${API_BASE_URL}/api/v1/notifications`,
        // 标记消息已读
        MARK_READ: `${API_BASE_URL}/api/v1/notifications/read`,
        // 获取未读数量
        UNREAD_COUNT: `${API_BASE_URL}/api/v1/notifications/unread-count`,
        // WebSocket 连接地址
        WEBSOCKET: `${API_BASE_URL.replace('http', 'ws')}/ws/notifications`,
    },

    // ==================== 许可证相关 ====================
    LICENSE: {
        // 验证许可证
        VALIDATE: `${API_BASE_URL}/api/v1/license/validate`,
        // 获取许可证信息
        INFO: `${API_BASE_URL}/api/v1/license/info`,
        // 激活许可证
        ACTIVATE: `${API_BASE_URL}/api/v1/license/activate`,
    },

    // ==================== 数据同步相关 ====================
    SYNC: {
        // 上传配置文件备份
        UPLOAD_BACKUP: `${API_BASE_URL}/api/v1/sync/backup`,
        // 下载配置文件备份
        DOWNLOAD_BACKUP: `${API_BASE_URL}/api/v1/sync/backup`,
        // 获取备份列表
        BACKUP_LIST: `${API_BASE_URL}/api/v1/sync/backups`,
    },

    // ==================== 统计上报相关 ====================
    ANALYTICS: {
        // 上报应用启动
        APP_START: `${API_BASE_URL}/api/v1/analytics/app-start`,
        // 上报崩溃日志
        CRASH_REPORT: `${API_BASE_URL}/api/v1/analytics/crash`,
        // 上报使用统计
        USAGE_STATS: `${API_BASE_URL}/api/v1/analytics/usage`,
    },

    // ==================== 反馈支持相关 ====================
    SUPPORT: {
        // 提交反馈
        FEEDBACK: `${API_BASE_URL}/api/v1/support/feedback`,
        // 获取帮助文档
        DOCS: `${API_BASE_URL}/api/v1/support/docs`,
        // 获取常见问题
        FAQ: `${API_BASE_URL}/api/v1/support/faq`,
    },
} as const

/**
 * API 请求配置
 */
export const ApiConfig = {
    // 请求超时时间 (毫秒)
    TIMEOUT: 30000,

    // 重试配置
    RETRY: {
        // 最大重试次数
        MAX_ATTEMPTS: 3,
        // 重试延迟 (毫秒)
        DELAY: 1000,
        // 需要重试的 HTTP 状态码
        RETRY_STATUS_CODES: [408, 429, 500, 502, 503, 504],
    },

    // 请求头配置
    HEADERS: {
        'Content-Type': 'application/json',
        'X-Client-Version': '0.3.0',
        'X-Client-Platform': 'windows',
    },
} as const

// ==================== 降级/备用配置 ====================

/**
 * 默认内核下载地址（当 API 不可用时的降级方案）
 */
export const FALLBACK_KERNEL_URL = 'https://github.com/user/repo/releases/download/latest/chromium-kernel-win64-v146.zip'

/**
 * IP 地理位置查询 API（第三方服务）
 */
export const IP_GEO_API_URL = 'http://ip-api.com/json'

// ==================== 工具函数 ====================

/**
 * 构建完整的 API URL
 * @param endpoint 端点路径
 * @param params 查询参数
 */
export function buildApiUrl(endpoint: string, params?: Record<string, any>): string {
    const url = new URL(endpoint)
    
    if (params) {
        Object.entries(params).forEach(([key, value]) => {
            if (value !== undefined && value !== null) {
                url.searchParams.append(key, String(value))
            }
        })
    }
    
    return url.toString()
}

/**
 * 获取 WebSocket URL
 */
export function getWebSocketUrl(path: string): string {
    const wsProtocol = API_BASE_URL.startsWith('https') ? 'wss' : 'ws'
    return `${wsProtocol}://${API_BASE_URL.replace(/^https?:\/\//, '')}${path}`
}

export type ApiEndpointsType = typeof ApiEndpoints
