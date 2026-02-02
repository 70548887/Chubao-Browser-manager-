/**
 * @description i18n 国际化配置
 * @author DeepAgent
 */
import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN'
import enUS from './en-US'
import jaJP from './ja-JP'
import koKR from './ko-KR'

// 支持的语言列表
export const SUPPORTED_LOCALES = ['zh-CN', 'en-US', 'ja-JP', 'ko-KR'] as const
export type SupportedLocale = typeof SUPPORTED_LOCALES[number]

// 语言显示名称
export const LOCALE_NAMES: Record<SupportedLocale, string> = {
  'zh-CN': '简体中文',
  'en-US': 'English',
  'ja-JP': '日本語',
  'ko-KR': '한국어'
}

// 获取默认语言
function getDefaultLocale(): SupportedLocale {
  // 1. 优先使用保存的语言设置
  const savedLocale = localStorage.getItem('locale') as SupportedLocale
  if (savedLocale && SUPPORTED_LOCALES.includes(savedLocale)) {
    return savedLocale
  }
  
  // 2. 尝试匹配浏览器语言
  const browserLocale = navigator.language
  if (SUPPORTED_LOCALES.includes(browserLocale as SupportedLocale)) {
    return browserLocale as SupportedLocale
  }
  
  // 3. 尝试匹配语言前缀 (如 zh -> zh-CN)
  const shortLocale = browserLocale.split('-')[0]
  const matchedLocale = SUPPORTED_LOCALES.find(l => l.startsWith(shortLocale))
  if (matchedLocale) {
    return matchedLocale
  }
  
  // 4. 默认中文
  return 'zh-CN'
}

// 创建 i18n 实例
const i18n = createI18n({
  legacy: false, // 使用 Composition API 模式
  locale: getDefaultLocale(),
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS,
    'ja-JP': jaJP,
    'ko-KR': koKR
  },
  // 关闭控制台警告
  missingWarn: false,
  fallbackWarn: false
})

/**
 * 切换语言
 * @param locale 目标语言
 */
export function setLocale(locale: SupportedLocale) {
  if (!SUPPORTED_LOCALES.includes(locale)) {
    console.warn(`Unsupported locale: ${locale}`)
    return
  }
  
  // 更新 i18n 语言（使用类型断言兼容未加载的语言）
  ;(i18n.global.locale as { value: string }).value = locale
  
  // 保存到本地存储
  localStorage.setItem('locale', locale)
  
  // 更新 HTML lang 属性
  document.documentElement.setAttribute('lang', locale)
  
  // 更新 HTML dir 属性（为将来支持 RTL 语言预留）
  document.documentElement.setAttribute('dir', 'ltr')
}

/**
 * 获取当前语言
 */
export function getLocale(): SupportedLocale {
  return i18n.global.locale.value as SupportedLocale
}

/**
 * 获取语言显示名称
 */
export function getLocaleName(locale: SupportedLocale): string {
  return LOCALE_NAMES[locale] || locale
}

export default i18n
