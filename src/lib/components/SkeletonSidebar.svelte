<div class="skeleton-list">
  {#each [80, 120, 96, 64, 108] as width}
    <div class="skeleton-item">
      <div class="skeleton-dot"></div>
      <div class="skeleton-label" style="width: {width}px"></div>
    </div>
  {/each}
</div>

<style>

.skeleton-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 0 8px;
}

.skeleton-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 10px;
  border-radius: 6px;
}

.skeleton-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--border);
  flex-shrink: 0;
  animation: pulse 1.4s ease-in-out infinite;
}

.skeleton-label {
  height: 11px;
  border-radius: 4px;
  background: var(--border);
  animation: pulse 1.4s ease-in-out infinite;
}

.skeleton-item:nth-child(1) .skeleton-dot,
.skeleton-item:nth-child(1) .skeleton-label { animation-delay: 0ms }
.skeleton-item:nth-child(2) .skeleton-dot,
.skeleton-item:nth-child(2) .skeleton-label { animation-delay: 80ms }
.skeleton-item:nth-child(3) .skeleton-dot,
.skeleton-item:nth-child(3) .skeleton-label { animation-delay: 160ms }
.skeleton-item:nth-child(4) .skeleton-dot,
.skeleton-item:nth-child(4) .skeleton-label { animation-delay: 240ms }
.skeleton-item:nth-child(5) .skeleton-dot,
.skeleton-item:nth-child(5) .skeleton-label { animation-delay: 320ms }

@keyframes pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
}

</style>
