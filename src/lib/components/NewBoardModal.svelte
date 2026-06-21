<script lang="ts">
  import { createBoard } from "$lib/stores/board";

  export let open = false
  let name = ''

  function focusInput(node: HTMLElement) {
    node.focus()
  }

  async function submit() {
    console.log('[submit] name:', name)
    if (!name.trim()) return
    try {
      await createBoard(name.trim())
      name = ''
      open = false
    } catch (e) {
      console.error('[submit] Failed to create board:', e)
    }
  }

  function close() {
    name = ''
    open = false
  }
</script>

{#if open}
  <div
    class="overlay"
    role="presentation"
    on:click={close}
    on:keydown={(e) => e.key === 'Escape' && close()}
  >
    <div
      class="modal"
      role="dialog"
      aria-modal="true"
      aria-label="New board"
      tabindex="-1"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      <p class="modal-title">New board</p>
      <input
        type="text"
        bind:value={name}
        placeholder="Board name"
        on:keydown={(e) => { if (e.key === 'Enter') submit(); else if (e.key === 'Escape') close() }}
        use:focusInput
      />
      <div class="modal-actions">
        <button class="btn-secondary" on:click={close}>Cancel</button>
	<button class="btn-primary" on:click={submit}>Create</button>
      </div>
    </div>
  </div>
{/if}

<style>

.overlay{
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 50;
}

.modal {
  width: 360px;
  background: var(--surface);
  border-radius: 10px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
}

.modal-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-1);
}

input {
  width: 100%;
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 9px 11px;
  font-size: 13px;
  font-family: var(--font-sans);
  outline: none;
}

input:focus {
  border-color: var(--text-1);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.btn-secondary, .btn-primary {
  border: none;
  border-radius: 6px;
  padding: 8px 14px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  font-family: var(--font-sans);
}

.btn-secondary {
  background: transparent;
  color: var(--text-2);
}

.btn-secondary:hover {
  background: var(--canvas);
}

.btn-primary {
  background: var(--text-1);
  color: #fff;
}

.btn-primary:hover {
  background: #333;
}

</style>
