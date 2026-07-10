<script lang="ts">
    import { getServerUrl } from '$lib/api';

  
  import { goto } from '$app/navigation'
  import { apiLogin, apiSetup, apiRegister, getServerUrl } from '$lib/api'
  import { saveAuth } from '$lib/stores/auth'

  // mode: login, setup, register
  export let mode: 'login' | 'setup' | 'register' = 'login'

  //read the invite token from URL if present
  import { page } from '$app/stores'
  $: inviteToken = $page.url.searchParams.get('invite') ?? ''
  $: if (inviteToken) mode = 'register'

  let username = ''
  let password = ''
  let error = ''
  let loading = false

  const serverUrl = getServerUrl()

  const titles = {
    login: 'Sign in',
    setup: 'Create your account',
    register: 'Join the workspace',
  }

  const subtitles = {
    login: 'Continue to your workspace.',
    setup: 'First time-setup - you\'ll be the server owner.',
    register: 'You\'ve been invited to a shared Slate workspace.',
  }

  const ctas = {
    login: 'Sign in',
    setup: 'Create account',
    register: 'Create account',
  }

  async function submit() {
    error = ''
    if (!username.trim() || !password.trim()) {
      error = 'Username and password are required.'
      return
    }
    if (password.length < 8) {
      error = 'Password must be at least 8 characters.'
      return
    }

    loading = true
    try {
      let result
      if (mode === 'setup') {
        result = await apiSetup(username.trim(), password)
      } else if (mode === 'register') {
        result = await apiRegister(username.trim(), password, inviteToken)
      } else {
        result = await apiLogin(username.trim(), password)
      }
      saveAuth(result.token, result.user_id, result.username)
      goto('/')
    } catch (e: any) {
       if (e.message.inludes('401')) error = 'Incorrect username or password.'
       else if (e.message.includes('409')) error = 'Username already taken.'
       else if (e.message.includes('403')) error = 'Invalid or expired invite link.'
       else error = 'Something went wrong. Check your connection.'
    } finally {
      loading = false
    }
  }

</script>

<div class="shell">
  <div class="card">

    <div class="wordmark">Slate</div>

    <div class="header">
      <h1>{titles[mode]}</h1>
      <p class="sub">{subtitles[mode]}</p>
    </div>

    <div class="form">
      <div class="field">
        <label for="username">Username</label>
	<input
	  id="username"
	  type="text"
	  bind:value={username}
	  placeholder="your username"
	  autocomplete="username"
	  on:keydown={(e) => e.key === 'Enter' && submit()}
	  disabled={loading}
	/>
      </div>

      <div class="field">
        <label for="password">Password</label>
	<input
	  id="password"
	  type="password"
	  bind:value={password}
	  placeholder={mode === 'login' ? '••••••••' : 'min. 8 characters'}
	  autocomplete={mode === 'login' ? 'current-password' : 'new-password'}
	  on:keydown={(e) => e.key === 'Enter' && submit()}
	  disabled={loading}
	/>
      </div>

      {#if error}
        <p class="error">{error}</p>
      {/if}

      <button class="cta" on:click={submit} disabled={loading}>
        {#if loading}
	  <span class="spinner"></span>
	{:else}
	  {ctas[mode]}
	{/if}
      </button>
    </div>

    {#if mode === 'login'}
      <p class="footer-hint">
        Need an account? Ask the server owner for an invite link.
      </p>
    {:else if mode !== 'setup'}
      <p class="footer-hint">
        Already have an account?
        <button class="text-btn" on:click={() => { mode = 'login'; error = '' }}>
          Sign in
        </button>
      </p>
    {/if}

    <div class="server-tag">
      <span class="dot"></span>
      {serverUrl ?? 'No server configured'}
    </div>
  
  </div>
</div>

<style>

  .shell {
    min-height: 100vh;
    background: #F7F6F3;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
  }

  .card {
    background: #FFFFFF;
    border: 1px solid #EAEAEA;
    border-radius: 12px;
    padding: 40px;
    width: 100%;
    max-width: 400px;
    display: flex;
    flex-direction: column;
    gap: 28px;
  }

  .wordmark {
    font-family: 'Newsreader', 'Playfair Display', 'Georgia', serif;
    font-size: 22px;
    font-weight: 500;
    letter-spacing: -0.02em;
    color: #111111;
  }

  .header {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  h1 {
    'SF Pro Display', 'Helvetica Neue', sans-serif;
    font-size: 20px;
    font-weight: 500;
    letter-spacing: -0.02em;
    color: #111111;
    margin: 0;
  }

  .sub {
    font-size: 13px;
    color: #787774;
    margin: 0;
    line-height: 1.5;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  label {
    font-size: 11px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #787774;
    font-family: 'Geist Mono', 'SF Mono', monospace;
  }

  input {
    width: 100%;
    padding: 10px 12px;
    font-size: 14px;
    font-family: 'SF Pro Display', 'Helvetica Neue', sans-serif;
    color: #111111;
    background: #FBFBFA;
    border: 1px solid #EAEAEA;
    border-radius: 6px;
    outline: none;
    transition: border-color 150ms;
    box-sizing: border-box;
  }

  input:focus {
    border-color: #111111;
    background: #FFFFFF;
  }

  input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  input::placeholder {
    color: #BBBAB6;
  }

  .error {
    font-size: 12px;
    color: #9F2F2D;
    background: #FDEBEC;
    border: 1px solid #F5C4C3;
    border-radius: 6px;
    padding: 8px 12px;
    margin: 0;
  }

  .cta {
    width: 100%;
    padding: 11px;
    background: #111111;
    color: #FFFFFF;
    border: none;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    font-family: 'SF Pro Display', 'Helvetica Neue', sans-serif;
    cursor: pointer;
    transition: background 150ms, transform 80ms;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 40px;
  }

  .cta:hover:not(:disabled) {
    background: #333333;
  }

  .cta:active:not(:disabled) {
    transform: scale(0.98);
  }

  .cta:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 1.5px solid rgba(255,255,255,0.3);
    border-top-color: #FFFFFF;
    border-radius: 50%;
    animation: spin 600ms linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .footer-hint {
    font-size: 12px;
    color: #787774;
    text-align: center;
    margin: 0;
  }

  .text-btn {
    background: none;
    border: none;
    color: #111111;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    padding: 0;
    text-decoration: underline;
    text-underline-offset: 2px;
    text-decoration-color: #EAEAEA;
  }

  .text-btn:hover {
    text-decoration-color: #111111;
  }

  .server-tag {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    font-family: 'Geist Mono', 'SF Mono', monospace;
    color: #BBBAB6;
    border-top: 1px solid #EAEAEA;
    padding-top: 16px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #346538;
    flex-shrink: 0;
  }

</style>
