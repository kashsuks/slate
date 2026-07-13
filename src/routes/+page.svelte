<script lang="ts">
import { onMount } from 'svelte';
import { goto } from '$app/navigation'
import ActivityBar from '$lib/components/ActivityBar.svelte'
import Sidebar from '$lib/components/Sidebar.svelte'
import KanbanColumn from '$lib/components/KanbanColumn.svelte';
import SkeletonBoard from '$lib/components/SkeletonBoard.svelte';
import NewBoardModal from '$lib/components/NewBoardModal.svelte';
import CardModal from '$lib/components/CardModal.svelte';
import {
boards,
activeBoardId,
columns,
cardsByColumn,
loadBoards,
selectBoard,
createColumn,
loadingBoards,
loadingBoard,
} from '$lib/stores/board'

let showNewBoard = false
let addingColumn = false
let newColumnName = ''
let activeCard: import('$lib/types').Card | null = null

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

<ActivityBar active="kanban" onSettings={() => goto('/settings')} />
<Sidebar 
  boards={$boards} 
  activeBoardId={$activeBoardId}
  onSelect={selectBoard}
  onNewBoard={() => (showNewBoard = true)}
  loading={$loadingBoards}
/>

<main class="content">
  {#if !activeBoard}
    <div class="empty-state">
      <div class="empty-icon">
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none">
	  <rect x="3" y="5" width="6" height="14" rx="2" stroke="currentColor" stroke-width="1.4"/>
	  <rect x="12" y="5" width="6" height="9" rx="2" stroke="currentColor"	stroke-width="1.4"/>
	</svg>
      </div>
      <p class="empty-title">No boards yet</p>
      <p class="empty-sub">Create a board to start organising your work.</p>
      <button class="empty-cta" on:click={() => (showNewBoard = true)}>
        Create board
      </button>
    </div>
  {:else if $loadingBoard}
    <div class="board-header skeleton-header-bar">
      <div class="skeleton-block" style="width: 140x; height: 18px; border-radius: 5px;"></div>
    </div>
    <SkeletonBoard />
  {:else if activeBoard}
    <div class="board-header">
      <h1>{activeBoard.name}</h1>
    </div>
    <div class="board-canvas">
      {#each $columns as column (column.id)}
        <KanbanColumn
          {column}
          cards={$cardsByColumn[column.id] ?? []}
          allCardsByColumn={$cardsByColumn}
	  onOpenCard={(card) => (activeCard = card)}
        />
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
      {:else if $columns.length === 0}
        <div class="empty-columns">
	  <div class="empty-icon">
	    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
	      <path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
	    </svg>
	  </div>
	  <p class="empty-title">This board is empty</p>
	  <p class="empty-sub">Add a column to start tracking work.</p>
	  <button class="empty-cta" on:click={() => (addingColumn = true)}>
	    Add column
	  </button>
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
<CardModal card={activeCard} onClose={() => (activeCard = null)} />

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
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.empty-columns {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 10px;
  width: 320px;
  padding: 48px 32px;
  border: 1px dashed var(--border);
  border-radius: 12px;
  margin: auto 0;
}

.empty-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: var(--surface);
  border: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-3);
  margin-bottom: 4px;
}

.empty-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-1);
  letter-spacing: -0.01em;
  margin: 0;
}

.empty-sub {
  font-size: 12px;
  color: var(--text-3);
  text-align: center;
  line-height: 1.5;
  margin: 0;
}

.empty-cta {
  margin-top: 6px;
  padding: 8px 16px;
  background: #111111;
  color: #FFFFFF;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: background 150ms, transform 80ms;
}

.empty-cta:hover {
  background: #333333;
}

.empty-cta:active {
  transform: scale(0.98);
}

.board-header {
  padding: 28px 32px 16px;
  border-bottom: 1px solid var(--border);
}

.skeleton-header-bar {
  display: flex;
  align-items: center;
}

.skeleton-block {
  background: var(--border);
  animation: pulse 1.4x ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
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
