<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { getServerUrl, checkConnection } from '$lib/api'
  import { getToken } from '$lib/stores/auth'
  import type { Board, Column, Card } from '$lib/types'

  export let onDone: () => void = () => {}

  let state: 'prompt' | 'migrating' | 'done' | 'error' = 'prompt'
  let progress = ''
  let localBoardCount = 0
  let errorMsg = ''

  // Load local board count on mount
  import { onMount } from 'svelte'
  onMount(async () => {
    const boards = await invoke<Board[]>('get_boards')
    localBoardCount = boards.length
  })

  async function migrate() {
    state = 'migrating'
    const url = getServerUrl()!
    const token = getToken()!
    const headers = {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    }

    try {
      const boards = await invoke<Board[]>('get_boards')

      for (const board of boards) {
        progress = `Migrating "${board.name}"...`

        // Create board on server
        const boardRes = await fetch(`${url}/api/boards`, {
          method: 'POST',
          headers,
          body: JSON.stringify({ name: board.name }),
        })
        if (!boardRes.ok) throw new Error(`Failed to create board: ${board.name}`)
        const newBoard = await boardRes.json()

        // Get local columns
        const columns = await invoke<Column[]>('get_columns', { boardId: board.id })

        for (const col of columns) {
          // Create column on server
          const colRes = await fetch(`${url}/api/columns`, {
            method: 'POST',
            headers,
            body: JSON.stringify({ board_id: newBoard.id, name: col.name }),
          })
          if (!colRes.ok) throw new Error(`Failed to create column: ${col.name}`)
          const newCol = await colRes.json()

          // Update color if non-default
          if (col.color !== '#EAEAEA') {
            await fetch(`${url}/api/columns/${newCol.id}/color`, {
              method: 'PUT',
              headers,
              body: JSON.stringify({ color: col.color }),
            })
          }

          // Get local cards
          const cards = await invoke<Card[]>('get_cards', { columnId: col.id })

          for (const card of cards) {
            await fetch(`${url}/api/cards`, {
              method: 'POST',
              headers,
              body: JSON.stringify({ column_id: newCol.id, title: card.title }),
            }).then(async (res) => {
              if (!res.ok) return
              const newCard = await res.json()
              // Update full card details if there's anything beyond the title
              if (card.description || card.priority !== 'none' || card.due_date) {
                await fetch(`${url}/api/cards/${newCard.id}`, {
                  method: 'PUT',
                  headers,
                  body: JSON.stringify({
                    title: card.title,
                    description: card.description,
                    priority: card.priority,
                    due_date: card.due_date,
                  }),
                })
              }
            })
          }
        }
      }

      // Mark migration as done for this server URL
      localStorage.setItem(`migratedToServer:${url}`, 'true')
      progress = ''
      state = 'done'
    } catch (e: any) {
      errorMsg = e.message ?? 'Migration failed.'
      state = 'error'
    }
  }

  function skip() {
    const url = getServerUrl()!
    localStorage.setItem(`migratedToServer:${url}`, 'true')
    onDone()
  }
</script>

<!-- Backdrop -->
<div class="backdrop"></div>

<!-- Modal -->
<div class="modal">
  {#if state === 'prompt'}
    <div class="icon-wrap">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
        <path d="M5 12h14M12 5l7 7-7 7" stroke="#111111" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>

    <div class="body">
      <h2>Push local boards to server?</h2>
      <p>
        You have {localBoardCount} local board{localBoardCount === 1 ? '' : 's'}.
        Migrate them to the shared server so they're available to everyone on the network.
        Your local data stays intact.
      </p>
    </div>

    <div class="actions">
      <button class="btn-ghost" on:click={skip}>Start fresh</button>
      <button class="btn-primary" on:click={migrate}>
        Migrate {localBoardCount} board{localBoardCount === 1 ? '' : 's'}
      </button>
    </div>

  {:else if state === 'migrating'}
    <div class="migrating">
      <div class="spinner"></div>
      <p class="progress-label">{progress}</p>
    </div>

  {:else if state === 'done'}
    <div class="icon-wrap success">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
        <path d="M5 13l4 4L19 7" stroke="#346538" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>

    <div class="body">
      <h2>Migration complete</h2>
      <p>Your boards are now on the server and available to everyone on the network.</p>
    </div>

    <div class="actions">
      <button class="btn-primary" on:click={onDone}>Open workspace</button>
    </div>

  {:else if state === 'error'}
    <div class="icon-wrap danger">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
        <path d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" stroke="#9F2F2D" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </div>

    <div class="body">
      <h2>Migration failed</h2>
      <p class="error-msg">{errorMsg}</p>
    </div>

    <div class="actions">
      <button class="btn-ghost" on:click={skip}>Skip for now</button>
      <button class="btn-primary" on:click={() => { state = 'prompt'; errorMsg = '' }}>
        Try again
      </button>
    </div>
  {/if}
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
    background: rgba(0, 0, 0, 0.12);
    backdrop-filter: blur(2px);
    -webkit-backdrop-filter: blur(2px);
  }

  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 50;
    width: 100%;
    max-width: 420px;
    background: #FFFFFF;
    border: 1px solid #EAEAEA;
    border-radius: 12px;
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .icon-wrap {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    background: #F7F6F3;
    border: 1px solid #EAEAEA;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .icon-wrap.success {
    background: #EDF3EC;
    border-color: #C8DFC7;
  }

  .icon-wrap.danger {
    background: #FDEBEC;
    border-color: #F5C4C3;
  }

  .body {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  h2 {
    font-family: 'SF Pro Display', 'Helvetica Neue', sans-serif;
    font-size: 16px;
    font-weight: 500;
    letter-spacing: -0.01em;
    color: #111111;
    margin: 0;
  }

  p {
    font-size: 13px;
    color: #787774;
    line-height: 1.6;
    margin: 0;
  }

  .error-msg {
    color: #9F2F2D;
    background: #FDEBEC;
    border: 1px solid #F5C4C3;
    border-radius: 6px;
    padding: 8px 12px;
    font-family: 'Geist Mono', 'SF Mono', monospace;
    font-size: 11px;
  }

  .actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn-primary {
    padding: 9px 16px;
    background: #111111;
    color: #FFFFFF;
    border: none;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    font-family: 'SF Pro Display', 'Helvetica Neue', sans-serif;
    cursor: pointer;
    transition: background 150ms, transform 80ms;
  }

  .btn-primary:hover {
    background: #333333;
  }

  .btn-primary:active {
    transform: scale(0.98);
  }

  .btn-ghost {
    padding: 9px 16px;
    background: transparent;
    color: #787774;
    border: 1px solid #EAEAEA;
    border-radius: 6px;
    font-size: 13px;
    font-family: 'SF Pro Display', 'Helvetica Neue', sans-serif;
    cursor: pointer;
    transition: border-color 150ms, color 150ms;
  }

  .btn-ghost:hover {
    border-color: #BBBAB6;
    color: #111111;
  }

  .migrating {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 16px 0;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 1.5px solid #EAEAEA;
    border-top-color: #111111;
    border-radius: 50%;
    animation: spin 700ms linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .progress-label {
    font-size: 12px;
    font-family: 'Geist Mono', 'SF Mono', monospace;
    color: #787774;
    text-align: center;
  }
</style>
