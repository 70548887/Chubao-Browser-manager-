<!--
  @file DashboardView.refactored.vue
  @description 仪表盘主视图 - 重构示例（CSS 和 JS 分离）
  @note 这是按照新规范重构后的示例文件
-->

<script setup lang="ts">
/**
 * 导入业务逻辑层
 */
import {
  // 状态
  profiles,
  selectedIds,
  isLoading,
  searchKeyword,
  filterStatus,
  sortField,
  sortOrder,
  drawerVisible,
  editingProfile,
  
  // 计算属性
  filteredProfiles,
  isAllSelected,
  runningCount,
  
  // 方法
  handleSelectAll,
  handleSelect,
  handleSort,
  handleLaunch,
  handleStop,
  handleEdit,
  handleDelete,
  handleCreateNew,
  handleDrawerSuccess,
  handleBatchLaunch,
  handleBatchStop,
  
  // 生命周期
  initDashboard
} from './DashboardView'

/**
 * 导入子组件
 */
import ListHeader from './components/ListHeader.vue'
import ProfileRow from './components/ProfileRow.vue'
import ProfileDrawer from '@/features/profile-editor/ProfileDrawer.vue'

/**
 * 初始化
 */
initDashboard()

/**
 * 暴露给父组件的方法
 */
defineExpose({
  handleCreateNew,
  handleDrawerSuccess,
})
</script>

<template>
  <div class="dashboard-view">
    <!-- 页面内通知横幅 -->
    <div class="page-notification">
      <el-icon class="notification-icon"><InfoFilled /></el-icon>
      <span class="notification-text">
        当前您使用的是免费版，可以免费使用个人账号管理功能，升级后可使用团队协作、云端同步等高级功能
      </span>
      <el-button 
        type="primary" 
        size="small" 
        class="upgrade-btn"
      >
        升级账号
      </el-button>
    </div>
    
    <!-- 工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <!-- 统计 -->
        <div class="stats">
          <span class="stat-item">
            共 <strong>{{ profiles.length }}</strong> 个环境
          </span>
          <span class="stat-divider">|</span>
          <span class="stat-item running">
            <span class="dot" />
            运行中 <strong>{{ runningCount }}</strong>
          </span>
        </div>
        
        <!-- 筛选 -->
        <el-select v-model="filterStatus" size="small" style="width: 120px">
          <el-option label="全部状态" value="all" />
          <el-option label="运行中" value="running" />
          <el-option label="已停止" value="stopped" />
        </el-select>
      </div>
      
      <div class="toolbar-right">
        <!-- 批量操作 -->
        <el-button-group>
          <el-button size="small" @click="handleBatchLaunch">
            <el-icon><VideoPlay /></el-icon>
            批量启动
          </el-button>
          <el-button size="small" @click="handleBatchStop">
            <el-icon><VideoPause /></el-icon>
            批量停止
          </el-button>
        </el-button-group>
        
        <!-- 搜索 -->
        <el-input
          v-model="searchKeyword"
          placeholder="搜索环境..."
          size="small"
          clearable
          style="width: 200px"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
      </div>
    </div>
    
    <!-- 列表 -->
    <div class="list-container" v-loading="isLoading">
      <ListHeader
        :is-all-selected="isAllSelected"
        :sort-field="sortField"
        :sort-order="sortOrder"
        @select-all="handleSelectAll"
        @sort="handleSort"
      />
      
      <div class="list-body">
        <ProfileRow
          v-for="(profile, index) in filteredProfiles"
          :key="profile.id"
          :profile="profile"
          :index="index + 1"
          :is-selected="selectedIds.has(profile.id)"
          @select="handleSelect"
          @launch="handleLaunch"
          @stop="handleStop"
          @edit="handleEdit"
          @delete="handleDelete"
        />
        
        <el-empty 
          v-if="!isLoading && filteredProfiles.length === 0" 
          description="暂无环境数据"
        />
      </div>
    </div>
    
    <!-- 环境配置抽屉 -->
    <ProfileDrawer
      v-model:visible="drawerVisible"
      :profile="editingProfile"
      @success="handleDrawerSuccess"
    />
  </div>
</template>

<!-- 引用外部样式文件 -->
<style scoped lang="scss" src="./DashboardView.scss" />
