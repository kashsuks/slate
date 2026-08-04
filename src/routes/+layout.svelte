<script lang="ts">
  import '../app.css'
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { isOnboardingComplete, loadTools } from '$lib/stores/config'
  import { checkConnection, isConnected, getServerUrl } from '$lib/api'
  import { loadAuthFromStorage, getToken } from '$lib/stores/auth'
  import MigrationModal from '$lib/components/MigrationModal.svelte'

  let ready = false
  let showMigration = false

  // Reactive to the current route rather than a one-time onMount snapshot -
  // otherwise navigating away from /onboarding (client-side, no reload)
  // leaves this stuck true and the app renders outside .app-shell, which
  // the main app layout depends on for sizing (looks like a blank screen).
  $: isOnboarding = $page.url.pathname.startsWith('/onboarding')

  function needsMigration(): boolean {
    const url = getServerUrl()
    if (!url || !isConnected()) return false
    return !localStorage.getItem(`migratedToServer:${url}`)
  }

  async function init(attempt = 0) {
    try {
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

      if (isOnboarding) {
        ready = true
        return
      }
      const done = await isOnboardingComplete()
      if (!done) {
        goto('/onboarding')
      } else {
        await loadTools()
      }

      // show the migration modal if first time connecting to this server
      if (needsMigration()) {
        showMigration = true
      }

      ready = true
    } catch (e) {
      // In standalone (Tauri) mode, the very first invoke() calls can lose a
      // startup race against Rust's .setup() (which opens the sqlite db and
      // calls app.manage() before commands relying on that state work). This
      // is more common on a slow first run - retry briefly instead of
      // leaving the app on a blank screen.
      if (attempt < 10) {
        setTimeout(() => init(attempt + 1), 200)
      } else {
        console.error('Failed to initialize app', e)
      }
    }
  }

  onMount(() => {
    init()
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
