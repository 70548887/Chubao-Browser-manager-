<script setup lang="ts">
/**
 * @description 窗口回收站页面
 * @author DeepAgent
 */

import { onMounted, onUnmounted } from 'vue'
import BatchResultDialog from '@/components/BatchResultDialog.vue'
import { useRecycleBin } from './composables/useRecycleBin'

// 使用 composable 管理所有业务逻辑
const {
  // 状态
  selectedIds,
  isLoading,
  groupFilter,
  currentPage,
  pageSize,
  pageSizes,
  batchResultVisible,
  batchResultData,
  
  // 计算属性
  filteredItems,
  paginatedItems,
  totalCount,
  isAllSelected,
  
  // 方法
  formatDate,
  handleSelectAll,
  handleSelect,
  handleRestore,
  handleDelete,
  handleBatchRestore,
  handleBatchDelete,
  handleDeleteAll,
  initRecycleBin,
  cleanupRecycleBin,
} = useRecycleBin()

// ==================== 生命周期 ====================
onMounted(async () => {
  await initRecycleBin()
})

onUnmounted(() => {
  cleanupRecycleBin()
})
</script>

<template>
  <div class="recycle-bin">
    <!-- 工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <div class="filter-dropdown">
          <button class="dropdown-trigger">
            <span class="material-symbols-outlined">folder_open</span>
            <span>{{ groupFilter || '全部分组' }}</span>
            <span class="material-symbols-outlined">expand_more</span>
          </button>
        </div>
        
        <div class="divider"></div>
        
        <button class="action-btn restore" @click="handleBatchRestore">
          <span class="material-symbols-outlined">restore_from_trash</span>
          <span>恢复选中</span>
        </button>
        
        <button class="action-btn delete" @click="handleBatchDelete">
          <span class="material-symbols-outlined">delete_forever</span>
          <span>删除选中</span>
        </button>
        
        <button class="action-btn danger" @click="handleDeleteAll">
          <span class="material-symbols-outlined">cleaning_services</span>
          <span>清空回收站</span>
        </button>
      </div>
      
      <div class="toolbar-right">
        <span class="tip-text">回收站中的项目将保留 30 天后自动永久删除</span>
      </div>
    </div>
    
    <!-- 表格容器 -->
    <div class="list-container" v-loading="isLoading">
      <!-- 表头 -->
      <div class="list-header-grid">
        <div class="col-cb">
          <input 
            type="checkbox" 
            class="custom-checkbox"
            :checked="isAllSelected"
            @change="(e: any) => handleSelectAll(e.target.checked)"
          />
        </div>
        <div class="col-index">序号</div>
        <div class="col-group">分组名称</div>
        <div class="col-window">窗口名称</div>
        <div class="col-remark">窗口备注</div>
        <div class="col-operator">操作用户</div>
        <div class="col-time">删除时间</div>
        <div class="col-actions text-right">操作</div>
      </div>
      
      <!-- 表体 -->
      <div class="list-body">
        <div 
          v-for="item in paginatedItems" 
          :key="item.id"
          class="list-row"
        >
          <div class="col-cb">
            <input 
              type="checkbox" 
              class="custom-checkbox"
              :checked="selectedIds.has(item.id)"
              @change="() => handleSelect(item)"
            />
          </div>
          
          <div class="col-index">
            <span class="index-num">{{ item.index }}</span>
          </div>
          
          <div class="col-group">
            <span class="group-badge">
              {{ item.groupName }}
            </span>
          </div>
          
          <div class="col-window">
            <div class="window-info">
              <div class="window-icon" :class="`icon-${item.iconColor}`">
                <span class="material-symbols-outlined">{{ item.icon }}</span>
              </div>
              <div class="window-details">
                <p class="window-name">{{ item.windowName }}</p>
                <p class="window-id">ID: {{ item.windowId }}</p>
              </div>
            </div>
          </div>
          
          <div class="col-remark">
            <p class="remark-text">{{ item.remark || '-' }}</p>
          </div>
          
          <div class="col-operator">
            <div class="operator-info">
              <div class="operator-avatar">
                {{ item.operatorAvatar }}
              </div>
              <span class="operator-name">{{ item.operator }}</span>
            </div>
          </div>
          
          <div class="col-time">
            <span class="time-text">{{ formatDate(item.deletedAt) }}</span>
          </div>

          <div class="col-actions text-right">
            <div class="row-actions">
              <button class="act-btn restore" title="恢复" @click="handleRestore(item)">
                <span class="material-symbols-outlined">settings_backup_restore</span>
              </button>
              <button class="act-btn delete" title="永久删除" @click="handleDelete(item)">
                <span class="material-symbols-outlined">delete_forever</span>
              </button>
            </div>
          </div>
        </div>
        
        <!-- 空状态 -->
        <div v-if="!isLoading && filteredItems.length === 0" class="empty-view">
          <span class="material-symbols-outlined">delete</span>
          <p>暂无数据</p>
        </div>
      </div>
      
      <!-- 分页区域 -->
      <div v-if="totalCount > 0" class="pagination-bar">
        <div class="page-info">
          显示 {{ (currentPage - 1) * pageSize + 1 }} 到 {{ Math.min(currentPage * pageSize, totalCount) }} 条，共 <strong>{{ totalCount }}</strong> 条
        </div>
        <div class="page-controls">
          <el-select v-model="pageSize" size="small" class="size-select">
            <el-option v-for="size in pageSizes" :key="size" :label="size + ' / 页'" :value="size" />
          </el-select>
          <el-pagination
            v-model:current-page="currentPage"
            :page-size="pageSize"
            :total="totalCount"
            :pager-count="5"
            layout="prev, pager, next"
            background
            size="small"
          />
        </div>
      </div>
    </div>
    
    <!-- 批量结果对话框 -->
    <BatchResultDialog
      v-model:visible="batchResultVisible"
      :result="batchResultData"
      title="批量操作结果"
    />
  </div>
