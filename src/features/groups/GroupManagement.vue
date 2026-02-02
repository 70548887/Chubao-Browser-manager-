<script setup lang="ts">
/**
 * @description 分组管理页面 - 完整CRUD功能
 * @author DeepAgent
 */

import { useGroupManagement } from './composables/useGroupManagement'

// 使用 composable 管理所有业务逻辑
const {
  // 状态
  isLoading,
  currentPage,
  pageSize,
  pageSizes,
  dialogVisible,
  drawerVisible,
  dialogTitle,
  editingId,
  sortBy,
  selectedIds,
  formData,
  
  // 计算属性
  paginatedGroups,
  totalCount,
  isAllSelected,
  editingGroupProfileCount,
  
  // 方法
  handleSelectAll,
  formatDate,
  formatShortId,
  copyIdToClipboard,
  getGroupIcon,
  getIconClass,
  handleAdd,
  handleEdit,
  handleDelete,
  handleBatchDelete,
  handleSubmit,
  
  // 常量
  sortOptions
} = useGroupManagement()
</script>

<template>
  <div class="group-management">
    <!-- 工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <!-- 筛选功能暂时屏蔽，后期开发
        <button class="filter-btn" @click="filterDrawerVisible = true">
          <span class="material-symbols-outlined">filter_list</span>
          <span>筛选</span>
        </button>
        -->
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
          class="btn-danger batch-delete" 
          @click="handleBatchDelete"
          :title="`删除选中的 ${selectedIds.length} 个分组`"
        >
          <span class="material-symbols-outlined">delete</span>
          <span>批量删除 ({{ selectedIds.length }})</span>
        </button>
        
        <button class="btn-primary group" @click="handleAdd">
          <span class="material-symbols-outlined group-hover:rotate-90 transition-transform">add</span>
          <span>新建分组</span>
        </button>
      </div>
    </div>
    
    <!-- 列表容器 -->
    <div class="list-container" v-loading="isLoading">
      <!-- 表头 -->
      <div class="list-header-grid">
        <div class="col-cb">
          <input type="checkbox" class="custom-checkbox" @change="handleSelectAll" :checked="isAllSelected" />
        </div>
        <div class="col-index">序号</div>
        <div class="col-name">分组名称</div>
        <div class="col-count">窗口数量</div>
        <div class="col-sort">排序</div>
        <div class="col-remark">备注</div>
        <div class="col-time">创建时间</div>
        <div class="col-actions text-right">操作</div>
      </div>
      
      <!-- 列表主体 -->
      <div class="list-body">
        <div 
          v-for="group in paginatedGroups" 
          :key="group.id" 
          class="list-row-grid group"
        >
          <div class="col-cb">
            <input 
              type="checkbox" 
              class="custom-checkbox" 
              v-model="selectedIds" 
              :value="group.id"
              :disabled="group.id === 'default'"
            />
          </div>
          
          <div class="col-index">
            <span class="index-num">{{ group.index }}</span>
          </div>
          
          <div class="col-name">
            <div class="group-identity">
              <div class="icon-box" :class="getIconClass(group.icon)">
                <span class="material-symbols-outlined">{{ getGroupIcon(group.icon) }}</span>
              </div>
              <div class="text-info">
                <p class="main-name">{{ group.name }}</p>
              </div>
            </div>
          </div>
          
          <div class="col-count">
            <span class="count-badge">
              <span class="material-symbols-outlined">web</span>
              {{ group.profileCount || 0 }}
            </span>
          </div>
          
          <div class="col-sort">
            <span class="sort-badge">{{ group.sort }}</span>
          </div>
          
          <div class="col-remark">
            <p class="remark-text" :title="group.remark">{{ group.remark || '-' }}</p>
          </div>
          
          <div class="col-time">
            <span class="time-text">{{ formatDate(group.createdAt) || '暂无' }}</span>
          </div>
          
          <div class="col-actions text-right">
            <div class="row-actions">
              <button class="act-btn edit" title="编辑" @click="handleEdit(group)">
                <span class="material-symbols-outlined">edit</span>
              </button>
              <button 
                v-if="group.id !== 'default'"
                class="act-btn delete" 
                title="删除" 
                @click="handleDelete(group)"
              >
                <span class="material-symbols-outlined">delete</span>
              </button>
              <el-tooltip v-else content="默认分组不可删除" placement="top">
                <button class="act-btn delete disabled" disabled>
                  <span class="material-symbols-outlined">block</span>
                </button>
              </el-tooltip>
            </div>
          </div>
        </div>
        
        <!-- 空状态 -->
        <div v-if="!isLoading && totalCount === 0" class="empty-view">
          <span class="material-symbols-outlined">folder_open</span>
          <p>暂无分组数据</p>
          <button class="btn-empty" @click="handleAdd">新建分组</button>
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

    <!-- 添加弹窗 (对话框模式) -->
    <el-dialog 
      v-model="dialogVisible" 
      :title="dialogTitle" 
      width="480px" 
      :close-on-click-modal="false" 
      class="create-group-dialog"
    >
      <div class="dialog-body">
        <div class="form-group">
          <label class="form-label">
            分组名称 <span class="required">*</span>
          </label>
          <input 
            v-model="formData.name" 
            class="form-input" 
            placeholder="请输入分组名称" 
            maxlength="50"
          />
        </div>

        <div class="form-group">
          <label class="form-label">图标</label>
          <div class="icon-grid">
            <div 
              v-for="icon in ['folder', 'shopping_bag', 'campaign', 'movie', 'payments', 'mail']" 
              :key="icon"
              class="icon-option"
              :class="{ 'active': formData.icon === icon }"
              @click="formData.icon = icon"
            >
              <span class="material-symbols-outlined">{{ getGroupIcon(icon) }}</span>
            </div>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">备注信息</label>
          <textarea 
            v-model="formData.remark" 
            class="form-textarea" 
            placeholder="可选填备注信息" 
            rows="3"
          ></textarea>
        </div>

        <div class="form-group">
          <label class="form-label">排序</label>
          <el-input-number 
            v-model="formData.sort" 
            :min="0" 
            :max="999" 
            style="width: 100%"
          />
          <div class="form-tip">数值越小排序越靠前</div>
        </div>

        <div class="info-tip">
          <span class="material-symbols-outlined">info</span>
          <p>创建分组后，您可以将现有的浏览器环境移动到该分组中，或在该分组下直接创建新环境。</p>
        </div>
      </div>

      <template #footer>
        <div class="dialog-footer-custom">
          <button class="btn-cancel" @click="dialogVisible = false">取消</button>
          <button class="btn-confirm" @click="handleSubmit">确定创建</button>
        </div>
      </template>
    </el-dialog>

    <!-- 编辑抽屉 (侧边栏模式) -->
    <el-drawer
      v-model="drawerVisible"
      direction="rtl"
      size="400px"
      :close-on-click-modal="false"
      class="edit-group-drawer"
    >
      <template #header>
        <div class="drawer-header">
          <div class="header-left">
            <div class="group-icon-preview" :class="getIconClass(formData.icon)">
              <span class="material-symbols-outlined">{{ getGroupIcon(formData.icon) }}</span>
            </div>
            <div class="header-text">
              <h2 class="drawer-title">{{ formData.name || '分组详情' }}</h2>
              <el-tooltip :content="editingId || ''" placement="bottom" :show-after="200">
                <p class="drawer-subtitle" @dblclick="copyIdToClipboard">ID: {{ formatShortId(editingId) }}</p>
              </el-tooltip>
            </div>
          </div>
        </div>
      </template>

      <div class="drawer-body">
        <div class="section">
          <h3 class="section-title">
            <span class="material-symbols-outlined">info</span>
            基本信息
          </h3>
          <div class="form-group">
            <label class="form-label">分组名称</label>
            <input v-model="formData.name" class="form-input" placeholder="请输入分组名称" />
          </div>

          <div class="form-group">
            <label class="form-label">图标</label>
            <div class="icon-grid">
              <div 
                v-for="icon in ['folder', 'shopping_bag', 'campaign', 'movie', 'payments', 'mail']" 
                :key="icon"
                class="icon-option"
                :class="{ 'active': formData.icon === icon }"
                @click="formData.icon = icon"
              >
                <span class="material-symbols-outlined">{{ getGroupIcon(icon) }}</span>
              </div>
            </div>
          </div>

          <div class="form-group">
            <label class="form-label">备注说明</label>
            <textarea 
              v-model="formData.remark" 
              class="form-textarea" 
              rows="3"
              placeholder="请输入备注说明"
            ></textarea>
          </div>

          <div class="form-group">
            <label class="form-label">排序</label>
            <el-input-number 
              v-model="formData.sort" 
              :min="0" 
              :max="999" 
              style="width: 100%"
            />
            <div class="form-tip">数值越小排序越靠前</div>
          </div>
        </div>

        <div class="section-divider"></div>

        <div class="section">
          <h3 class="section-title">
            <span class="material-symbols-outlined">analytics</span>
            资源统计
          </h3>
          <div class="stat-grid">
            <div class="stat-box">
              <p class="stat-label">关联窗口</p>
              <p class="stat-value">{{ editingGroupProfileCount }}</p>
            </div>
            <div class="stat-box">
              <p class="stat-label">活跃会话</p>
              <p class="stat-value green">0</p>
            </div>
          </div>
        </div>

        <div class="section-divider"></div>

        <div class="section">
          <h3 class="section-title">
            <span class="material-symbols-outlined">bolt</span>
            快捷操作
          </h3>
          <div class="quick-actions">
            <button class="action-item">
              <span class="action-icon">
                <span class="material-symbols-outlined">settings_ethernet</span>
              </span>
              <span class="action-text">批量配置代理</span>
              <span class="material-symbols-outlined arrow">chevron_right</span>
            </button>
            <button class="action-item">
              <span class="action-icon">
                <span class="material-symbols-outlined">extension</span>
              </span>
              <span class="action-text">管理扩展插件</span>
              <span class="material-symbols-outlined arrow">chevron_right</span>
            </button>
            <button class="action-item">
              <span class="action-icon">
                <span class="material-symbols-outlined">share</span>
              </span>
              <span class="action-text">分享给团队成员</span>
              <span class="material-symbols-outlined arrow">chevron_right</span>
            </button>
          </div>
        </div>
      </div>

      <template #footer>
        <div class="drawer-footer-custom">
          <button class="btn-cancel" @click="drawerVisible = false">取消</button>
          <button class="btn-confirm" @click="handleSubmit">保存更改</button>
        </div>
      </template>
    </el-drawer>

    <!-- 筛选抽屉 - 暂时屏蔽，后期开发
    <el-drawer
      v-model="filterDrawerVisible"
      title="筛选选项"
      direction="rtl"
      size="360px"
    >
      <div class="filter-content">
        <el-alert
          title="筛选功能待实现"
          type="info"
          :closable="false"
          description="可以添加按权限、按环境数量等条件筛选"
        />
      </div>
    </el-drawer>
    -->
  </div>
