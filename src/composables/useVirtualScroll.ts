/**
 * @file useVirtualScroll.ts
 * @description 虚拟滚动 Composable - 优化大数据量列表性能
 * @author DeepAgent
 */

import { computed, Ref } from 'vue'

export interface VirtualScrollOptions {
    /**
     * 每项的高度（像素）
     * @default 60
     */
    itemHeight?: number

    /**
     * 缓冲区大小（渲染可见区域外的额外项数）
     * @default 5
     */
    buffer?: number
}

/**
 * 虚拟滚动配置
 * @param items - 数据列表
 * @param options - 配置选项
 */
export function useVirtualScroll<T>(
    items: Ref<T[]>,
    options: VirtualScrollOptions = {}
) {
    const { itemHeight = 60, buffer = 5 } = options

    /**
     * 估算列表总高度
     */
    const estimatedSize = computed(() => items.value.length * itemHeight)

    /**
     * 虚拟滚动配置对象（传递给 RecycleScroller）
     */
    const scrollerConfig = computed(() => ({
        items: items.value,
        itemSize: itemHeight,
        buffer,
        keyField: 'id' as const,
    }))

    return {
        estimatedSize,
        scrollerConfig,
        itemHeight,
        buffer,
    }
}
