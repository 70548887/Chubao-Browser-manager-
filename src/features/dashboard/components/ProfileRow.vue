<script setup lang="ts">
/**
 * @description 环境列表行组件 - 完全对齐原型
 * @author DeepAgent
 */

import type { Profile } from '@/types'
import { ElMessage } from 'element-plus'

interface Props {
  profile: Profile
  index: number
  isSelected?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isSelected: false,
})

const emit = defineEmits<{
  (e: 'select', profile: Profile): void
  (e: 'launch', id: string): void
  (e: 'stop', id: string): void
  (e: 'edit', profile: Profile): void
  (e: 'delete', id: string): void
  (e: 'clone', id: string): void
  (e: 'export', id: string): void
  (e: 'exportCookie', id: string): void
  (e: 'clearCache', id: string): void
}>()

// 复制 ID 到剪切板
const copyId = async () => {
  try {
    await navigator.clipboard.writeText(props.profile.id)
    ElMessage.success('ID 已复制')
  } catch {
    ElMessage.error('复制失败')
  }
}

// 格式化时间
const formatDate = (timestamp: number) => {
  const now = Date.now()
  const diff = now - timestamp
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  return `${days}天前`
}

// 获取头像缩写和颜色
const getAvatarInfo = (name: string) => {
  const colors = [
    { bg: '#FED7AA', text: '#EA580C' }, // orange
    { bg: '#BFDBFE', text: '#2563EB' }, // blue
    { bg: '#E9D5FF', text: '#9333EA' }, // purple
    { bg: '#CCFBF1', text: '#0D9488' }, // teal
    { bg: '#FEF08A', text: '#CA8A04' }, // yellow
    { bg: '#FBCFE8', text: '#DB2777' }, // pink
  ]
  
  // 根据名称生成稳定的颜色索引
  const index = name.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0) % colors.length
  
  // 提取首字母缩写（最多2个字符）
  const words = name.split(/[-_\s]/)
  const initials = words.slice(0, 2).map(w => w[0]?.toUpperCase() || '').join('')
  
  return {
    initials: initials || name.substring(0, 2).toUpperCase(),
    ...colors[index]
  }
}

// 获取系统平台图标和标签
const getSystemPlatform = (profile: Profile) => {
  // 从指纹配置中获取平台信息
  const platform = (profile.fingerprint as any)?.platform || 'windows'
  
  switch (platform.toLowerCase()) {
    case 'windows':
      return { type: 'windows', label: 'Windows' }
    case 'macos':
      return { type: 'macos', label: 'macOS' }
    case 'android':
      return { type: 'android', label: 'Android' }
    case 'ios':
      return { type: 'ios', label: 'iOS' }
    case 'linux':
      return { type: 'linux', label: 'Linux' }
    default:
      return { type: 'windows', label: 'Windows' }
  }
}

const handleAction = (action: string) => {
  switch (action) {
    case 'launch':
      emit('launch', props.profile.id)
      break
    case 'stop':
      emit('stop', props.profile.id)
      break
    case 'edit':
      emit('edit', props.profile)
      break
    case 'delete':
      emit('delete', props.profile.id)
      break
    case 'clone':
      emit('clone', props.profile.id)
      break
    case 'export':
      emit('export', props.profile.id)
      break
    case 'exportCookie':
      emit('exportCookie', props.profile.id)
      break
    case 'clearCache':
      emit('clearCache', props.profile.id)
      break
  }
}
</script>

