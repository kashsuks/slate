<script lang="ts">
  import type { Card } from '$lib/types'

  export let card: Card
  export let onOpen: (card: Card) => void = () => {}

  const priorityColor: Record<string, string> = {
    high: '#9F2F2D',
    medium: '#956400',
    low: '#346538',
    none: 'transparent',
  }
</script>

<div class="card" on:click={() => onOpen(card)}>
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
  border-radius: 8px;
  padding: 14px 16px;
  cursor: pointer;
  transition: border-color 120ms, box-shadow 120ms;
  display: flex;
  flex-direction: column;
  gap: 6px;
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
  font-size: 14px;
  color: var(--text-1);
  line-height: 1.45;
}

.title:hover {
  text-decoration: underline;
  text-decoration-color: var(--border);
  text-underline-offset: 3px;
}

.due {
  font-size: 11px;
  color: var(--text-3);
  font-family: var(--font-mono);
}
</style>
