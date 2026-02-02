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
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches

        isDark.value = savedTheme === 'dark' || (!savedTheme && prefersDark)

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