<template>
  <div 
    class="profile-row"
    :class="{ selected: isSelected, running: profile.status === 'running' }"
    @click="emit('select', profile)"
  >
    <!-- 选择框 -->
    <div class="col-checkbox" @click.stop>
      <input 
        type="checkbox" 
        :checked="isSelected"
        class="checkbox-input"
        @change="emit('select', profile)"
      />
    </div>
    
    <!-- 序号 -->
    <div class="col-index">
      <span class="index-num">{{ index }}</span>
    </div>
    
    <!-- 状态标签 -->
    <div class="col-status">
      <span v-if="profile.status === 'running'" class="status-tag status-running">
        <span class="status-dot"></span>
        运行中
      </span>
      <span v-else class="status-tag status-stopped">
        <span class="status-dot"></span>
        已停止
      </span>
    </div>
    
    <!-- 环境名称 / ID -->
    <div class="col-name">
      <div class="name-cell">
        <div 
          class="avatar"
          :style="{ 
            backgroundColor: getAvatarInfo(profile.name).bg,
            color: getAvatarInfo(profile.name).text
          }"
        >
          {{ getAvatarInfo(profile.name).initials }}
        </div>
        <div class="name-info">
          <p class="profile-name">{{ profile.name }}</p>
          <p class="profile-id" @click.stop="copyId" title="点击复制完整 ID">ID: {{ profile.id.slice(0, 4) }}...{{ profile.id.slice(-4) }}</p>
        </div>
      </div>
    </div>
    
    <!-- 分组 -->
    <div class="col-group">
      <span class="group-tag">
        {{ profile.group || '默认分组' }}
      </span>
    </div>
    
    <!-- 代理信息 -->
    <div class="col-proxy">
      <div class="proxy-info">
        <span class="proxy-host">
          <span class="material-symbols-outlined proxy-icon">{{ profile.proxy ? 'public' : 'language' }}</span>
          {{ profile.proxy ? `${profile.proxy.host}` : '直连模式' }}
        </span>
        <span class="proxy-detail" v-if="profile.proxy">{{ profile.proxy.type?.toUpperCase() || 'SOCKS5' }} · {{ profile.proxy.port }}</span>
      </div>
    </div>
    
    <!-- 平台（系统类型） -->
    <div class="col-platform">
      <!-- Windows 图标 -->
      <svg v-if="getSystemPlatform(profile).type === 'windows'" class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
        <path d="M0 3.449L9.75 2.1V11.719H0V3.449ZM0 12.281V20.551L9.75 19.2V12.281H0ZM10.547 1.983L24 0V11.719H10.547V1.983ZM10.547 12.281V19.017L24 20.893V12.281H10.547Z"/>
      </svg>
      <!-- macOS 图标 -->
      <svg v-else-if="getSystemPlatform(profile).type === 'macos'" class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
        <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.1 2.48-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.31-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.36 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
      </svg>
      <!-- Android 图标 -->
      <svg v-else-if="getSystemPlatform(profile).type === 'android'" class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
        <path d="M17.523 15.341c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m-11.046 0c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m11.4-6.039l1.94-3.36c.11-.19.04-.43-.15-.54-.19-.11-.43-.04-.54.15l-1.96 3.4c-1.45-.66-3.08-1.03-4.82-1.03s-3.37.37-4.82 1.03l-1.96-3.4c-.11-.19-.35-.26-.54-.15-.19.11-.26.35-.15.54l1.94 3.36C3.03 10.94 0 14.81 0 19.34h24c0-4.53-3.03-8.4-7.12-10.04"/>
      </svg>
      <!-- iOS 图标 -->
      <svg v-else-if="getSystemPlatform(profile).type === 'ios'" class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
        <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.1 2.48-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.31-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.36 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
      </svg>
      <!-- Linux 图标 -->
      <svg v-else class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12.504 0c-.155 0-.315.008-.48.021-4.226.333-3.105 4.807-3.17 6.298-.076 1.092-.3 1.953-1.05 3.02-.885 1.051-2.127 2.75-2.716 4.521-.278.832-.41 1.684-.287 2.489a.424.424 0 00-.11.135c-.26.268-.45.6-.663.839-.199.199-.485.267-.797.4-.313.136-.658.269-.864.68-.09.189-.136.394-.132.602 0 .199.027.4.055.536.058.399.116.728.04.97-.249.68-.28 1.145-.106 1.484.174.334.535.47.94.601.81.2 1.91.135 2.774.6.926.466 1.866.67 2.616.47.526-.116.97-.464 1.208-.946.587-.003 1.23-.269 2.26-.334.699-.058 1.574.267 2.577.2.025.134.063.198.114.333l.003.003c.391.778 1.113 1.132 1.884 1.071.771-.06 1.592-.536 2.257-1.306.631-.765 1.683-1.084 2.378-1.503.348-.199.629-.469.649-.853.023-.4-.2-.811-.714-1.376v-.097l-.003-.003c-.17-.2-.25-.535-.338-.926-.085-.401-.182-.786-.492-1.046h-.003c-.059-.054-.123-.067-.188-.135a.357.357 0 00-.19-.064c.431-1.278.264-2.55-.173-3.694-.533-1.41-1.465-2.638-2.175-3.483-.796-1.005-1.576-1.957-1.56-3.368.026-2.152.236-6.133-3.544-6.139zm.529 3.405h.013c.213 0 .396.062.584.198.19.135.33.332.438.533.105.259.158.459.166.724 0-.02.006-.04.006-.06v.105a.086.086 0 01-.004-.021l-.004-.024a1.807 1.807 0 01-.15.706.953.953 0 01-.213.335.71.71 0 00-.088-.042c-.104-.045-.198-.064-.284-.133a1.312 1.312 0 00-.22-.066c.05-.06.146-.133.183-.198.053-.128.082-.264.088-.402v-.02a1.21 1.21 0 00-.061-.4c-.045-.134-.101-.2-.183-.333-.084-.066-.167-.132-.267-.132h-.016c-.093 0-.176.03-.262.132a.8.8 0 00-.205.334 1.18 1.18 0 00-.09.4v.019c.002.089.008.179.02.267-.193-.067-.438-.135-.607-.202a1.635 1.635 0 01-.018-.2v-.02a1.772 1.772 0 01.15-.768c.082-.22.232-.406.43-.534a.985.985 0 01.594-.2zm-2.962.059h.036c.142 0 .27.048.399.135.146.129.264.288.344.465.09.199.14.4.153.667v.004c.007.134.006.2-.002.266v.08c-.03.007-.056.018-.083.024-.152.055-.274.135-.393.2.012-.09.013-.18.003-.267v-.015c-.012-.133-.04-.2-.082-.333a.613.613 0 00-.166-.267.248.248 0 00-.183-.064h-.021c-.071.006-.13.04-.186.132a.552.552 0 00-.12.27.944.944 0 00-.023.33v.015c.012.135.037.2.08.334.046.134.098.2.166.268.01.009.02.018.034.024-.07.057-.117.07-.176.136a.304.304 0 01-.131.068 2.62 2.62 0 01-.275-.402 1.772 1.772 0 01-.155-.667 1.759 1.759 0 01.08-.668 1.43 1.43 0 01.283-.535c.128-.133.26-.2.418-.2zm1.37 1.706c.332 0 .733.065 1.216.399.293.2.523.269 1.052.468h.003c.255.136.405.266.478.399v-.131a.571.571 0 01.016.47c-.123.31-.516.643-1.063.842v.002c-.268.135-.501.333-.775.465-.276.135-.588.292-1.012.267a1.139 1.139 0 01-.448-.067 3.566 3.566 0 01-.322-.198c-.195-.135-.363-.332-.612-.465v-.005h-.005c-.4-.246-.616-.512-.686-.71-.07-.268-.005-.47.193-.6.224-.135.38-.271.483-.336.104-.074.143-.102.176-.131h.002v-.003c.169-.202.436-.47.839-.601.139-.036.294-.065.466-.065zm2.8 2.142c.358 1.417 1.196 3.475 1.735 4.473.286.534.855 1.659 1.102 3.024.156-.005.33.018.513.064.646-1.671-.546-3.467-1.089-3.966-.22-.2-.232-.335-.123-.335.59.534 1.365 1.572 1.646 2.757.13.535.16 1.104.021 1.67.067.028.135.06.205.067 1.032.534 1.413.938 1.23 1.537v-.002c-.06-.135-.12-.2-.184-.268-.193-.135-.406-.199-.603-.336-.19-.135-.367-.332-.534-.468-.19-.135-.38-.271-.59-.336a.927.927 0 00-.36-.064c-.12 0-.238.015-.357.06a1.05 1.05 0 01-.12.054c-.147.057-.305.134-.39.333-.043.099-.067.201-.072.303-.046-.06-.096-.108-.16-.159-.1-.135-.208-.27-.328-.336a.51.51 0 00-.246-.065c-.09 0-.181.016-.267.067-.082.034-.164.135-.164.268 0 .2.065.466.164.668.098.27.262.601.514.869a.974.974 0 01-.197-.135c-.267-.2-.455-.47-.585-.804l.003.003c-.06-.135-.104-.264-.15-.4-.12-.334-.18-.668-.214-.936-.016-.135-.022-.268-.022-.4 0-.535.084-1.005.199-1.407-.073-.266-.145-.599-.212-1.003-.123-.733-.143-1.668.147-2.535-.48.266-.868.599-1.121 1.003-.57.802-.788 1.87-.588 3.07.064.4.156.8.259 1.137.084.268.157.468.212.601.072.267.118.401.116.535-.007.267-.08.466-.2.6a.641.641 0 01-.348.2 1.046 1.046 0 01-.4.006 1.82 1.82 0 01-.466-.135c-.267-.2-.4-.401-.467-.668a1.727 1.727 0 01-.06-.4c-.018-.135-.018-.268-.023-.4-.023-.401-.057-.802-.156-1.136-.074-.268-.169-.535-.295-.802-.053-.135-.111-.269-.178-.4-.11-.2-.227-.4-.356-.6a9.752 9.752 0 00-.53-.733c-.2-.268-.4-.535-.586-.735a24.91 24.91 0 01-.372-.4c-.082-.134-.139-.268-.177-.4a1.53 1.53 0 01-.034-.4c.007-.267.077-.535.209-.802.264-.534.68-1.003 1.165-1.337.484-.333 1.03-.535 1.539-.6.29-.033.59-.033.857.033.084.023.168.05.249.084v-.003c-.082.265-.191.537-.33.807-.17.267-.4.535-.686.8-.284.268-.618.469-1 .601a1.416 1.416 0 01-.5.133c-.127.01-.236-.017-.353-.033-.114-.017-.227-.058-.34-.1-.11-.042-.216-.1-.323-.15l.003.003z"/>
      </svg>
      <span class="platform-label">{{ getSystemPlatform(profile).label }}</span>
    </div>
    
    <!-- 最近更新 -->
    <div class="col-time">
      <span class="time-text">{{ formatDate(profile.updatedAt) }}</span>
    </div>
    
    <!-- 操作 -->
    <div class="col-actions" @click.stop>
      <!-- 启动/停止按钮 -->
      <button 
        v-if="profile.status === 'running'"
        class="action-btn action-stop" 
        title="停止"
        @click="handleAction('stop')"
      >
        <span class="material-symbols-outlined">stop_circle</span>
      </button>
      <button 
        v-else
        class="action-btn action-start" 
        title="启动"
        @click="handleAction('launch')"
      >
        <span class="material-symbols-outlined">play_circle</span>
      </button>
      
      <!-- 编辑按钮 -->
      <button class="action-btn" title="编辑" @click="handleAction('edit')">
        <span class="material-symbols-outlined">edit</span>
      </button>
      
      <!-- 更多操作下拉菜单 -->
      <el-dropdown trigger="click" @command="handleAction">
        <button class="action-btn action-more" title="更多操作">
          <span class="material-symbols-outlined">more_vert</span>
        </button>
        <template #dropdown>
          <el-dropdown-menu class="profile-action-menu">
            <el-dropdown-item command="clone">
              <span class="material-symbols-outlined menu-icon">content_copy</span>
              <span>克隆窗口</span>
            </el-dropdown-item>
            <el-dropdown-item command="export">
              <span class="material-symbols-outlined menu-icon">download</span>
              <span>导出窗口</span>
            </el-dropdown-item>
            <el-dropdown-item command="exportCookie">
              <span class="material-symbols-outlined menu-icon">cookie</span>
              <span>导出 Cookie</span>
            </el-dropdown-item>
            <el-dropdown-item divided command="clearCache">
              <span class="material-symbols-outlined menu-icon">cleaning_services</span>
              <span>清空窗口缓存</span>
            </el-dropdown-item>
            <el-dropdown-item command="delete" class="danger-item">
              <span class="material-symbols-outlined menu-icon">delete</span>
              <span>删除窗口</span>
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>
</template>

