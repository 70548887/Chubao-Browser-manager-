<script setup lang="ts">
/**
 * @description RPA 机器人管理页面
 * @author DeepAgent
 */

import CreateFlowDialog from './components/CreateFlowDialog.vue'
import { useRPAManagement } from './composables/useRPAManagement'

// 使用 composable 管理所有业务逻辑
const {
  // 状态
  createDialogVisible,
  
  // 计算属性
  filteredRobots,
  
  // 方法
  handleCreate,
  handleCreateSubmit,
  handleImport,
  handleFilter,
  handleEdit,
  handleStop,
  handleStart,
  handleLogs,
  handleViewError,
} = useRPAManagement()
</script>

<template>
  <div class="rpa-management">
    <!-- 工具栏 -->
    <div class="toolbar">
      <div class="stats-wrapper">
        <div class="stats-card">
          <div class="stat-item">
            <div class="stat-icon-box bg-blue">
              <span class="material-symbols-outlined">schema</span>
            </div>
            <div class="stat-info">
              <span class="label">总流程</span>
              <span class="value">24</span>
            </div>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <div class="stat-icon-box bg-emerald relative">
              <span class="pulse-dot">
                <span class="ping"></span>
                <span class="dot"></span>
              </span>
              <span class="material-symbols-outlined">play_arrow</span>
            </div>
            <div class="stat-info">
              <span class="label">运行中</span>
              <span class="value">8</span>
            </div>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <div class="stat-icon-box bg-orange">
              <span class="material-symbols-outlined">bolt</span>
            </div>
            <div class="stat-info">
              <span class="label">节省工时</span>
              <span class="value">128h</span>
            </div>
          </div>
        </div>
      </div>

      <div class="action-buttons">
        <button class="btn-outline" @click="handleFilter">
          <span class="material-symbols-outlined">filter_list</span>
          <span>筛选</span>
        </button>
        <button class="btn-outline" @click="handleImport">
          <span class="material-symbols-outlined">file_upload</span>
          <span>导入</span>
        </button>
        <button class="btn-primary" @click="handleCreate">
          <span class="material-symbols-outlined">add</span>
          <span>创建流程</span>
        </button>
      </div>
    </div>

    <!-- 机器人网格 -->
    <div class="robots-grid">
      <div 
        v-for="robot in filteredRobots" 
        :key="robot.id" 
        class="robot-card group"
      >
        <!-- 侧边装饰点 -->
        <div class="card-dot left"></div>
        <div class="card-dot right"></div>
        
        <!-- 顶部装饰条 -->
        <div class="top-line" :class="robot.gradient"></div>

        <div class="card-header">
          <div class="identity">
            <div class="icon-box" :class="[robot.iconBg, robot.iconColor]">
              <span class="material-symbols-outlined">{{ robot.icon }}</span>
            </div>
            <div class="name-id">
              <h3 class="name">{{ robot.name }}</h3>
              <p class="id">ID: {{ robot.id }}</p>
            </div>
          </div>
          <el-dropdown trigger="click">
            <button class="more-btn">
              <span class="material-symbols-outlined">more_horiz</span>
            </button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item @click="handleEdit(robot)">
                  <span class="material-symbols-outlined dropdown-icon">edit</span>
                  编辑流程
                </el-dropdown-item>
                <el-dropdown-item @click="handleLogs(robot)">
                  <span class="material-symbols-outlined dropdown-icon">history</span>
                  运行日志
                </el-dropdown-item>
                <el-dropdown-item @click="handleFilter">
                  <span class="material-symbols-outlined dropdown-icon">settings</span>
                  流程设置
                </el-dropdown-item>
                <el-dropdown-item divided type="danger" @click="handleStop(robot)">
                  <span class="material-symbols-outlined dropdown-icon">delete</span>
                  删除流程
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>

        <div class="status-section">
          <span 
            class="status-badge" 
            :class="robot.status"
          >
            <span class="status-dot-wrapper" v-if="robot.status === 'running'">
              <span class="ping"></span>
              <span class="dot"></span>
            </span>
            <span class="dot-fixed" v-else></span>
            {{ 
              robot.status === 'running' ? `运行中 (${robot.runningCount}/${robot.totalCount})` : 
              robot.status === 'idle' ? '空闲' : '异常中断'
            }}
          </span>
        </div>

        <div class="card-body">
          <div class="success-rate">
            <div class="rate-header">
              <span>最近成功率</span>
              <span class="rate-val" :class="{ 'error': robot.successRate < 60 }">{{ robot.successRate }}%</span>
            </div>
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :class="robot.status === 'error' ? 'bg-red' : 'bg-emerald'"
                :style="{ width: robot.successRate + '%' }"
              ></div>
            </div>
          </div>

          <div class="info-grid">
            <div class="info-item" :class="{ 'error': robot.status === 'error' }">
              <span class="material-symbols-outlined">{{ robot.status === 'error' ? 'error' : 'schedule' }}</span>
              <span>{{ robot.lastRun }}</span>
            </div>
            <div class="info-item">
              <span class="material-symbols-outlined">format_list_numbered</span>
              <span>{{ robot.steps }} 步骤</span>
            </div>
            <div class="info-item full">
              <span class="material-symbols-outlined">group</span>
              <span class="truncate">{{ robot.group }} ({{ robot.accountCount }} 账号)</span>
            </div>
          </div>
        </div>

        <div class="card-footer">
          <button 
            v-if="robot.status === 'error'"
            class="footer-link error" 
            @click="handleViewError(robot)"
          >
            <span class="material-symbols-outlined">bug_report</span>
            查看错误
          </button>
          <button 
            v-else
            class="footer-link" 
            @click="handleLogs(robot)"
          >
            <span class="material-symbols-outlined">history</span>
            日志
          </button>

          <div class="footer-actions">
            <button class="action-btn" title="编辑" @click="handleEdit(robot)">
              <span class="material-symbols-outlined">edit</span>
            </button>
            <button 
              v-if="robot.status === 'running'"
              class="action-btn stop" 
              title="停止" 
              @click="handleStop(robot)"
            >
              <span class="material-symbols-outlined">stop</span>
            </button>
            <button 
              v-else
              class="action-btn start" 
              title="启动" 
              @click="handleStart(robot)"
            >
              <span class="material-symbols-outlined">play_arrow</span>
            </button>
          </div>
        </div>
      </div>

      <!-- 新建卡片 -->
      <button class="add-card" @click="handleCreate">
        <div class="add-icon-box">
          <span class="material-symbols-outlined">add</span>
        </div>
        <span class="add-title">新建自动化流程</span>
        <span class="add-desc">支持可视化编排与脚本导入</span>
      </button>
    </div>

    <!-- 新建流程对话框 -->
    <CreateFlowDialog 
      :visible="createDialogVisible" 
      @close="createDialogVisible = false"
      @submit="handleCreateSubmit"
    />
  </div>
