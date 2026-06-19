<script lang="ts">
  import '../app.css'
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { isOnboardingComplete, loadTools } from '$lib/stores/config'

  let ready = false

  onMount(async () => {
    const done = await isOnboardingComplete()
    if (!done) {
      goto('/onboarding')
    } else {
      await loadTools()
    }
    ready = true
  })

  $: isOnboarding = $page.url.pathname.startsWith('/onboarding')
</script>

{#if isOnboarding}
  <slot />
{:else if ready}
  <div class="app-shell">
    <slot />
  </div>
{:else}
  <div></div>
{/if}

<style>
.app-shell {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--canvas);
}
</style>
