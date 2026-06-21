<script lang="ts">
  import { goto } from "$app/navigation"
  import ActivityBar from "$lib/components/ActivityBar.svelte"
</script>

<ActivityBar active="settings" onKanban={() => goto('/')} />

<main class="content">
  <div class="header">
    <h1>Settings</h1>
  </div>
  <div class="body">
    <p class="placeholder">Nothing here yet.</p>
  </div>
</main>

<style>
.content {
  flex: 1;
  overflow: hidden;
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
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder {
  color: var(--text-3);
  font-size: 14px;
}
</style>
