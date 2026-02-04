<script setup lang="ts">
/**
 * @description 标签管理页面
 * @author DeepAgent
 */

import { onMounted, onUnmounted } from 'vue'
import { useTagManagement } from './composables/useTagManagement'

// 使用 composable 管理所有业务逻辑
const {
  // 状态
  isLoading,
  currentPage,
  pageSize,
  pageSizes,
  dialogVisible,
  dialogTitle,
  sortBy,
  formData,
  selectedIds,
  
  // 计算属性
  filteredTags,
  paginatedTags,
  isAllSelected,
  totalCount,
  
  // 常量
  colorOptions,
  sortOptions,
  
  // 方法
  formatDate,
  getColorStyle,
  handleSelectAll,
  handleAdd,
  handleEdit,
  handleDelete,
  handleBatchDelete,
  handleSubmit,
  initTagManagement,
  cleanupTagManagement,
} = useTagManagement()

// ==================== 生命周期 ====================
onMounted(async () => {
  await initTagManagement()
})

onUnmounted(() => {
  cleanupTagManagement()
})
</script>

<template>
  <div class="tag-management">
    <!-- 工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <el-dropdown @command="(cmd: string) => sortBy = cmd as any" trigger="click">
          <button class="sort-btn">
            <span class="material-symbols-outlined">sort</span>
            <span>{{ sortOptions.find(o => o.value === sortBy)?.label }}</span>
            <span class="material-symbols-outlined arrow">expand_more</span>
          </button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item 
                v-for="option in sortOptions" 
                :key="option.value" 
                :command="option.value"
                :class="{ 'is-active': sortBy === option.value }"
              >
                <span class="material-symbols-outlined menu-icon">{{ option.value === 'updated' ? 'schedule' : option.value === 'name' ? 'sort_by_alpha' : 'format_list_numbered' }}</span>
                {{ option.label }}
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
      
      <div class="toolbar-right">
        <!-- 批量删除按钮（多选时显示） -->
        <button 
          v-if="selectedIds.length > 0" 
          class="btn-danger" 
          @click="handleBatchDelete"
          :title="`删除选中的 ${selectedIds.length} 个标签`"
        >
          <span class="material-symbols-outlined">delete</span>
          <span>批量删除 ({{ selectedIds.length }})</span>
        </button>
        
        <button class="btn-primary" @click="handleAdd">
          <span class="material-symbols-outlined">add</span>
          <span>新建标签</span>
        </button>
      </div>
    </div>
    
    <!-- 列表容器 -->
    <div class="list-container" v-loading="isLoading">
      <!-- 列表主体 -->
      <div class="list-body">
        <table v-if="filteredTags.length > 0" class="tag-table">
          <thead>
            <tr>
              <th class="col-cb">
                <input type="checkbox" class="custom-checkbox" @change="handleSelectAll" :checked="isAllSelected" />
              </th>
              <th class="col-index">序号</th>
              <th class="col-tag">标签</th>
              <th class="col-remark">备注</th>
              <th class="col-sort">排序</th>
              <th class="col-count">窗口数</th>
              <th class="col-time">创建时间</th>
              <th class="col-actions">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="tag in paginatedTags" :key="tag.id" class="table-row">
              <td class="col-cb">
                <input type="checkbox" class="custom-checkbox" v-model="selectedIds" :value="tag.id" />
              </td>
              <td class="col-index">
                <span class="index-num">{{ tag.index }}</span>
              </td>
              <td class="col-tag">
                <span 
                  class="tag-badge" 
                  :style="{
                    backgroundColor: getColorStyle((tag as any).color || 'blue').bg,
                    color: getColorStyle((tag as any).color || 'blue').text
                  }"
                >
                  {{ tag.name }}
                </span>
              </td>
              <td class="col-remark">
                <span class="remark-text" :title="tag.remark">{{ tag.remark || '-' }}</span>
              </td>
              <td class="col-sort">
                <span class="sort-num">{{ tag.sort }}</span>
              </td>
              <td class="col-count">
                <span class="count-num">{{ tag.windowCount || 0 }}</span>
              </td>
              <td class="col-time">
                <span class="time-text">{{ formatDate(tag.createdAt) }}</span>
              </td>
              <td class="col-actions">
                <div class="action-btns">
                  <button class="action-btn edit" @click="handleEdit(tag)">
                    <span class="material-symbols-outlined">edit</span>
                  </button>
                  <button class="action-btn delete" @click="handleDelete(tag)">
                    <span class="material-symbols-outlined">delete</span>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>

        <!-- 空状态 -->
        <div v-if="!isLoading && filteredTags.length === 0" class="empty-state">
          <div class="empty-icon-wrapper">
            <span class="material-symbols-outlined empty-icon">label_off</span>
          </div>
          <h2 class="empty-title">暂无标签</h2>
          <p class="empty-desc">
            您还没有创建任何标签。使用标签可以更好地分类和管理您的浏览器环境。
          </p>
          <button class="btn-create-first" @click="handleAdd">
            <span class="material-symbols-outlined">add</span>
            <span>创建第一个标签</span>
          </button>
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

    <!-- 新建/编辑弹窗 -->
    <el-dialog 
      v-model="dialogVisible" 
      :title="dialogTitle" 
      width="500px" 
      :close-on-click-modal="false"
      class="tag-dialog"
    >
      <div class="dialog-body">
        <div class="form-group">
          <label class="form-label">标签名称</label>
          <input 
            v-model="formData.name" 
            class="form-input" 
            placeholder="请输入标签名称" 
            maxlength="30"
          />
        </div>

        <div class="form-group">
          <label class="form-label">标签颜色</label>
          <div class="color-picker">
            <button
              v-for="color in colorOptions"
              :key="color.value"
              class="color-option"
              :class="{ 'active': formData.color === color.value }"
              :style="{ 
                backgroundColor: color.bg,
                borderColor: formData.color === color.value ? color.border : color.bg
              }"
              @click="formData.color = color.value"
            >
              <span 
                v-if="formData.color === color.value" 
                class="material-symbols-outlined check-icon"
                :style="{ color: color.text }"
              >
                check
              </span>
            </button>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">备注信息 (可选)</label>
          <textarea 
            v-model="formData.remark" 
            class="form-textarea" 
            placeholder="请输入备注信息"
            rows="3"
            maxlength="200"
          ></textarea>
        </div>

        <div class="form-group">
          <label class="form-label">排序权重 (可选)</label>
          <input 
            v-model.number="formData.sort" 
            class="form-input" 
            type="number" 
            placeholder="0"
            min="0"
          />
          <p class="form-hint">数值越大排序越靠前</p>
        </div>
      </div>

      <template #footer>
        <div class="dialog-footer-custom">
          <button class="btn-cancel" @click="dialogVisible = false">取消</button>
          <button class="btn-confirm" @click="handleSubmit">
            <span>确认创建</span>
          </button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped lang="scss">
