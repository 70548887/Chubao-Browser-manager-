<script setup lang="ts">
import { ref, onMounted } from 'vue'
import CustomTitlebar from '@/components/layout/CustomTitlebar.vue'
import AppSidebar from '@/components/layout/AppSidebar.vue'
import AppHeader from '@/components/layout/AppHeader.vue'

defineEmits(['create-new'])

const sidebarCollapsed = ref(false)

const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

onMounted(() => {
  // 基础初始化
})
</script>

<template>
  <div class="main-layout">
    <!-- Custom Titlebar -->
    <CustomTitlebar />
    
    <div class="layout-body">
      <!-- Sidebar -->
      <AppSidebar 
        :collapsed="sidebarCollapsed"
        @create-new="$emit('create-new')"
        @toggle-collapse="toggleSidebar"
      />
      
      <!-- Main Content Area -->
      <div class="main-content">
        <!-- Header -->
        <AppHeader 
          @create-new="$emit('create-new')"
        />
        
        <!-- Page Content - 直接渲染，不加额外容器 -->
        <slot />
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.main-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background-color: var(--color-bg-primary);
  transition: background-color var(--duration-normal);
}

.layout-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--color-bg-primary);
  transition: background-color var(--duration-normal);
}
</style>