</template>

<style scoped lang="scss">
.rpa-management {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--color-bg-primary);
  background-image: radial-gradient(var(--color-border-strong) 1px, transparent 1px);
  background-size: 20px 20px;
  padding: 24px;
  gap: 24px;
  transition: background-color var(--duration-normal);
}

// 工具栏
.toolbar {
  display: flex;
  flex-direction: column;
  gap: 16px;

  @media (min-width: 1280px) {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }
}

.stats-card {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 10px 20px;
  background: var(--color-bg-secondary);
  backdrop-filter: blur(8px);
  border: 1px solid var(--color-border-default);
  border-radius: 12px;
  box-shadow: var(--shadow-xs);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);

  .stat-item {
    display: flex;
    align-items: center;
    gap: 12px;

    .stat-icon-box {
      width: 32px;
      height: 32px;
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;

      &.bg-blue { background: var(--color-selected-bg); color: var(--color-accent-blue); }
      &.bg-emerald { background: var(--color-status-success-bg); color: var(--color-accent-success); }
      &.bg-orange { background: var(--color-status-warning-bg); color: var(--color-accent-warning); }

      .material-symbols-outlined { font-size: 20px; }
    }

    .stat-info {
      display: flex;
      flex-direction: column;
      
      .label { font-size: 11px; color: var(--color-text-tertiary); transition: color var(--duration-normal); }
      .value { font-size: 16px; font-weight: 700; color: var(--color-text-primary); line-height: 1.2; transition: color var(--duration-normal); }
    }
  }

  .stat-divider {
    width: 1px;
    height: 32px;
    background: var(--color-border-default);
    transition: background-color var(--duration-normal);
  }
}

