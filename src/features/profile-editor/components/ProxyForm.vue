<script setup lang="ts">
/**
 * @description 代理配置表单
 */

import { computed } from 'vue'
import type { ProfileCreateDTO, ProxyConfig } from '@/types'

interface Props {
  modelValue: ProfileCreateDTO
}

interface Emits {
  (e: 'update:modelValue', value: ProfileCreateDTO): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 是否启用代理
const proxyEnabled = computed({
  get: () => !!props.modelValue.proxy,
  set: (value: boolean) => {
    if (value) {
      emit('update:modelValue', {
        ...props.modelValue,
        proxy: {
          type: 'http',
          host: '',
          port: 8080,
        },
      })
    } else {
      emit('update:modelValue', {
        ...props.modelValue,
        proxy: undefined,
      })
    }
  },
})

// 更新代理字段
const updateProxy = (field: keyof ProxyConfig, value: any) => {
  if (!props.modelValue.proxy) return
  
  emit('update:modelValue', {
    ...props.modelValue,
    proxy: {
      ...props.modelValue.proxy,
      [field]: value,
    },
  })
}

// 代理类型选项
const proxyTypeOptions = [
  { label: 'HTTP', value: 'http' },
  { label: 'HTTPS', value: 'https' },
  { label: 'SOCKS5', value: 'socks5' },
]
</script>

<template>
  <div class="step-content">
    <div class="form-section">
      <!-- 启用代理 -->
      <div class="form-row">
        <label class="form-label">启用代理</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="proxyEnabled" @change="proxyEnabled = !proxyEnabled" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <template v-if="proxyEnabled && modelValue.proxy">
        <!-- 代理类型 -->
        <div class="form-row">
          <label class="form-label">代理类型</label>
          <div class="form-control">
            <div class="select-wrapper">
              <select
                :value="modelValue.proxy.type"
                @change="updateProxy('type', ($event.target as HTMLSelectElement).value)"
                class="select"
              >
                <option v-for="item in proxyTypeOptions" :key="item.value" :value="item.value">
                  {{ item.label }}
                </option>
              </select>
              <span class="material-symbols-outlined select-icon">expand_more</span>
            </div>
          </div>
        </div>

        <!-- 代理地址 -->
        <div class="form-row">
          <label class="form-label">代理地址</label>
          <div class="form-control">
            <input
              :value="modelValue.proxy.host"
              @input="updateProxy('host', ($event.target as HTMLInputElement).value)"
              type="text"
              class="input"
              placeholder="例如：127.0.0.1 或 proxy.example.com"
            />
          </div>
        </div>

        <!-- 端口 -->
        <div class="form-row">
          <label class="form-label">端口</label>
          <div class="form-control">
            <input
              :value="modelValue.proxy.port"
              @input="updateProxy('port', Number(($event.target as HTMLInputElement).value))"
              type="number"
              class="input port-input"
              min="1"
              max="65535"
              placeholder="8080"
            />
          </div>
        </div>

        <!-- 认证信息分隔线 -->
        <div class="section-divider">
          <span class="divider-text">认证信息（可选）</span>
        </div>

        <!-- 用户名 -->
        <div class="form-row">
          <label class="form-label">用户名</label>
          <div class="form-control">
            <input
              :value="modelValue.proxy.username"
              @input="updateProxy('username', ($event.target as HTMLInputElement).value)"
              type="text"
              class="input"
              placeholder="代理用户名"
            />
          </div>
        </div>

        <!-- 密码 -->
        <div class="form-row">
          <label class="form-label">密码</label>
          <div class="form-control">
            <input
              :value="modelValue.proxy.password"
              @input="updateProxy('password', ($event.target as HTMLInputElement).value)"
              type="password"
              class="input"
              placeholder="代理密码"
            />
          </div>
        </div>
      </template>

      <!-- 未启用提示 -->
      <div v-if="!proxyEnabled" class="info-alert">
        <span class="material-symbols-outlined alert-icon">info</span>
        <div class="alert-content">
          <span class="alert-title">未启用代理</span>
          <span class="alert-desc">浏览器将使用直连方式访问网络</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.step-content {
  max-width: 600px;
  margin: 0 auto;
  padding: 24px;
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.form-row {
  display: grid;
  grid-template-columns: 100px 1fr;
  gap: 16px;
  align-items: start;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
  padding-top: 10px;
}

.form-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

// 开关
.toggle-switch {
  position: relative;
  width: 44px;
  height: 24px;
  display: inline-block;
  
  input {
    display: none;
  }
  
  .toggle-slider {
    position: absolute;
    inset: 0;
    background: #cbd5e1;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
    
    &::before {
      content: '';
      position: absolute;
      left: 2px;
      top: 2px;
      width: 20px;
      height: 20px;
      background: white;
      border-radius: 50%;
      transition: all 0.2s;
    }
  }
  
  input:checked + .toggle-slider {
    background: #2563eb;
    
    &::before {
      left: 22px;
    }
  }
}

// 输入框
.input {
  width: 100%;
  height: 42px;
  padding: 0 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  transition: all 0.2s;
  
  &::placeholder {
    color: #94a3b8;
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
  
  &.port-input {
    width: 120px;
  }
}

// 下拉框
.select-wrapper {
  position: relative;
  width: 150px;
  
  .select-icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: #94a3b8;
    font-size: 20px;
    pointer-events: none;
  }
}

.select {
  width: 100%;
  height: 42px;
  padding: 0 36px 0 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  cursor: pointer;
  appearance: none;
  transition: all 0.2s;
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

// 分隔线
.section-divider {
  display: flex;
  align-items: center;
  gap: 16px;
  margin: 8px 0;
  
  &::before,
  &::after {
    content: '';
    flex: 1;
    height: 1px;
    background: #e2e8f0;
  }
  
  .divider-text {
    font-size: 12px;
    color: #94a3b8;
  }
}

// 提示框
.info-alert {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 16px;
  background: #eff6ff;
  border: 1px solid #bfdbfe;
  border-radius: 8px;
  
  .alert-icon {
    font-size: 20px;
    color: #2563eb;
    flex-shrink: 0;
  }
  
  .alert-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
    
    .alert-title {
      font-size: 14px;
      font-weight: 500;
      color: #1e40af;
    }
    
    .alert-desc {
      font-size: 13px;
      color: #3b82f6;
    }
  }
}
</style>
