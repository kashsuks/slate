<script lang="ts">
  import '../app.css'
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { isOnboardingComplete, loadTools } from '$lib/stores/config'
  import { checkConnection } from '$lib/api'

  let ready = false
  let isOnboarding = false

  onMount(async () => {
    // Restore dark mode before anything renders
    const theme = localStorage.getItem('theme')
    if (theme === 'dark') {
      document.documentElement.classList.add('dark')
    }

    // Check server connection before any store calls fire
    await checkConnection()

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
    ready = true
  })
</script>

{#if isOnboarding}
  <slot />
{:else if ready}
  <div class="app-shell">
    <slot />
  </div>
{/if}

<style>
.app-shell {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--canvas);
}
</style>