<style scoped lang="scss">
.profile-row {
  display: grid;
  grid-template-columns: 48px 60px 100px 3fr 160px 200px 100px 100px 120px;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid #e2e8f0;
  transition: all 0.15s ease;
  cursor: pointer;
  background-color: white;
  
  &:last-child {
    border-bottom: none;
  }
  
  &:hover {
    background-color: rgba(59, 130, 246, 0.03);
    
    .col-actions {
      opacity: 1;
    }
  }
  
  &.selected {
    background-color: rgba(59, 130, 246, 0.08);
  }
}

// 列定义
.col-checkbox {
  display: flex;
  align-items: center;
  justify-content: center;
}

.col-index {
  display: flex;
  align-items: center;
  justify-content: center;
  
  .index-num {
    font-size: 14px;
    color: #64748b;
    font-weight: 500;
  }
}

.col-status {}

.col-name {}

.col-group {}

.col-proxy {}

.col-platform {
  display: flex;
  align-items: center;
  gap: 4px;
}

.col-time {}

.col-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  opacity: 0;
  transition: opacity 0.2s;
}

// 复选框
.checkbox-input {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  border: 1px solid #d1d5db;
  cursor: pointer;
  
  &:checked {
    background-color: #2563eb;
    border-color: #2563eb;
  }
}

