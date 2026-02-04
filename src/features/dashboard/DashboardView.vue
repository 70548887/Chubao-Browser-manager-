<script setup lang="ts">
/**
 * @description 仪表盘主视图 - 环境列表 (桌面原生化)
 * @author DeepAgent
 */

import { ref, onMounted, inject, onUnmounted } from 'vue'
import type { Profile } from '@/types/profile.types'
import ListHeader from './components/ListHeader.vue'
import ProfileRow from './components/ProfileRow.vue'
import ProfileDrawer from '@/features/profile-editor/ProfileDrawer.vue'
import BatchResultDialog from '@/components/BatchResultDialog.vue'
import ArrangeWindowsDialog from '@/components/ArrangeWindowsDialog.vue'
import LaunchProgressDialog from '@/components/LaunchProgressDialog.vue'
import { useDashboard } from './composables/useDashboard'
import { useLaunchProgress } from '@/composables/useLaunchProgress'

const navigateTo = inject<(page: string) => void>('navigateTo')

// 定义 emit 事件
const emit = defineEmits<{
  (e: 'create-new-window'): void
}>()

// 排列窗口弹窗状态
const arrangeDialogVisible = ref(false)

// 使用 composable 管理所有业务逻辑
const {
  // Stores
  groupStore,

  // 状态
  isLoading,
  selectedIds,
  filterStatus,
  filterGroup,
  sortField,
  sortOrder,
  currentPage,
  pageSize,
  pageSizes,
  drawerVisible,
  editingProfile,
  showNotification,
  batchResultVisible,
  batchResultTitle,
  batchResultData,
  moveGroupDialogVisible,
  targetGroupId,

  // 计算属性
  filteredProfiles,
  paginatedProfiles,
  totalCount,
  isAllSelected,
  profiles,
  runningCount,

  // 方法
  closeNotification,
  handleSelectAll,
  handleSelect,
  handleSort,
  handleLaunch,
  handleStop,
  handleEdit,
  handleDelete,
  handleCreateNew,
  handleDrawerSuccess,
  handleEditSubmit,
  handleBatchLaunch,
  handleBatchStop,
  handleBatchCommand,
  handleClone,
  handleExport,
  handleExportCookie,
  handleClearCache,
  confirmMoveGroup,
  initDashboard,
  setOnBeforeLaunch,
} = useDashboard(navigateTo)

// 启动进度对话框
const {
  visible: launchProgressVisible,
  currentProfileName: launchProfileName,
  progress: launchProgress,
  error: launchError,
  steps: launchSteps,
  startLaunch,
  closeLaunch,
  cancelLaunch,
  stopListening: stopProgressListening,
} = useLaunchProgress()

// 设置启动前回调
setOnBeforeLaunch((profileId, profileName) => {
  startLaunch(profileId, profileName)
})

// ==================== 生命周期 ====================
onMounted(async () => {
  await initDashboard()
})

onUnmounted(() => {
  stopProgressListening()
})

// 暴露方法给父组件
defineExpose({
  handleCreateNew,
  handleDrawerSuccess,
})
</script>

