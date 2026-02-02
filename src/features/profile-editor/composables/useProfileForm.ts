import { ref, computed } from 'vue'
import type { Profile, ProfileCreateDTO, Fingerprint } from '@/types'

/**
 * @description 环境配置表单逻辑
 */
export function useProfileForm(initialProfile?: Profile) {
    // 表单数据
    const formData = ref<ProfileCreateDTO>({
        name: initialProfile?.name || '',
        group: initialProfile?.group || '',
        remark: initialProfile?.remark || '',
        fingerprint: initialProfile?.fingerprint || generateRandomFingerprint(),
        proxy: initialProfile?.proxy || undefined,
        basicSettings: initialProfile?.basicSettings || {
            language: 'auto',
            displayLanguage: 'auto',
            timezone: 'auto',
            geolocationPrompt: 'ask',
            geolocation: 'auto',
            audio: true,
            image: true,
            video: true,
            windowSize: 'custom',
            windowWidth: 1200,
            windowHeight: 800,
        },
        preferences: initialProfile?.preferences || {
            windowName: false,
            customBookmarks: false,
            extensions: [],
            startupPage: 'blank',
            startupUrl: '',
            syncBookmarks: false,
            syncHistory: false,
            syncTabs: false,
            syncCookies: false,
            syncExtensions: false,
            syncPasswords: false,
            syncIndexedDB: false,
            syncLocalStorage: false,
            syncSessionStorage: false,
            clearCacheOnStart: false,
            clearCookiesOnStart: false,
            clearLocalStorageOnStart: false,
            clearHistoryOnExit: false,
            clearCookiesOnExit: false,
            clearCacheOnExit: false,
            cloudSync: false,
            cloudSyncExtensions: false,
            cloudSyncBookmarks: false,
            randomFingerprintOnStart: false,
            showPasswordSavePrompt: true,
            stopOnNetworkError: false,
            stopOnIpChange: false,
            stopOnCountryChange: false,
            openWorkbench: false,
            ipChangeNotification: false,
            enableGoogleLogin: false,
        },
    })

    // 当前步骤
    const currentStep = ref(0)

    // 表单验证规则
    const rules = {
        name: [
            { required: true, message: '请输入环境名称', trigger: 'blur' },
            { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' },
        ],
    }

    // 是否为编辑模式
    const isEditMode = computed(() => !!initialProfile)

    // 生成随机指纹
    function generateRandomFingerprint(): Fingerprint {
        const platforms = ['windows', 'macos', 'linux'] as const
        const browsers = ['chrome', 'edge', 'brave'] as const
        const cpuCores = [4, 8, 12, 16]
        const memoryGB = [8, 16, 32]
        const timezones = ['Asia/Shanghai', 'America/New_York', 'Europe/London']
        const languages = ['zh-CN', 'en-US', 'ja-JP']

        return {
            seed: Date.now(),
            platform: platforms[Math.floor(Math.random() * platforms.length)],
            browser: browsers[Math.floor(Math.random() * browsers.length)],
            userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            hardwareConcurrency: cpuCores[Math.floor(Math.random() * cpuCores.length)],
            deviceMemory: memoryGB[Math.floor(Math.random() * memoryGB.length)],
            screenResolution: '1920x1080',
            timezone: timezones[Math.floor(Math.random() * timezones.length)],
            language: languages[Math.floor(Math.random() * languages.length)],
            canvasNoise: Math.random() > 0.5,
            webglNoise: Math.random() > 0.5,
            audioNoise: Math.random() > 0.5,
            webrtc: 'real',
            webglVendor: 'Intel Inc.',
            webglRenderer: 'Intel Iris OpenGL Engine',
            webgpu: true,
            canvas: 'noise',
            audioContext: 'noise',
            doNotTrack: 'unspecified',
            clientRects: true,
            mediaDevices: 'real',
            portScanProtection: true,
            hardwareAcceleration: true,
            disableSandbox: false,
        }
    }

    // 重新生成指纹
    function regenerateFingerprint() {
        formData.value.fingerprint = generateRandomFingerprint()
    }

    // 下一步
    function nextStep() {
        console.log(`[nextStep] 当前步骤: ${currentStep.value}, 将跳转到: ${currentStep.value + 1}`)
        if (currentStep.value < 4) {  // 改为 4，因为有 5 个步骤（0-4）
            currentStep.value++
            console.log(`[nextStep] 跳转成功，当前步骤: ${currentStep.value}`)
        } else {
            console.log(`[nextStep] 已到最后一步，无法继续`)
        }
    }

    // 上一步
    function prevStep() {
        console.log(`[prevStep] 当前步骤: ${currentStep.value}, 将跳转到: ${currentStep.value - 1}`)
        if (currentStep.value > 0) {
            currentStep.value--
            console.log(`[prevStep] 跳转成功，当前步骤: ${currentStep.value}`)
        }
    }

    // 重置表单
    function resetForm() {
        formData.value = {
            name: '',
            group: '',
            remark: '',
            fingerprint: generateRandomFingerprint(),
            proxy: undefined,
            basicSettings: {
                language: 'auto',
                displayLanguage: 'auto',
                timezone: 'auto',
                geolocationPrompt: 'ask',
                geolocation: 'auto',
                audio: true,
                image: true,
                video: true,
                windowSize: 'custom',
                windowWidth: 1200,
                windowHeight: 800,
            },
            preferences: {
                windowName: false,
                customBookmarks: false,
                extensions: [],
                startupPage: 'blank',
                startupUrl: '',
                syncBookmarks: false,
                syncHistory: false,
                syncTabs: false,
                syncCookies: false,
                syncExtensions: false,
                syncPasswords: false,
                syncIndexedDB: false,
                syncLocalStorage: false,
                syncSessionStorage: false,
                clearCacheOnStart: false,
                clearCookiesOnStart: false,
                clearLocalStorageOnStart: false,
                clearHistoryOnExit: false,
                clearCookiesOnExit: false,
                clearCacheOnExit: false,
                cloudSync: false,
                cloudSyncExtensions: false,
                cloudSyncBookmarks: false,
                randomFingerprintOnStart: false,
                showPasswordSavePrompt: true,
                stopOnNetworkError: false,
                stopOnIpChange: false,
                stopOnCountryChange: false,
                openWorkbench: false,
                ipChangeNotification: false,
                enableGoogleLogin: false,
            },
        }
        currentStep.value = 0
    }

    return {
        formData,
        currentStep,
        rules,
        isEditMode,
        generateRandomFingerprint,
        regenerateFingerprint,
        nextStep,
        prevStep,
        resetForm,
    }
}
