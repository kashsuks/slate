<script lang="ts">
  import type { Card } from '$lib/types'
  import { updateCard } from '$lib/stores/board'

  export let card: Card | null = null
  export let onClose: () => void = () => {}

  let title = ''
  let description = ''
  let priority = 'none'
  let dueDate = ''

  // sync local state when the card changes
  $: if (card) {
    title = card.title
    description = card.description ?? ''
    priority = card.priority
    dueDate = card.due_date ?? ''
  }

  async function save() {
    if (!card) return
    await updateCard(
      card.column_id,
      card.id,
      title.trim() || card.title,
      description.trim() || null,
      priority,
      dueDate || null,
    )
  }

  async function handleClose() {
    await save()
    onClose()
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') handleClose()
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if card}
  <!-- backdrop -->
  <div class="backdrop" on:click={handleClose}></div>

  <!-- panel -->
  <aside class="panel">
    <div class="panel-header">
      <button class="close-btn" on:click={handleClose} title="Close">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
	  <path d="M3 3110 10M13 3L3 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
    </div>

    <div class="panel-body">
      <!-- Title -->
      <div class="field">
        <label class="field-label">Title</label>
	<input
	  class="title-input"
	  bind:value={title}
	  on:blur={save}
	  placeholder="Card title"
	/>
      </div>

      <!-- priority settings -->
      <div class="field">
        <label class="field-label">Priority</label>
	<div class="priority-group">
	  {#each ['none', 'low', 'medium', 'high'] as p}
	    <button
	      class="priority-btn"
	      class:active={priority === p}
	      data-priority={p}
	      on:click={() => { priority = p; save() }}
	    >
	      {p === 'none' ? '-' : p}
	    </button>
	  {/each}
	</div>
      </div>

      <div class="field">
        <label class="field-label">Due date</label>
	<input
	  class="date-input"
	  type="date"
	  bind:value={dueDate}
	  on:change={save}
	/>
      </div>

      <div class="field field-grow">
        <label class="field-label">Description</label>
	<textarea
	  class="desc-textarea"
	  bind:value={description}
	  on:blur={save}
	  placeholder="Add a description..."
	  rows="6"
	></textarea>
      </div>
    </div>
  </aside>
{/if}

<style>
.backdrop {
  position: fixed;
  inset: 0;
  z-index: 40;
  background: transparent;
}

.panel {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  width: 360px;
  background: var(--surface);
  border-left: 1px solid var(--border);
  z-index: 50;
  display: flex;
  flex-direction: column;
  box-shadow: -4px 0 24px rgba(0,0,0,0.06);
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 16px 16px 0;
  flex-shrink: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  color: var(--text-3);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 120ms, color 120ms;
}

.close-btn:hover {
  background: var(--canvas);
  color: var(--text-1);
}

.panel-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px 24px 32px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field-grow {
  flex: 1;
}

.field-label {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-3);
  font-family: var(--font-mono);
}

.title-input {
  font-size: 16px;
  font-weight: 500;
  font-family: var(--font-sans);
  color: var(--text-1);
  border: none;
  border-bottom: 1px solid var(--border);
  padding: 4px 0 8px;
  outline: none;
  background: transparent;
  width: 100%;
  letter-spacing: -0.01em;
}

.title-input:focus {
  border-bottom-color: var(--text-1);
}

.priority-group {
  display: flex;
  gap: 6px;
}

.priority-btn {
  flex: 1;
  padding: 7px 0;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: transparent;
  font-size: 12px;
  font-family: var(--font-sans);
  color: var(--text-2);
  cursor: pointer;
  text-transform: capitalize;
  transition: all 120ms;
}

.priority-btn:hover {
  border-color: #BBBAB6;
  color: var(--text-1);
}

.priority-btn.active[data-priority="none"] {
  background: var(--canvas);
  border-color: var(--border);
  color: var(--text-1);
}

.priority-btn.active[data-priority="low"] {
  background: #EDFBEE;
  border-color: #346538;
  color: #346538;
}

.priority-btn.active[data-priority="medium"] {
  background: #FFF8EC;
  border-color: #956400;
  color: #956400;
}

.priority-btn.active[data-priority="high"] {
  background: #FFF0F0;
  border-color: #9F2F2D;
  color: #9F2F2D;
}

.date-input {
  font-size: 13px;
  font-family: var(--font-mono);
  color: var(--text-1);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 8px 10px;
  outline: none;
  background: transparent;
  width: 100%;
}

.date-input:focus {
  border-color: var(--text-1);
}

.desc-textarea {
  font-size: 14px;
  font-family: var(--font-sans);
  color: var(--text-1);
  line-height: 1.6;
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 10px 12px;
  outline: none;
  background: transparent;
  resize: none;
  width: 100%;
  flex: 1;
  min-height: 140px;
}

.desc-textarea:focus {
  border-color: var(--text-1);
}

.desc-textarea::placeholder {
  color: var(--text-3);
}

</style>
