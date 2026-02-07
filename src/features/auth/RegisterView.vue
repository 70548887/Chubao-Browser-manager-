<script setup lang="ts">
/**
 * @description 注册页面
 * @author DeepAgent
 */
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import CustomTitlebar from '@/components/layout/CustomTitlebar.vue'
import * as authApi from '@/api/authApi'
import './register.scss'

const { t } = useI18n()
const router = useRouter()

// 表单数据
const formData = ref({
  phone: '',
  verifyCode: '',
  password: '',
  confirmPassword: '',
  agreement: false
})

// 状态
const isLoading = ref(false)
const showPassword = ref(false)
const showConfirmPassword = ref(false)
const countdown = ref(0)
let countdownTimer: ReturnType<typeof setInterval> | null = null

// 表单验证
const isFormValid = computed(() => {
  return formData.value.phone.trim() !== '' &&
    formData.value.verifyCode.trim() !== '' &&
    formData.value.password !== '' &&
    formData.value.confirmPassword !== '' &&
    formData.value.agreement
})

const canSendCode = computed(() => {
  return formData.value.phone.trim() !== '' && countdown.value === 0
})

// 发送验证码
const handleSendCode = async () => {
  if (!canSendCode.value) return
  
  try {
    // TODO: 调用发送验证码 API
    console.log('Send code to:', formData.value.phone)
    
    // 开始倒计时
    countdown.value = 60
    countdownTimer = setInterval(() => {
      countdown.value--
      if (countdown.value <= 0) {
        clearInterval(countdownTimer!)
        countdownTimer = null
      }
    }, 1000)
  } catch (error) {
    console.error('Send code failed:', error)
  }
}

// 注册处理
const handleRegister = async () => {
  if (!isFormValid.value || isLoading.value) return
  
  // 密码确认
  if (formData.value.password !== formData.value.confirmPassword) {
    console.error('Password mismatch')
    return
  }
  
  isLoading.value = true
  try {
    // 调用注册 API（使用手机号作为用户名，需要后端支持）
    await authApi.register(
      formData.value.phone,           // username（手机号）
      `${formData.value.phone}@temp.com`, // email（临时邮箱，后续可修改）
      formData.value.password,
      formData.value.confirmPassword,
      formData.value.verifyCode       // 验证码作为邀请码传递
    )
    
    ElMessage.success('注册成功！请登录')
    router.push('/login')
  } catch (error: any) {
    ElMessage.error(error.message || '注册失败，请重试')
    console.error('Register failed:', error)
  } finally {
    isLoading.value = false
  }
}

// 跳转登录
const goToLogin = () => {
  router.push('/login')
}
</script>