</template>

<style scoped lang="scss">
.group-management {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--color-bg-page);
  color: var(--color-text-primary);
}

// 头部样式
.page-header {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-default);
  box-shadow: var(--shadow-xs);
  z-index: 10;
  position: sticky;
  top: 0;

  .page-title {
    font-size: 20px;
    font-weight: 700;
    color: var(--color-text-primary);
    letter-spacing: -0.025em;
  }

  .search-wrapper {
    position: relative;
    width: 384px;

    .search-icon {
      position: absolute;
      left: 12px;
      top: 50%;
      transform: translateY(-50%);
      color: var(--color-text-disabled);
      font-size: 20px;
    }

    .search-input {
      width: 100%;
      height: 38px;
      padding: 0 12px 0 40px;
      background: var(--color-bg-elevated);
      border: 1px solid var(--color-border-default);
      border-radius: 8px;
      font-size: 14px;
      transition: all 0.2s;

      &:focus {
        outline: none;
        background: var(--color-bg-secondary);
        border-color: var(--color-accent-blue);
        box-shadow: var(--shadow-focus-primary);
      }
    }
  }
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

.filter-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 38px;
  padding: 0 14px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  color: var(--color-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined { font-size: 18px; }
  &:hover { background: var(--color-bg-hover); border-color: var(--color-border-strong); }
}

