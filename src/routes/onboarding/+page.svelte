<script lang="ts">
  import { goto } from '$app/navigation';
  import { completeOnboarding } from '$lib/stores/config';

  let tools = [
    {
      id: 'kanban',
      label: 'Kanban board',
      description: 'Visual task management with columns and cards',
      available: true,
      selected: true,
    },
    {
      id: 'docs',
      label: 'Documents',
      description: 'Rich-text notes and wikis',
      available: false,
      selected: false,
    },
    {
      id: 'calendar',
      label: 'Calendar',
      description: 'Deadline and milestone tracking',
      available: false,
      selected: false,
    },
    {
      id: 'sprint',
      label: 'Sprint planner',
      description: 'Velocity tracking and sprint retrospectives',
      available: false,
      selected: false,
    },
  ];

  function toggle(tool: typeof tools[0]) {
    if (!tool.available) return;
    tool.selected = !tool.selected;
    tools = [...tools];
  }

  async function confirm() {
    await completeOnboarding({
      kanban: tools.find(t => t.id === 'kanban')?.selected ?? false,
      docs: tools.find(t => t.id === 'docs')?.selected ?? false,
      calendar: tools.find(t => t.id === 'calendar')?.selected ?? false,
      sprint: tools.find(t => t.id === 'sprint')?.selected ?? false,
    });
    goto('/');
  }
</script>

<div class="onboarding">
  <div class="panel">
    <div class="header">
      <p class="eyebrow">Welcome to Slate</p>
      <h1>Choose your tools</h1>
      <p class="sub">You can change this later in settings.</p>
    </div>

    <div class="grid">
      {#each tools as tool}
        <button
          class="tool-card"
          class:selected={tool.selected}
          class:disabled={!tool.available}
          on:click={() => toggle(tool)}
          disabled={!tool.available}
        >
          <div class="tool-top">
            <span class="tool-label">{tool.label}</span>
            {#if !tool.available}
              <span class="badge">Soon</span>
            {:else if tool.selected}
              <span class="check">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                  <path d="M2.5 7L5.5 10L11.5 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </span>
            {/if}
          </div>
          <p class="tool-desc">{tool.description}</p>
        </button>
      {/each}
    </div>

    <button class="cta" on:click={confirm}>
      Open Slate
    </button>
  </div>
</div>

<style>
.onboarding {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--canvas);
}

.panel {
  width: 560px;
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.header { display: flex; flex-direction: column; gap: 6px; }

.eyebrow {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--text-3);
  font-family: var(--font-mono);
}

h1 {
  font-size: 28px;
  font-weight: 500;
  letter-spacing: -0.02em;
  line-height: 1.2;
  color: var(--text-1);
}

.sub { color: var(--text-2); font-size: 14px; }

.grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

.tool-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 16px;
  text-align: left;
  cursor: pointer;
  transition: border-color 150ms, box-shadow 150ms;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.tool-card:hover:not(.disabled) {
  border-color: #BBBAB6;
  box-shadow: 0 2px 8px rgba(0,0,0,0.04);
}

.tool-card.selected {
  border-color: var(--text-1);
}

.tool-card.disabled {
  opacity: 0.45;
  cursor: default;
}

.tool-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.tool-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-1);
}

.tool-desc {
  font-size: 12px;
  color: var(--text-2);
  line-height: 1.5;
}

.badge {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  background: #F7F6F3;
  border: 1px solid var(--border);
  border-radius: 3px;
  padding: 2px 6px;
  color: var(--text-3);
  font-family: var(--font-mono);
}

.check {
  width: 20px;
  height: 20px;
  background: var(--text-1);
  border-radius: 4px;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
}

.cta {
  background: var(--text-1);
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 11px 20px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background 150ms, transform 80ms;
  align-self: flex-end;
  font-family: var(--font-sans);
  letter-spacing: 0.01em;
}

.cta:hover { background: #333; }
.cta:active { transform: scale(0.98); }
</style>
