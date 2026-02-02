<script setup lang="ts">
/**
 * @description 代理管理页面
 * @author DeepAgent
 */

import { useProxyManagement, IP_CHECK_CHANNELS } from './composables/useProxyManagement'

// 使用 composable 管理所有业务逻辑
const {
  // 状态
  isLoading,
  selectedIds,
  filterType,
  currentPage,
  pageSize,
  pageSizes,
  dialogVisible,
  dialogTitle,
  isEditMode,
  importMode,
  batchInput,
  testStatus,
  formData,
  
  // 计算属性
  activeCount,
  errorCount,
  paginatedProxies,
  totalCount,
  isAllSelected,
  
  // 方法
  handleSelectAll,
  handleAdd,
  handleEdit,
  handleCheck,
  handleDelete,
  handleBatchCheck,
  handleBatchDelete,
  handleTestConnection,
  handleSubmit,
  
  // 辅助函数
  getProxyIcon,
  getProxyIconClass,
  getCountryFlag,
  getStatusClass,
  getStatusText,
  getTypeClass
} = useProxyManagement()
</script>

<template>
  <div class="proxy-management">
    <!-- 工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <!-- 统计信息 -->
        <div class="stats-card-single">
          <div class="stat-item">
            <span class="stat-dot stat-dot-neutral"></span>
            <span class="stat-text">总数: <strong>{{ totalCount }}</strong></span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-dot stat-dot-running"></span>
            <span class="stat-text">连通: <strong>{{ activeCount }}</strong></span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-dot stat-dot-error"></span>
            <span class="stat-text">异常: <strong>{{ errorCount }}</strong></span>
          </div>
        </div>
      </div>
      
      <div class="toolbar-right">
        <!-- 协议筛选 -->
        <div class="filter-dropdown">
          <el-select v-model="filterType" size="default" placeholder="所有协议" clearable>
            <el-option label="所有协议" value="" />
            <el-option label="HTTP" value="http" />
            <el-option label="HTTPS" value="https" />
            <el-option label="SOCKS5" value="socks5" />
          </el-select>
        </div>
        
        <div class="toolbar-divider"></div>
        
        <!-- 操作按钮 -->
        <div class="action-buttons">
          <!-- 批量删除按钮（多选时显示） -->
          <button 
            v-if="selectedIds.length > 0" 
            class="btn-danger" 
            @click="handleBatchDelete"
            :title="`删除选中的 ${selectedIds.length} 个代理`"
          >
            <span class="material-symbols-outlined">delete</span>
            <span>批量删除 ({{ selectedIds.length }})</span>
          </button>
          
          <button class="btn-primary" @click="handleAdd">
            <span class="material-symbols-outlined">add</span>
            <span>添加代理</span>
          </button>
          <button class="btn-outline" @click="handleBatchCheck">
            <span class="material-symbols-outlined">network_check</span>
            <span>批量检查</span>
          </button>
        </div>
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
        <div class="col-name">代理名称</div>
        <div class="col-proto">协议</div>
        <div class="col-host">代理主机</div>
        <div class="col-port">端口</div>
        <div class="col-exit">出口 IP</div>
        <div class="col-status">连通性</div>
        <div class="col-bind text-center">绑定窗口</div>
        <div class="col-actions text-right">操作</div>
      </div>
      
      <!-- 列表主体 -->
      <div class="list-body">
        <div 
          v-for="proxy in paginatedProxies" 
          :key="proxy.id" 
          class="list-row-grid group"
        >
          <div class="col-cb">
            <input type="checkbox" class="custom-checkbox" v-model="selectedIds" :value="proxy.id" />
          </div>
          
          <div class="col-index">
            <span class="index-num">{{ proxy.index }}</span>
          </div>
          
          <div class="col-name">
            <div class="proxy-identity">
              <div class="icon-box" :class="getProxyIconClass(proxy)">
                <span class="material-symbols-outlined">{{ getProxyIcon(proxy) }}</span>
              </div>
              <div class="text-info">
                <p class="main-name">{{ proxy.name || (proxy.host + ':' + proxy.port) }}</p>
                <p class="sub-id">ID: {{ proxy.id.slice(0, 8) }}</p>
              </div>
            </div>
          </div>
          
          <div class="col-proto">
            <span class="proto-tag" :class="getTypeClass(proxy.type)">
              {{ proxy.type.toUpperCase() }}
            </span>
          </div>
          
          <div class="col-host">
            <p class="font-mono text-sm">{{ proxy.host }}</p>
          </div>
          
          <div class="col-port">
            <p class="font-mono text-sm">{{ proxy.port }}</p>
          </div>
          
          <div class="col-exit">
            <div class="ip-info" v-if="proxy.ipAddress">
              <div class="ip-row">
                <img 
                  v-if="proxy.location"
                  :src="getCountryFlag(proxy.location)" 
                  class="flag-img"
                  :alt="proxy.location"
                />
                <span class="ip-addr">{{ proxy.ipAddress }}</span>
              </div>
              <p class="loc-text">{{ proxy.location || '未知地区' }}</p>
            </div>
            <div class="ip-empty" v-else>
              <span class="material-symbols-outlined">help</span>
              <span>-</span>
            </div>
          </div>
          
          <div class="col-status">
            <span 
              class="status-pill" 
              :class="getStatusClass(proxy)"
            >
              <span class="dot" v-if="proxy.status === 'active'"></span>
              <span class="material-symbols-outlined" v-else-if="proxy.status === 'error'">error</span>
              {{ getStatusText(proxy) }}
            </span>
          </div>
          
          <div class="col-bind text-center">
            <span class="bind-badge" :class="{ 'active': (proxy.usedCount ?? 0) > 0 }">
              {{ proxy.usedCount || '-' }}
            </span>
          </div>
          
          <div class="col-actions text-right">
            <div class="row-actions">
              <button class="act-btn edit" title="编辑" @click="handleEdit(proxy)">
                <span class="material-symbols-outlined">edit</span>
              </button>
              <button class="act-btn check" title="检查" @click="handleCheck(proxy)">
                <span class="material-symbols-outlined">refresh</span>
              </button>
              <button class="act-btn delete" title="删除" @click="handleDelete(proxy)">
                <span class="material-symbols-outlined">delete</span>
              </button>
            </div>
          </div>
        </div>
        
        <!-- 空状态 -->
        <div v-if="!isLoading && totalCount === 0" class="empty-view">
          <span class="material-symbols-outlined">dns</span>
          <p>暂无代理数据</p>
          <button class="btn-empty" @click="handleAdd">添加代理</button>
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

    <!-- 添加/编辑弹窗 -->
    <el-dialog 
      v-model="dialogVisible" 
      :title="dialogTitle" 
      :width="isEditMode ? '480px' : '520px'" 
      :close-on-click-modal="false" 
      class="proxy-dialog"
    >
      <!-- 编辑模式表单 -->
      <div v-if="isEditMode" class="edit-form">
        <div class="form-group">
          <label class="form-label">代理名称</label>
          <input 
            v-model="formData.tag" 
            class="form-input" 
            placeholder="US-Res-Premium-01" 
          />
        </div>
        
        <div class="form-row">
          <div class="form-group">
            <label class="form-label">协议类型</label>
            <el-select v-model="formData.type" class="form-select" style="width: 100%">
              <el-option label="SOCKS5" value="socks5" />
              <el-option label="HTTP" value="http" />
              <el-option label="HTTPS" value="https" />
            </el-select>
          </div>
          <div class="form-group">
            <label class="form-label">端口</label>
            <input 
              v-model="formData.port" 
              class="form-input" 
              placeholder="50001" 
            />
          </div>
        </div>
        
        <div class="form-group">
          <label class="form-label">代理主机 / IP</label>
          <input 
            v-model="formData.host" 
            class="form-input" 
            placeholder="proxy.example.com" 
          />
        </div>
        
        <div class="form-group">
          <label class="form-label">IP查询渠道 <span class="optional">(可选)</span></label>
          <el-select v-model="formData.ipCheckChannel" class="form-select" style="width: 100%">
            <el-option 
              v-for="channel in IP_CHECK_CHANNELS" 
              :key="channel.value" 
              :label="channel.label" 
              :value="channel.value" 
            />
          </el-select>
        </div>
        
        <div class="form-row">
          <div class="form-group">
            <label class="form-label">用户名 <span class="optional">(可选)</span></label>
            <input 
              v-model="formData.username" 
              class="form-input" 
              placeholder="user123" 
            />
          </div>
          <div class="form-group">
            <label class="form-label">密码 <span class="optional">(可选)</span></label>
            <input 
              v-model="formData.password" 
              class="form-input" 
              type="password" 
              placeholder="••••••" 
            />
          </div>
        </div>
        
        <div class="form-group">
          <label class="form-label">备注 <span class="optional">(可选)</span></label>
          <textarea 
            v-model="formData.remark" 
            class="form-textarea" 
            placeholder="备注信息" 
            rows="2"
          />
        </div>
      </div>
      
      <!-- 添加模式表单 -->
      <div v-else class="add-form">
        <!-- 单个添加/批量导入切换 -->
        <div class="mode-switch">
          <button 
            :class="['mode-btn', { active: importMode === 'single' }]" 
            @click="importMode = 'single'"
          >
            单个添加
          </button>
          <button 
            :class="['mode-btn', { active: importMode === 'batch' }]" 
            @click="importMode = 'batch'"
          >
            批量导入
          </button>
        </div>
        
        <!-- 单个添加表单 -->
        <div v-if="importMode === 'single'" class="single-form">
          <div class="form-group">
            <label class="form-label">代理协议</label>
            <div class="protocol-buttons">
              <button 
                :class="['protocol-btn', { active: formData.type === 'socks5' }]" 
                @click="formData.type = 'socks5'"
              >
                SOCKS5
              </button>
              <button 
                :class="['protocol-btn', { active: formData.type === 'http' }]" 
                @click="formData.type = 'http'"
              >
                HTTP
              </button>
              <button 
                :class="['protocol-btn', { active: formData.type === 'https' }]" 
                @click="formData.type = 'https'"
              >
                HTTPS
              </button>
            </div>
          </div>
          
          <div class="form-row host-port-row">
            <div class="form-group host-group">
              <label class="form-label">主机地址 / IP</label>
              <div class="input-with-icon">
                <input 
                  v-model="formData.host" 
                  class="form-input" 
                  placeholder="192.168.1.1" 
                />
                <span class="material-symbols-outlined input-icon">dns</span>
              </div>
            </div>
            <div class="form-group port-group">
              <label class="form-label">端口</label>
              <input 
                v-model="formData.port" 
                class="form-input" 
                placeholder="1080" 
              />
            </div>
          </div>
          
          <div class="form-row">
            <div class="form-group">
              <label class="form-label">用户名 <span class="optional">(选填)</span></label>
              <input 
                v-model="formData.username" 
                class="form-input" 
              />
            </div>
            <div class="form-group">
              <label class="form-label">密码 <span class="optional">(选填)</span></label>
              <input 
                v-model="formData.password" 
                class="form-input" 
                type="password" 
              />
            </div>
          </div>
          
          <div class="form-group">
            <label class="form-label">代理名称 <span class="optional">(选填)</span></label>
            <input 
              v-model="formData.tag" 
              class="form-input" 
              placeholder="给代理起个名字，例如: 美国住宅IP-01" 
            />
          </div>
          
          <div class="form-group">
            <label class="form-label">IP查询渠道 <span class="optional">(选填)</span></label>
            <el-select v-model="formData.ipCheckChannel" class="form-select" style="width: 100%">
              <el-option 
                v-for="channel in IP_CHECK_CHANNELS" 
                :key="channel.value" 
                :label="channel.label" 
                :value="channel.value" 
              />
            </el-select>
          </div>
          
          <div class="form-group">
            <label class="form-label">备注 <span class="optional">(选填)</span></label>
            <textarea 
              v-model="formData.remark" 
              class="form-textarea" 
              placeholder="备注信息" 
              rows="2"
            />
          </div>
          
          <!-- 测试连接 -->
          <div class="test-connection">
            <div class="test-status">
              <span 
                :class="[
                  'status-dot',
                  testStatus === 'testing' ? 'testing' : '',
                  testStatus === 'success' ? 'success' : '',
                  testStatus === 'error' ? 'error' : ''
                ]"
              ></span>
              <span class="status-text">
                {{ 
                  testStatus === 'idle' ? '未测试连接' : 
                  testStatus === 'testing' ? '测试中...' : 
                  testStatus === 'success' ? '连接正常' : 
                  '连接失败' 
                }}
              </span>
            </div>
            <button class="test-btn" @click="handleTestConnection" :disabled="testStatus === 'testing'">
              <span class="material-symbols-outlined">network_check</span>
              <span>测试连接</span>
            </button>
          </div>
        </div>
        
        <!-- 批量导入表单 -->
        <div v-else class="batch-form">
          <div class="batch-tip">
            <span class="material-symbols-outlined">info</span>
            <span>每行一个代理，支持格式：协议://用户名:密码@主机:端口</span>
          </div>
          <textarea 
            v-model="batchInput"
            class="batch-textarea" 
            placeholder="http://user123:pass456@proxy.example.com:8080&#10;socks5://myuser:mypass@192.168.1.1:1080&#10;https://username:password@10.0.0.1:3128" 
            rows="8"
          />
        </div>
      </div>
      
      <template #footer>
        <div class="dialog-footer">
          <button class="dialog-btn cancel" @click="dialogVisible = false">{{ isEditMode ? '取消' : '取消' }}</button>
          <button class="dialog-btn primary" @click="handleSubmit">{{ isEditMode ? '保存修改' : '确定添加' }}</button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped lang="scss">