.sort-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 38px;
  padding: 0 14px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  color: var(--color-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined { 
    font-size: 18px; 
    color: var(--color-text-tertiary);
  }
  
  .arrow {
    color: var(--color-text-disabled);
    margin-left: -4px;
  }
  
  &:hover { 
    background: var(--color-bg-hover); 
    border-color: var(--color-border-strong); 
  }
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
  height: 38px;
  padding: 0 16px;
  background: var(--color-accent-danger);
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

// 列表样式
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

.list-header-grid {
  display: grid;
  grid-template-columns: 60px 60px 3fr 2fr 1fr 3fr 2fr 120px;
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
}

.list-row-grid {
  display: grid;
  grid-template-columns: 60px 60px 3fr 2fr 1fr 3fr 2fr 120px;
  gap: 16px;
  padding: 16px 24px;
  align-items: center;
  border-bottom: 1px solid var(--color-border-subtle);
  transition: all 0.2s;

  &:hover { background-color: var(--color-hover-bg); }
  &:last-child { border-bottom: none; }
}

.group-identity {
  display: flex;
  align-items: center;
  gap: 12px;

  .icon-box {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid transparent;

    .material-symbols-outlined { font-size: 20px; }

    &.icon-indigo { background: var(--color-badge-indigo-bg); color: var(--color-badge-indigo-text); border-color: var(--color-badge-indigo-bg); }
    &.icon-blue { background: var(--color-status-info-bg); color: var(--color-accent-blue); border-color: var(--color-status-info-border); }
    &.icon-slate { background: var(--color-status-neutral-bg); color: var(--color-text-secondary); border-color: var(--color-border-subtle); }
    &.icon-orange { background: var(--color-status-warning-bg); color: var(--color-status-warning-text); border-color: var(--color-status-warning-border); }
    &.icon-emerald { background: var(--color-status-success-bg); color: var(--color-status-success-text); border-color: var(--color-status-success-border); }
  }

  .main-name { font-size: 14px; font-weight: 700; color: var(--color-text-primary); margin: 0; }
}

.index-num {
  font-size: 14px;
  color: var(--color-text-tertiary);
  font-weight: 500;
}

.count-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: var(--color-status-neutral-bg);
  color: var(--color-text-secondary);
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  border: 1px solid var(--color-border-default);

  .material-symbols-outlined { font-size: 14px; color: var(--color-text-disabled); }
}

