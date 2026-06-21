<script lang="ts">
  import type { Card } from '$lib/types'

  export let card: Card

  const priorityColor: Record<string, string> = {
    high: '#9F2F2D',
    medium: '#956400',
    low: '#346538',
    none: 'transparent',
  }
</script>

<div class="card">
  {#if card.priority !== 'none'}
    <span class="priority-dot" style="background: {priorityColor[card.priority]}"></span>
  {/if}
  <p class="title">{card.title}</p>
  {#if card.due_date}
    <p class="due">{card.due_date}</p>
  {/if}
</div>

<style>
.card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 10px 12px;
  cursor: pointer;
  transition: border-color 120ms, box-shadow 120ms;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.card:hover {
  border-color: #BBBAB6;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.priority-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.title {
  font-size: 13px;
  color: var(--text-1);
  line-height: 1.4;
}

.due {
  font-size: 11px;
  color: var(--text-3);
  font-family: var(--font-mono);
}
</style>
