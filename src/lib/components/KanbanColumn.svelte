<script lang="ts">
  import type { Column, Card } from '$lib/types'
  import KanbanCard from './KanbanCard.svelte'
  import { createCard, renameColumn, moveCard, deleteColumn, cardsByColumn } from '$lib/stores/board'
  import { dndzone, TRIGGERS } from 'svelte-dnd-action'
  import { flip } from 'svelte/animate'

  export let column: Column
  export let cards: Card[] = []
  export let allCardsByColumn: Record<number, Card[]> = {}
  export let onOpenCard: (card: Card) => void = () => {}

  let adding = false
  let newTitle = ''
  let renamingColumn = false
  let columnDraft = ''

  let localCards: Card[] = []
  let dragging = false

  $: if (!dragging) localCards = cards

  async function submitCard() {
    if (!newTitle.trim()) { adding = false; return }
    await createCard(column.id, newTitle.trim())
    newTitle = ''
    adding = false
  }

  function startRename() {
    columnDraft = column.name
    renamingColumn = true
  }

  async function commitRename() {
    if (columnDraft.trim() && columnDraft !== column.name) {
      await renameColumn(column.id, columnDraft.trim())
    }
    renamingColumn = false
  }

  function cancelRename() {
    renamingColumn = false
  }

  async function handleDelete() {
    const hasCards = cards.length > 0
    if (hasCards) {
      const ok = confirm(`Delete "${column.name}"? This will also delete its ${cards.length} card${cards.length === 1 ? '' : 's'}.`)
      if (!ok) return
    }
    await deleteColumn(column.id)
  }

  function focusInput(node: HTMLInputElement) {
    node.focus()
    return {}
  }

  function focusAll(node: HTMLInputElement) {
    node.focus()
    node.select()
    return {}
  }

  function handleConsider(e: CustomEvent) {
    dragging = true
    localCards = e.detail.items
  }

  async function handleFinalize(e: CustomEvent) {
    const items: Card[] = e.detail.items
    const trigger = e.detail.info.trigger
    const movedId = e.detail.info.id
    localCards = items

    if (trigger === TRIGGERS.DROPPED_INTO_ANOTHER) {
      // Update store immediately so the reactive `$: if (!dragging) localCards = cards`
      // sees the card already removed — otherwise it snaps back before moveCard runs
      cardsByColumn.update(m => ({ ...m, [column.id]: items }))
      dragging = false
      return
    }

    const newIndex = items.findIndex((c: Card) => c.id === movedId)
    // Use the card's own column_id as source of truth — avoids stale allCardsByColumn lookups
    let fromColumnId = column.id
    for (const [colId, colCards] of Object.entries(allCardsByColumn)) {
      if (Number(colId) !== column.id && colCards.some((c: Card) => c.id === movedId)) {
        fromColumnId = Number(colId)
	break
      }
    }

    await moveCard(movedId, fromColumnId, column.id, newIndex)
    dragging = false
  }
</script>


<div class="column">
  <div class="column-header">
    {#if renamingColumn}
      <input
        class="rename-input"
	bind:value={columnDraft}
	on:keydown={(e) => {
      if (e.key === 'Enter') commitRename()
      if (e.key === 'Escape') cancelRename()
        }}
	on:blur={commitRename}
	use:focusAll
      />
    {:else}
      <span class="column-name" on:click={startRename} title="Click to rename">
        {column.name}
      </span>
    {/if}
    <div class="column-actions">
      <span class="column-count">{cards.length}</span>
      <button class="delete-col-btn" on:click={handleDelete} title="Delete column">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
	  <path d="M2 2L8 8M10 2L2 10" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
	</svg>
      </button>
    </div>
  </div>
  <div
    class="cards-list"
    use:dndzone={{ items: localCards, flipDurationMs: 150, dropTargetStyle: {} }}
    on:consider={handleConsider}
    on:finalize={handleFinalize}
  >
    {#each localCards as card (card.id)}
      <div animate:flip={{ duration: 150 }}>
        <KanbanCard {card} onOpen={onOpenCard} />
      </div>
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
    <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
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
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-col);
  padding: 8px;
  max-height: 100%;
  transition: border-color var(--transition-fast);
}

.column:hover {
  border-color: #C8C7C3;
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

.column-actions {
  display: flex;
  align-items: center;
  gap: 6px;
}

.column-count {
  font-size: 11px;
  color: var(--text-3);
  font-family: var(--font-mono);
}

.delete-col-btn {
  width: 20px;
  height: 20px;
  border: none;
  background: transparent;
  color: var(--text-3);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  opacity: 0;
  transition: opacity 120ms, background 120ms, color 120ms;
}

.column:hover .delete-col-btn {
  opacity: 1;
}

.delete-col-btn:hover {
  background: #FFF0F0;
  color: #9F2F2D;
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
  transition: background var(--transition-fast), color var(--transition-fast);
}

.add-card-btn:hover {
  background: var(--surface);
  color: var(--text-1);
}

.column-name {
  cursor: text;
}

/* ghost card while dragging */
.cards-list :global([data-is-dnd-shadow-item]) {
  opacity: 0.4;
  border: 1px dashed var(--border) !important;
  background: var(--canvas) !important;
  box-shadow: none !important;
}

.column-name:hover {
  color: var(--text-1);
  text-decoration: underline;
  text-decoration-color: var(--border);
  text-underline-offset: 3px;
}

.rename-input {
  font-size: 13px;
  font-weight: 500;
  font-family: var(--font-sans);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  color: var(--text-1);
  background: var(--surface);
  border: 1px solid var(--text-1);
  border-radius: 4px;
  padding: 2px 6px;
  outline: none;
  width: 0;
  flex: 1;
  min-width: 0;
}
</style>
