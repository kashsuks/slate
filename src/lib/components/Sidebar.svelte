<script lang="ts">
  import type { Board } from "$lib/types";

  export let boards: { id: number; name: string }[] = []
  export let activeBoardId: number | null = null
  export let onSelect: (id: number) => void = () => {}
  export let onNewBoard: () => void = () => {}
</script>

<aside class="sidebar">
  <div class="section-header">
    <span class="section-label">Boards</span>
    <button class="add-btn" title="New board" on:click={onNewBoard}>
      <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
        <path d="M7 1v12M1 7h12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
      </svg>
    </button>
  </div>

  <ul class="board-list">
    {#each boards as board (board.id)}
      <li>
        <button
	  class="board-item"
	  class:active={board.id === activeBoardId}
	  on:click={() => onSelect(board.id)}
	>
	  <span class="board-dot"></span>
	  {board.name}
	</button>
      </li>
    {:else}
      <li class="empty-hint">No boards yet</li>
    {/each}
  </ul>
</aside>

<style>
.sidebar {
  width: 220px;
  height: 100vh;
  background: var(--surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  padding: 16px 0;
  flex-shrink: 0;
  overflow-y: auto;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 14px 8px;
}

.section-label {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-3);
  font-family: var(--font-mono);
}

.add-btn {
  width: 22px;
  height: 22px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-3);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 120ms, color 120ms;
}

.add-btn:hover {
  background: var(--canvas);
  color: var(--text-1);
}

.board-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 0 8px;
}

.board-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 5px;
  border: none;
  background: transparent;
  font-size: 13px;
  color: var(--text-2);
  cursor: pointer;
  text-align: left;
  transition: background 100ms, color 100ms;
  font-family: var(--font-sans);
}

.board-item:hover {
  background: var(--canvas);
  color: var(--text-1);
}

.board-item.active {
  background: var(--canvas);
  color: var(--text-1);
  font-weight: 500;
}

.board-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--border);
  flex-shrink: 0;
}

.board-item.active .board-dot {
  background: var(--text-1);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-3);
  padding: 6px 8px;
  font-style: italic;
}

</style>
