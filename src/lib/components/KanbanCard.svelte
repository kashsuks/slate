<script lang="ts">
  import type { Card } from '$lib/types'
  import { renameCard } from '$lib/stores/board';
    import { preventDefault } from 'svelte/legacy';

  export let card: Card

  const priorityColor: Record<string, string> = {
    high: '#9F2F2D',
    medium: '#956400',
    low: '#346538',
    none: 'transparent',
  }

  let renaming = false
  let draft = ''

  function startRename(e: MouseEvent) {
    e.stopPropagation()
    draft = card.title
    renaming = true
  }

  async function commitRename() {
    renaming = false
    const trimmed = draft.trim()
    if (!trimmed || trimmed === card.title) return
    await renameCard(card.column_id, card.id, trimmed)
  }

  function cancelRename() {
    renaming = false
    draft = card.title
  }

  function focusAll(node: HTMLTextAreaElement) {
    node.focus()
    node.select()
  }
</script>

<div class="card">
  {#if card.priority !== 'none'}
    <span class="priority-dot" style="background: {priorityColor[card.priority]}"></span>
  {/if}

  {#if renaming}
    <textarea
      class="rename-textarea"
      bind:value={draft}
      rows="2"
      on:keydown={(e) => {
        if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); commitRename() }
	if (e.key === 'Escape') cancelRename()
      }}
      on:blur={commitRename}
      use:focusAll
    />
  {:else}
    <p class="title" on:click={startRename} title="Click to rename">{card.title}</p>
  {/if}

  {#if card.due_date}
    <p class="due">{card.due_date}</p>
  {/if}
</div>

<style>
.card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 14px 16px;
  cursor: pointer;
  transition: border-color 120ms, box-shadow 120ms;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.card:hover {
  border-color: #BBBAB6;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.priority-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.title {
  font-size: 14px;
  color: var(--text-1);
  line-height: 1.45;
}

.title:hover {
  text-decoration: underline;
  text-decoration-color: var(--border);
  text-underline-offset: 3px;
}

.rename-textarea {
  font-size: 14px;
  font-family: var(--font-sans);
  color: var(--text-1);
  line-height: 1.45;
  background: var(--surface);
  border: 1px solid var(--text-1);
  border-radius: 4px;
  padding: 2px 6px;
  outline: none;
  resize: none;
  width: 100%;
}

.due {
  font-size: 11px;
  color: var(--text-3);
  font-family: var(--font-mono);
}
</style>
