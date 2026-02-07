import { ref, onMounted } from 'vue'

/**
 * @file useDarkMode.ts
 * @description 深色模式管理 composable
 * @author DeepAgent
 */

const isDark = ref(false)

export function useDarkMode() {
    const toggleDark = () => {
        isDark.value = !isDark.value
        if (isDark.value) {
            document.documentElement.classList.add('dark')
            localStorage.setItem('theme', 'dark')
        } else {
            document.documentElement.classList.remove('dark')
            localStorage.setItem('theme', 'light')
        }
    }

    const initDarkMode = () => {
        const savedTheme = localStorage.getItem('theme')

        // 新用户默认使用浅色模式，只有明确保存为 dark 时才启用深色模式
        isDark.value = savedTheme === 'dark'

        if (isDark.value) {
            document.documentElement.classList.add('dark')
        }
    }

    onMounted(() => {
        initDarkMode()
    })

    return {
        isDark,
        toggleDark,
        initDarkMode
    }
}
