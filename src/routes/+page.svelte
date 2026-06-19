<script lang="ts">
  import ActivityBar from '$lib/components/ActivityBar.svelte'
  import Sidebar from '$lib/components/Sidebar.svelte'
</script>

<ActivityBar active="kanban" />
<Sidebar boards={[]} activeBoardId={null} />

<main class="content">
  <div class="empty-state">
    <p>Select or create a board.</p>
  </div>
</main>

<style>
.content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-3);
  font-size: 13px;
}
</style>
