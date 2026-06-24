<script lang="ts">
  export let active: string = 'kanban'
  export let onKanban: () => void = () => {}
  export let onSettings: () => void = () => {}
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
      <svg width="20" height="20" viewBox="0 0 18 18" fill="none">
        <circle cx="9" cy="9" r="2.5" stroke="currentColor" stroke-width="1.4"/>
	<path d="M9 1.5v2M9 14.5v2M1.5 9h2M14.5 9h2M3.2 3.211.4 1.4M13.4 13.411.4 1.4M14.8 3.2l-1.4 1.4M4.6 13.4l-1.4 1.4" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
      </svg>
    </button>
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

</style>
