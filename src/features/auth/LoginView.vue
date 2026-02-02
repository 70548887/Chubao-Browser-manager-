<script setup lang="ts">
/**
 * @description 登录页面
 * @author DeepAgent
 */
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import CustomTitlebar from '@/components/layout/CustomTitlebar.vue'
import './login.scss'

const { t } = useI18n()
const router = useRouter()

// 表单数据
const formData = ref({
  account: '',
  password: '',
  remember: false
})

// 状态
const isLoading = ref(false)
const showPassword = ref(false)

// 表单验证
const isFormValid = computed(() => {
  return formData.value.account.trim() !== '' && formData.value.password !== ''
})

// 登录处理
const handleLogin = async () => {
  if (!isFormValid.value || isLoading.value) return
  
  isLoading.value = true
  try {
    // TODO: 调用登录 API
    console.log('Login:', formData.value)
    // 模拟登录成功后跳转
    // router.push('/')
  } catch (error) {
    console.error('Login failed:', error)
  } finally {
    isLoading.value = false
  }
}

// 社交登录
const handleSocialLogin = (provider: 'wechat' | 'feishu') => {
  console.log('Social login:', provider)
  // TODO: 实现社交登录
}

// 跳转注册
const goToRegister = () => {
  router.push('/register')
}

// 忘记密码
const handleForgotPassword = () => {
  // TODO: 跳转忘记密码页面
  console.log('Forgot password')
}
</script>

<template>
  <!-- 自定义标题栏 -->
  <CustomTitlebar />
  
  <div class="login-page">
    <!-- 外层卡片容器 -->
    <div class="login-card">
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
            <span class="material-symbols-outlined">verified_user</span>
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
      
      <!-- 右侧登录表单区 -->
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
          <h2 class="welcome-title">{{ t('auth.login.welcome') }}</h2>
          <p class="welcome-subtitle">{{ t('auth.login.subtitle') }}</p>
        </div>
        
        <!-- 登录表单 -->
        <form class="login-form" @submit.prevent="handleLogin">
          <!-- 账号输入 -->
          <div class="form-group">
            <label class="form-label" for="account">{{ t('auth.login.account') }}</label>
            <div class="input-wrapper">
              <div class="input-icon">
                <span class="material-symbols-outlined">person</span>
              </div>
              <input
                id="account"
                v-model="formData.account"
                type="text"
                class="form-input"
                :placeholder="t('auth.login.accountPlaceholder')"
                autocomplete="username"
              />
            </div>
          </div>
          
          <!-- 密码输入 -->
          <div class="form-group">
            <div class="label-row">
              <label class="form-label" for="password">{{ t('auth.login.password') }}</label>
              <a class="forgot-link" href="#" @click.prevent="handleForgotPassword">
                {{ t('auth.login.forgotPassword') }}
              </a>
            </div>
            <div class="input-wrapper">
              <div class="input-icon">
                <span class="material-symbols-outlined">lock</span>
              </div>
              <input
                id="password"
                v-model="formData.password"
                :type="showPassword ? 'text' : 'password'"
                class="form-input"
                :placeholder="t('auth.login.passwordPlaceholder')"
                autocomplete="current-password"
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
          
          <!-- 记住我 -->
          <div class="remember-row">
            <label class="checkbox-label">
              <input
                type="checkbox"
                v-model="formData.remember"
                class="checkbox-input"
              />
              <span class="checkbox-text">{{ t('auth.login.rememberMe') }}</span>
            </label>
          </div>
          
          <!-- 登录按钮 -->
          <button
            type="submit"
            class="login-button"
            :disabled="!isFormValid || isLoading"
          >
            <span v-if="isLoading" class="loading-spinner"></span>
            <span v-else>{{ t('auth.login.loginButton') }}</span>
            <span v-if="!isLoading" class="material-symbols-outlined">arrow_forward</span>
          </button>
          
          <!-- 分隔线 -->
          <div class="divider">
            <span class="divider-text">{{ t('auth.login.socialLogin') }}</span>
          </div>
          
          <!-- 社交登录 -->
          <div class="social-buttons">
            <button type="button" class="social-btn" @click="handleSocialLogin('wechat')">
              <img src="https://lh3.googleusercontent.com/aida-public/AB6AXuAuk61wv7cuRSMPJKsiU2-jTMSgU3Nj7uoPZczFIR43xj5BoMPn-FbhLWAex-qEa9F-ZBofF0QQCfiqy57F8b7fsZ1vL9UYiQODfIvvQB3Hkt7KhiPBlGbSUtyy_TK7TobsaBuqUv_JlM876voJZdsL_zbVKpi4kmlbFcEaF0pRo2b57aTzOKyZ4KyINb3z8yjHWzI_bIDXU7eaByj2u8i_CTmOBLnl5sC9j2Bhjr_-SDfuWcB4aCp_M0ZYNeZezkmm9eFitURH0Q" alt="WeChat" class="social-icon" />
              <span>{{ t('auth.login.wechatLogin') }}</span>
            </button>
            <button type="button" class="social-btn" @click="handleSocialLogin('feishu')">
              <span class="material-symbols-outlined social-icon-material">alternate_email</span>
              <span>{{ t('auth.login.feishuLogin') }}</span>
            </button>
          </div>
        </form>
        
        <!-- 注册引导 -->
        <p class="register-hint">
          {{ t('auth.login.noAccount') }}
          <a class="register-link" href="#" @click.prevent="goToRegister">
            {{ t('auth.login.registerNow') }}
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