.pulse-dot {
  position: absolute;
  top: 0;
  right: 0;
  margin-top: -4px;
  margin-right: -4px;
  display: flex;
  height: 10px;
  width: 10px;

  .ping {
    position: absolute;
    display: inline-flex;
    height: 100%;
    width: 100%;
    border-radius: 9999px;
    background-color: var(--color-accent-success);
    opacity: 0.75;
    animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
  }

  .dot {
    position: relative;
    display: inline-flex;
    border-radius: 9999px;
    height: 10px;
    width: 10px;
    background-color: var(--color-accent-success);
  }
}

@keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn-outline {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 42px;
  padding: 0 16px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 10px;
  color: var(--color-text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--shadow-xs);

  .material-symbols-outlined { color: var(--color-text-tertiary); font-size: 20px; }

  &:hover {
    background: var(--color-hover-bg);
    border-color: var(--color-border-strong);
    color: var(--color-text-primary);
  }
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 8px;
  height: 42px;
  padding: 0 20px;
  background: linear-gradient(to right, var(--color-accent-blue), var(--color-accent-blue-hover));
  border: none;
  border-radius: 10px;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);

  &:hover {
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(37, 99, 235, 0.4);
  }

  &:active { transform: scale(0.98); }
}

// 机器人网格
.robots-grid {
  display: grid;
  grid-template-columns: repeat(1, minmax(0, 1fr));
  gap: 24px;
  animation: fade-in-up 0.5s ease-out;

  @media (min-width: 768px) { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  @media (min-width: 1280px) { grid-template-columns: repeat(3, minmax(0, 1fr)); }
  @media (min-width: 1536px) { grid-template-columns: repeat(4, minmax(0, 1fr)); }
}

@keyframes fade-in-up {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}


.robot-card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    box-shadow: var(--shadow-lg);
    transform: translateY(-4px);
  }

  .top-line {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background-image: linear-gradient(to right, var(--tw-gradient-stops));
    
    &.from-blue-500 { background: var(--gradient-brand-blue); }
    &.from-orange-400 { background: var(--gradient-brand-orange); }
    &.from-pink-500 { background: var(--gradient-brand-pink); }
    &.from-purple-500 { background: var(--gradient-brand-purple); }
    &.from-red-500 { background: var(--gradient-brand-red); }
  }

  .card-dot {
    position: absolute;
    top: 50%;
    width: 12px;
    height: 12px;
    background: var(--color-bg-overlay);
    border-radius: 50%;
    border: 1px solid var(--color-border-default);
    z-index: 5;
    opacity: 0;
    transition: opacity 0.2s;

    &.left { left: -6px; transform: translateY(-50%); }
    &.right { right: -6px; transform: translateY(-50%); }
  }

  &:hover .card-dot { opacity: 1; }
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;

  .identity {
    display: flex;
    align-items: center;
    gap: 12px;

    .icon-box {
      width: 48px;
      height: 48px;
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      border: 1px solid transparent;

      .material-symbols-outlined { font-size: 26px; }
    }

    .name-id {
      .name { font-size: 15px; font-weight: 700; color: var(--color-text-primary); margin: 0; line-height: 1.2; transition: color var(--duration-normal); }
      .id { font-size: 11px; color: var(--color-text-tertiary); margin-top: 2px; font-family: monospace; transition: color var(--duration-normal); }
    }
  }

  .more-btn {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-text-tertiary);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;

    &:hover { background: var(--color-hover-bg); color: var(--color-text-secondary); }
  }

  :deep(.el-dropdown) {
    line-height: 1;
  }
}

