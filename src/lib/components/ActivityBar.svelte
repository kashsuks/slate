<script lang="ts">
  import { onMount, onDestroy } from "svelte"
  import { getServerUrl, isConnected, checkConnection } from "$lib/api"

  export let active: string = 'kanban'
  export let onKanban: () => void = () => {}
  export let onSettings: () => void = () => {}

  let connected = false
  let serverUrl: string | null = null
  let pollInterval: ReturnType<typeof setInterval> | null = null

  onMount(async () => {
    serverUrl = getServerUrl()
    connected = isConnected()

    // recheck every 10s so the dot updates if the server goes down
    if (serverUrl) {
      pollInterval = setInterval(async () => {
        connected = await checkConnection()
      }, 10_000)
    }
  })

  onDestroy(() => {
    if (pollInterval) clearInterval(pollInterval)
  })

  $: tooltipText = !serverUrl
    ? 'Standalone - no server configured'
    : connected
      ? `Connected · ${serverUrl}`
      : `Unreachable · ${serverUrl}`
</script>

<nav class="activity-bar">
  <div class="top">
    <div class="logo">S</div>
    <button
      class="icon-btn"
      class:active={active === 'kanban' }
      title="Kanban"
      on:click={onKanban}
    >
      <svg width="20" height="20" viewBox="0 0 18 18" fill="none">
        <rect x="1.5" y="3" width="5" height="12" rx="1.5" stroke="currentColor" stroke-width="1.4"/>
	<rect x="9.5" y="3" width="5" height="8" rx="1.5" stroke="currentColor" stroke-width="1.4"/>
	<rect x="1.5" y="3" width="5" height="12" rx="1.5" stroke="currentColor" stroke-width="1.4"/>
      </svg>
    </button>
  </div>

  <div class="bottom">
    <button 
      class="icon-btn"
      class:active={active === 'settings'}
      title="Settings"
      on:click={onSettings}
    >
      <img src="/icons/settings.svg" alt="Settings" width="18" height="18" class="icon-asset" />
    </button>

    <div class="status-wrap">
      <div
        class="status-dot"
	class:connected
	class:standalone={!serverUrl}
      ></div>
      <div class="status-tooltip">
        <span class="tootlip-dot" class:connected class:standalone={!serverUrl}></span>
	{tooltipText}
      </div>
    </div>
  </div>
</nav>

<style>
.activity-bar {
  width: 60px;
  height: 100vh;
  background: var(--surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  flex-shrink: 0;
}

.top, .bottom {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.logo {
  width: 34px;
  height: 34px;
  background: var(--text-1);
  color: #fff;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 15px;
  font-weight: 600;
  margin-bottom: 16px;
  letter-spacing: -0.02em;
}

.icon-btn {
  width: 42px;
  height: 42px;
  border-radius: 8px;
  border: none;
  background: transparent;
  color: var(--text-3);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 120ms, color 120ms;
}

.icon-btn:hover {
  background: var(--canvas);
  color: var(--text-1);
}

.icon-btn.active {
  background: var(--canvas);
  color: var(--text-1);
}

.icon-asset {
  opacity: 0.45;
  transition: opacity var(--transition-fast);
}

.icon-btn:hover .icon-asset,
.icon-btn.active .icon-asset {
  opacity: 1;
}

.status-wrap {
  position: relative;
  width: 42px;
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 4px;
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: #BBBAB6;
  transition: background 300ms;
  flex-shrink: 0;
}

.status-dot.connected {
  background: #346538;
}

.status-dot:not(.connected):not(.standalone) {
  background: #9F2F2D;
}

.status-tooltip {
  positon: absolute;
  bottom: calc(100% + 8px);
  left: 50%;
  transform: translateX(-50%) translateX(26px);
  background: #FFFFFF;
  border: 1px solid #EAEAEA;
  border-radius: 6px;
  padding: 7px 10px;
  white-space: nowrap;
  font-size: 11px;
  font-family: 'Geist Mono', 'SF Mono', monospace;
  color: #787774;
  pointer-events: none;
  opacity: 0;
  transition: opacity 150ms, transform 150ms;
  transform-origin: bottom left;
  display: flex;
  align-items: center;
  gap: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.);
  z-index: 100;
}

.status-wrap:hover .status-tooltip {
  opacity: 1;
  transform: translateX(-50%) translateX(26px) translateY(-2px);
}

.tooltip-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #BBBAB6;
  flex-shrink: 0;
}

.tooltip-dot.connected {
  background: #346538;
}

.tooltip-dot:not(.connected):not(.standalone) {
  background: #9F2F2D;
}

</style>
