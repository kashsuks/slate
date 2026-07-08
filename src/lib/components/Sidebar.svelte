<script lang="ts">
  import ContextMenu from "./ContextMenu.svelte"
  import { renameBoard, deleteBoard } from "$lib/stores/board"

  type BoardListItem = { id: number; name: string }

  export let boards: BoardListItem[] = []
  export let activeBoardId: number | null = null
  export let onSelect: (id: number) => void = () => {}
  export let onNewBoard: () => void = () => {}

  let menu: { x: number; y: number; board: BoardListItem } | null = null
  let renamingId: number | null = null
  let renameDraft = ''

  function openMenu(e: MouseEvent, board: BoardListItem) {
    e.preventDefault()
    menu = { x: e.clientX, y: e.clientY, board }
  }

  function startRename(board: BoardListItem) {
    renamingId = board.id
    renameDraft = board.name
  }

  async function commitRename() {
    if (renamingId === null) return
    const trimmed = renameDraft.trim()
    if (trimmed) await renameBoard(renamingId, trimmed)
    renamingId = null
  }

  function cancelRename() {
    renamingId = null
  }

  function focusAll(node: HTMLInputElement) {
    node.focus()
    node.select()
  }
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
        {#if renamingId === board.id}
	  <input
	    class="rename-input"
	    bind:value={renameDraft}
	    on:keydown={(e) => {
	      if (e.key === 'Enter') commitRename()
	      if (e.key === 'Escape') cancelRename()
	    }}
	    on:blur={commitRename}
	    use:focusAll
	  />
	{:else}
	  <button
	    class="board-item"
	    class:active={board.id === activeBoardId}
	    on:click={() => onSelect(board.id)}
	    on:contextmenu={(e) => openMenu(e, board)}
	  >
	    <span class="board-dot"></span>
	    {board.name}
	  </button>
	{/if}
      </li>
    {:else}
      <li class="empty-hint">No boards yet.</li>
    {/each}
  </ul>

{#if menu}
  <ContextMenu
    x={menu.x}
    y={menu.y}
    items={[
      { label: 'Rename', action: () => startRename(menu!.board) },
      { label: 'Delete board', danger: true, action: () => deleteBoard(menu!.board.id) },
    ]}
    onClose={() => (menu = null)}
  />
{/if}


</aside>

<style>
.sidebar {
  width: 260px;
  height: 100vh;
  background: var(--surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  padding: 20px 0;
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
  gap: 10px;
  padding: 9px 10px;
  border-radius: 6px;
  border: none;
  background: transparent;
  font-size: 14px;
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

.rename-input {
  width: 100%;
  padding: 6px 8px;
  margin: 0 0 2px;
  border: 1px solid var(--text-1);
  border-radius: 5px;
  font-size: 13px;
  font-family: var(--font-sans);
  color: var(--text-1);
  background: var(--surface);
  outline: none;
}

</style>
