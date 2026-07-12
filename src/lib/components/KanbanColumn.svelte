<script lang="ts">
  import type { Column, Card } from '$lib/types'
  import KanbanCard from './KanbanCard.svelte'
  import { createCard, renameColumn, updateColumnColor, moveCard, deleteColumn, cardsByColumn } from '$lib/stores/board'

  const COLORS = ['#EAEAEA', '#FADADD', '#D4EDDA', '#D0E8FF', '#FFF3CD', '#E8D5F5', '#FFE5CC']
  let showColorPicker = false

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

  // track the true origin at drag-start
  let dragOriginColumnId: number | null = null

  function handleConsider(e: CustomEvent) {
    if (!dragging) {
      // first consider the event
      // a card is being picked up, column_id is still correct
      const movedId = e.detail.info.id
      const card = (e.detail.items as Card[]).find((c: Card) => c.id === movedId)
        ?? Object.values(allCardsByColumn).flat().find((c: Card) => c.id === movedId)
      dragOriginColumnId = card?.column_id ?? column.id
    }
    dragging = true
    localCards = e.detail.items
  }

  async function handleFinalize(e: CustomEvent) {
    const items: Card[] = e.detail.items
    const trigger = e.detail.info.trigger
    const movedId = e.detail.info.id
    localCards = items

    if (trigger === TRIGGERS.DROPPED_INTO_ANOTHER) {
      cardsByColumn.update(m => ({ ...m, [column.id]: items }))
      dragging = false
      dragOriginColumnId = null
      return
    }

    const newIndex = items.findIndex((c: Card) => c.id === movedId)
    const fromColumnId = dragOriginColumnId ?? column.id

    await moveCard(movedId, fromColumnId, column.id, newIndex)
    dragging = false
    dragOriginColumnId = null
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

      <!-- Color swatch trigger -->
      <button
        class="color-btn"
        style="background: {column.color}"
        on:click={() => (showColorPicker = !showColorPicker)}
        title="Change color"
      />

      <button class="delete-col-btn" on:click={handleDelete} title="Delete column">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M4 4L20 20M20 4L4 20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </button>
    </div>
  </div>

  {#if showColorPicker}
    <div class="color-picker">
      {#each COLORS as c}
        <button
          class="color-swatch"
          class:active={column.color === c}
          style="background: {c}"
          on:click={() => { updateColumnColor(column.id, c); showColorPicker = false }}
        />
      {/each}
    </div>
  {/if}
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
    {:else if localCards.length === 0}
      <div class="empty-cards">
        <span>No cards yet</span>
	<button class="empty-add" on:click={() => (adding = true)}>
	  Add one
	</button>
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

.color-btn {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 1px solid var(--border);
  cursor: pointer;
  padding: 0;
  flex-shrink: 0;
  transition: transform 120ms;
}

.color-btn:hover {
  transform: scale(1.2);
}

.color-picker {
  display: flex;
  gap: 6px;
  padding: 6px 10px;
  flex-wrap: wrap;
}

.color-swatch {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 1px solid var(--border);
  cursor: pointer;
  padding: 0;
  transition: transform 120ms;
}

.color-swatch:hover {
  transform: scale(1.15);
}

.color-swatch.active {
  border: 2px solid var(--text-1);
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

.empty-cards {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px 12px;
  border: 1px dashed var(--border);
  border-radius: 8px;
  margin: 2px 0;
}

.empty-cards span {
  font-size: 11px;
  color: var(--text-3);
  font-family: var(--font-mono);
  letter-spacing: 0.02em;
}

.empty-add {
  font-size: 11px;
  font-family: var(--font-sans);
  color: var(--text-3);
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 4px 10px;
  cursor: pointer;
  transition: border-color 120ms, color 120ms;
}

.empty-add:hover {
  border-color: var(--text-1);
  color: var(--text-1);
}
</style>
