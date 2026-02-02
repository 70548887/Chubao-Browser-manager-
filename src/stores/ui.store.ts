import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUIStore = defineStore('ui', () => {
  const searchKeyword = ref('')
  
  // 更新弹窗状态
  const showUpdateDialog = ref(false)
  const updateData = ref({
    version: 'v2.4.0',
    build: 'Build 20260128',
    notes: [
      '新增指纹内核 v118，提升防关联能力',
      '优化 RPA 机器人执行效率，速度提升 30%',
      '修复已知问题，提升软件稳定性'
    ]
  })

  const setSearchKeyword = (keyword: string) => {
    searchKeyword.value = keyword
  }

  const clearSearchKeyword = () => {
    searchKeyword.value = ''
  }

  const setUpdateDialogVisible = (visible: boolean) => {
    showUpdateDialog.value = visible
  }

  const setUpdateData = (data: any) => {
    updateData.value = { ...updateData.value, ...data }
  }

  return {
    searchKeyword,
    showUpdateDialog,
    updateData,
    setSearchKeyword,
    clearSearchKeyword,
    setUpdateDialogVisible,
    setUpdateData
  }
})
