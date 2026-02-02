<script setup lang="ts">
/**
 * @description 扩展中心页面
 * @author DeepAgent
 */

import { useExtensions } from './composables/useExtensions'

// 使用 composable 管理所有业务逻辑
const {
  // 常量
  categories,
  
  // 状态
  activeCategory,
  featuredExtensions,
  
  // 计算属性
  installedExtensions,
  
  // 方法
  handleCategoryChange,
  handleInstall,
  handleManage,
  handleToggleEnable,
  handleDelete,
  handleUpload,
} = useExtensions()
</script>

<template>
  <div class="extensions-center">
    <!-- 顶部操作栏 -->
    <div class="top-bar">
      <button class="upload-btn" @click="handleUpload">
        <span class="material-symbols-outlined">upload</span>
        <span>上传扩展</span>
      </button>
    </div>

    <!-- 主内容区域 -->
    <div class="content-wrapper">
      <div class="content-container">
        <!-- 分类标签 -->
        <div class="categories-tabs">
          <button
            v-for="cat in categories"
            :key="cat.id"
            class="category-btn"
            :class="{ active: activeCategory === cat.id }"
            @click="handleCategoryChange(cat.id)"
          >
            {{ cat.name }}
          </button>
        </div>

        <!-- 精选推荐 -->
        <div class="section">
          <h2 class="section-title">
            <span class="material-symbols-outlined title-icon">star</span>
            精选推荐
          </h2>

          <div class="extensions-grid">
            <div
              v-for="ext in featuredExtensions"
              :key="ext.id"
              class="extension-card"
            >
              <!-- 已安装徽章 -->
              <div v-if="ext.installed" class="installed-badge">
                <span>Installed</span>
              </div>

              <div class="card-header">
                <div class="icon-box" :class="[ext.iconBg, ext.iconColor]">
                  <span class="material-symbols-outlined">{{ ext.icon }}</span>
                </div>
                <div class="ext-info">
                  <h3 class="ext-name">{{ ext.name }}</h3>
                  <p class="ext-category">{{ ext.category }}</p>
                  <div class="rating" v-if="ext.rating">
                    <span class="material-symbols-outlined star-icon">star</span>
                    <span class="rating-score">{{ ext.rating }}</span>
                    <span class="rating-count">({{ (ext.reviews ?? 0) > 1000 ? Math.floor((ext.reviews ?? 0) / 1000) + 'k' : (ext.reviews ?? 0) }})</span>
                  </div>
                </div>
              </div>

              <p class="ext-desc">{{ ext.description }}</p>

              <button
                v-if="ext.installed"
                class="action-btn secondary"
                @click="handleManage(ext)"
              >
                <span>管理</span>
                <span class="material-symbols-outlined">settings</span>
              </button>
              <button
                v-else
                class="action-btn primary"
                @click="handleInstall(ext)"
              >
                <span>添加到浏览器</span>
                <span class="material-symbols-outlined">add</span>
              </button>
            </div>
          </div>
        </div>

        <!-- 已安装扩展 -->
        <div class="section installed-section">
          <div class="section-header">
            <h2 class="section-title">
              <span class="material-symbols-outlined title-icon primary">extension</span>
              已安装扩展
            </h2>
            <div class="ext-count">共 {{ installedExtensions.length }} 个</div>
          </div>

          <div class="installed-list">
            <div
              v-for="ext in installedExtensions"
              :key="ext.id"
              class="installed-item"
            >
              <div class="item-icon" :class="[ext.iconBg, ext.iconColor]">
                <span class="material-symbols-outlined">{{ ext.icon }}</span>
              </div>

              <div class="item-info">
                <div class="name-version">
                  <h4 class="item-name">{{ ext.name }}</h4>
                  <span class="version-tag">{{ ext.version }}</span>
                </div>
                <p class="item-id">ID: {{ ext.extensionId }}</p>
              </div>

              <div class="item-actions">
                <div class="toggle-wrapper">
                  <span class="toggle-label">启用</span>
                  <button
                    class="toggle-switch"
                    :class="{ active: ext.enabled }"
                    @click="handleToggleEnable(ext)"
                  >
                    <span class="toggle-dot"></span>
                  </button>
                </div>

                <button class="delete-btn" @click="handleDelete(ext)">
                  <span class="material-symbols-outlined">delete</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.extensions-center {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--color-bg-primary);
  transition: background-color var(--duration-normal);
}

.top-bar {
  height: 64px;
  padding: 0 24px;
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-default);
  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex-shrink: 0;
  box-shadow: var(--shadow-xs);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);

  .upload-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 36px;
    padding: 0 16px;
    background: linear-gradient(to right, var(--color-accent-blue), var(--color-accent-blue-hover));
    border: none;
    border-radius: 8px;
    color: white;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: 0 1px 3px rgba(37, 99, 235, 0.2);

    .material-symbols-outlined { font-size: 18px; }

    &:hover {
      transform: translateY(-1px);
      box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
    }

    &:active { transform: scale(0.98); }
  }
}

.content-wrapper {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.content-container {
  max-width: 1536px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 32px;
}

// 分类标签
.categories-tabs {
  display: flex;
  gap: 12px;
  overflow-x: auto;
  padding-bottom: 8px;

  &::-webkit-scrollbar { display: none; }

  .category-btn {
    flex-shrink: 0;
    padding: 8px 16px;
    border-radius: 9999px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border-default);
    color: var(--color-text-tertiary);

    &:hover {
      background: var(--color-hover-bg);
      border-color: var(--color-border-strong);
    }

    &.active {
      background: var(--color-text-primary);
      color: var(--color-bg-secondary);
      border-color: var(--color-text-primary);
    }
  }
}

