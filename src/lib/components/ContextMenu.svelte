<script lang="ts">
    import { preventDefault, stopPropagation } from "svelte/legacy";

  export let x = 0
  export let y = 0
  export let items: { label: string; danger?: boolean; action: () => void }[] = []
  export let onClose: () => void = () => {}

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Escape') onClose()
  }
</script>

<svelte:window on:keydown={handleKey} />

<div class="overlay" on:click={onClose} on:contextmenu|preventDefault={onClose}>
  <div
    class="menu"
    style="left: {x}px; top: {y}px"
    on:click|stopPropagation
  >
    {#each items as item}
      <button
        class="menu-item"
	class:danger={item.danger}
	on:click={() => { item.action(); onClose() }}
      >
        {item.label}
      </button>
    {/each}
  </div>
</div>

<style>

.overlay {
  position: fixed;
  inset: 0;
  z-index: 100;
  background: transparent
}

.menu {
  position: fixed;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 4px;
  min-width: 160px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.10), 0 1px 4px rgba(0, 0, 0, 0.06);
  display: flex;
  flex-direction: column;
  gap: 1px;
  z-index: 101;
}

.menu-item {
  width: 100%;
  text-align: left;
  padding: 8px 10px;
  border: none;
  background: transparent;
  border-radius: 5px;
  font-size: 13px;
  font-family: var(--font-sans);
  color: var(--text-1);
  cursor: pointer;
  transition: background 100ms;
}

.menu-item:hover {
  background: var(--canvas);
}

.menu-item.danger {
  color: #9F2F2D;
}

.menu-item.danger:hover {
  background: #FFF0F0;
}

</style>
