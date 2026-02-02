<script setup lang="ts">
/**
 * @description 创建窗口 - 第五步：偏好设置
 */
import type { CreateProfileFormData } from '../../composables/useCreateProfile'

const props = defineProps<{
  modelValue: CreateProfileFormData
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: CreateProfileFormData): void
}>()

// 双向绑定辅助
const updateField = <K extends keyof CreateProfileFormData>(key: K, value: CreateProfileFormData[K]) => {
  emit('update:modelValue', { ...props.modelValue, [key]: value })
}
</script>

<template>
  <div class="step-pane scrollable">
    <div class="form-section">
      <!-- 窗口设置 -->
      <div class="section-header">
        <h3 class="section-title">窗口设置</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">自定义标题</label>
        <input 
          :value="modelValue.windowTitle"
          class="form-input" 
          type="text" 
          placeholder="自定义浏览器窗口标题（可选）"
          @input="updateField('windowTitle', ($event.target as HTMLInputElement).value)"
        />
      </div>
      
      <div class="form-row">
        <label class="form-label">自定义书签</label>
        <textarea 
          :value="modelValue.customBookmarks"
          class="form-textarea" 
          rows="3" 
          placeholder="书签JSON格式（可选）"
          @input="updateField('customBookmarks', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
      
      <!-- 数据同步 -->
      <div class="section-header">
        <h3 class="section-title">数据同步</h3>
      </div>
      
      <div class="form-row checkbox-grid">
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncBookmarks"
            @change="updateField('syncBookmarks', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步书签</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncHistory"
            @change="updateField('syncHistory', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步历史</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncTabs"
            @change="updateField('syncTabs', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步标签页</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncCookies"
            @change="updateField('syncCookies', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步Cookie</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncExtensions"
            @change="updateField('syncExtensions', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步扩展</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncPasswords"
            @change="updateField('syncPasswords', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步密码</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncIndexedDB"
            @change="updateField('syncIndexedDB', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步IndexedDB</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncLocalStorage"
            @change="updateField('syncLocalStorage', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步LocalStorage</span>
        </label>
        <label class="checkbox-item">
          <input 
            type="checkbox" 
            :checked="modelValue.syncSessionStorage"
            @change="updateField('syncSessionStorage', ($event.target as HTMLInputElement).checked)"
          />
          <span>同步SessionStorage</span>
        </label>
      </div>
      
      <!-- 启动时操作 -->
      <div class="section-header">
        <h3 class="section-title">启动时操作</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">清除缓存</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.clearCacheOnStart }" 
          @click="updateField('clearCacheOnStart', !modelValue.clearCacheOnStart)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">清除Cookie</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.clearCookiesOnStart }" 
          @click="updateField('clearCookiesOnStart', !modelValue.clearCookiesOnStart)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">清除LocalStorage</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.clearLocalStorageOnStart }" 
          @click="updateField('clearLocalStorageOnStart', !modelValue.clearLocalStorageOnStart)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">随机指纹</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.randomFingerprintOnStart }" 
          @click="updateField('randomFingerprintOnStart', !modelValue.randomFingerprintOnStart)"
        >
          <div class="toggle-handle"></div>
        </div>
        <span class="form-help">每次启动时随机更新部分指纹</span>
      </div>
      
      <!-- 其他设置 -->
      <div class="section-header">
        <h3 class="section-title">其他设置</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">显示保存密码提示</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.showPasswordSavePrompt }" 
          @click="updateField('showPasswordSavePrompt', !modelValue.showPasswordSavePrompt)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">启用Google登录</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.enableGoogleLogin }" 
          @click="updateField('enableGoogleLogin', !modelValue.enableGoogleLogin)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">打开工作台</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.openWorkbench }" 
          @click="updateField('openWorkbench', !modelValue.openWorkbench)"
        >
          <div class="toggle-handle"></div>
        </div>
        <span class="form-help">启动浏览器后自动打开工作台面板</span>
      </div>
      
      <!-- 监控设置 -->
      <div class="section-header">
        <h3 class="section-title">监控设置</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">网络错误时停止</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.stopOnNetworkError }" 
          @click="updateField('stopOnNetworkError', !modelValue.stopOnNetworkError)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">IP变化时停止</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.stopOnIpChange }" 
          @click="updateField('stopOnIpChange', !modelValue.stopOnIpChange)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">国家变化时停止</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.stopOnCountryChange }" 
          @click="updateField('stopOnCountryChange', !modelValue.stopOnCountryChange)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">IP变化告警</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.ipChangeAlert }" 
          @click="updateField('ipChangeAlert', !modelValue.ipChangeAlert)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <!-- URL过滤 -->
      <div class="section-header">
        <h3 class="section-title">URL过滤</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">URL黑名单</label>
        <textarea 
          :value="modelValue.urlBlacklist"
          class="form-textarea" 
          rows="3" 
          placeholder="禁止访问的URL，每行一个"
          @input="updateField('urlBlacklist', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
      
      <div class="form-row">
        <label class="form-label">URL白名单</label>
        <textarea 
          :value="modelValue.urlWhitelist"
          class="form-textarea" 
          rows="3" 
          placeholder="仅允许访问的URL，每行一个（留空则不限制）"
          @input="updateField('urlWhitelist', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import './_step-form.scss';
</style>
