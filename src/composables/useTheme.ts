/**
 * @file useTheme.ts
 * @description 主题管理 Composable
 */

import { ref, watch } from 'vue'
import type { ThemeMode } from '@/config'

const THEME_STORAGE_KEY = 'chubao-theme'

// 全局主题状态
const currentTheme = ref<ThemeMode>(
    (localStorage.getItem(THEME_STORAGE_KEY) as ThemeMode) || 'light'
)

/**
 * 应用主题到 DOM
 */
function applyTheme(theme: ThemeMode) {
    const root = document.documentElement

    // 移除旧主题类
    root.classList.remove('theme-dark', 'theme-light')

    // 添加新主题类
    root.classList.add(`theme-${theme}`)

    // 设置 data 属性 (用于 CSS 选择器)
    root.setAttribute('data-theme', theme)
}

/**
 * 主题管理 Hook
 */
export function useTheme() {
    /**
     * 切换主题
     */
    const toggleTheme = () => {
        currentTheme.value = currentTheme.value === 'dark' ? 'light' : 'dark'
    }

    /**
     * 设置主题
     */
    const setTheme = (theme: ThemeMode) => {
        currentTheme.value = theme
    }

    /**
     * 获取当前主题
     */
    const getTheme = () => currentTheme.value

    /**
     * 是否为暗黑模式
     */
    const isDark = () => currentTheme.value === 'dark'

    // 监听主题变化
    watch(currentTheme, (newTheme) => {
        applyTheme(newTheme)
        localStorage.setItem(THEME_STORAGE_KEY, newTheme)
    }, { immediate: true })

    return {
        theme: currentTheme,
        toggleTheme,
        setTheme,
        getTheme,
        isDark,
    }
}
