<script lang="ts">

  import { goto } from '$app/navigation'
  import ActivityBar from '$lib/components/ActivityBar.svelte';
  import { onMount } from 'svelte'

  let darkMode = false

  onMount (() => {
    darkMode = document.documentElement.classList.contains('dark')
  })

  function toggleDark() {
    darkMode = !darkMode
    document.documentElement.classList.toggle('dark', darkMode)
    localStorage.setItem('theme', darkMode ? 'dark' : 'light')
  }

</script>

<ActivityBar active="settings" onKanban={() => goto('/')} onSettings={() => {}} />

<main class="content">
  <div class="header">
    <h1>Settings</h1>
  </div>
  <div class="body">
    <div class="section">
      <p class="section-title">Appearance</p>
      <div class="setting-row">
        <div class="settings-info">
          <p class="setting-label">Dark mode</p>
          <p class="setting-desc">Switch to a darker colour scheme.</p>
        </div>
        <button
          class="toggle"
          class:on={darkMode}
          on:click={toggleDark}
          role="switch"
          aria-checked={darkMode}
        >
          <span class="toggle-thumb"></span>
        </button>
      </div>
    </div>
  </div>
</main>

<style>
.content {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 28px 32px 16px;
  border-bottom: 1px solid var(--border);
}

.header h1 {
  font-size: 22px;
  font-weight: 500;
  color: var(--text-1);
  letter-spacing: -0.01em;
}

.body {
  padding: 28px 32px;
  display: flex;
  flex-direction: column;
  gap: 32px;
  max-width: 560px;
}

.section {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.section-title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-3);
  font-family: var(--font-mono);
  margin-bottom: 12px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 0;
  border-bottom: 1px solid var(--border);
}

.setting-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-1);
}

.setting-desc {
  font-size: 12px;
  color: var(--text-3);
  margin-top: 2px;
}

.toggle {
  width: 40px;
  height: 24px;
  border-radius: 12px;
  border: none;
  background: var(--border);
  cursor: pointer;
  position: relative;
  transition: background 200ms;
  flex-shrink: 0;
}

.toggle.on {
  background: var(--text-1);
}

.toggle-thumb {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: white;
  transition: transform 200ms cubic-bezier(0.34, 1.56, 0.64, 1);
  box-shadow: 0 1px 3px rgba(0,0,0,0.15);
}

.toggle.on .toggle-thumb {
  transform: translateX(16px);
}
</style>

