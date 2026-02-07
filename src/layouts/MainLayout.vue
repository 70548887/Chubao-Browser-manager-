<script setup lang="ts">
import { ref, onMounted } from 'vue'
import CustomTitlebar from '@/components/layout/CustomTitlebar.vue'
import AppSidebar from '@/components/layout/AppSidebar.vue'
import AppHeader from '@/components/layout/AppHeader.vue'
import { useUIStore } from '@/stores/ui.store'
import * as updateApi from '@/api/updateApi'

defineEmits(['create-new'])

const uiStore = useUIStore()
const sidebarCollapsed = ref(false)

const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

// 启动时静默检查更新
const checkUpdateOnStartup = async () => {
  try {
    uiStore.setCheckingUpdate(true)
    
    const [launcher, kernel] = await Promise.allSettled([
      updateApi.checkAppUpdate(),
      updateApi.checkKernelUpdate(),
    ])

    if (launcher.status === 'fulfilled') {
      uiStore.setLauncherUpdate(launcher.value)
    }
    
    if (kernel.status === 'fulfilled') {
      uiStore.setKernelUpdate(kernel.value)
    }
  } catch (error) {
    console.error('启动时检查更新失败:', error)
  } finally {
    uiStore.setCheckingUpdate(false)
  }
}

onMounted(() => {
  // 延迟 2 秒检查更新，避免影响启动速度
  setTimeout(() => {
    checkUpdateOnStartup()
  }, 2000)
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