.proxy-management {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--color-bg-primary);
  color: var(--color-text-primary);
  transition: background-color var(--duration-normal), color var(--duration-normal);
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
  transition: background-color var(--duration-normal), border-color var(--duration-normal);

  .page-title {
    font-size: 20px;
    font-weight: 700;
    color: var(--color-text-primary);
    letter-spacing: -0.025em;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: color var(--duration-normal);

    .pro-badge {
      background: var(--color-status-info-bg);
      color: var(--color-accent-blue);
      font-size: 12px;
      padding: 2px 8px;
      border-radius: 9999px;
      border: 1px solid var(--color-status-info-border);
      font-weight: 500;
    }
  }

  .search-wrapper {
    position: relative;
    width: 384px;

    .search-icon {
      position: absolute;
      left: 12px;
      top: 50%;
      transform: translateY(-50%);
      color: var(--color-text-tertiary);
      font-size: 20px;
    }

    .search-input {
      width: 100%;
      height: 38px;
      padding: 0 12px 0 40px;
      background: var(--color-bg-primary);
      border: 1px solid var(--color-border-default);
      border-radius: 8px;
      font-size: 14px;
      color: var(--color-text-primary);
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

.stats-card-single {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px 24px;
  padding: 8px 16px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  box-shadow: var(--shadow-xs);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);

  @media (max-width: 640px) {
    gap: 12px 16px;
    .stat-divider { display: none; }
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: var(--color-text-secondary);
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

    &-neutral { background: var(--color-border-strong); }
    &-running { 
      background: var(--color-accent-success); 
      animation: pulse 2s infinite;
    }
    &-error { background: var(--color-status-error); }
  }
}

@keyframes pulse {
  0% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(1.1); }
  100% { opacity: 1; transform: scale(1); }
}

