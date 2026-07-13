<script lang="ts">
  // each column gets a fixed set of card heights to keep the skeleton stable
  const fakeCols = [
    { cards: [48, 64, 48] },
    { cards: [64, 48, 80, 48] },
    { cards: [48, 64] },
    { cards: [80, 48, 48, 64] },
  ]
</script>

<div class="skeleton-canvas">
  {#each fakeCols as col}
    <div class="skeleton-col">
      <div class="skeleton-header">
        <div class="skeleton-block" style="width: 72px; height: 10px;"></div>
	<div class="skeleton-block" style="width: 18px; height: 10px;"></div>
      </div>

      <div class="skeleton-cards">
        {#each col.cards as cardHeight, i}
          <div
	    class="skeleton-card"
	    style="height: {cardHeight}px; animation-delay: {i * 60}ms"
	  >
	    <div class="skeleton-block title" style="width: {60 + (i % 3) * 20}%; height: 10px;"></div>
	    {#if cardHeight > 52}
	      <div class="skeleton-block" style="width: 40%; height: 9px; margin-top: 6px;"></div>
	    {/if}
	  </div>
	{/each}
      </div>
    </div>
  {/each}
</div>

<style>

.skeleton-canvas {
  display: flex;
  gap: 20px;
  padding: 24px 32px;
  flex: 1;
  overflow: hidden;
  align-items: flex-start;
}

.skeleton-col {
  width: clamp(280px, 22vw, 360px);
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 8px;
}

.skeleton-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px 4px;
}

.skeleton-cards {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 0 2px;
}

.skeleton-card {
  border-radius: 8px;
  background: var(--canvas);
  border: 1px solid var(--border);
  padding: 12px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  animation: pulse 1.4s ease-in-out infinite;
}

.skeleton-block {
  border-radius: 4px;
  background: var(--border);
  animation: pulse 1.4x ease-in-out infinite;
  flex-shrink: 0;
}

.skeleton-col:nth-child(1) .skeleton-card { animation-delay: 0ms }
.skeleton-col:nth-child(2) .skeleton-card { animation-delay: 100ms }
.skeleton-col:nth-child(3) .skeleton-card { animation-delay: 200ms }
.skeleton-col:nth-child(4) .skeleton-card { animation-delay: 300ms }

@keyframes pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 } 
}

</style>
