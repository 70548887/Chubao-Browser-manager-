<script setup lang="ts">
import { useUIStore } from '@/stores/ui.store'

const uiStore = useUIStore()

const handleClose = () => {
  uiStore.setUpdateDialogVisible(false)
}

const handleUpdate = () => {
  console.log('🚀 [UpdateDialog] 开始更新...')
  // TODO: 实现具体的更新逻辑
  uiStore.setUpdateDialogVisible(false)
}
</script>

<template>
  <div v-if="uiStore.showUpdateDialog" class="update-dialog-overlay">
    <div class="update-dialog-content" role="dialog" aria-modal="true">
      <!-- 装饰背景 -->
      <div class="decoration decoration-top"></div>
      <div class="decoration decoration-bottom"></div>

      <div class="relative">
        <!-- 头部 -->
        <div class="header">
          <div class="header-left">
            <div class="icon-box">
              <span class="material-symbols-outlined">rocket_launch</span>
            </div>
            <div class="title-area">
              <h3>发现新版本</h3>
              <p class="version-info">
                {{ uiStore.updateData.version }} 
                <span class="build-info">{{ uiStore.updateData.build }}</span>
              </p>
            </div>
          </div>
          <button class="close-btn" @click="handleClose">
            <span class="material-symbols-outlined">close</span>
          </button>
        </div>

        <!-- 内容区域 -->
        <div class="body">
          <div class="changelog-box">
            <p class="section-title">更新内容</p>
            <ul class="changelog-list">
              <li v-for="(note, index) in uiStore.updateData.notes" :key="index">
                <span class="material-symbols-outlined check-icon">check_circle</span>
                <span>{{ note }}</span>
              </li>
            </ul>
          </div>
          <p class="footer-tip">
            建议您尽快更新以获得最佳体验。更新过程大约需要 1-2 分钟。
          </p>
        </div>

        <!-- 底部按钮 -->
        <div class="footer">
          <button class="btn-later" @click="handleClose">
            稍后提醒
          </button>
          <button class="btn-update" @click="handleUpdate">
            <span class="material-symbols-outlined">download</span>
            立即更新
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.update-dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1.5rem;
  background-color: rgba(15, 23, 42, 0.4);
  backdrop-filter: blur(4px);
}

.update-dialog-content {
  position: relative;
  width: 100%;
  max-width: 440px;
  background-color: white;
  border-radius: 1.25rem;
  padding: 1.5rem;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  border: 1px solid #f1f5f9;
  overflow: hidden;
  animation: dialog-appear 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes dialog-appear {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.decoration {
  position: absolute;
  width: 8rem;
  height: 8rem;
  border-radius: 9999px;
  filter: blur(40px);
  opacity: 0.6;
  z-index: 0;

  &-top {
    top: -2.5rem;
    right: -2.5rem;
    background-color: #dbeafe;
  }

  &-bottom {
    bottom: -2.5rem;
    left: -2.5rem;
    background-color: #f3e8ff;
  }
}

.relative {
  position: relative;
  z-index: 1;
}

.header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 1.25rem;

  .header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;

    .icon-box {
      display: flex;
      height: 3rem;
      width: 3rem;
      align-items: center;
      justify-content: center;
      border-radius: 9999px;
      background-color: #dbeafe;
      color: #2563eb;
      box-shadow: 0 0 0 4px #eff6ff;

      .material-symbols-outlined {
        font-size: 1.5rem;
      }
    }

    .title-area {
      h3 {
        font-size: 1.125rem;
        font-weight: 700;
        color: #0f172a;
        margin: 0;
      }

      .version-info {
        font-size: 0.75rem;
        font-weight: 600;
        color: #2563eb;
        margin: 0.125rem 0 0 0;

        .build-info {
          color: #94a3b8;
          font-weight: 400;
          margin-left: 0.25rem;
        }
      }
    }
  }

  .close-btn {
    padding: 0.25rem;
    border-radius: 9999px;
    color: #94a3b8;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      background-color: #f1f5f9;
      color: #64748b;
    }
  }
}

.body {
  margin-top: 1rem;

  .changelog-box {
    background-color: #f8fafc;
    border-radius: 0.75rem;
    padding: 1rem;
    border: 1px solid #f1f5f9;
    margin-bottom: 1rem;

    .section-title {
      font-size: 0.7rem;
      font-weight: 700;
      color: #64748b;
      text-transform: uppercase;
      margin-bottom: 0.75rem;
      letter-spacing: 0.05em;
    }

    .changelog-list {
      list-style: none;
      padding: 0;
      margin: 0;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      li {
        display: flex;
        align-items: flex-start;
        gap: 0.5rem;
        font-size: 0.875rem;
        color: #334155;

        .check-icon {
          font-size: 1rem;
          color: #10b981;
          margin-top: 0.125rem;
        }
      }
    }
  }

  .footer-tip {
    font-size: 0.875rem;
    color: #64748b;
    line-height: 1.5;
  }
}

.footer {
  margin-top: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 0.75rem;

  .btn-later {
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #475569;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      background-color: #f1f5f9;
      color: #0f172a;
    }
  }

  .btn-update {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.5rem 1.25rem;
    border-radius: 0.5rem;
    background-color: #2563eb;
    color: white;
    font-size: 0.875rem;
    font-weight: 600;
    border: none;
    cursor: pointer;
    box-shadow: 0 10px 15px -3px rgba(37, 99, 235, 0.3);
    transition: all 0.2s;

    &:hover {
      background-color: #1d4ed8;
      transform: scale(1.05);
    }

    &:active {
      transform: scale(1);
    }

    .material-symbols-outlined {
      font-size: 1.125rem;
    }
  }
}

// 暗色模式支持
:global(.dark) {
  .update-dialog-content {
    background-color: #1e293b;
    border-color: #334155;
    
    .decoration-top { background-color: rgba(30, 58, 138, 0.3); }
    .decoration-bottom { background-color: rgba(88, 28, 135, 0.3); }
    
    .header .header-left {
      .icon-box { background-color: rgba(30, 58, 138, 0.3); color: #60a5fa; box-shadow: 0 0 0 4px rgba(30, 58, 138, 0.2); }
      .title-area h3 { color: white; }
    }
    
    .body {
      .changelog-box {
        background-color: rgba(15, 23, 42, 0.5);
        border-color: #334155;
        .changelog-list li { color: #cbd5e1; }
      }
      .footer-tip { color: #94a3b8; }
    }
    
    .footer .btn-later {
      color: #94a3b8;
      &:hover { background-color: #334155; color: white; }
    }
  }
}
</style>