<template>
  <div class="dashboard-view">
    <!-- 内容区域 - 原型 p-6 space-y-5 -->
    <div class="content-area">
      <!-- 通知横幅 -->
      <div v-if="showNotification" class="notification-banner">
        <span class="material-symbols-outlined banner-icon">info</span>
        <span class="banner-text">当前您使用的是免费版，可以免费使用个人账号管理功能，升级后可使用团队协作、云端同步等功能</span>
        <button class="banner-close-btn" @click="closeNotification">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>

      <!-- 工具栏 -->
      <div class="toolbar">
        <!-- 统计栏 -->
        <div class="stats-card">
          <div class="stat-item">
            <span class="stat-dot stat-dot-neutral"></span>
            <span class="stat-text">总数: <strong>{{ profiles.length }}</strong></span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-dot stat-dot-running"></span>
            <span class="stat-text">运行中: <strong>{{ runningCount }}</strong></span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-dot stat-dot-error"></span>
            <span class="stat-text">异常: <strong>0</strong></span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-dot stat-dot-stopped"></span>
            <span class="stat-text">停止: <strong>{{ profiles.length - runningCount }}</strong></span>
          </div>
        </div>

        <!-- 操作栏 -->
        <div class="toolbar-right">
          <!-- 左侧：筛选器 -->
          <div class="filter-group">
            <el-select v-model="filterStatus" size="default" placeholder="所有状态">
              <el-option label="所有状态" value="all" />
              <el-option label="运行中" value="running" />
              <el-option label="已停止" value="stopped" />
            </el-select>
            <el-select v-model="filterGroup" size="default" placeholder="所有分组">
              <el-option label="所有分组" value="all" />
              <el-option v-for="group in groupStore.sortedGroups" :key="group.id"
                :label="`${group.name} (${group.profileCount || 0})`" :value="group.id" />
            </el-select>
          </div>

          <!-- 右侧：批量操作按钮 -->
          <div class="batch-actions">
            <button class="batch-btn batch-start" @click="handleBatchLaunch">
              <span class="material-symbols-outlined">play_arrow</span>
              <span>批量启动</span>
            </button>
            <button class="batch-btn batch-stop" @click="handleBatchStop">
              <span class="material-symbols-outlined">stop</span>
              <span>批量停止</span>
            </button>
            <button class="batch-btn batch-arrange" @click="arrangeDialogVisible = true">
              <span class="material-symbols-outlined">grid_view</span>
              <span>排列窗口</span>
            </button>

            <!-- 更多操作下拉菜单 -->
            <el-dropdown trigger="click" @command="handleBatchCommand">
              <button class="batch-btn batch-more">
                <span class="material-symbols-outlined">more_horiz</span>
                <span>更多</span>
                <span class="material-symbols-outlined dropdown-icon">arrow_drop_down</span>
              </button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="delete">
                    <span class="material-symbols-outlined">delete</span>
                    <span>批量删除</span>
                  </el-dropdown-item>
                  <el-dropdown-item command="move">
                    <span class="material-symbols-outlined">drive_file_move</span>
                    <span>移动分组</span>
                  </el-dropdown-item>
                  <el-dropdown-item command="duplicate">
                    <span class="material-symbols-outlined">content_copy</span>
                    <span>批量复制</span>
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
        </div>
      </div>

      <!-- 列表 -->
      <div class="list-container" v-loading="isLoading">
        <ListHeader :is-all-selected="isAllSelected" :sort-field="sortField" :sort-order="sortOrder"
          @select-all="handleSelectAll" @sort="handleSort" />

        <div class="list-body">
          <ProfileRow v-for="profile in paginatedProfiles" :key="profile.id" :profile="profile" :index="profile.index"
            :is-selected="selectedIds.has(profile.id)" @select="handleSelect" @launch="handleLaunch" @stop="handleStop"
            @edit="handleEdit" @delete="handleDelete" @clone="handleClone" @export="handleExport" 
            @export-cookie="handleExportCookie" @clear-cache="handleClearCache" />

          <!-- 空状态 -->
          <div v-if="!isLoading && filteredProfiles.length === 0" class="empty-state">
            <el-icon class="empty-icon">
              <Files />
            </el-icon>
            <p class="empty-text">暂无窗口数据</p>
            <button class="empty-action" @click="emit('create-new-window')">
              <el-icon>
                <Plus />
              </el-icon>
              <span>创建新窗口</span>
            </button>
          </div>
        </div>

        <!-- 分页 -->
        <div v-if="totalCount > 0" class="pagination-container">
          <div class="pagination-info">
            共 <strong>{{ totalCount }}</strong> 条
          </div>
          <el-pagination v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="pageSizes"
            :total="totalCount" :pager-count="7" layout="sizes, prev, pager, next, jumper" background size="small" />
        </div>
      </div>
    </div>

    <!-- 编辑窗口抽屉（右侧展开） -->
    <ProfileDrawer v-model:visible="drawerVisible" :profile="editingProfile" @success="handleDrawerSuccess" />

    <!-- 批量操作结果对话框 -->
    <BatchResultDialog v-model:visible="batchResultVisible" :title="batchResultTitle" :result="batchResultData" />

    <!-- 批量移动分组对话框 -->
    <el-dialog v-model="moveGroupDialogVisible" title="批量移动到分组" width="480px" :close-on-click-modal="false">
      <div class="move-group-dialog-body">
        <div class="dialog-info">
          <span class="material-symbols-outlined">info</span>
          <p>已选中 <strong>{{ selectedIds.size }}</strong> 个环境，请选择目标分组</p>
        </div>

        <div class="form-group">
          <label class="form-label">目标分组</label>
          <el-select v-model="targetGroupId" placeholder="请选择分组" size="large" style="width: 100%">
            <el-option v-for="group in groupStore.sortedGroups" :key="group.id"
              :label="`${group.name} (${group.profileCount || 0} 个环境)`" :value="group.id">
              <div style="display: flex; align-items: center; gap: 8px;">
                <span class="material-symbols-outlined"
                  style="font-size: 18px; color: var(--color-text-tertiary);">folder</span>
                <span>{{ group.name }}</span>
                <span style="color: var(--color-text-tertiary); font-size: 12px;">({{ group.profileCount || 0 }})</span>
              </div>
            </el-option>
          </el-select>
        </div>
      </div>

      <template #footer>
        <div class="dialog-footer">
          <el-button @click="moveGroupDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="confirmMoveGroup">确定移动</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 排列窗口对话框 -->
    <ArrangeWindowsDialog v-model:visible="arrangeDialogVisible" :running-count="runningCount" />

    <!-- 启动进度对话框 -->
    <LaunchProgressDialog
      :visible="launchProgressVisible"
      :profile-name="launchProfileName"
      :progress="launchProgress"
      :steps="launchSteps"
      :error="launchError"
      @close="closeLaunch"
      @cancel="cancelLaunch"
    />
  </div>
