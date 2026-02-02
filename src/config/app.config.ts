// 应用配置 - 禁止硬编码
// 所有常量都应该定义在这里

export const AppConfig = {
    // 应用信息
    APP_NAME: '触宝指纹浏览器',
    APP_VERSION: '1.0.0',

    // 路径配置 (相对路径，运行时解析)
    CACHE_DIR_NAME: 'chubao-cache',
    DATABASE_NAME: 'profiles.db',

    // 性能配置
    MAX_CONCURRENT_BROWSERS: 50,
    VIRTUAL_SCROLL_THRESHOLD: 100,

    // UI 配置
    SIDEBAR_WIDTH: 240,
    SIDEBAR_COLLAPSED_WIDTH: 64,
    HEADER_HEIGHT: 60,

    // 超时配置 (毫秒)
    API_TIMEOUT: 10000,
    BROWSER_LAUNCH_TIMEOUT: 30000,
    PROXY_CHECK_TIMEOUT: 5000,
} as const

export type AppConfigType = typeof AppConfig