.sort-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 32px;
  height: 24px;
  padding: 0 8px;
  background: var(--color-status-info-bg);
  color: var(--color-accent-blue);
  border-radius: 6px;
  font-size: 13px;
  font-weight: 700;
  border: 1px solid var(--color-status-info-border);
}

.remark-text {
  font-size: 14px;
  color: var(--color-text-tertiary);
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.time-text { font-size: 14px; color: var(--color-text-tertiary); }

.row-actions {
  display: flex;
  justify-content: flex-end;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;
  .group:hover & { opacity: 1; }
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

  .material-symbols-outlined { font-size: 18px; }

  &:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
    &.edit { color: var(--color-accent-blue); background: var(--color-primary-bg); }
    &.delete { color: var(--color-accent-danger); background: var(--color-danger-bg); }
  }
}

.pagination-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  background: var(--color-bg-page);
  border-top: 1px solid var(--color-border-default);

  .page-info { font-size: 12px; color: var(--color-text-tertiary); strong { color: var(--color-text-primary); } }
  .page-controls { display: flex; align-items: center; gap: 16px; .size-select { width: 100px; } }
}

.empty-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 0;
  color: var(--color-text-disabled);
  .material-symbols-outlined { font-size: 48px; margin-bottom: 12px; opacity: 0.5; }
  p { font-size: 14px; margin-bottom: 20px; }
  .btn-empty { padding: 8px 24px; background: var(--color-accent-blue); color: white; border: none; border-radius: 8px; font-weight: 500; cursor: pointer; &:hover { background: var(--color-accent-blue-hover); } }
}

.custom-checkbox { width: 16px; height: 16px; border-radius: 4px; border: 1px solid var(--color-border-strong); cursor: pointer; accent-color: var(--color-accent-blue); }

// 新建分组对话框样式
.create-group-dialog {
  :deep(.el-dialog) {
    --el-dialog-padding-primary: 0;
    border-radius: 16px;
    overflow: hidden;
    
    .el-dialog__header {
      margin: 0;
      padding: 16px 24px;
      border-bottom: 1px solid var(--color-border-default);
      
      .el-dialog__title {
        font-size: 18px;
        font-weight: 700;
        color: var(--color-text-primary);
      }
    }
    
    .el-dialog__body {
      padding: 0 !important;
      margin: 0 !important;
    }
    
    .el-dialog__footer {
      padding: 0 !important;
      margin: 0 !important;
    }
  }
}

.dialog-body {
  padding: 24px 24px 0;
}

.form-group {
  margin-bottom: 20px;
  
  &:last-child { margin-bottom: 0; }
}

.form-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 6px;
  
  .required { color: var(--color-accent-danger); margin-left: 2px; }
}

