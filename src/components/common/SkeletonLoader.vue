<parameter name="CodeContent"><template>
  <div class="skeleton-loader">
    <div v-for="n in count" :key="n" class="skeleton-item">
      <div class="skeleton-checkbox skeleton"></div>
      <div class="skeleton-content">
        <div class="skeleton-title skeleton"></div>
        <div class="skeleton-subtitle skeleton"></div>
      </div>
      <div class="skeleton-actions">
        <div class="skeleton-button skeleton"></div>
        <div class="skeleton-button skeleton"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * @description 骨架屏加载组件 - 替换 loading 状态
 * @author DeepAgent
 */

interface Props {
  /** 骨架屏数量 */
  count?: number
}

withDefaults(defineProps<Props>(), {
  count: 5,
})
</script>

<style scoped lang="scss">
.skeleton-loader {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.skeleton-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
  padding: var(--spacing-lg);
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-subtle);
}

.skeleton-checkbox {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  flex-shrink: 0;
}

.skeleton-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.skeleton-title {
  height: 16px;
  width: 200px;
  border-radius: 4px;
}

.skeleton-subtitle {
  height: 14px;
  width: 150px;
  border-radius: 4px;
}

.skeleton-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.skeleton-button {
  width: 70px;
  height: 32px;
  border-radius: var(--radius-md);
}

// 骨架屏动画
.skeleton {
  animation: skeleton-loading 1.5s ease-in-out infinite;
  background: linear-gradient(
    90deg,
    var(--color-bg-secondary) 0%,
    var(--color-bg-elevated) 50%,
    var(--color-bg-secondary) 100%
  );
  background-size: 200px 100%;
}

@keyframes skeleton-loading {
  0% {
    background-position: -200px 0;
  }
  100% {
    background-position: calc(200px + 100%) 0;
  }
}
</style>
