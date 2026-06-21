<script lang="ts">
  import { onMount } from 'svelte';
  import ActivityBar from '$lib/components/ActivityBar.svelte'
  import Sidebar from '$lib/components/Sidebar.svelte'
  import KanbanColumn from '$lib/components/KanbanColumn.svelte';
  import NewBoardModal from '$lib/components/NewBoardModal.svelte';
  import {
    boards,
    activeBoardId,
    columns,
    cardsByColumn,
    loadBoards,
    selectBoard,
    createColumn
  } from '$lib/stores/board'

  let showNewBoard = false
  let addingColumn = false
  let newColumnName = ''

  function focusInput(node: HTMLElement) {
    node.focus()
  }

  onMount(() => {
    loadBoards()
  })

  $: activeBoard = $boards.find(b => b.id === $activeBoardId)

  async function submitColumn() {
    if (!newColumnName.trim() || $activeBoardId === null) {
      addingColumn = false
      return
    }
    await createColumn($activeBoardId, newColumnName.trim())
    newColumnName = ''
    addingColumn = false
  }
</script>

<ActivityBar active="kanban" />
<Sidebar 
  boards={$boards} 
  activeBoardId={$activeBoardId}
  onSelect={selectBoard}
  onNewBoard={() => (showNewBoard = true)}
/>

<main class="content">
  {#if !activeBoard}
    <div class="empty-state">
      <p>Select or create a board.</p>
    </div>
  {:else}
    <div class="board-header">
      <h1>{activeBoard.name}</h1>
    </div>
    <div class="board-canvas">
      {#each $columns as column (column.id)}
        <KanbanColumn {column} cards={$cardsByColumn[column.id] ?? []} />
      {/each}

      {#if addingColumn}
        <div class="new-column-form">
          <input
            type="text"
            bind:value={newColumnName}
            placeholder="Column name"
            on:keydown={(e) => e.key === 'Enter' && submitColumn()}
            on:blur={submitColumn}
            use:focusInput
          />
        </div>
      {:else}
        <button class="add-column-btn" on:click={() => (addingColumn = true)}>
          <svg width="13" height="13" viewBox="0 0 14 14" fill="none">
            <path d="M7 1v12M1 7h12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
          </svg>
          Add column
        </button>
      {/if}
    </div>
  {/if}
</main>

<NewBoardModal bind:open={showNewBoard} />

<style>
.content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-3);
  font-size: 13px;
}

.board-header {
  padding: 28px 32px 16px;
  border-bottom: 1px solid var(--border);
}

.board-header h1 {
  font-size: 22px;
  font-weight: 500;
  color: var(--text-1);
  letter-spacing: -0.01em;
}

.board-canvas {
  flex: 1;
  display: flex;
  align-items: flex-start;
  gap: 20px;
  padding: 24px 32px;
  overflow-x: auto;
  overflow-y: hidden;
}

.new-column-form {
  width: clamp(280px, 22vw, 360px); 
  flex-shrink: 0;
}

.new-column-form input {
  width: 100%;
  border: 1px solid var(--text-1);
  border-radius: 8px;
  padding: 10px 12px;
  font-size: 13px;
  font-family: var(--font-sans);
  background: var(--surface);
  outline: none;
}

.add-column-btn {
  width: 220px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 8px;
  border: none;
  background: transparent;
  color: var(--text-3);
  font-size: 14px;
  padding: 12px 14px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 120ms, color 120ms;
}

.add-column-btn:hover {
  background: var(--surface);
  color: var(--text-1);
}
</style>
