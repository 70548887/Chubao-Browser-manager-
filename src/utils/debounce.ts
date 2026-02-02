/**
 * @file debounce.ts
 * @description 防抖和节流工具函数
 * @author DeepAgent
 */

/**
 * 防抖函数 - 延迟执行，在连续触发时只执行最后一次
 * @param fn - 要防抖的函数
 * @param delay - 延迟时间（毫秒）
 * @returns 防抖后的函数
 */
export function debounce<T extends (...args: any[]) => any>(
    fn: T,
    delay: number = 300
): (...args: Parameters<T>) => void {
    let timeoutId: ReturnType<typeof setTimeout> | null = null

    return function (this: any, ...args: Parameters<T>) {
        if (timeoutId) {
            clearTimeout(timeoutId)
        }

        timeoutId = setTimeout(() => {
            fn.apply(this, args)
            timeoutId = null
        }, delay)
    }
}

/**
 * 节流函数 - 固定时间间隔内只执行一次
 * @param fn - 要节流的函数
 * @param limit - 时间间隔（毫秒）
 * @returns 节流后的函数
 */
export function throttle<T extends (...args: any[]) => any>(
    fn: T,
    limit: number = 300
): (...args: Parameters<T>) => void {
    let inThrottle = false

    return function (this: any, ...args: Parameters<T>) {
        if (!inThrottle) {
            fn.apply(this, args)
            inThrottle = true
            setTimeout(() => {
                inThrottle = false
            }, limit)
        }
    }
}
