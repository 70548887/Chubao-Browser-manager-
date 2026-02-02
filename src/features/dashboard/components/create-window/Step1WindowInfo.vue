<script setup lang="ts">
/**
 * @description 步骤1 - 窗口信息
 */

const model = defineModel<any>({ required: true })

// 分组选项
const groupOptions = [
  { value: 'default', label: '默认分组' },
  { value: 'ecommerce', label: '电商核心组' },
  { value: 'social', label: '社媒推广组' },
  { value: 'video', label: '短视频矩阵' },
  { value: 'payment', label: '支付账号' }
]
</script>

<template>
  <div class="step-content">
    <div class="form-section">
      <!-- 窗口名称 -->
      <div class="form-row">
        <label class="form-label">
          窗口名称 <span class="optional">(选填)</span>
        </label>
        <div class="form-control">
          <input 
            v-model="model.name"
            type="text" 
            class="input"
            placeholder="留空将自动生成：未命名+序号"
          />
        </div>
      </div>
      
      <!-- 分组 -->
      <div class="form-row">
        <label class="form-label">分组</label>
        <div class="form-control">
          <div class="select-wrapper">
            <select v-model="model.groupId" class="select">
              <option v-for="opt in groupOptions" :key="opt.value" :value="opt.value">
                {{ opt.label }}
              </option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>
      
      <!-- 备注 -->
      <div class="form-row">
        <label class="form-label">备注</label>
        <div class="form-control">
          <textarea 
            v-model="model.remark"
            class="textarea"
            rows="4"
            placeholder="选填，输入备注信息"
          ></textarea>
        </div>
      </div>
      
      <!-- 启动页地址 -->
      <div class="form-row">
        <label class="form-label">
          <span class="material-symbols-outlined label-icon">home</span>
          启动页地址
        </label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: model.startupPage === 'blank' }">
              <input type="radio" v-model="model.startupPage" value="blank" />
              <span class="radio-indicator"></span>
              <span class="radio-label">空白页</span>
            </label>
            <label class="radio-item" :class="{ active: model.startupPage === 'url' }">
              <input type="radio" v-model="model.startupPage" value="url" />
              <span class="radio-indicator"></span>
              <span class="radio-label">指定网页</span>
            </label>
          </div>
          <input 
            v-if="model.startupPage === 'url'"
            v-model="model.startupUrl"
            type="text"
            class="input mt-2"
            placeholder="输入网址，例如：https://www.google.com"
          />
        </div>
      </div>
      
      <!-- 导入 Cookie -->
      <div class="form-row">
        <label class="form-label">导入 Cookie</label>
        <div class="form-control">
          <div class="cookie-header">
            <span></span>
            <div class="cookie-actions">
              <a href="#" class="link">格式说明</a>
              <a href="#" class="link link-primary">检测格式</a>
            </div>
          </div>
          <textarea 
            v-model="model.cookies"
            class="textarea code-textarea"
            rows="6"
            placeholder='[{"domain": ".facebook.com", "expirationDate": 1718915444, "name": "c_user", "path": "/", "value": "1000384728472"}, ...]'
          ></textarea>
          <div class="form-hint">
            <span class="material-symbols-outlined hint-icon">info</span>
            支持 JSON 或 Netscape 格式的 Cookie 数据，导入后将自动同步到云端。
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.step-content {
  max-width: 600px;
  margin: 0 auto;
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
  color: var(--color-text-primary);
  padding-top: 10px;
  display: flex;
  align-items: center;
  gap: 6px;
  
  .label-icon {
    font-size: 18px;
    color: #64748b;
  }
  
  .required {
    color: #ef4444;
    margin-left: 2px;
  }
}

.form-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input {
  width: 100%;
  height: 42px;
  padding: 0 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: white;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &::placeholder {
    color: var(--color-text-tertiary);
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

.select-wrapper {
  position: relative;
  
  .select-icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--color-text-tertiary);
    font-size: 20px;
    pointer-events: none;
  }
}

.select {
  width: 100%;
  height: 42px;
  padding: 0 36px 0 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: white;
  cursor: pointer;
  appearance: none;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

.textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: white;
  resize: vertical;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &::placeholder {
    color: var(--color-text-tertiary);
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

.code-textarea {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  line-height: 1.5;
  background: #f8fafc;
  
  .dark & {
    background: rgba(30, 41, 59, 0.5);
  }
}

.cookie-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.cookie-actions {
  display: flex;
  gap: 16px;
}

.link {
  font-size: 12px;
  color: var(--color-text-tertiary);
  text-decoration: none;
  
  &:hover {
    text-decoration: underline;
  }
  
  &.link-primary {
    color: #2563eb;
    font-weight: 500;
  }
}

.form-hint {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  font-size: 12px;
  color: var(--color-text-tertiary);
  
  .hint-icon {
    font-size: 14px;
    color: #2563eb;
    flex-shrink: 0;
    margin-top: 1px;
  }
}

// 单选按钮组
.radio-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: white;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  input {
    display: none;
  }
  
  &:hover {
    border-color: #cbd5e1;
  }
  
  &.active {
    border-color: #2563eb;
    background: #eff6ff;
    
    .dark & {
      background: rgba(37, 99, 235, 0.1);
    }
    
    .radio-indicator {
      border-color: #2563eb;
      background: #2563eb;
      
      &::after {
        opacity: 1;
      }
    }
  }
}

.radio-indicator {
  width: 18px;
  height: 18px;
  border: 2px solid #cbd5e1;
  border-radius: 50%;
  position: relative;
  transition: all 0.2s;
  
  .dark & {
    border-color: var(--color-border-dark);
  }
  
  &::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 8px;
    height: 8px;
    background: white;
    border-radius: 50%;
    opacity: 0;
    transition: opacity 0.2s;
  }
}

.radio-label {
  font-size: 14px;
  color: #374151;
  
  .dark & {
    color: var(--color-text-primary);
  }
}

.mt-2 {
  margin-top: 8px;
}
</style>
