<script lang="ts">
  import { onMount } from 'svelte';
  import { appState, initializeApp } from '$lib/stores';
  import Titlebar from '$lib/components/Titlebar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import AgentsView from '$lib/components/AgentsView.svelte';
  import SkillsView from '$lib/components/SkillsView.svelte';
  import InstructionsView from '$lib/components/InstructionsView.svelte';
  import MCPView from '$lib/components/MCPView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';

  let initialized = false;

  onMount(async () => {
    await initializeApp();
    initialized = true;
  });
</script>

<Titlebar />

<div class="app-container">
  <!-- Ambient background orbs -->
  <div class="ambient-bg">
    <div class="ambient-orb ambient-orb-1"></div>
    <div class="ambient-orb ambient-orb-2"></div>
  </div>

  {#if initialized}
    <Sidebar />
    <main class="main-content">
      {#if $appState.currentView === 'agents'}
        <AgentsView />
      {:else if $appState.currentView === 'skills'}
        <SkillsView />
      {:else if $appState.currentView === 'instructions'}
        <InstructionsView />
      {:else if $appState.currentView === 'mcp'}
        <MCPView />
      {:else if $appState.currentView === 'settings'}
        <SettingsView />
      {/if}
    </main>
  {:else}
    <div class="loading-screen">
      <div class="loading-content">
        <div class="loading-icon">🧠</div>
        <h1 class="loading-title">Prompt Forge</h1>
        <p class="loading-subtitle">Preparing your workspace...</p>
        <div class="spinner"></div>
      </div>
    </div>
  {/if}
</div>

<style>
  .app-container {
    display: flex;
    height: calc(100vh - 38px);
    width: 100vw;
    overflow: hidden;
    position: relative;
    margin-top: 38px;
  }

  .main-content {
    flex: 1;
    overflow: hidden;
    background: var(--color-bg-primary);
    position: relative;
  }

  .loading-screen {
    width: 100vw;
    height: calc(100vh - 38px);
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-bg-primary);
    position: relative;
  }

  .loading-content {
    text-align: center;
    animation: fade-in 0.6s ease-out;
  }

  .loading-icon {
    font-size: 4rem;
    margin-bottom: var(--space-lg);
    filter: drop-shadow(0 0 30px var(--color-accent-glow));
    animation: emotional-pulse 2.5s ease-in-out infinite;
  }

  .loading-title {
    font-family: var(--font-display);
    font-size: 2.5rem;
    font-weight: 400;
    letter-spacing: -0.03em;
    margin-bottom: var(--space-sm);
    background: linear-gradient(135deg, var(--color-accent-primary), var(--color-accent-tertiary));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .loading-subtitle {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin-bottom: var(--space-xl);
  }

  .loading-content .spinner {
    margin: 0 auto;
  }
</style>