</template>

<style scoped lang="scss">
.dashboard-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--color-bg-primary);
  overflow: hidden;
  transition: background-color var(--duration-normal);
}

// 内容区域 - 对应原型 flex-1 overflow-auto p-6 space-y-5
.content-area {
  flex: 1;
  min-height: 0; // 重要：允许收缩
  overflow: hidden; // 不在这里滚动，让 list-body 内部滚动
  padding: 24px 24px 0 24px; // 底部无 padding，让分页贴底
  display: flex;
  flex-direction: column;
  gap: 20px; // space-y-5 = 1.25rem = 20px
}

// 通知横幅 - 品牌色（保持蓝色系）
.notification-banner {
  flex-shrink: 0; // 不收缩
  display: flex;
  align-items: center;
  gap: 12px; // gap-3
  padding: 12px 16px; // px-4 py-3
  background: var(--color-selected-bg);
  border: 1px solid var(--color-border-interactive);
  border-radius: 8px;
  transition: background-color var(--duration-normal), border-color var(--duration-normal);

  .banner-icon {
    font-size: 20px;
    color: var(--color-accent-blue);
  }

  .banner-text {
    flex: 1;
    font-size: 14px; // text-sm
    color: var(--color-accent-blue);
  }

  .banner-close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--color-accent-blue-hover);
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      color: var(--color-accent-blue);
      background: var(--color-active-bg);
    }

    .material-symbols-outlined {
      font-size: 18px;
    }
  }
}

// 工具栏 - 响应式布局（空间不够时上下两行，空间够时一行）
.toolbar {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-start; // 子元素不拉伸
  gap: 12px;

  // 大屏时横向排列
  @media (min-width: 1400px) {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  align-self: stretch; // 操作栏通栏

  // 小屏时左右分布
  @media (max-width: 1399px) {
    justify-content: space-between;
  }
}

// 统计卡片 - 支持主题切换
.dashboard-view .stats-card,
.stats-card {
  display: inline-flex;
  align-items: center;
  gap: 24px;
  padding: 10px 20px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  box-shadow: var(--shadow-md);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);

  .stat-item {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    color: var(--color-text-tertiary);
    transition: color var(--duration-normal);

    strong {
      color: var(--color-text-primary);
      font-weight: 700;
      transition: color var(--duration-normal);
    }
  }

  .stat-divider {
    width: 1px;
    height: 16px;
    background: var(--color-border-default);
    transition: background-color var(--duration-normal);
  }

  .stat-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;

    &.stat-dot-neutral {
      background: var(--color-border-strong);
    }

    &.stat-dot-running {
      background: var(--color-status-running);
      animation: pulse-dot 2s infinite;
    }

    &.stat-dot-error {
      background: var(--color-status-error);
    }

    &.stat-dot-stopped {
      background: var(--color-text-tertiary);
    }
  }
}