<template>
  <!-- 自定义标题栏 -->
  <CustomTitlebar />
  
  <div class="register-page">
    <!-- 外层卡片容器 -->
    <div class="register-card">
      <!-- 左侧品牌展示区 -->
      <div class="brand-panel">
      <div class="brand-content">
        <!-- Logo -->
        <div class="brand-logo">
          <div class="logo-icon">
            <span class="material-symbols-outlined">shield_person</span>
          </div>
          <span class="logo-text">DeepBrowser</span>
        </div>
        
        <!-- 标题 -->
        <h1 class="brand-title">
          {{ t('auth.brand.title1') }}<br/>
          <span class="highlight">{{ t('auth.brand.title2') }}</span>
        </h1>
        
        <!-- 描述 -->
        <p class="brand-description">{{ t('auth.brand.description') }}</p>
      </div>
      
      <!-- 中央图标展示 -->
      <div class="brand-illustration">
        <div class="glow-effect"></div>
        <div class="icon-container">
          <span class="material-symbols-outlined">person_add</span>
        </div>
        
        <!-- 浮动标签 -->
        <div class="floating-tag tag-fingerprint">
          <span class="material-symbols-outlined">fingerprint</span>
          {{ t('auth.brand.tagFingerprint') }}
        </div>
        <div class="floating-tag tag-proxy">
          <span class="material-symbols-outlined">public</span>
          {{ t('auth.brand.tagProxy') }}
        </div>
      </div>
      
      <!-- 底部版权 -->
      <div class="brand-copyright">
        © 2024 DeepBrowser. {{ t('auth.brand.copyright') }}
      </div>
      
      <!-- 背景装饰 -->
      <div class="bg-blob blob-1"></div>
      <div class="bg-blob blob-2"></div>
    </div>
    
    <!-- 右侧注册表单区 -->
    <div class="form-panel">
      <!-- 移动端 Logo -->
      <div class="mobile-logo">
        <div class="logo-icon">
          <span class="material-symbols-outlined">shield_person</span>
        </div>
        <span class="logo-text">DeepBrowser</span>
      </div>
      
      <!-- 欢迎语 -->
      <div class="welcome-section">
        <h2 class="welcome-title">{{ t('auth.register.title') }}</h2>
        <p class="welcome-subtitle">{{ t('auth.register.subtitle') }}</p>
      </div>
      
      <!-- 注册表单 -->
      <form class="register-form" @submit.prevent="handleRegister">
        <!-- 手机号输入 -->
        <div class="form-group">
          <label class="form-label" for="phone">{{ t('auth.register.phone') }}</label>
          <div class="input-wrapper">
            <div class="input-icon">
              <span class="material-symbols-outlined">phone_android</span>
            </div>
            <input
              id="phone"
              v-model="formData.phone"
              type="tel"
              class="form-input"
              :placeholder="t('auth.register.phonePlaceholder')"
              autocomplete="tel"
            />
          </div>
        </div>
        
        <!-- 验证码输入 -->
        <div class="form-group">
          <label class="form-label" for="verifyCode">{{ t('auth.register.verifyCode') }}</label>
          <div class="input-wrapper with-button">
            <div class="input-icon">
              <span class="material-symbols-outlined">pin</span>
            </div>
            <input
              id="verifyCode"
              v-model="formData.verifyCode"
              type="text"
              class="form-input"
              :placeholder="t('auth.register.verifyCodePlaceholder')"
              autocomplete="one-time-code"
            />
            <button
              type="button"
              class="code-button"
              :disabled="!canSendCode"
              @click="handleSendCode"
            >
              {{ countdown > 0 ? t('auth.register.resendCode', { seconds: countdown }) : t('auth.register.getCode') }}
            </button>
          </div>
        </div>
        
        <!-- 密码输入 -->
        <div class="form-group">
          <label class="form-label" for="password">{{ t('auth.register.password') }}</label>
          <div class="input-wrapper">
            <div class="input-icon">
              <span class="material-symbols-outlined">lock</span>
            </div>
            <input
              id="password"
              v-model="formData.password"
              :type="showPassword ? 'text' : 'password'"
              class="form-input"
              :placeholder="t('auth.register.passwordPlaceholder')"
              autocomplete="new-password"
            />
            <button
              type="button"
              class="toggle-password"
              @click="showPassword = !showPassword"
            >
              <span class="material-symbols-outlined">
                {{ showPassword ? 'visibility_off' : 'visibility' }}
              </span>
            </button>
          </div>
        </div>
        
        <!-- 确认密码 -->
        <div class="form-group">
          <label class="form-label" for="confirmPassword">{{ t('auth.register.confirmPassword') }}</label>
          <div class="input-wrapper">
            <div class="input-icon">
              <span class="material-symbols-outlined">lock_reset</span>
            </div>
            <input
              id="confirmPassword"
              v-model="formData.confirmPassword"
              :type="showConfirmPassword ? 'text' : 'password'"
              class="form-input"
              :placeholder="t('auth.register.confirmPasswordPlaceholder')"
              autocomplete="new-password"
            />
            <button
              type="button"
              class="toggle-password"
              @click="showConfirmPassword = !showConfirmPassword"
            >
              <span class="material-symbols-outlined">
                {{ showConfirmPassword ? 'visibility_off' : 'visibility' }}
              </span>
            </button>
          </div>
        </div>
        
        <!-- 用户协议 -->
        <div class="agreement-row">
          <label class="checkbox-label">
            <input
              type="checkbox"
              v-model="formData.agreement"
              class="checkbox-input"
            />
            <span class="checkbox-text">
              {{ t('auth.register.agreement') }}
              <a href="#" class="link" @click.prevent>{{ t('auth.register.terms') }}</a>
              {{ t('auth.register.and') }}
              <a href="#" class="link" @click.prevent>{{ t('auth.register.privacy') }}</a>
            </span>
          </label>
        </div>
        
        <!-- 注册按钮 -->
        <button
          type="submit"
          class="register-button"
          :disabled="!isFormValid || isLoading"
        >
          <span v-if="isLoading" class="loading-spinner"></span>
          <span v-else>{{ t('auth.register.registerButton') }}</span>
          <span v-if="!isLoading" class="material-symbols-outlined">arrow_forward</span>
        </button>
      </form>
      
      <!-- 登录引导 -->
      <p class="login-hint">
        {{ t('auth.register.hasAccount') }}
        <a class="login-link" href="#" @click.prevent="goToLogin">
          {{ t('auth.register.loginNow') }}
        </a>
      </p>
    </div>
  </div>
    
    <!-- 底部链接（桌面端） -->
    <div class="footer-links">
      <a href="#">{{ t('auth.footer.terms') }}</a>
      <a href="#">{{ t('auth.footer.privacy') }}</a>
      <a href="#">{{ t('auth.footer.help') }}</a>
      <a href="#">{{ t('auth.footer.contact') }}</a>
    </div>
  </div>
</template>
