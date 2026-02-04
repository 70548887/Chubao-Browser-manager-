import { ref, reactive, computed } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { LaunchStep } from '@/components/LaunchProgressDialog.vue'

// 启动进度事件类型
interface LaunchProgressEvent {
  profileId: string
  step: string
  label: string
  progress: number
  done: boolean
  error: string | null
}

// 步骤定义
const STEP_DEFINITIONS: Record<string, { order: number; defaultLabel: string }> = {
  check_config: { order: 0, defaultLabel: '检查并同步配置中...' },
  sync_extensions: { order: 1, defaultLabel: '检查并同步扩展中...' },
  setup_proxy: { order: 2, defaultLabel: '开始获取IP...' },
  sync_fingerprint: { order: 3, defaultLabel: '正在同步指纹信息...' },
  sync_cache: { order: 4, defaultLabel: '正在同步缓存配置...' },
  launching: { order: 5, defaultLabel: '正在打开浏览器窗口...' },
  done: { order: 6, defaultLabel: '窗口已打开' },
}

const STEP_KEYS = Object.keys(STEP_DEFINITIONS)

export function useLaunchProgress() {
  // 状态
  const visible = ref(false)
  const currentProfileId = ref<string | null>(null)
  const currentProfileName = ref<string>('')
  const progress = ref(0)
  const error = ref<string | undefined>(undefined)
  const steps = reactive<LaunchStep[]>([])
  
  // 事件监听器
  let unlisten: UnlistenFn | null = null
  
  // 初始化步骤
  const initSteps = () => {
    steps.length = 0
    for (const key of STEP_KEYS) {
      steps.push({
        key,
        label: STEP_DEFINITIONS[key].defaultLabel,
        status: 'pending',
      })
    }
  }
  
  // 更新步骤状态
  const updateStep = (stepKey: string, label: string, status: LaunchStep['status'], message?: string) => {
    const stepIndex = steps.findIndex(s => s.key === stepKey)
    if (stepIndex >= 0) {
      steps[stepIndex].label = label
      steps[stepIndex].status = status
      if (message) {
        steps[stepIndex].message = message
      }
      
      // 将之前的步骤标记为完成
      for (let i = 0; i < stepIndex; i++) {
        if (steps[i].status !== 'error') {
          steps[i].status = 'done'
        }
      }
    }
  }
  
  // 处理进度事件
  const handleProgressEvent = (event: LaunchProgressEvent) => {
    // 只处理当前正在启动的 profile
    if (event.profileId !== currentProfileId.value) {
      return
    }
    
    progress.value = event.progress
    
    if (event.error) {
      error.value = event.error
      updateStep(event.step, event.label, 'error', event.error)
    } else if (event.done) {
      updateStep(event.step, event.label, 'done')
    } else {
      updateStep(event.step, event.label, 'running')
    }
  }
  
  // 开始监听
  const startListening = async () => {
    if (unlisten) {
      unlisten()
    }
    
    unlisten = await listen<LaunchProgressEvent>('browser:launch_progress', (event) => {
      handleProgressEvent(event.payload)
    })
  }
  
  // 停止监听
  const stopListening = () => {
    if (unlisten) {
      unlisten()
      unlisten = null
    }
  }
  
  // 开始启动（显示对话框）
  const startLaunch = async (profileId: string, profileName?: string) => {
    currentProfileId.value = profileId
    currentProfileName.value = profileName || ''
    progress.value = 0
    error.value = undefined
    initSteps()
    visible.value = true
    
    await startListening()
  }
  
  // 关闭对话框
  const closeLaunch = () => {
    visible.value = false
    currentProfileId.value = null
    currentProfileName.value = ''
    progress.value = 0
    error.value = undefined
    stopListening()
  }
  
  // 取消启动（目前暂不支持，仅关闭对话框）
  const cancelLaunch = () => {
    closeLaunch()
  }
  
  // 是否完成
  const isComplete = computed(() => progress.value >= 100)
  
  // 是否有错误
  const hasError = computed(() => !!error.value)
  
  return {
    // 状态
    visible,
    currentProfileId,
    currentProfileName,
    progress,
    error,
    steps,
    
    // 计算属性
    isComplete,
    hasError,
    
    // 方法
    startLaunch,
    closeLaunch,
    cancelLaunch,
    startListening,
    stopListening,
  }
}
