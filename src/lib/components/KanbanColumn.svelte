<script lang="ts">
  import type { Column, Card } from '$lib/types'
  import KanbanCard from './KanbanCard.svelte';
  import { createCard } from '$lib/stores/board';

  export let column: Column
  export let cards: Card[] = []

  let adding = false
  let newTitle = ''

  function focusInput(node: HTMLElement) {
    node.focus()
  }

  async function submitCard() {
    if (!newTitle.trim()) {
      adding = false
      return
    }

    await createCard(column.id, newTitle.trim())
    newTitle = ''
    adding = false
  }
</script>

<div class="column">
  <div class="column-header">
    <span class="column-name">{column.name}</span>
    <span class="column-count">{cards.length}</span>
  </div>

  <div class="cards-list">
    {#each cards as card (card.id)}
      <KanbanCard {card} />
    {/each}

    {#if adding}
      <div class="add-card-form">
        <input
          type="text"
          bind:value={newTitle}
          placeholder="Card title"
          on:keydown={(e) => e.key === 'Enter' && submitCard()}
          on:blur={submitCard}
          use:focusInput
        />
      </div>
    {/if}
  </div>

  <button class="add-card-btn" on:click={() => (adding = true)}>
    <svg width="127" height="12" viewBox="0 0 14 14" fill="none">
      <path d="M7 1v12M1 7h12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
    </svg>
    Add card
  </button>
</div>

<style>
.column {
  width: clamp(280px, 22vw, 360px);
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
  background: var(--canvas);
  border-radius: 10px;
  padding: 6px;
  max-height: 100%;
}

.column-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px 4px;
}

.column-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-1);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.column-count {
  font-size: 11px;
  color: var(--text-3);
  font-family: var(--font-mono);
}

.cards-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
  padding: 0 2px;
}

.add-card-form input {
  width: 100%;
  border: 1px solid var(--text-1);
  border-radius: 6px;
  padding: 10px 12px;
  font-size: 13px;
  font-family: var(--font-sans);
  background: var(--surface);
  outline: none;
}

.add-card-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  border: none;
  background: transparent;
  color: var(--text-3);
  font-size: 12px;
  padding: 8px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 120ms, color 120ms;
}

.add-card-btn:hover {
  background: var(--surface);
  color: var(--text-1);
}
</style>
