<script setup lang="ts">
/**
 * @description 新建自动化流程对话框
 * @author DeepAgent
 */

defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', data: any): void
}>()

const templates = [
  { id: 1, name: '社媒自动养号', desc: '支持 Facebook/Instagram 自动点赞、评论、关注', icon: 'public', color: 'blue' },
  { id: 2, name: '电商价格监控', desc: '实时监控 Amazon/eBay 商品价格并导出 Excel', icon: 'shopping_cart', color: 'orange' },
  { id: 3, name: '短视频自动发布', desc: 'TikTok/Youtube Shorts 多账号定时同步发布', icon: 'music_note', color: 'pink' },
  { id: 4, name: '邮箱批量收发', desc: 'Gmail/Outlook 批量预热与自动化收发邮件', icon: 'mail', color: 'purple' },
]

const handleClose = () => {
  emit('close')
}

const handleSelectTemplate = (template: any) => {
  emit('submit', { type: 'template', template })
}

const handleCreateBlank = () => {
  emit('submit', { type: 'blank' })
}

const handleImport = () => {
  emit('submit', { type: 'import' })
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="新建自动化流程"
    width="800px"
    @close="handleClose"
    class="create-flow-dialog"
    destroy-on-close
  >
    <div class="dialog-content">
      <!-- 左侧快速选项 -->
      <div class="quick-options">
        <button class="option-btn" @click="handleCreateBlank">
          <div class="icon-box bg-blue">
            <span class="material-symbols-outlined">add_circle</span>
          </div>
          <div class="text">
            <span class="title">空白流程</span>
            <span class="desc">从零开始编排你的自动化逻辑</span>
          </div>
        </button>

        <button class="option-btn" @click="handleImport">
          <div class="icon-box bg-slate">
            <span class="material-symbols-outlined">upload_file</span>
          </div>
          <div class="text">
            <span class="title">导入文件</span>
            <span class="desc">导入 .flow 或 .json 流程文件</span>
          </div>
        </button>
      </div>

      <!-- 模板市场区域 -->
      <div class="templates-section">
        <div class="section-header">
          <h3 class="title">从模板创建</h3>
          <button class="more-link">查看更多模板 <span class="material-symbols-outlined">arrow_forward</span></button>
        </div>

        <div class="templates-grid">
          <div 
            v-for="tpl in templates" 
            :key="tpl.id" 
            class="template-card"
            @click="handleSelectTemplate(tpl)"
          >
            <div class="tpl-icon" :class="tpl.color">
              <span class="material-symbols-outlined">{{ tpl.icon }}</span>
            </div>
            <div class="tpl-info">
              <h4 class="tpl-name">{{ tpl.name }}</h4>
              <p class="tpl-desc">{{ tpl.desc }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </el-dialog>
</template>

<style scoped lang="scss">
.create-flow-dialog {
  :deep(.el-dialog) {
    border-radius: 16px;
    overflow: hidden;
    padding: 0;
    background: var(--color-bg-secondary);
    transition: background-color var(--duration-normal);

    .el-dialog__header {
      margin: 0;
      padding: 20px 24px;
      border-bottom: 1px solid var(--color-border-subtle);
      background: var(--color-bg-overlay);
      transition: background-color var(--duration-normal), border-color var(--duration-normal);

      .el-dialog__title {
        font-size: 18px;
        font-weight: 700;
        color: var(--color-text-primary);
        transition: color var(--duration-normal);
      }
    }

    .el-dialog__body {
      padding: 24px;
    }
  }
}

.dialog-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.quick-options {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;

  .option-btn {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 20px;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border-default);
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;

    .icon-box {
      width: 48px;
      height: 48px;
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      
      &.bg-blue { background: var(--color-selected-bg); color: var(--color-accent-blue); }
      &.bg-slate { background: var(--color-bg-overlay); color: var(--color-text-tertiary); }

      .material-symbols-outlined { font-size: 28px; }
    }

    .text {
      display: flex;
      flex-direction: column;
      
      .title { font-size: 16px; font-weight: 700; color: var(--color-text-primary); transition: color var(--duration-normal); }
      .desc { font-size: 12px; color: var(--color-text-tertiary); margin-top: 2px; }
    }

    &:hover {
      border-color: var(--color-accent-blue);
      background: var(--color-selected-bg);
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(37, 99, 235, 0.1);
    }
  }
}

.templates-section {
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;

    .title { font-size: 14px; font-weight: 700; color: var(--color-text-secondary); margin: 0; transition: color var(--duration-normal); }
    
    .more-link {
      font-size: 13px;
      color: var(--color-accent-blue);
      font-weight: 500;
      background: none;
      border: none;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 4px;

      .material-symbols-outlined { font-size: 16px; }

      &:hover { text-decoration: underline; }
    }
  }

  .templates-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }

  .template-card {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px;
    background: var(--color-bg-overlay);
    border: 1px solid var(--color-border-subtle);
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s;

    .tpl-icon {
      flex-shrink: 0;
      width: 36px;
      height: 36px;
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;

      &.blue { background: var(--color-badge-blue-bg); color: var(--color-badge-blue-text); }
      &.orange { background: var(--color-badge-orange-bg); color: var(--color-badge-orange-text); }
      &.pink { background: var(--color-badge-pink-bg); color: var(--color-badge-pink-text); }
      &.purple { background: var(--color-badge-purple-bg); color: var(--color-badge-purple-text); }

      .material-symbols-outlined { font-size: 20px; }
    }

    .tpl-info {
      .tpl-name { font-size: 13px; font-weight: 600; color: var(--color-text-secondary); margin: 0; transition: color var(--duration-normal); }
      .tpl-desc { font-size: 11px; color: var(--color-text-tertiary); margin-top: 2px; line-height: 1.4; }
    }

    &:hover {
      background: var(--color-bg-secondary);
      border-color: var(--color-border-strong);
      box-shadow: var(--shadow-sm);
    }
  }
}
</style>
