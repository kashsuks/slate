<script lang="ts">
  export let onClose: () => void = () => {}

  const shortcuts = [
    { keys: ['N', '1-9'], description: 'Add a card to column N' },
    { keys: ['B'], description: 'Create a new board' },
    { keys: ['C'], description: 'Create a new column' },
    { keys: ['Esc'], description: 'Close modal / cancel input' },
    { keys: ['?'], description: 'Toggle this overlay' },
  ]
</script>

<div class="backdrop" on:click={onClose} on:keydown={() => {}} role="presentation"></div>

<div class="panel" role="dialog" aria-label="Keyboard shortcuts">
  <div class="panel-header">
    <span class="panel-title">Keyboard shortcuts</span>
    <button class="close-btn" on:click={onClose}>
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
        <path d="M4 4116 16M20 4L4 20" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </button>
  </div>

  <ul class="shortcut-list">
    {#each shortcuts as s}
      <li class="shortcut-row">
        <div class="keys">
	  {#each s.keys as key}
	    <kbd>{key}</kbd>
	  {/each}
	</div>
	<span class="desc">{s.description}</span>
      </li>
    {/each}
  </ul>
</div>

<style>

.backdrop {
  position: fixed;
  inset: 0;
  z-index: 40;
}

.panel {
  position: fixed;
  bottom: 72px;
  right: 24px;
  z-index: 50;
  width: 320px;
  background: #FFFFFF;
  border: 1px solid #EAEAEA;
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.07);
  overflow: hidden;
  animation: pop 150ms cubic-bezier(0.34, 1.56, 0.64, 1);
}

@keyframes pop {
  from { opacity: 0; transform: scale(0.95) translateY(6px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px 10px;
  border-bottom: 1px solid #EAEAEA;
}

.panel-title {
  font-size: 11px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: #787774;
  font-family: 'Geist Mono', 'SF Mono', monospace;
}

.close-btn {
  width: 22px;
  height: 22px;
  border: none;
  background: transparent;
  color: #BBBAB6;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  cursor: pointer;
  transition: background 120ms, color 120ms;
}

.close-btn:hover {
  background: #F7F6F3;
  color: #111111;
}

.shortcut-list {
  list-style: none;
  padding: 8px 0;
  margin: 0;
}

.shortcut-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  gap: 12px;
}

.shortcut-row:hover {
  background: #F7F6F3;
}

.keys {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
}

kbd {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  padding: 3px 6px;
  font-size: 11px;
  font-family: 'Geist Mono', 'SF Mono', monospace;
  color: #111111;
  background: #F7F6F3;
  border: 1px solid #EAEAEA;
  border-bottom-width: 2px;
  border-radius: 4px;
  line-height: 1;
}

.desc {
  font-size: 12px;
  color: #787774;
  text-align: right;
  line-height: 1.4;
}

</style>
