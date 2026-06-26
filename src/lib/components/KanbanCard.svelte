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
.title:hover {
  color: var(--text-2);
}
</style>