.form-input {
  width: 100%;
  height: 40px;
  padding: 0 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  background: var(--color-bg-elevated);
  color: var(--color-text-primary);
  font-size: 14px;
  transition: all 0.2s;
  
  &::placeholder { color: var(--color-text-disabled); }
  
  &:focus {
    outline: none;
    background: var(--color-bg-secondary);
    border-color: var(--color-accent-blue);
    box-shadow: var(--shadow-focus-primary);
  }
}

.form-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  background: var(--color-bg-elevated);
  color: var(--color-text-primary);
  font-size: 14px;
  resize: none;
  transition: all 0.2s;
  font-family: inherit;
  
  &::placeholder { color: var(--color-text-disabled); }
  
  &:focus {
    outline: none;
    background: var(--color-bg-secondary);
    border-color: var(--color-accent-blue);
    box-shadow: var(--shadow-focus-primary);
  }
}

.icon-grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 8px;
  
  .icon-option {
    height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--color-border-default);
    border-radius: 8px;
    background: var(--color-bg-elevated);
    cursor: pointer;
    transition: all 0.2s;
    color: var(--color-text-tertiary);
    
    &:hover {
      border-color: var(--color-border-strong);
      background: var(--color-bg-secondary);
    }
    
    &.active {
      border-color: var(--color-accent-blue);
      background: var(--color-status-info-bg);
      color: var(--color-accent-blue);
    }
    
    .material-symbols-outlined { font-size: 22px; }
  }
}

.info-tip {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px;
  background: var(--color-status-info-bg);
  border-radius: 8px;
  margin-top: 20px;
  margin-bottom: 24px;
  
  .material-symbols-outlined {
    font-size: 20px;
    color: var(--color-accent-blue);
    margin-top: 2px;
  }
  
  p {
    flex: 1;
    font-size: 12px;
    color: var(--color-status-info-text);
    line-height: 1.5;
    margin: 0;
  }
}

.dialog-footer-custom {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  background: var(--color-bg-page);
  border-top: 1px solid var(--color-border-default);
  
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
    background: var(--color-bg-secondary);
    color: var(--color-text-secondary);
    border: 1px solid var(--color-border-default);
    
    &:hover { background: var(--color-bg-hover); border-color: var(--color-border-strong); }
  }
  
  .btn-confirm {
    background: var(--color-accent-blue);
    color: white;
    border: none;
    box-shadow: var(--shadow-primary-sm);
    
    &:hover {
      background: var(--color-accent-blue-hover);
      transform: translateY(-1px);
      box-shadow: var(--shadow-primary-md);
    }
  }
}

// 编辑抽屉样式
.edit-group-drawer {
  :deep(.el-drawer) {
    border-left: 1px solid var(--color-border-default);
    box-shadow: var(--shadow-drawer);
  }
  
  :deep(.el-drawer__header) {
    margin: 0;
    padding: 0;
  }
  
  :deep(.el-drawer__body) {
    padding: 0;
    display: flex;
    flex-direction: column;
  }
  
  :deep(.el-drawer__footer) {
    padding: 0;
    margin-top: auto;
  }
}

.drawer-header {
  padding: 20px 24px;
  background: transparent;
  display: flex;
  align-items: center;
  flex: 1;
  
  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
    flex: 1;
  }
  
  .group-icon-preview {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid transparent;
    flex-shrink: 0;
    
    .material-symbols-outlined { 
      font-size: 24px;
      font-variation-settings: 'FILL' 0, 'wght' 300, 'GRAD' 0, 'opsz' 24;
    }
    
    &.icon-indigo { background: var(--color-badge-indigo-bg); color: var(--color-badge-indigo-text); border-color: var(--color-badge-indigo-bg); }
    &.icon-blue { background: var(--color-status-info-bg); color: var(--color-accent-blue); border-color: var(--color-status-info-border); }
    &.icon-slate { background: var(--color-status-neutral-bg); color: var(--color-text-secondary); border-color: var(--color-border-subtle); }
    &.icon-orange { background: var(--color-status-warning-bg); color: var(--color-status-warning-text); border-color: var(--color-status-warning-border); }
    &.icon-emerald { background: var(--color-status-success-bg); color: var(--color-status-success-text); border-color: var(--color-status-success-border); }
  }
  
  .header-text {
    flex: 1;
    min-width: 0;
    
    .drawer-title {
      font-size: 18px;
      font-weight: 700;
      color: var(--color-text-primary);
      margin: 0 0 4px 0;
      line-height: 1.3;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    
    .drawer-subtitle {
      font-size: 11px;
      color: var(--color-text-tertiary);
      margin: 0;
      font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
      line-height: 1.4;
      letter-spacing: 0.5px;
      white-space: nowrap;
      cursor: pointer;
      transition: color 0.2s;
      
      &:hover {
        color: var(--color-accent-blue);
      }
    }
  }
}

