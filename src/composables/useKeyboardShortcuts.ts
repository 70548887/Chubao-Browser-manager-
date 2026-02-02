/**
 * @file useKeyboardShortcuts.ts
 * @description 快捷键管理 Composable
 * @author DeepAgent
 */

import { onMounted, onUnmounted } from 'vue'

export interface KeyboardShortcut {
    /** 快捷键组合，如 'Ctrl+N', 'Ctrl+F', 'Delete' */
    keys: string
    /** 快捷键描述 */
    description: string
    /** 回调函数 */
    handler: (event: KeyboardEvent) => void
    /** 是否阻止默认行为 */
    preventDefault?: boolean
}

/**
 * 解析快捷键字符串为键盘事件匹配器
 */
function parseShortcut(keys: string) {
    const parts = keys.toLowerCase().split('+')
    return {
        ctrl: parts.includes('ctrl') || parts.includes('control'),
        alt: parts.includes('alt'),
        shift: parts.includes('shift'),
        meta: parts.includes('meta') || parts.includes('cmd'),
        key: parts[parts.length - 1],
    }
}

/**
 * 检查键盘事件是否匹配快捷键
 */
function matchesShortcut(event: KeyboardEvent, shortcut: ReturnType<typeof parseShortcut>) {
    return (
        event.ctrlKey === shortcut.ctrl &&
        event.altKey === shortcut.alt &&
        event.shiftKey === shortcut.shift &&
        event.metaKey === shortcut.meta &&
        event.key.toLowerCase() === shortcut.key
    )
}

/**
 * 使用快捷键
 * @param shortcuts - 快捷键配置数组
 */
export function useKeyboardShortcuts(shortcuts: KeyboardShortcut[]) {
    const handleKeyDown = (event: KeyboardEvent) => {
        // 如果焦点在输入框中，跳过部分快捷键
        const target = event.target as HTMLElement
        const isInputElement =
            target.tagName === 'INPUT' ||
            target.tagName === 'TEXTAREA' ||
            target.isContentEditable

        for (const shortcut of shortcuts) {
            const parsed = parseShortcut(shortcut.keys)

            if (matchesShortcut(event, parsed)) {
                // Ctrl+F 允许在输入框中使用
                if (isInputElement && shortcut.keys !== 'Ctrl+F') {
                    continue
                }

                if (shortcut.preventDefault !== false) {
                    event.preventDefault()
                }

                shortcut.handler(event)
                break
            }
        }
    }

    onMounted(() => {
        window.addEventListener('keydown', handleKeyDown)
    })

    onUnmounted(() => {
        window.removeEventListener('keydown', handleKeyDown)
    })

    return {
        shortcuts,
    }
}
