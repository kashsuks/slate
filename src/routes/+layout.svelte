<script lang="ts">
  import '../app.css'
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { isOnboardingComplete, loadTools } from '$lib/stores/config'

  onMount(async () => {
    const done = await isOnboardingComplete()
    if (!done) {
      goto('/onboarding')
    } else {
      await loadTools()
    } 
  })
</script>

<slot />