@keyframes pulse-dot {

  0%,
  100% {
    box-shadow: 0 0 0 rgba(34, 197, 94, 0.4);
  }

  50% {
    box-shadow: 0 0 8px rgba(34, 197, 94, 0.8);
  }
}

// 筛选组
.filter-group {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;

  :deep(.el-select) {
    width: 140px;

    .el-input__wrapper {
      background: var(--color-bg-secondary);
      border: 1px solid var(--color-border-default);
      border-radius: 8px;
      box-shadow: var(--shadow-xs);
      height: 36px;
      transition: background-color var(--duration-normal), border-color var(--duration-normal);

      &:hover {
        border-color: var(--color-border-strong);
      }

      &.is-focus {
        border-color: var(--color-accent-blue);
        box-shadow: 0 0 0 2px rgba(37, 99, 235, 0.1);
      }
    }
  }
}

// 工具栏分隔线
.toolbar-divider {
  width: 1px;
  height: 24px;
  background: var(--color-border-default);
  margin: 0 4px;
  transition: background-color var(--duration-normal);
}

// 批量操作按钮
.batch-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.batch-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 36px;
  padding: 0 16px;
  border: 1px solid;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--shadow-xs);
  white-space: nowrap;

  .material-symbols-outlined {
    font-size: 18px;
  }

  &.batch-start {
    background: var(--color-status-success-bg);
    border-color: var(--color-status-success-border);
    color: var(--color-status-success-text);

    &:hover {
      background: var(--color-status-success-border);
      border-color: var(--color-accent-success);
      transform: translateY(-1px);
      box-shadow: 0 4px 6px rgba(16, 185, 129, 0.2);
    }
  }

  &.batch-stop {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-default);
    color: var(--color-text-tertiary);

    &:hover {
      color: var(--color-status-error);
      border-color: var(--color-status-error-border);
      background: var(--color-status-error-bg);
      transform: translateY(-1px);
      box-shadow: 0 4px 6px rgba(239, 68, 68, 0.15);
    }
  }

  &.batch-arrange {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-default);
    color: var(--color-text-tertiary);

    &:hover {
      color: var(--color-accent-blue);
      border-color: var(--color-border-interactive);
      background: var(--color-selected-bg);
      transform: translateY(-1px);
      box-shadow: 0 4px 6px rgba(37, 99, 235, 0.15);
    }
  }

  &.batch-delete {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-default);
    color: var(--color-text-tertiary);

    &:hover {
      color: var(--color-accent-danger);
      border-color: var(--color-status-error-border);
      background: var(--color-status-error-bg);
      transform: translateY(-1px);
      box-shadow: 0 4px 6px rgba(220, 38, 38, 0.2);
    }
  }

  &.batch-move {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-default);
    color: var(--color-text-tertiary);

    &:hover {
      color: var(--color-accent-blue);
      border-color: var(--color-border-interactive);
      background: var(--color-selected-bg);
      transform: translateY(-1px);
      box-shadow: 0 4px 6px rgba(37, 99, 235, 0.15);
    }
  }

  &.batch-duplicate {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-default);
    color: var(--color-text-tertiary);

    &:hover {
      color: #7c3aed;
      border-color: #ddd6fe;
      background: #f5f3ff;
      transform: translateY(-1px);
      box-shadow: 0 4px 6px rgba(124, 58, 237, 0.15);
    }
  }

  &.batch-more {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-default);
    color: var(--color-text-tertiary);

    .dropdown-icon {
      font-size: 18px;
      margin-left: -2px;
    }

    &:hover {
      color: var(--color-text-primary);
      border-color: var(--color-border-strong);
      background: var(--color-hover-bg);
      transform: translateY(-1px);
      box-shadow: var(--shadow-sm);
    }
  }

  &:active {
    transform: translateY(0);
  }
}

// Element Plus 下拉菜单项样式
:deep(.el-dropdown-menu__item) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;

  .material-symbols-outlined {
    font-size: 18px;
    color: var(--color-text-tertiary);
  }

  &:hover {
    background: var(--color-hover-bg);

    .material-symbols-outlined {
      color: var(--color-accent-blue);
    }
  }
}

