// 主题配置 - 支持暗黑模式和白天模式

export type ThemeMode = 'dark' | 'light'

export const ThemeConfig = {
    // 暗黑模式 (Deep Night Premium)
    dark: {
        bg: '#1a1b1e',
        surface: '#25262b',
        card: '#2c2e33',
        border: '#363b45',
        borderLight: '#4c4f57',

        text: {
            primary: '#e5eaf3',
            secondary: '#a3a6ad',
            placeholder: '#6e7177',
            disabled: '#4c4f57',
        },

        primary: {
            DEFAULT: '#409eff',
            hover: '#66b1ff',
            active: '#337ecc',
        },

        status: {
            running: '#67c23a',
            stopped: '#909399',
            error: '#f56c6c',
            warning: '#e6a23c',
        },
    },

    // 白天模式 (Light Premium)
    light: {
        bg: '#f5f7fa',
        surface: '#ffffff',
        card: '#ffffff',
        border: '#e4e7ed',
        borderLight: '#dcdfe6',

        text: {
            primary: '#303133',
            secondary: '#606266',
            placeholder: '#909399',
            disabled: '#c0c4cc',
        },

        primary: {
            DEFAULT: '#409eff',
            hover: '#66b1ff',
            active: '#337ecc',
        },

        status: {
            running: '#67c23a',
            stopped: '#909399',
            error: '#f56c6c',
            warning: '#e6a23c',
        },
    },

    // 间距 (通用)
    spacing: {
        xs: 4,
        sm: 8,
        md: 16,
        lg: 24,
        xl: 32,
    },

    // 圆角 (通用)
    borderRadius: {
        sm: 4,
        md: 6,
        lg: 8,
        xl: 12,
    },

    // 动画 (通用)
    transition: {
        fast: '0.15s ease',
        normal: '0.3s ease',
        slow: '0.5s ease',
    },
} as const

export type ThemeConfigType = typeof ThemeConfig
