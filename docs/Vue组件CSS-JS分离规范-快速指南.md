# Vue 组件 CSS 和 JS 分离规范 - 快速指南

> **更新日期**: 2026-01-27  
> **适用项目**: 触宝指纹浏览器 (browser-manager)  
> **规范版本**: v1.0

---

## 📋 目录

1. [为什么要分离](#为什么要分离)
2. [分离标准](#分离标准)
3. [文件组织](#文件组织)
4. [重构步骤](#重构步骤)
5. [代码示例](#代码示例)
6. [常见问题](#常见问题)

---

## 🎯 为什么要分离

### 问题背景

```vue
<!-- ❌ 500+ 行的单文件组件 - 难以维护 -->
<script setup lang="ts">
// 100+ 行逻辑代码
// ...
</script>

<template>
  <!-- 200+ 行模板 -->
</template>

<style scoped lang="scss">
// 200+ 行样式
// ...
</style>
```

### 分离后的优势

| 优势 | 说明 |
|------|------|
| **提高可维护性** | 单一职责，逻辑、样式、模板分离 |
| **便于团队协作** | 前端开发、UI 设计师可并行工作 |
| **代码复用** | 逻辑层可在多个组件间共享 |
| **易于测试** | 独立的逻辑文件更容易单元测试 |
| **减少合并冲突** | Git 合并冲突大幅减少 |

---

## 📊 分离标准

### 三级分类标准

| 复杂度 | 样式行数 | 逻辑行数 | 处理方式 | 示例组件 |
|--------|---------|---------|----------|----------|
| **简单** | < 50 | < 100 | 保持单文件 | `StatusDot.vue` |
| **中等** | 50-150 | 100-300 | 仅分离样式 | `ProfileCard.vue` |
| **复杂** | > 150 | > 300 | **样式+逻辑都分离** | `DashboardView.vue` |

### 判断规则

```bash
# 快速判断脚本
行数=$(wc -l < Component.vue)
if [ $行数 -gt 300 ]; then
  echo "建议分离 CSS 和 JS"
elif [ $行数 -gt 150 ]; then
  echo "建议分离 CSS"
else
  echo "可保持单文件"
fi
```

---

## 📁 文件组织

### 标准目录结构

```
features/dashboard/
├── DashboardView.vue          # 模板层 (Template)
├── DashboardView.ts           # 逻辑层 (Logic)
├── DashboardView.scss         # 样式层 (Style)
├── components/
│   ├── ProfileRow.vue
│   ├── ProfileRow.ts          # 复杂组件分离逻辑
│   ├── ProfileRow.scss        # 复杂组件分离样式
│   └── ListHeader.vue         # 简单组件不分离
└── composables/
    └── useProfileForm.ts      # 可复用逻辑
```

### 命名规范

| 文件类型 | 命名规则 | 示例 |
|---------|---------|------|
| Vue 组件 | `PascalCase.vue` | `ProfileCard.vue` |
| 逻辑文件 | `PascalCase.ts` | `ProfileCard.ts` |
| 样式文件 | `PascalCase.scss` | `ProfileCard.scss` |

---

## 🔄 重构步骤

### Step 1: 提取逻辑层

```typescript
// ProfileCard.ts - 提取所有响应式逻辑
import { ref, computed } from 'vue'
import type { Profile } from '@/types'

// 1. 导出状态
export const isExpanded = ref(false)

// 2. 导出计算属性
export const statusClass = computed(() => ({
  running: profile.status === 'running',
  stopped: profile.status === 'stopped',
}))

// 3. 导出方法
export const handleToggle = () => {
  isExpanded.value = !isExpanded.value
}

// 4. 导出生命周期（如需要）
export const initComponent = () => {
  onMounted(() => {
    // 初始化逻辑
  })
}
```

### Step 2: 提取样式层

```scss
// ProfileCard.scss - 提取所有样式
.profile-card {
  padding: 16px;
  background-color: var(--color-card);
  border-radius: 8px;
  transition: all 0.3s ease;
  
  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
  
  &.running {
    border-left: 3px solid var(--color-status-running);
  }
  
  &__header {
    display: flex;
    justify-content: space-between;
    
    h3 {
      font-size: 16px;
      font-weight: 600;
    }
  }
}
```

### Step 3: 简化 Vue 文件

```vue
<!-- ProfileCard.vue - 只保留模板和引用 -->
<script setup lang="ts">
// 导入逻辑
import {
  isExpanded,
  statusClass,
  handleToggle,
  initComponent
} from './ProfileCard'

// 导入类型（如需要）
import type { Profile } from '@/types'

// Props
interface Props {
  profile: Profile
}
const props = defineProps<Props>()

// 初始化
initComponent()
</script>

<template>
  <div class="profile-card" :class="statusClass">
    <div class="profile-card__header">
      <h3>{{ profile.name }}</h3>
      <button @click="handleToggle">展开</button>
    </div>
  </div>
</template>

<!-- 引用外部样式 -->
<style scoped lang="scss" src="./ProfileCard.scss" />
```

---

## 💡 代码示例

### 示例 1: 仅分离样式（中等复杂度）

```vue
<!-- ProfileCard.vue -->
<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Profile } from '@/types'

interface Props {
  profile: Profile
}
const props = defineProps<Props>()

const isExpanded = ref(false)
const statusClass = computed(() => ({
  running: props.profile.status === 'running',
}))
</script>

<template>
  <div class="profile-card" :class="statusClass">
    <!-- 内容 -->
  </div>
</template>

<style scoped lang="scss" src="./ProfileCard.scss" />
```

### 示例 2: 逻辑+样式都分离（高复杂度）

#### 方式 A: 导入方式（推荐）

```vue
<!-- DashboardView.vue -->
<script setup lang="ts">
import {
  profiles,
  isLoading,
  handleLaunch,
  handleBatchStop,
  initDashboard
} from './DashboardView'

initDashboard()
</script>

<template>
  <div class="dashboard-view">
    <!-- 模板内容 -->
  </div>
</template>

<style scoped lang="scss" src="./DashboardView.scss" />
```

#### 方式 B: src 属性方式

```vue
<!-- DashboardView.vue -->
<script setup lang="ts" src="./DashboardView.ts" />

<template>
  <div class="dashboard-view">
    <!-- 模板内容 -->
  </div>
</template>

<style scoped lang="scss" src="./DashboardView.scss" />
```

---

## 🚫 禁止事项

| ❌ 禁止 | ✅ 正确做法 | 原因 |
|--------|------------|------|
| 500+ 行单文件组件 | 拆分成 `.ts` + `.scss` | 可维护性 |
| 内联样式 `style="color: red"` | 使用 class + CSS 变量 | 主题切换 |
| 硬编码颜色 `#409eff` | `var(--color-primary)` | 统一管理 |
| 深层嵌套 SCSS (> 4层) | BEM 扁平化 | 性能优化 |
| 逻辑和样式混在一起 | 严格分层 | 职责清晰 |

---

## ❓ 常见问题

### Q1: 什么时候必须分离？

**A**: 页面级组件（`features/*/`）必须分离。超过 300 行的组件建议分离。

### Q2: 分离后如何共享状态？

**A**: 使用 Pinia Store 或 Composables：

```typescript
// composables/useProfileForm.ts
export function useProfileForm() {
  const form = ref({})
  const validate = () => {}
  return { form, validate }
}
```

### Q3: CSS 变量在哪里定义？

**A**: 在 `src/assets/styles/main.scss` 中定义全局 CSS 变量：

```scss
:root {
  --color-primary: #409eff;
  --color-surface: #25262b;
}
```

### Q4: 如何处理组件 Props？

**A**: Props 定义保留在 `.vue` 文件中，逻辑层通过参数接收：

```vue
<!-- Component.vue -->
<script setup lang="ts">
import { handleAction } from './Component'

interface Props {
  data: Data
}
const props = defineProps<Props>()

// 传递给逻辑层
const onClick = () => handleAction(props.data)
</script>
```

### Q5: TypeScript 类型如何组织？

**A**: 类型定义统一放在 `src/types/` 目录：

```typescript
// types/profile.types.ts
export interface Profile {
  id: string
  name: string
  status: 'running' | 'stopped'
}
```

---

## 📚 参考资料

- [项目架构规范](../项目架构规范.md) - 完整规范文档
- [DashboardView.refactored.vue](../../src/features/dashboard/DashboardView.refactored.vue) - 重构示例
- [Vue 3 文档](https://cn.vuejs.org/) - 官方文档

---

## 🎓 最佳实践总结

1. **渐进式重构**: 从最复杂的页面组件开始分离
2. **保持一致性**: 同一功能模块采用相同的分离策略
3. **注释说明**: 在文件头部添加 `@file` 和 `@description` 注释
4. **分组组织**: 按"状态-计算属性-方法-生命周期"顺序组织代码
5. **使用 BEM**: 样式采用 BEM 命名规范，避免深层嵌套

---

**最后更新**: 2026-01-27 by DeepAgent