// 区块标题
.section {
  .section-title {
    font-size: 18px;
    font-weight: 700;
    color: var(--color-text-primary);
    margin: 0 0 16px 0;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: color var(--duration-normal);

    .title-icon {
      font-size: 22px;
      color: var(--color-rating-star);
      &.primary { color: var(--color-accent-blue); }
    }
  }
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;

  .ext-count {
    font-size: 14px;
    color: var(--color-text-tertiary);
  }
}

// 扩展网格
.extensions-grid {
  display: grid;
  grid-template-columns: repeat(1, minmax(0, 1fr));
  gap: 24px;

  @media (min-width: 768px) { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  @media (min-width: 1024px) { grid-template-columns: repeat(3, minmax(0, 1fr)); }
  @media (min-width: 1280px) { grid-template-columns: repeat(4, minmax(0, 1fr)); }
}

.extension-card {
  position: relative;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  transition: all 0.3s;
  overflow: hidden;

  &:hover {
    box-shadow: var(--shadow-md);
  }

  .installed-badge {
    position: absolute;
    top: 12px;
    right: 12px;

    span {
      display: inline-block;
      padding: 2px 8px;
      background: var(--color-status-success-border);
      color: var(--color-status-success-text);
      font-size: 10px;
      font-weight: 700;
      text-transform: uppercase;
      border-radius: 9999px;
      letter-spacing: 0.5px;
    }
  }

  .card-header {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    margin-bottom: 16px;

    .icon-box {
      flex-shrink: 0;
      width: 64px;
      height: 64px;
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 8px;
      border: 1px solid transparent;

      .material-symbols-outlined { font-size: 36px; }
    }

    .ext-info {
      flex: 1;
      min-width: 0;

      .ext-name { font-size: 18px; font-weight: 700; color: var(--color-text-primary); margin: 0; line-height: 1.2; transition: color var(--duration-normal); }
      .ext-category { font-size: 12px; color: var(--color-text-tertiary); margin: 2px 0 4px 0; }

      .rating {
        display: flex;
        align-items: center;
        gap: 4px;

        .star-icon { font-size: 14px; color: var(--color-rating-star); }
        .rating-score { font-size: 12px; font-weight: 700; color: var(--color-text-secondary); transition: color var(--duration-normal); }
        .rating-count { font-size: 10px; color: var(--color-text-tertiary); }
      }
    }
  }

  .ext-desc {
    font-size: 14px;
    color: var(--color-text-tertiary);
    line-height: 1.5;
    margin: 0 0 16px 0;
    flex: 1;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    transition: color var(--duration-normal);
  }

  .action-btn {
    width: 100%;
    height: 40px;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;

    .material-symbols-outlined { font-size: 16px; }

    &.primary {
      background: linear-gradient(to right, var(--color-accent-blue), var(--color-accent-blue-hover));
      border: none;
      color: white;
      box-shadow: 0 1px 3px rgba(37, 99, 235, 0.2);

      &:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
      }
    }

    &.secondary {
      background: var(--color-bg-secondary);
      border: 1px solid var(--color-border-default);
      color: var(--color-text-secondary);

      &:hover { background: var(--color-hover-bg); border-color: var(--color-border-strong); }
    }
  }
}

// 已安装列表
.installed-section {
  margin-top: 8px;
}

.installed-list {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 16px;
  overflow: hidden;
  box-shadow: var(--shadow-xs);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.installed-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  transition: background 0.2s;

  &:not(:last-child) {
    border-bottom: 1px solid var(--color-border-subtle);
  }

  &:hover { background: var(--color-hover-bg); }

  .item-icon {
    flex-shrink: 0;
    width: 40px;
    height: 40px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid transparent;

    .material-symbols-outlined { font-size: 20px; }
  }

  .item-info {
    flex: 1;
    min-width: 0;

    .name-version {
      display: flex;
      align-items: center;
      gap: 8px;

      .item-name { font-size: 14px; font-weight: 700; color: var(--color-text-primary); margin: 0; transition: color var(--duration-normal); }

      .version-tag {
        font-size: 10px;
        padding: 2px 6px;
        background: var(--color-bg-overlay);
        color: var(--color-text-tertiary);
        border-radius: 4px;
        font-weight: 500;
        transition: background-color var(--duration-normal), color var(--duration-normal);
      }
    }

    .item-id { font-size: 12px; color: var(--color-text-tertiary); margin: 2px 0 0 0; font-family: monospace; }
  }

  .item-actions {
    display: flex;
    align-items: center;
    gap: 16px;

    .toggle-wrapper {
      display: flex;
      align-items: center;
      gap: 8px;

      .toggle-label { font-size: 14px; color: var(--color-text-tertiary); }

      .toggle-switch {
        position: relative;
        width: 36px;
        height: 20px;
        border-radius: 9999px;
        background: var(--color-border-strong);
        border: 2px solid transparent;
        cursor: pointer;
        transition: all 0.2s;

        &.active { background: var(--color-accent-blue); }

        .toggle-dot {
          position: absolute;
          top: 2px;
          left: 2px;
          width: 12px;
          height: 12px;
          border-radius: 50%;
          background: white;
          transition: transform 0.2s;
          box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
        }

        &.active .toggle-dot { transform: translateX(16px); }

        &:focus { outline: none; box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.15); }
      }
    }

    .delete-btn {
      width: 32px;
      height: 32px;
      border-radius: 8px;
      background: transparent;
      border: none;
      color: var(--color-text-tertiary);
      cursor: pointer;
      transition: all 0.2s;
      display: flex;
      align-items: center;
      justify-content: center;

      .material-symbols-outlined { font-size: 18px; }

      &:hover { background: var(--color-status-error-border); color: var(--color-accent-danger); }
    }
  }
}
</style>