// 状态标签
.status-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 100px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid;
  
  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }
  
  &.status-running {
    background: #ecfdf5;
    color: #059669;
    border-color: #a7f3d0;
    
    .status-dot {
      background: #22c55e;
    }
  }
  
  &.status-stopped {
    background: #f1f5f9;
    color: #64748b;
    border-color: #e2e8f0;
    
    .status-dot {
      background: #94a3b8;
    }
  }
}

// 名称单元格
.name-cell {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.avatar {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 12px;
  flex-shrink: 0;
}

.name-info {
  min-width: 0;
  
  .profile-name {
    font-size: 14px;
    font-weight: 500;
    color: #1e293b;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .profile-id {
    font-size: 12px;
    color: #94a3b8;
    font-family: monospace;
    cursor: pointer;
    transition: color 0.2s;
    
    &:hover {
      color: var(--color-primary);
    }
  }
}

// 分组标签
.group-tag {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  background: #f1f5f9;
  color: #475569;
}

// 代理信息
.proxy-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.proxy-host {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
  color: #334155;
  
  .proxy-icon {
    font-size: 16px;
    color: #94a3b8;
  }
}

.proxy-detail {
  font-size: 12px;
  color: #94a3b8;
}

// 平台
.platform-icon {
  width: 16px;
  height: 16px;
  color: #94a3b8;
  flex-shrink: 0;
}

.platform-label {
  font-size: 14px;
  color: #64748b;
}

// 时间
.time-text {
  font-size: 12px;
  color: #94a3b8;
}

// 操作按钮
.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: #94a3b8;
  cursor: pointer;
  transition: all 0.2s;
  
  .material-symbols-outlined {
    font-size: 18px;
  }
  
  &:hover {
    color: #2563eb;
    background: #eff6ff;
  }
  
  &.action-stop:hover {
    color: #ef4444;
    background: #fef2f2;
  }
  
  &.action-start:hover {
    color: #22c55e;
    background: #f0fdf4;
  }
  
  &.action-more:hover {
    color: #64748b;
    background: #f1f5f9;
  }
}

// 更多操作下拉菜单样式
:deep(.profile-action-menu) {
  .el-dropdown-menu__item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    font-size: 13px;
    
    .menu-icon {
      font-size: 16px;
      color: #64748b;
    }
    
    &.danger-item {
      color: #ef4444;
      
      .menu-icon {
        color: #ef4444;
      }
      
      &:hover {
        background: #fef2f2;
        color: #dc2626;
      }
    }
  }
}
</style>