.drawer-body {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.section {
  margin-bottom: 32px;
  
  &:last-child { margin-bottom: 0; }
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 16px 0;
  
  .material-symbols-outlined {
    font-size: 18px;
    color: var(--color-accent-blue);
  }
}

.section-divider {
  height: 1px;
  background: var(--color-border-default);
  margin: 32px 0;
}

.stat-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.stat-box {
  padding: 12px;
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  
  .stat-label {
    font-size: 12px;
    color: var(--color-text-tertiary);
    margin: 0 0 4px 0;
  }
  
  .stat-value {
    font-size: 24px;
    font-weight: 700;
    color: var(--color-text-primary);
    margin: 0;
    
    &.green { color: var(--color-status-success-text); }
  }
}

.quick-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.action-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  width: 100%;
  text-align: left;
  
  .action-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    
    .material-symbols-outlined {
      font-size: 20px;
      color: var(--color-text-disabled);
    }
  }
  
  .action-text {
    flex: 1;
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-secondary);
  }
  
  .arrow {
    font-size: 16px;
    color: var(--color-text-disabled);
  }
  
  &:hover {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-strong);
    
    .action-icon .material-symbols-outlined {
      color: var(--color-accent-blue);
    }
  }
}

.drawer-footer-custom {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  background: var(--color-bg-page);
  border-top: 1px solid var(--color-border-default);
  
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
    background: var(--color-bg-secondary);
    color: var(--color-text-secondary);
    border: 1px solid var(--color-border-default);
    
    &:hover { background: var(--color-bg-hover); border-color: var(--color-border-strong); }
  }
  
  .btn-confirm {
    background: var(--color-accent-blue);
    color: white;
    border: none;
    box-shadow: var(--shadow-primary-sm);
    
    &:hover {
      background: var(--color-accent-blue-hover);
      transform: translateY(-1px);
      box-shadow: var(--shadow-primary-md);
    }
  }
}
</style>

<!-- 非 scoped 样式，用于影响 Teleport 渲染的 Dialog 和 Drawer -->
<style lang="scss">
.create-group-dialog.el-dialog {
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

// 编辑抽屉样式覆盖
.edit-group-drawer.el-drawer {
  .el-drawer__header {
    margin: 0 !important;
    padding: 0 !important;
    background: var(--color-bg-page) !important;
    border-bottom: 1px solid var(--color-border-default);
    display: flex !important;
    align-items: center !important;
    min-height: 80px;
  }
  
  .el-drawer__close-btn {
    position: relative !important;
    top: auto !important;
    right: auto !important;
    margin-right: 16px;
    color: var(--color-text-tertiary) !important;
    width: 32px !important;
    height: 32px !important;
    flex-shrink: 0;
    display: flex !important;
    align-items: center !important;
    justify-content: center !important;
    border-radius: 6px;
    transition: all 0.2s ease;
    
    .el-icon {
      display: flex;
      align-items: center;
      justify-content: center;
    }
    
    &:hover {
      color: var(--color-text-primary) !important;
      background: var(--color-hover-bg) !important;
    }
  }
  
  .el-drawer__body {
    padding: 0 !important;
    display: flex;
    flex-direction: column;
  }
  
  .el-drawer__footer {
    padding: 0 !important;
    margin-top: auto;
  }
}

// MessageBox 危险按钮样式
.el-message-box {
  .el-button--danger {
    background-color: var(--color-accent-danger) !important;
    border-color: var(--color-accent-danger) !important;
    color: white !important;
    
    &:hover {
      background-color: var(--color-danger-hover) !important;
      border-color: var(--color-danger-hover) !important;
    }
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
      color: var(--color-text-disabled);
    }
    
    &.is-active {
      background: var(--color-status-info-bg);
      color: var(--color-accent-blue);
      
      .menu-icon {
        color: var(--color-accent-blue);
      }
    }
    
    &:hover:not(.is-active) {
      .menu-icon {
        color: var(--color-text-tertiary);
      }
    }
  }
}
</style>