.dropdown-icon {
  font-size: 18px !important;
  margin-right: 8px;
  opacity: 0.7;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 600;
  border: 1px solid transparent;

  &.running {
    background: var(--color-status-success-bg);
    color: var(--color-accent-success);
    border-color: var(--color-status-success-border);
  }

  &.idle {
    background: var(--color-bg-overlay);
    color: var(--color-text-secondary);
    border-color: var(--color-border-default);
  }

  &.error {
    background: var(--color-status-error-bg);
    color: var(--color-accent-danger);
    border-color: var(--color-status-error-border);
  }

  .status-dot-wrapper {
    position: relative;
    display: flex;
    width: 8px;
    height: 8px;

    .ping {
      position: absolute;
      display: inline-flex;
      height: 100%;
      width: 100%;
      border-radius: 9999px;
      background-color: var(--color-accent-success);
      opacity: 0.75;
      animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
    }

    .dot {
      position: relative;
      display: inline-flex;
      border-radius: 9999px;
      height: 8px;
      width: 8px;
      background-color: var(--color-accent-success);
    }
  }

  .dot-fixed {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: currentColor;
    opacity: 0.5;
  }
}

.card-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-top: 16px;

  .success-rate {
    .rate-header {
      display: flex;
      justify-content: space-between;
      font-size: 12px;
      margin-bottom: 6px;
      color: var(--color-text-tertiary);
      transition: color var(--duration-normal);

      .rate-val { font-weight: 700; color: var(--color-text-secondary); transition: color var(--duration-normal); }
      .rate-val.error { color: var(--color-status-error); }
    }

    .progress-bar {
      height: 6px;
      background: var(--color-bg-overlay);
      border-radius: 999px;
      overflow: hidden;
      transition: background-color var(--duration-normal);

      .progress-fill {
        height: 100%;
        border-radius: 999px;
        transition: width 0.5s ease;

        &.bg-emerald { background: var(--color-accent-success); }
        &.bg-red { background: var(--color-status-error); }
      }
    }
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 8px;

    .info-item {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 12px;
      color: var(--color-text-tertiary);
      transition: color var(--duration-normal);

      .material-symbols-outlined { font-size: 16px; opacity: 0.7; }
      &.full { grid-column: span 2; }
      &.error { color: var(--color-status-error); .material-symbols-outlined { color: var(--color-status-error); opacity: 1; } }
    }
  }
}

.card-footer {
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--color-border-subtle);
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: border-color var(--duration-normal);

  .footer-link {
    font-size: 12px;
    font-weight: 600;
    color: var(--color-text-tertiary);
    background: transparent;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-radius: 6px;
    transition: all 0.2s;

    .material-symbols-outlined { font-size: 16px; }

    &:hover { background: var(--color-hover-bg); color: var(--color-accent-blue); }
    &.error { color: var(--color-status-error); &:hover { background: var(--color-status-error-bg); } }
  }

  .footer-actions {
    display: flex;
    gap: 8px;

    .action-btn {
      width: 36px;
      height: 36px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--color-text-tertiary);
      background: transparent;
      border: 1px solid transparent;
      cursor: pointer;
      transition: all 0.2s;

      .material-symbols-outlined { font-size: 20px; }

      &:hover { background: var(--color-hover-bg); color: var(--color-text-secondary); }

      &.stop {
        color: var(--color-status-error);
        background: var(--color-status-error-bg);
        border-color: var(--color-status-error-border);
        &:hover { background: var(--color-status-error-border); }
      }

      &.start {
        color: var(--color-accent-blue);
        background: var(--color-selected-bg);
        border-color: var(--color-border-interactive);
        &:hover { background: var(--color-active-bg); }
      }
    }
  }
}

.add-card {
  border: 2px dashed var(--color-border-default);
  border-radius: 16px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: var(--color-bg-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  min-height: 280px;

  .add-icon-box {
    width: 64px;
    height: 64px;
    border-radius: 50%;
    background: var(--color-bg-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 16px;
    color: var(--color-text-tertiary);
    transition: all 0.2s;

    .material-symbols-outlined { font-size: 32px; }
  }

  .add-title { font-size: 18px; font-weight: 600; color: var(--color-text-tertiary); transition: color var(--duration-normal); }
  .add-desc { font-size: 13px; color: var(--color-text-tertiary); margin-top: 8px; }

  &:hover {
    border-color: var(--color-accent-blue);
    background: var(--color-selected-bg);
    
    .add-icon-box { background: var(--color-active-bg); color: var(--color-accent-blue); transform: scale(1.1); }
    .add-title { color: var(--color-accent-blue); }
  }
}
</style>
