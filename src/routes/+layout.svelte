<script lang="ts">
  import '../app.css'
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { isOnboardingComplete, loadTools } from '$lib/stores/config'
  import { checkConnection, isConnected, getServerUrl } from '$lib/api'
  import { loadAuthFromStorage, getToken } from '$lib/stores/auth'
  import MigrationModal from '$lib/components/MigrationModal.svelte'

  let ready = false
  let isOnboarding = false
  let showMigration = false

  function needsMigration(): boolean {
    const url = getServerUrl()
    if (!url || !isConnected()) return false
    return !localStorage.getItem(`migratedToServer:${url}`)
  }

  onMount(async () => {
    // Restore dark mode before anything renders
    const theme = localStorage.getItem('theme')
    if (theme === 'dark') {
      document.documentElement.classList.add('dark')
    }

    loadAuthFromStorage()

    // Check server connection before any store calls fire
    await checkConnection()

    if (isConnected() && !getToken()) {
      goto('/login')
      return
    }

    isOnboarding = window.location.pathname.startsWith('/onboarding')
    if (isOnboarding) {
      ready = true
      return
    }
    const done = await isOnboardingComplete()
    if (!done) {
      goto('/onboarding')
      isOnboarding = true
    } else {
      await loadTools()
    }

    // show the migration modal if first time connecting to this server
    if (needsMigration()) {
      showMigration = true
    }

    ready = true
  })

  function onMigrationDone() {
    showMigration = false
  }
</script>

{#if isOnboarding}
  <slot />
{:else if ready}
  <div class="app-shell">
    <slot />
  </div>
  {#if showMigration}
    <MigrationModal onDone={onMigrationDone} />
  {/if}
{/if}

<style>
.app-shell {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--canvas);
}
</style>