</template>

<style scoped lang="scss">
.recycle-bin {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--color-bg-page);
  color: var(--color-text-primary);
}

// 工具栏
.toolbar {
  padding: 20px 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.toolbar-right {
  .tip-text {
    font-size: 14px;
    color: var(--color-text-muted);
  }
}

.filter-dropdown {
  .dropdown-trigger {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 38px;
    padding: 0 14px;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    color: var(--color-text-secondary);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;

    .material-symbols-outlined {
      font-size: 18px;
      
      &:first-child {
        color: var(--color-text-muted);
      }
      
      &:last-child {
        color: var(--color-text-placeholder);
      }
    }

    &:hover {
      background: var(--color-bg-hover);
      border-color: var(--color-border-hover);
    }
  }
}

.divider {
  width: 1px;
  height: 24px;
  background: var(--color-border);
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 38px;
  padding: 0 16px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  background: var(--color-bg-elevated);
  color: var(--color-text-secondary);

  .material-symbols-outlined {
    font-size: 18px;
  }

  &.restore {
    &:hover {
      color: var(--color-primary);
      border-color: var(--color-primary-light);
      background: var(--color-primary-bg);
    }
  }

  &.delete {
    &:hover {
      color: var(--color-danger);
      border-color: var(--color-danger-light);
      background: var(--color-danger-bg);
    }
  }

  &.danger {
    background: var(--color-danger-bg);
    border-color: var(--color-danger-light);
    color: var(--color-danger);

    &:hover {
      background: var(--color-status-error-bg);
      border-color: var(--color-status-error-border);
    }
  }
}

// 列表容器
.list-container {
  flex: 1;
  margin: 0 24px 24px;
  background-color: #ffffff;
  border: 1px solid var(--color-border-default);
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

.list-header-grid {
  display: grid;
  grid-template-columns: 60px 60px 2fr 3fr 2fr 1fr 2fr 120px;
  gap: 16px;
  padding: 12px 24px;
  background: var(--color-bg-page);
  border-bottom: 1px solid var(--color-border-default);
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.list-body {
  flex: 1;
  overflow-y: auto;
  background-color: #ffffff;
}

.list-row {
  display: grid;
  grid-template-columns: 60px 60px 2fr 3fr 2fr 1fr 2fr 120px;
  gap: 16px;
  padding: 16px 24px;
  align-items: center;
  border-bottom: 1px solid var(--color-border-subtle);
  transition: all 0.2s;
  background-color: #ffffff;

  &:hover {
    background-color: var(--color-hover-bg);
  }

  &:last-child {
    border-bottom: none;
  }
}

.custom-checkbox {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  border: 1px solid var(--color-border-strong);
  cursor: pointer;
  accent-color: var(--color-accent-blue);
}

.index-num {
  font-size: 14px;
  color: var(--color-text-tertiary);
  font-weight: 500;
}

.group-badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 8px;
  background: var(--color-status-neutral-bg);
  color: var(--color-text-secondary);
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  border: 1px solid var(--color-border-default);
}

.window-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.window-icon {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  .material-symbols-outlined {
    font-size: 18px;
  }

  &.icon-blue {
    background: var(--color-badge-blue-bg);
    color: var(--color-primary);
  }

  &.icon-purple {
    background: var(--color-badge-purple-bg);
    color: var(--color-badge-purple-text);
  }

  &.icon-orange {
    background: var(--color-badge-orange-bg);
    color: var(--color-status-warning-text);
  }

  &.icon-green {
    background: var(--color-badge-green-bg);
    color: var(--color-badge-green-text);
  }

  &.icon-indigo {
    background: var(--color-badge-indigo-bg);
    color: var(--color-badge-indigo-text);
  }

  &.icon-pink {
    background: var(--color-badge-pink-bg);
    color: var(--color-badge-pink-text);
  }
}

.window-details {
  min-width: 0;
}

.window-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 2px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.window-id {
  font-size: 12px;
  color: var(--color-text-tertiary);
  margin: 0;
  font-family: 'Courier New', monospace;
}

.remark-text {
  font-size: 14px;
  color: var(--color-text-tertiary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.operator-info {
  display: flex;
  align-items: center;
  gap: 6px;
}

.operator-avatar {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--color-badge-indigo-bg);
  color: var(--color-badge-indigo-text);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  font-weight: 700;
}

.operator-name {
  font-size: 14px;
  color: var(--color-text-secondary);
}

.time-text {
  font-size: 14px;
  color: var(--color-text-tertiary);
}

.row-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  opacity: 0;
  transition: opacity 0.2s;

  .list-row:hover & {
    opacity: 1;
  }
}

.act-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--color-text-disabled);
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined {
    font-size: 18px;
  }

  &:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);

    &.restore {
      color: var(--color-accent-blue);
      background: var(--color-primary-bg);
    }

    &.delete {
      color: var(--color-accent-danger);
      background: var(--color-danger-bg);
    }
  }
}

.text-right {
  text-align: right;
}

// 空状态
.empty-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 0;
  color: var(--color-text-disabled);

  .material-symbols-outlined {
    font-size: 48px;
    margin-bottom: 12px;
    opacity: 0.5;
  }

  p {
    font-size: 14px;
    margin: 0;
  }
}

// 分页
.pagination-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  background-color: #ffffff;
  border-top: 1px solid var(--color-border-default);
  flex-shrink: 0;
  min-height: 48px;
  flex-wrap: wrap;
  gap: 8px 16px;

  .page-info {
    font-size: 12px;
    color: var(--color-text-tertiary);
    white-space: nowrap;

    strong {
      color: var(--color-text-primary);
    }
  }

  .page-controls {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-shrink: 0;

    .size-select {
      width: 90px;
    }
  }
  
  // 小屏幕适配
  @media (max-width: 768px) {
    padding: 10px 16px;
    justify-content: center;
    
    .page-info {
      display: none;
    }
    
    .page-controls {
      gap: 8px;
      .size-select { width: 80px; }
    }
  }
}
</style>