.toolbar-right {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 12px;
  
  @media (min-width: 768px) {
    flex-direction: row;
    align-items: center;
  }
}

.toolbar-divider {
  display: none;
  
  @media (min-width: 768px) {
    display: block;
    width: 1px;
    height: 24px;
    background-color: var(--color-border);
    margin: 0 4px;
  }
}

.filter-dropdown {
  :deep(.el-select) {
    width: 140px;
    .el-select__wrapper {
      height: 38px;
      background: var(--color-bg-elevated);
      border: 1px solid var(--color-border);
      border-radius: 8px;
      box-shadow: var(--shadow-xs);
      transition: all 0.2s;
      
      &:hover { border-color: var(--color-border-hover); }
      &.is-focus { border-color: var(--color-primary); }
    }
  }
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 38px;
  padding: 0 16px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--shadow-primary-sm);

  .material-symbols-outlined { font-size: 18px; }

  &:hover {
    background: var(--color-primary-hover);
    box-shadow: var(--shadow-primary-md);
  }
  
  &:active { transform: translateY(1px); }
}

.btn-outline {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 38px;
  padding: 0 16px;
  background: var(--color-bg-elevated);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: var(--shadow-xs);

  .material-symbols-outlined { font-size: 18px; color: var(--color-text-placeholder); }

  &:hover {
    background: var(--color-bg-hover);
    border-color: var(--color-border-hover);
    color: var(--color-text-primary);
    .material-symbols-outlined { color: var(--color-text-muted); }
  }
  
  &:active { transform: translateY(1px); }
}

