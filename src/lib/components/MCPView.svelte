<script lang="ts">
	import { mcpStatus, startMcpServer, stopMcpServer, settings } from '$lib/stores';

	async function handleMcpToggle() {
		if ($mcpStatus.running) {
			await stopMcpServer();
		} else {
			await startMcpServer();
		}
	}

	async function handleAutoStartToggle() {
		await settings.update({
			...$settings,
			auto_start_mcp: !$settings.auto_start_mcp
		});
	}

	function copyConnectionCommand() {
		const command = `"mcp-server-prompt-forge": {
  "command": "npx",
  "args": ["-y", "@promptforge/mcp-server"],
  "env": {
    "PROMPT_FORGE_PORT": "3333"
  }
}`;
		navigator.clipboard.writeText(command);
		alert('Connection command copied to clipboard!');
	}
</script>

<div class="mcp-view">
	<header class="view-header">
		<div class="header-content">
			<h1 class="view-title">MCP Server</h1>
			<p class="view-description">
				Connect Prompt Forge to Claude Code via MCP (Model Context Protocol)
			</p>
		</div>
	</header>

	<div class="mcp-content">
		<!-- Server Status Card -->
		<section class="mcp-card">
			<div class="card-header">
				<h2 class="section-title">Server Status</h2>
				<div class="status-indicator" class:running={$mcpStatus.running}>
					<span class="status-dot"></span>
					<span class="status-text">
						{$mcpStatus.running ? 'Running' : 'Stopped'}
					</span>
				</div>
			</div>

			{#if $mcpStatus.running}
				<div class="server-details">
					<div class="detail-row">
						<span class="detail-label">Port:</span>
						<span class="detail-value">{$mcpStatus.port}</span>
					</div>
					<div class="detail-row">
						<span class="detail-label">Available Tools:</span>
						<span class="detail-value">{$mcpStatus.available_tools.length}</span>
					</div>
					<div class="detail-row">
						<span class="detail-label">Connected Clients:</span>
						<span class="detail-value">{$mcpStatus.connected_clients}</span>
					</div>
				</div>

				<div class="tools-list">
					<h3 class="tools-title">Available MCP Tools</h3>
					<div class="tools-grid">
						{#each $mcpStatus.available_tools as tool}
							<span class="tool-badge">{tool}</span>
						{/each}
					</div>
				</div>
			{/if}

			<button class="btn btn-primary btn-large" class:btn-danger={$mcpStatus.running} onclick={handleMcpToggle}>
				{$mcpStatus.running ? '⏹️ Stop Server' : '▶️ Start Server'}
			</button>
		</section>

		<!-- Configuration Card -->
		<section class="mcp-card">
			<h2 class="section-title">Configuration</h2>
			<p class="section-description">
				Configure how the MCP server behaves
			</p>

			<div class="form-group">
				<label class="checkbox-label">
					<input type="checkbox" checked={$settings.auto_start_mcp} onchange={handleAutoStartToggle} />
					<span>Auto-start server when app launches</span>
				</label>
			</div>

			<div class="form-group">
				<label for="mcp-port">Server Port</label>
				<input
					id="mcp-port"
					type="number"
					value={$settings.mcp_server_port}
					disabled={$mcpStatus.running}
					placeholder="3333"
				/>
				<p class="field-hint">
					{$mcpStatus.running ? 'Stop the server to change the port' : 'Port for the MCP server (default: 3333)'}
				</p>
			</div>
		</section>

		<!-- Setup Instructions Card -->
		<section class="mcp-card">
			<h2 class="section-title">Claude Code Setup</h2>
			<p class="section-description">
				Follow these steps to connect Prompt Forge to Claude Code
			</p>

			<div class="setup-steps">
				<div class="step">
					<span class="step-number">1</span>
					<div class="step-content">
						<h3 class="step-title">Start the MCP Server</h3>
						<p class="step-text">Click the "Start Server" button above to launch the MCP server.</p>
					</div>
				</div>

				<div class="step">
					<span class="step-number">2</span>
					<div class="step-content">
						<h3 class="step-title">Add to Claude Code Config</h3>
						<p class="step-text">
							Open your Claude Code settings and add the Prompt Forge MCP server configuration.
						</p>
						<button class="btn btn-secondary btn-sm" onclick={copyConnectionCommand}>
							📋 Copy Configuration
						</button>
					</div>
				</div>

				<div class="step">
					<span class="step-number">3</span>
					<div class="step-content">
						<h3 class="step-title">Restart Claude Code</h3>
						<p class="step-text">
							Restart Claude Code to load the new MCP server connection. Your agents, skills, and
							instructions will now be available!
						</p>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	.mcp-view {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
	}

	.view-header {
		padding: var(--space-2xl) var(--space-2xl) var(--space-xl);
		border-bottom: 1px solid var(--color-border);
		background: var(--color-bg-primary);
	}

	.header-content {
		max-width: 800px;
	}

	.view-title {
		font-family: var(--font-display);
		font-size: 2rem;
		font-weight: 400;
		margin: 0 0 var(--space-sm);
		background: linear-gradient(135deg, var(--color-accent-primary), var(--color-accent-tertiary));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.view-description {
		font-size: 1rem;
		color: var(--color-text-secondary);
		margin: 0;
		line-height: 1.5;
	}

	.mcp-content {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-2xl);
		display: flex;
		flex-direction: column;
		gap: var(--space-xl);
		max-width: 800px;
	}

	.mcp-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-lg);
		padding: var(--space-xl);
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xl);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.section-title {
		font-family: var(--font-display);
		font-size: 1.25rem;
		font-weight: 500;
		margin: 0;
		color: var(--color-text-primary);
	}

	.section-description {
		font-size: 0.9rem;
		color: var(--color-text-secondary);
		margin: 0;
		line-height: 1.6;
	}

	.status-indicator {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
		padding: var(--space-sm) var(--space-md);
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-full);
	}

	.status-indicator.running {
		background: rgba(125, 155, 140, 0.1);
		border-color: rgba(125, 155, 140, 0.3);
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--color-text-muted);
		transition: all var(--transition-normal);
	}

	.status-indicator.running .status-dot {
		background: var(--color-accent-cool);
		box-shadow: 0 0 12px var(--color-accent-cool);
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.6;
			transform: scale(0.9);
		}
	}

	.status-text {
		font-family: var(--font-mono);
		font-size: 0.8rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.server-details {
		display: flex;
		flex-direction: column;
		gap: var(--space-md);
		padding: var(--space-lg);
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.detail-label {
		font-family: var(--font-mono);
		font-size: 0.8rem;
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.detail-value {
		font-family: var(--font-mono);
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--color-accent-primary);
	}

	.tools-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-md);
	}

	.tools-title {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--color-text-muted);
		margin: 0;
	}

	.tools-grid {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-sm);
	}

	.tool-badge {
		padding: var(--space-xs) var(--space-sm);
		background: var(--color-accent-glow);
		border: 1px solid rgba(212, 165, 116, 0.2);
		border-radius: var(--radius-md);
		font-family: var(--font-mono);
		font-size: 0.75rem;
		color: var(--color-accent-primary);
		font-weight: 500;
	}

	.btn-large {
		padding: var(--space-md) var(--space-xl);
		font-size: 1rem;
	}

	.btn-danger {
		background: rgba(200, 70, 70, 0.1);
		border-color: rgba(200, 70, 70, 0.3);
		color: #e74c3c;
	}

	.btn-danger:hover {
		background: rgba(200, 70, 70, 0.2);
		border-color: rgba(200, 70, 70, 0.5);
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-sm);
	}

	.checkbox-label {
		display: flex !important;
		align-items: center;
		gap: var(--space-md);
		cursor: pointer;
		font-size: 0.9rem;
		font-weight: 400;
		text-transform: none;
		letter-spacing: normal;
		color: var(--color-text-primary);
		margin-bottom: 0;
	}

	.checkbox-label input[type='checkbox'] {
		cursor: pointer;
		width: auto;
		flex-shrink: 0;
	}

	.checkbox-label span {
		flex: 1;
	}

	.field-hint {
		font-size: 0.8rem;
		color: var(--color-text-muted);
		margin: 0;
	}

	.setup-steps {
		display: flex;
		flex-direction: column;
		gap: var(--space-lg);
	}

	.step {
		display: flex;
		gap: var(--space-lg);
		padding: var(--space-lg);
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		transition: all var(--transition-fast);
	}

	.step:hover {
		border-color: var(--color-border-hover);
		background: var(--color-bg-hover);
	}

	.step-number {
		flex-shrink: 0;
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-accent-glow);
		border: 2px solid rgba(212, 165, 116, 0.3);
		border-radius: 50%;
		font-family: var(--font-display);
		font-size: 1.1rem;
		font-weight: 600;
		color: var(--color-accent-primary);
	}

	.step-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: var(--space-sm);
	}

	.step-title {
		font-size: 1rem;
		font-weight: 600;
		margin: 0;
		color: var(--color-text-primary);
	}

	.step-text {
		font-size: 0.9rem;
		color: var(--color-text-secondary);
		margin: 0;
		line-height: 1.6;
	}

	.btn-sm {
		padding: var(--space-xs) var(--space-md);
		font-size: 0.85rem;
		align-self: flex-start;
	}
</style>
