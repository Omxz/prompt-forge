<script lang="ts">
  import { onMount } from 'svelte';

  let appWindow: any = null;
  let isMac = $state(false);

  onMount(async () => {
    // Detect platform
    isMac = navigator.platform.toUpperCase().includes('MAC');

    try {
      const windowModule = await import('@tauri-apps/api/window');
      appWindow = windowModule.getCurrentWindow();
    } catch (err) {
      console.error('Failed to initialize Tauri window:', err);
    }
  });

  function handleMouseDown(e: MouseEvent) {
    if (e.button !== 0 || !appWindow) return;
    e.preventDefault();
    appWindow.startDragging();
  }

  function handleDblClick() {
    appWindow?.toggleMaximize();
  }
</script>

<div
  class="titlebar"
  class:mac={isMac}
  onmousedown={handleMouseDown}
  ondblclick={handleDblClick}
  role="toolbar"
  data-tauri-drag-region
>
  <div class="titlebar-content" data-tauri-drag-region>
    <div class="titlebar-brand">
      <span class="titlebar-icon">⚒</span>
      <span class="titlebar-text">Prompt Forge</span>
    </div>
  </div>
</div>

<style>
  .titlebar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 38px;
    z-index: 9998;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(
      180deg,
      rgba(12, 11, 10, 0.95) 0%,
      rgba(12, 11, 10, 0.85) 100%
    );
    border-bottom: 1px solid var(--color-border-subtle);
    user-select: none;
    -webkit-user-select: none;
    -webkit-app-region: drag;
    app-region: drag;
  }

  .titlebar::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(212, 165, 116, 0.15) 50%,
      transparent 100%
    );
    pointer-events: none;
  }

  .titlebar-content {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    -webkit-app-region: drag;
    app-region: drag;
  }

  /* On macOS, add left padding to avoid native traffic lights */
  .titlebar.mac .titlebar-content {
    padding-left: 70px;
  }

  .titlebar-brand {
    display: flex;
    align-items: center;
    gap: 10px;
    opacity: 0.6;
    transition: opacity var(--transition-normal);
    pointer-events: none;
  }

  .titlebar:hover .titlebar-brand {
    opacity: 0.85;
  }

  .titlebar-icon {
    font-size: 13px;
    filter: drop-shadow(0 0 8px var(--color-accent-glow));
  }

  .titlebar-text {
    font-family: var(--font-mono);
    font-size: 10px;
    font-weight: 500;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--color-text-tertiary);
  }
</style>