.tag-management {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--color-bg-page);
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
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.sort-btn {
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
    color: var(--color-text-muted);
  }
  
  .arrow {
    color: var(--color-text-placeholder);
    margin-left: -4px;
  }
  
  &:hover { 
    background: var(--color-bg-hover); 
    border-color: var(--color-border-hover); 
  }
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-secondary);
  margin: 0;
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 38px;
  padding: 0 16px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--shadow-primary-sm);

  .material-symbols-outlined {
    font-size: 18px;
  }

  &:hover {
    background: var(--color-primary-hover);
    transform: translateY(-1px);
    box-shadow: var(--shadow-primary-lg);
  }
}

.btn-danger {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 36px;
  padding: 0 16px;
  background: var(--color-danger);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--shadow-danger-sm);

  .material-symbols-outlined { 
    font-size: 18px;
  }

  &:hover {
    background: var(--color-danger-hover);
    transform: translateY(-1px);
    box-shadow: var(--shadow-danger-lg);
  }
  
  &:active {
    transform: translateY(0px);
  }
}

// 表格卡片
// 列表容器
.list-container {
  flex: 1;
  margin: 0 24px 24px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

.list-body {
  flex: 1;
  overflow-y: auto;
  background-color: #ffffff;
}

.tag-table {
  width: 100%;
  border-collapse: collapse;

  thead {
    background: var(--color-bg-page);
    
    th {
      padding: 12px 24px;
      text-align: left;
      font-size: 12px;
      font-weight: 600;
      color: var(--color-text-tertiary);
      text-transform: uppercase;
      letter-spacing: 0.05em;
      border-bottom: 1px solid var(--color-border-default);

      &.col-cb { width: 60px; }
            &.col-index { width: 80px; }
      &.col-remark { min-width: 200px; }
      &.col-sort { width: 100px; }
      &.col-count { width: 100px; }
      &.col-time { min-width: 180px; }
      &.col-actions { width: 120px; text-align: right; }
    }
  }

  tbody {
    .table-row {
      border-bottom: 1px solid var(--color-border-subtle);
      transition: background-color 0.2s;

      &:hover {
        background-color: var(--color-hover-bg);
      }

      &:last-child {
        border-bottom: none;
      }

      td {
        padding: 16px 24px;
        font-size: 14px;
        color: var(--color-text-secondary);

        &.col-actions {
          text-align: right;
        }
      }
    }
  }
}

.index-num {
  color: var(--color-text-tertiary);
}

.tag-badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 9999px;
  font-size: 12px;
  font-weight: 500;
}

.sort-num {
  color: var(--color-text-tertiary);
}

.count-num {
  color: var(--color-text-primary);
  font-weight: 500;
}

.time-text {
  color: var(--color-text-tertiary);
  font-size: 14px;
}

.remark-text {
  color: var(--color-text-tertiary);
  font-size: 14px;
  display: block;
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.action-btns {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
}

.action-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  padding: 0;

  .material-symbols-outlined {
    font-size: 20px;
    color: var(--color-text-disabled);
  }

  &.edit:hover {
    background: var(--color-primary-bg);
    .material-symbols-outlined {
      color: var(--color-accent-blue);
    }
  }

  &.delete:hover {
    background: var(--color-danger-bg);
    .material-symbols-outlined {
      color: var(--color-accent-danger);
    }
  }
}