.btn-danger {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 38px;
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

// 列表样式
.list-container {
  flex: 1;
  margin: 0 24px 24px;
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

.list-header-grid {
  display: grid;
  grid-template-columns: 60px 60px 2.5fr 1fr 2fr 100px 2fr 1.2fr 100px 120px;
  gap: 16px;
  padding: 12px 24px;
  background: var(--color-bg-page);
  border-bottom: 1px solid var(--color-border);
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.list-body {
  flex: 1;
  overflow-y: auto;
}

.list-row-grid {
  display: grid;
  grid-template-columns: 60px 60px 2.5fr 1fr 2fr 100px 2fr 1.2fr 100px 120px;
  gap: 16px;
  padding: 16px 24px;
  align-items: center;
  border-bottom: 1px solid var(--color-border-subtle);
  transition: all 0.2s;

  &:hover {
    background-color: var(--color-primary-bg);
  }

  &:last-child {
    border-bottom: none;
  }
}

.index-num {
  font-size: 14px;
  color: var(--color-text-muted);
  font-weight: 500;
}

.proxy-identity {
  display: flex;
  align-items: center;
  gap: 12px;

  .icon-box {
    width: 32px;
    height: 32px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;

    .material-symbols-outlined { font-size: 18px; }

    &.default { background: var(--color-badge-blue-bg); color: var(--color-primary); }
    &.socks5 { background: var(--color-badge-orange-bg); color: var(--color-status-warning-text); }
    &.https { background: var(--color-badge-green-bg); color: var(--color-badge-green-text); }
    &.error { background: var(--color-status-error-bg); color: var(--color-status-error-text); }
    &.purple { background: var(--color-badge-purple-bg); color: var(--color-badge-purple-text); }
    &.indigo { background: var(--color-badge-indigo-bg); color: var(--color-badge-indigo-text); }
    &.orange { background: var(--color-badge-orange-bg); color: var(--color-status-warning-text); }
  }

  .main-name {
    font-size: 14px;
    font-weight: 500;
    color: #1e293b;
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 180px;
  }

  .sub-id {
    font-size: 12px;
    color: #94a3b8;
    margin: 0;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  }
}

.proto-tag {
  display: inline-flex;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  border: 1px solid #e2e8f0;
  background: #f1f5f9;
  color: #475569;

  &.type-https { background: #f0fdf4; color: #16a34a; border-color: #bbf7d0; }
  &.type-socks5 { background: #fff7ed; color: #ea580c; border-color: #fed7aa; }
}

.ip-info {
  display: flex;
  flex-direction: column;
}

.ip-row {
  display: flex;
  align-items: center;
  gap: 6px;

  .flag-img {
    width: 16px;
    height: auto;
    border-radius: 2px;
    box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  }

  .ip-addr {
    font-size: 13px;
    font-weight: 500;
    color: #334155;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  }
}

.loc-text {
  font-size: 12px;
  color: #94a3b8;
  margin-top: 2px;
}

.status-pill {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 9999px;
  font-size: 12px;
  font-weight: 500;
  
  &.active {
    background: #ecfdf5;
    color: #059669;
    .dot {
      width: 6px;
      height: 6px;
      background: #10b981;
      border-radius: 50%;
    }
  }

  &.error {
    background: #fef2f2;
    color: #dc2626;
    .material-symbols-outlined { font-size: 14px; }
  }

  &.pending {
    background: #f8fafc;
    color: #64748b;
  }
}

.bind-badge {
  display: inline-block;
  min-width: 24px;
  padding: 2px 8px;
  background: #f1f5f9;
  color: #94a3b8;
  border-radius: 9999px;
  font-size: 12px;
  font-weight: 700;
  border: 1px solid #e2e8f0;

  &.active {
    background: #f8fafc;
    color: #334155;
    border-color: #cbd5e1;
  }
}

.row-actions {
  display: flex;
  justify-content: flex-end;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;

  .group:hover & {
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
  color: #94a3b8;
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined { font-size: 18px; }

  &:hover {
    background: #f1f5f9;
    color: #1e293b;

    &.edit { color: #2563eb; background: #eff6ff; }
    &.check { color: #10b981; background: #ecfdf5; }
    &.delete { color: #ef4444; background: #fef2f2; }
  }
}

.pagination-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  background: #f8fafc;
  border-top: 1px solid #e2e8f0;

  .page-info {
    font-size: 12px;
    color: #64748b;
    strong { color: #1e293b; }
  }

  .page-controls {
    display: flex;
    align-items: center;
    gap: 16px;

    .size-select {
      width: 100px;
    }
  }
}

.empty-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 0;
  color: #94a3b8;

  .material-symbols-outlined { font-size: 48px; margin-bottom: 12px; opacity: 0.5; }
  p { font-size: 14px; margin-bottom: 20px; }

  .btn-empty {
    padding: 8px 24px;
    background: #2563eb;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    &:hover { background: #1d4ed8; }
  }
}

.custom-checkbox {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  border: 1px solid #cbd5e1;
  cursor: pointer;
  accent-color: #2563eb;
}

// 弹窗样式补齐
:deep(.proxy-dialog) {
  border-radius: 12px;
  overflow: hidden;
  
  .el-dialog__header {
    margin: 0;
    padding: 16px 20px;
    background: white;
    border-bottom: 1px solid #f1f5f9;
    
    .el-dialog__title {
      font-weight: 600;
      font-size: 16px;
      color: #1e293b;
      display: flex;
      align-items: center;
      gap: 8px;
    }
  }
  
  .el-dialog__body {
    padding: 20px;
  }
  
  .el-dialog__footer {
    padding: 12px 20px;
    background: #f8fafc;
    border-top: 1px solid #f1f5f9;
  }
}

// 编辑表单样式
.edit-form {
  .form-group {
    margin-bottom: 16px;
    
    &:last-child {
      margin-bottom: 24px; // 最后一个元素的底部间距
    }
  }
  
  .form-label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: #475569;
    margin-bottom: 6px;
    
    .optional {
      color: #94a3b8;
      font-weight: 400;
    }
  }
  
  .form-input,
  .form-textarea {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #cbd5e1;
    border-radius: 8px;
    font-size: 14px;
    color: #1e293b;
    background: white;
    transition: all 0.2s;
    
    &::placeholder {
      color: #94a3b8;
    }
    
    &:focus {
      outline: none;
      border-color: #2563eb;
      box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
    }
  }
  
  .form-textarea {
    resize: vertical;
    font-family: inherit;
  }
  
  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin-bottom: 16px;
  }
  
  .form-select {
    :deep(.el-select__wrapper) {
      width: 100%;
      border: 1px solid #cbd5e1;
      border-radius: 8px;
      box-shadow: none;
      
      &:hover {
        border-color: #94a3b8;
      }
      
      &.is-focused {
        border-color: #2563eb;
        box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
      }
    }
  }
}

// 添加表单样式
.add-form {
  .mode-switch {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    padding: 4px;
    background: #f1f5f9;
    border-radius: 8px;
    margin-bottom: 20px;
    
    .mode-btn {
      padding: 6px 12px;
      border: none;
      border-radius: 6px;
      font-size: 13px;
      font-weight: 500;
      color: #64748b;
      background: transparent;
      cursor: pointer;
      transition: all 0.2s;
      
      &.active {
        background: white;
        color: #1e293b;
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
      }
      
      &:hover:not(.active) {
        color: #475569;
      }
    }
  }
  
  .single-form {
    .form-group {
      margin-bottom: 16px;
    }
    
    .form-label {
      display: block;
      font-size: 12px;
      font-weight: 500;
      color: #475569;
      margin-bottom: 6px;
      
      .optional {
        color: #94a3b8;
        font-weight: 400;
      }
    }
    
    .protocol-buttons {
      display: flex;
      gap: 12px;
      
      .protocol-btn {
        padding: 6px 12px;
        border: 1px solid #cbd5e1;
        border-radius: 6px;
        font-size: 13px;
        font-weight: 500;
        color: #64748b;
        background: white;
        cursor: pointer;
        transition: all 0.2s;
        
        &.active {
          border-color: #2563eb;
          color: #2563eb;
          background: #eff6ff;
        }
        
        &:hover:not(.active) {
          border-color: #94a3b8;
          color: #475569;
        }
      }
    }
    
    .form-row {
      display: grid;
      gap: 16px;
      margin-bottom: 16px;
      
      &.host-port-row {
        grid-template-columns: 2fr 1fr;
        
        .host-group { grid-column: 1; }
        .port-group { grid-column: 2; }
      }
      
      &:not(.host-port-row) {
        grid-template-columns: 1fr 1fr;
      }
    }
    
    .input-with-icon {
      position: relative;
      
      .form-input {
        padding-right: 36px;
      }
      
      .input-icon {
        position: absolute;
        right: 12px;
        top: 50%;
        transform: translateY(-50%);
        color: #94a3b8;
        font-size: 18px;
        pointer-events: none;
      }
    }
    
    .form-input,
    .form-textarea {
      width: 100%;
      padding: 8px 12px;
      border: 1px solid #cbd5e1;
      border-radius: 8px;
      font-size: 13px;
      color: #1e293b;
      background: white;
      transition: all 0.2s;
      
      &::placeholder {
        color: #94a3b8;
      }
      
      &:focus {
        outline: none;
        border-color: #2563eb;
        box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
      }
    }
    
    .form-textarea {
      resize: vertical;
      font-family: inherit;
    }
    
    .test-connection {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding-top: 8px;
      margin-bottom: 24px; // 底部间距，与 footer 分离
      
      .test-status {
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: 12px;
        color: #64748b;
        
        .status-dot {
          width: 8px;
          height: 8px;
          border-radius: 50%;
          background: #cbd5e1;
          
          &.testing {
            background: #f59e0b;
            animation: pulse 1.5s infinite;
          }
          
          &.success {
            background: #10b981;
          }
          
          &.error {
            background: #ef4444;
          }
        }
      }
      
      .test-btn {
        display: flex;
        align-items: center;
        gap: 6px;
        padding: 6px 10px;
        background: white;
        border: 1px solid #cbd5e1;
        border-radius: 6px;
        font-size: 12px;
        font-weight: 500;
        color: #475569;
        cursor: pointer;
        transition: all 0.2s;
        
        .material-symbols-outlined {
          font-size: 16px;
        }
        
        &:hover:not(:disabled) {
          background: #f8fafc;
          border-color: #94a3b8;
        }
        
        &:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }
      }
    }
  }
  
  .batch-form {
    .batch-tip {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 12px;
      background: #eff6ff;
      border: 1px solid #dbeafe;
      border-radius: 8px;
      font-size: 12px;
      color: #1e40af;
      margin-bottom: 16px;
      
      .material-symbols-outlined {
        font-size: 18px;
        color: #2563eb;
      }
    }
    
    .batch-textarea {
      width: 100%;
      padding: 12px;
      border: 1px solid #cbd5e1;
      border-radius: 8px;
      font-size: 13px;
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      color: #1e293b;
      background: #f8fafc;
      resize: vertical;
      margin-bottom: 24px; // 底部间距，与 footer 分离
      
      &::placeholder {
        color: #94a3b8;
      }
      
      &:focus {
        outline: none;
        border-color: #2563eb;
        background: white;
        box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
      }
    }
  }
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  background: #f8fafc;
  border-top: 1px solid #e2e8f0;
}

.dialog-btn {
  padding: 8px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  
  &.primary {
    background: #2563eb;
    color: white;
    border: none;
    &:hover { background: #1d4ed8; }
  }
  
  &.cancel {
    background: white;
    color: #475569;
    border: 1px solid #e2e8f0;
    &:hover { background: #f8fafc; }
  }
}
</style>

<!-- 非 scoped 样式，用于影响 Teleport 渲染的 Dialog -->
<style lang="scss">
.proxy-dialog.el-dialog {
  --el-dialog-padding-primary: 0 !important;
  
  .el-dialog__header {
    margin: 0;
    padding: 16px 24px;
    border-bottom: 1px solid #e2e8f0;
  }
  
  .el-dialog__body {
    padding: 24px 24px 0 !important;
    margin: 0 !important;
  }
  
  .el-dialog__footer {
    padding: 0 !important;
    margin: 0 !important;
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
</style>
