<script lang="ts">
  import { appState, mcpStatus, setView, toggleSidebar, startMcpServer, stopMcpServer } from '$lib/stores';
  import type { View } from '$lib/types';

  const navItems: { id: View; label: string; icon: string }[] = [
    { id: 'agents', label: 'Agents', icon: '🤖' },
    { id: 'skills', label: 'Skills', icon: '⚡' },
    { id: 'instructions', label: 'Instructions', icon: '📋' },
    { id: 'mcp', label: 'MCP Server', icon: '🔌' },
    { id: 'settings', label: 'Settings', icon: '⚙️' },
  ];

  async function handleMcpToggle() {
    if ($mcpStatus.running) {
      await stopMcpServer();
    } else {
      await startMcpServer();
    }
  }
</script>

<aside class="sidebar" class:collapsed={$appState.sidebarCollapsed}>
  <div class="sidebar-header">
    <div class="logo">
      <span class="logo-icon">⚒️</span>
      {#if !$appState.sidebarCollapsed}
        <span class="logo-text gradient-text">Prompt Forge</span>
      {/if}
    </div>
    <button class="btn-icon" onclick={toggleSidebar} aria-label="Toggle sidebar">
      {$appState.sidebarCollapsed ? '→' : '←'}
    </button>
  </div>

  <nav class="sidebar-nav">
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={$appState.currentView === item.id}
        onclick={() => setView(item.id)}
      >
        <span class="nav-icon">{item.icon}</span>
        {#if !$appState.sidebarCollapsed}
          <span class="nav-label">{item.label}</span>
        {/if}
      </button>
    {/each}
  </nav>

  {#if !$appState.sidebarCollapsed}
    <div class="sidebar-section mcp-status">
      <div class="section-header">
        <span class="section-title">MCP Server</span>
      </div>
      <div class="mcp-info">
        <div class="mcp-indicator" class:running={$mcpStatus.running}>
          <span class="indicator-dot"></span>
          <span class="indicator-text">
            {$mcpStatus.running ? 'Running' : 'Stopped'}
          </span>
        </div>
        {#if $mcpStatus.running}
          <div class="mcp-details">
            <span class="detail">Port: {$mcpStatus.port}</span>
            <span class="detail">Tools: {$mcpStatus.available_tools.length}</span>
          </div>
        {/if}
        <button
          class="btn mcp-toggle"
          class:running={$mcpStatus.running}
          onclick={handleMcpToggle}
        >
          {$mcpStatus.running ? '⏹️ Stop' : '▶️ Start'}
        </button>
      </div>
    </div>
  {/if}

  <div class="sidebar-footer">
    {#if !$appState.sidebarCollapsed}
      <div class="version-info">
        <span>Prompt Forge v0.2.0</span>
      </div>
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    height: 100vh;
    background: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    transition: width var(--transition-slow);
    overflow: hidden;
    position: relative;
  }

  .sidebar::after {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    width: 1px;
    height: 100%;
    background: linear-gradient(180deg, transparent, rgba(212, 165, 116, 0.1), transparent);
    pointer-events: none;
  }

  .sidebar.collapsed {
    width: var(--sidebar-collapsed-width);
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1rem;
    border-bottom: 1px solid var(--color-border);
    min-height: 72px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.875rem;
  }

  .logo-icon {
    font-size: 1.75rem;
    filter: drop-shadow(0 0 8px var(--color-accent-glow));
    animation: emotional-pulse 4s ease-in-out infinite;
  }

  .logo-text {
    font-family: var(--font-display);
    font-weight: 400;
    font-size: 1.25rem;
    letter-spacing: -0.02em;
    white-space: nowrap;
  }

  .btn-icon {
    background: transparent;
    border: none;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0.5rem;
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
    font-size: 0.875rem;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-icon:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    padding: 1rem 0.75rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    padding: 0.75rem 1rem;
    border-radius: var(--radius-lg);
    background: transparent;
    border: 1px solid transparent;
    color: var(--color-text-tertiary);
    cursor: pointer;
    transition: all var(--transition-fast);
    width: 100%;
    text-align: left;
    font-family: var(--font-sans);
    font-size: 0.9rem;
    font-weight: 500;
  }

  .nav-item:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    border-color: var(--color-border);
  }

  .nav-item.active {
    background: var(--color-accent-glow);
    color: var(--color-accent-primary);
    border-color: rgba(212, 165, 116, 0.15);
  }

  .nav-icon {
    font-size: 1.125rem;
    width: 24px;
    text-align: center;
    opacity: 0.9;
  }

  .nav-label {
    font-weight: 500;
    white-space: nowrap;
    letter-spacing: -0.01em;
  }

  .sidebar-section {
    padding: 1rem;
    border-top: 1px solid var(--color-border);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.875rem;
  }

  .section-title {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--color-text-muted);
  }

  .mcp-status {
    margin-top: auto;
  }

  .mcp-info {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
  }

  .mcp-indicator {
    display: flex;
    align-items: center;
    gap: 0.625rem;
  }

  .indicator-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--color-text-muted);
    transition: all var(--transition-normal);
  }

  .mcp-indicator.running .indicator-dot {
    background: var(--color-accent-cool);
    box-shadow: 0 0 12px var(--color-accent-cool);
    animation: pulse 2.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.6; transform: scale(0.9); }
  }

  .indicator-text {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    letter-spacing: 0.02em;
  }

  .mcp-details {
    display: flex;
    gap: 1rem;
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--color-text-muted);
    letter-spacing: 0.02em;
  }

  .mcp-toggle {
    width: 100%;
    padding: 0.625rem;
    border-radius: var(--radius-md);
    font-family: var(--font-mono);
    font-size: 0.7rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    cursor: pointer;
    transition: all var(--transition-fast);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    color: var(--color-text-secondary);
  }

  .mcp-toggle:hover {
    background: var(--color-bg-hover);
    border-color: var(--color-border-hover);
    color: var(--color-text-primary);
  }

  .mcp-toggle.running {
    border-color: rgba(125, 155, 140, 0.3);
    color: var(--color-accent-cool);
  }

  .sidebar-footer {
    padding: 1rem;
    border-top: 1px solid var(--color-border);
    margin-top: auto;
  }

  .version-info {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    color: var(--color-text-muted);
    text-align: center;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .gradient-text {
    background: linear-gradient(135deg, var(--color-accent-primary), var(--color-accent-tertiary));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }
</style>