// 空状态
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 24px;
  text-align: center;
}

.empty-icon-wrapper {
  width: 96px;
  height: 96px;
  background: var(--color-primary-bg);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 24px;
}

.empty-icon {
  font-size: 60px;
  color: var(--color-accent-blue);
}

.empty-title {
  font-size: 24px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 12px 0;
}

.empty-desc {
  font-size: 18px;
  color: var(--color-text-tertiary);
  margin: 0 0 24px 0;
  max-width: 500px;
  line-height: 1.6;
}

.btn-create-first {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 40px;
  padding: 0 24px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--shadow-primary-sm);

  .material-symbols-outlined {
    font-size: 20px;
  }

  &:hover {
    background: var(--color-primary-hover);
  }
}

// 弹窗样式
.tag-dialog {
  :deep(.el-dialog) {
    border-radius: 12px;
    overflow: hidden;

    .el-dialog__header {
      margin: 0;
      padding: 16px 24px;
      border-bottom: 1px solid var(--color-border);
      background: var(--color-bg-page);

      .el-dialog__title {
        font-size: 18px;
        font-weight: 600;
        color: var(--color-text-primary);
      }
    }

    .el-dialog__body {
      padding: 0 !important;
    }

    .el-dialog__footer {
      padding: 0 !important;
    }
  }
}

.dialog-body {
  padding: 24px 24px 0;
}

.form-group {
  margin-bottom: 20px;

  &:last-child {
    margin-bottom: 24px;
  }
}

.form-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.form-input {
  width: 100%;
  height: 40px;
  padding: 0 12px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-bg-elevated);
  color: var(--color-text-primary);
  font-size: 14px;
  transition: all 0.2s;

  &::placeholder {
    color: var(--color-text-placeholder);
  }

  &:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: var(--shadow-focus-primary);
  }
}

.form-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-bg-elevated);
  color: var(--color-text-primary);
  font-size: 14px;
  resize: vertical;
  transition: all 0.2s;
  font-family: inherit;
  line-height: 1.5;

  &::placeholder {
    color: var(--color-text-placeholder);
  }

  &:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: var(--shadow-focus-primary);
  }
}

.color-picker {
  display: flex;
  align-items: center;
  gap: 12px;
}

.color-option {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 2px solid transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);

  &.active {
    box-shadow: 0 0 0 2px white, 0 0 0 4px currentColor;
    border-width: 2px;
  }

  &:hover:not(.active) {
    transform: scale(1.1);
  }

  .check-icon {
    font-size: 18px;
  }
}

.form-hint {
  margin-top: 6px;
  font-size: 12px;
  color: #64748b;
}

.dialog-footer-custom {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  background: #f8fafc;
  border-top: 1px solid #e2e8f0;
}

.btn-cancel,
.btn-confirm {
  padding: 8px 16px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-cancel {
  background: white;
  color: #475569;
  border: 1px solid #e2e8f0;

  &:hover {
    background: #f8fafc;
    border-color: #cbd5e1;
  }
}

.btn-confirm {
  background: #2563eb;
  color: white;
  border: none;
  box-shadow: 0 1px 3px rgba(37, 99, 235, 0.2);

  &:hover {
    background: #1d4ed8;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
  }
}
</style>

<!-- 非 scoped 样式，用于影响 Teleport 渲染的 Dialog -->
<style lang="scss">
.tag-dialog.el-dialog {
  --el-dialog-padding-primary: 0 !important;
  
  .el-dialog__body {
    padding: 0 !important;
    margin: 0 !important;
  }
  
  .el-dialog__footer {
    padding: 0 !important;
    margin: 0 !important;
  }
}

// 排序下拉菜单样式
.el-dropdown-menu {
  .el-dropdown-menu__item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    font-size: 14px;
    
    .menu-icon {
      font-size: 18px;
      color: #94a3b8;
    }
    
    &.is-active {
      background: #eff6ff;
      color: #2563eb;
      
      .menu-icon {
        color: #2563eb;
      }
    }
    
    &:hover:not(.is-active) {
      .menu-icon {
        color: #64748b;
      }
    }
  }
}

// MessageBox 危险按钮样式
.el-message-box {
  .el-button--danger {
    background-color: #ef4444 !important;
    border-color: #ef4444 !important;
    color: white !important;
    
    &:hover {
      background-color: #dc2626 !important;
      border-color: #dc2626 !important;
    }
  }
}

// 复选框样式
.custom-checkbox {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  border: 1px solid #cbd5e1;
  cursor: pointer;
  accent-color: #2563eb;
}

// 分页栏
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
}

.page-info {
  font-size: 12px;
  color: var(--color-text-tertiary);
  white-space: nowrap;
  
  strong {
    color: var(--color-text-primary);
    font-weight: 600;
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
  .pagination-bar {
    padding: 10px 16px;
    justify-content: center;
  }
  
  .page-info {
    display: none;
  }
  
  .page-controls {
    gap: 8px;
    .size-select { width: 80px; }
  }
}
</style>