// 列表容器 - 支持主题切换
.dashboard-view .list-container,
.list-container {
  flex: 1;
  min-height: 0; // 重要：允许 flex 子元素收缩
  display: flex;
  flex-direction: column;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 12px; // rounded-xl
  box-shadow: var(--shadow-md);
  overflow: hidden; // 确保圆角不被内容溢出
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.list-body {
  flex: 1;
  min-height: 0; // 重要：允许收缩
  overflow-y: auto;
  background: var(--color-bg-secondary);
  transition: background-color var(--duration-normal);

  // 自定义滚动条
  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: var(--color-border-strong);
    border-radius: 3px;

    &:hover {
      background: var(--color-text-tertiary);
    }
  }
}

// 空状态
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 0;

  .empty-icon {
    font-size: 48px;
    color: var(--color-text-tertiary);
    margin-bottom: 16px;
  }

  .empty-text {
    color: var(--color-text-tertiary);
    font-size: 14px;
    margin-bottom: 16px;
  }

  .empty-action {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 20px;
    background: var(--color-accent-blue);
    border: none;
    border-radius: 8px;
    color: white;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;

    &:hover {
      filter: brightness(1.1);
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
    }

    &:active {
      transform: translateY(0);
    }
  }
}

// 分页区域 - 支持主题切换
.pagination-container {
  margin-top: auto; // 关键：推到底部
  flex-shrink: 0; // 不收缩
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  border-top: 1px solid var(--color-border-default);
  background: var(--color-bg-overlay);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
  min-height: 48px;
  flex-wrap: wrap;
  gap: 8px 16px;

  .pagination-info {
    color: var(--color-text-tertiary);
    font-size: 12px;
    white-space: nowrap;
  }
  
  // 小屏幕适配
  @media (max-width: 768px) {
    padding: 10px 16px;
    justify-content: center;
    
    .pagination-info {
      display: none;
    }
  }

  :deep(.el-pagination) {
    display: flex;
    align-items: center;
    gap: 8px;

    .el-pagination__sizes {
      .el-input__wrapper {
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-border-default);
        border-radius: 6px;
        height: 32px;
        padding: 0 8px;
      }
    }

    .btn-prev,
    .btn-next {
      width: 32px;
      height: 32px;
      background: transparent;
      border: none;
      color: var(--color-text-tertiary);
      border-radius: 6px;

      &:hover:not(:disabled) {
        background: var(--color-hover-bg);
        color: var(--color-text-primary);
      }

      &:disabled {
        opacity: 0.4;
        cursor: not-allowed;
      }
    }

    .el-pager {
      display: flex;
      gap: 4px;

      li {
        min-width: 28px;
        height: 28px;
        line-height: 28px;
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-border-default);
        border-radius: 6px;
        color: var(--color-text-tertiary);
        font-size: 12px;
        font-weight: 500;
        margin: 0;
        transition: all 0.2s;

        &:hover {
          background: var(--color-hover-bg);
          color: var(--color-text-primary);
          border-color: var(--color-border-strong);
        }

        &.is-active {
          background: var(--color-accent-blue);
          color: white;
          border-color: var(--color-accent-blue);
          box-shadow: 0 2px 4px rgba(37, 99, 235, 0.2);
        }

        &.more {
          border: none;
          background: transparent;
        }
      }
    }
  }
}

// 批量移动分组对话框样式
.move-group-dialog-body {
  .dialog-info {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px 16px;
    background: var(--color-selected-bg);
    border: 1px solid var(--color-border-interactive);
    border-radius: 8px;
    margin-bottom: 24px;

    .material-symbols-outlined {
      font-size: 20px;
      color: var(--color-accent-blue);
      margin-top: 2px;
    }

    p {
      flex: 1;
      margin: 0;
      font-size: 14px;
      color: var(--color-accent-blue);
      line-height: 1.5;

      strong {
        font-weight: 700;
        color: var(--color-accent-blue);
      }
    }
  }

  .form-group {
    .form-label {
      display: block;
      font-size: 14px;
      font-weight: 600;
      color: var(--color-text-primary);
      margin-bottom: 8px;
    }
  }
}
</style>

<!-- 非 scoped 样式，用于影响 MessageBox -->
<style lang="scss">
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
</style>
