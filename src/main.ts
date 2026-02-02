import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import ElementPlus from 'element-plus'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'

import App from './App.vue'
import router from './router'
import { initEventListeners } from './services/eventListener'
import i18n from './locales'

// Styles - 主样式文件（包含设计令牌和所有组件样式）
import './assets/styles/main.scss'
import './assets/styles/element-dark.scss'

const app = createApp(App)

// Pinia (State Management)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(pinia)

// Vue Router
app.use(router)

// i18n (Internationalization)
app.use(i18n)

// Element Plus (UI Framework) - 配置原型主题色和中文语言
app.use(ElementPlus, {
  locale: zhCn,
  // 配置主题色与原型一致
  size: 'default',
  zIndex: 100000, // 设置为最高层级，确保所有 Element Plus 组件（弹窗、消息等）都在最顶层
})

// Register Element Plus Icons globally
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

app.mount('#app')

// 初始化事件监听（在应用挂载后）
initEventListeners().catch(console.error)
