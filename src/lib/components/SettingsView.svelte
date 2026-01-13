<script lang="ts">
	import {
		settings,
		loadingState,
		exportAllData,
		importAllData,
		applyTheme,
		checkForUpdates,
		installUpdate,
		updateInfo,
		checkingForUpdate,
		installingUpdate,
		type ExportData
	} from '$lib/stores';
	import { toasts } from '$lib/stores/toasts';
	import type { Theme } from '$lib/types';

	let fileInput: HTMLInputElement;

	async function handleCheckForUpdates() {
		const info = await checkForUpdates();
		if (info && !info.available) {
			toasts.success('You are already on the latest version!');
		}
	}

	async function handleInstallUpdate() {
		if ($updateInfo?.available) {
			const confirmed = window.confirm(
				`A new version (${$updateInfo.version}) is available. Do you want to install it now? The app will restart automatically.`
			);
			if (confirmed) {
				await installUpdate();
			}
		}
	}

	async function handleSaveSettings() {
		await settings.save($settings);
	}

	async function handleExportAll() {
		const data = await exportAllData();
		if (data) {
			const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `prompt-forge-export-${new Date().toISOString().split('T')[0]}.json`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
		}
	}

	async function handleImportData() {
		fileInput.click();
	}

	async function processImportFile(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (!file) return;

		try {
			const text = await file.text();
			const data = JSON.parse(text) as ExportData;

			// Validate the data structure
			if (!data.agents || !data.skills || !data.instructions) {
				toasts.error('Invalid data file format');
				return;
			}

			const confirmed = window.confirm(
				'This will replace all your current data. Are you sure you want to continue?'
			);
			if (!confirmed) return;

			await importAllData(data);
		} catch {
			toasts.error('Failed to read or parse the file');
		}

		// Reset file input
		target.value = '';
	}

	function copyToClipboard(text: string, message: string) {
		navigator.clipboard.writeText(text);
		toasts.success(message);
	}

	function handleThemeModeChange(mode: Theme['mode']) {
		$settings.theme.mode = mode;
		applyTheme($settings.theme);
	}

	const themeModes: { value: Theme['mode']; label: string; icon: string; description: string }[] = [
		{ value: 'light', label: 'Light', icon: '☀️', description: 'Clean & airy interface' },
		{ value: 'dark', label: 'Dark', icon: '🌙', description: 'Warm charcoal tones' },
		{ value: 'auto', label: 'Auto', icon: '💻', description: 'Match system preference' }
	];

	const accentColors = [
		{ value: '#c4704b', label: 'Terracotta', description: 'Warm & earthy' },
		{ value: '#4a6572', label: 'Slate', description: 'Cool & professional' },
		{ value: '#5d8a6a', label: 'Sage', description: 'Natural & calm' },
		{ value: '#6a5a8a', label: 'Mauve', description: 'Refined & elegant' },
		{ value: '#8a6a5a', label: 'Umber', description: 'Rich & grounded' },
		{ value: '#5a7a8a', label: 'Steel', description: 'Modern & crisp' }
	];
</script>

<div class="settings-view">
	<header class="view-header">
		<div class="header-content">
			<h1 class="view-title">Settings</h1>
			<p class="view-description">Configure your Prompt Forge experience</p>
		</div>
	</header>

	<div class="settings-sections">
		<!-- Appearance -->
		<section class="settings-section card">
			<h2 class="section-title">Appearance</h2>
			<p class="section-description">
				Customize the look and feel of Prompt Forge to match your preferences.
			</p>

			<div class="form-group">
				<label>Theme Mode</label>
				<div class="theme-mode-options">
					{#each themeModes as mode}
						<button
							class="theme-mode-option"
							class:selected={$settings.theme.mode === mode.value}
							onclick={() => handleThemeModeChange(mode.value)}
						>
							<span class="theme-icon">{mode.icon}</span>
							<span class="theme-label">{mode.label}</span>
							<span class="theme-description">{mode.description}</span>
						</button>
					{/each}
				</div>
			</div>

			<div class="form-group">
				<label>Accent Color</label>
				<div class="accent-color-grid">
					{#each accentColors as color}
						<button
							class="accent-option"
							class:selected={$settings.theme.accent_color === color.value}
							style="--accent-color: {color.value}"
							onclick={() => ($settings.theme.accent_color = color.value)}
						>
							<span class="accent-swatch"></span>
							<span class="accent-info">
								<span class="accent-name">{color.label}</span>
								<span class="accent-desc">{color.description}</span>
							</span>
							{#if $settings.theme.accent_color === color.value}
								<span class="accent-check">✓</span>
							{/if}
						</button>
					{/each}
				</div>
			</div>

			<div class="form-group">
				<label>Interface Effects</label>
				<div class="effects-options">
					<label class="effect-toggle">
						<input type="checkbox" bind:checked={$settings.theme.emotional_ui} />
						<span class="effect-toggle-track">
							<span class="effect-toggle-thumb"></span>
						</span>
						<span class="effect-toggle-content">
							<span class="effect-toggle-label">Ambient Animations</span>
							<span class="effect-toggle-hint">Floating orbs and subtle motion effects</span>
						</span>
					</label>
				</div>
			</div>
		</section>

		<!-- Data Management -->
		<section class="settings-section card">
			<h2 class="section-title">Data Management</h2>

			<div class="data-actions">
				<button
					class="btn btn-secondary"
					onclick={handleExportAll}
					disabled={$loadingState.exporting}
				>
					{#if $loadingState.exporting}
						Exporting...
					{:else}
						Export All Data
					{/if}
				</button>
				<button
					class="btn btn-secondary"
					onclick={handleImportData}
					disabled={$loadingState.importing}
				>
					{#if $loadingState.importing}
						Importing...
					{:else}
						Import Data
					{/if}
				</button>
			</div>
			<p class="form-hint">
				Export your agents, skills, and instructions to share with colleagues or backup. Import to
				restore data from a backup.
			</p>

			<input
				type="file"
				bind:this={fileInput}
				accept=".json"
				onchange={processImportFile}
				style="display: none;"
			/>
		</section>

		<!-- Updates -->
		<section class="settings-section card">
			<h2 class="section-title">Updates</h2>
			<p class="section-description">
				Check for new versions and install updates automatically.
			</p>

			<div class="update-status">
				{#if $updateInfo}
					{#if $updateInfo.available}
						<div class="update-available">
							<div class="update-icon">🎉</div>
							<div class="update-details">
								<h4>New Version Available!</h4>
								<p class="update-version">
									Version {$updateInfo.version} (current: {$updateInfo.currentVersion})
								</p>
								{#if $updateInfo.body}
									<p class="update-notes">{$updateInfo.body}</p>
								{/if}
							</div>
						</div>
						<button
							class="btn btn-primary"
							onclick={handleInstallUpdate}
							disabled={$installingUpdate}
						>
							{#if $installingUpdate}
								Installing...
							{:else}
								Install Update
							{/if}
						</button>
					{:else}
						<div class="update-current">
							<div class="update-icon">✓</div>
							<div class="update-details">
								<h4>Up to Date</h4>
								<p class="update-version">You're running version {$updateInfo.currentVersion}</p>
							</div>
						</div>
					{/if}
				{/if}
			</div>

			<button
				class="btn btn-secondary"
				onclick={handleCheckForUpdates}
				disabled={$checkingForUpdate}
			>
				{#if $checkingForUpdate}
					Checking...
				{:else}
					Check for Updates
				{/if}
			</button>
		</section>

		<!-- About -->
		<section class="settings-section card">
			<h2 class="section-title">About</h2>
			<div class="about-info">
				<div class="app-logo">⚒️</div>
				<h3 class="gradient-text">Prompt Forge</h3>
				<p class="version">Version 0.1.0</p>
				<p class="description">
					A local workshop for crafting AI agents, skills, and instructions. Connect via MCP to use
					your custom prompts with Claude Code.
				</p>
				<div class="about-features">
					<span class="feature-badge">Agents</span>
					<span class="feature-badge">Skills</span>
					<span class="feature-badge">Instructions</span>
					<span class="feature-badge">MCP Server</span>
				</div>
			</div>
		</section>
	</div>

	<div class="settings-footer">
		<button
			class="btn btn-primary"
			onclick={handleSaveSettings}
			disabled={$loadingState.settings}
		>
			{#if $loadingState.settings}
				Saving...
			{:else}
				Save Settings
			{/if}
		</button>
	</div>
</div>

<style>
	.settings-view {
		height: 100%;
		overflow-y: auto;
		animation: fade-in 0.5s ease-out;
	}

	.view-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: var(--space-2xl);
		padding-bottom: var(--space-xl);
		border-bottom: 1px solid var(--color-border-subtle);
	}

	.view-title {
		font-family: var(--font-display);
		font-size: 2.25rem;
		font-weight: 400;
		margin: 0;
		letter-spacing: -0.03em;
	}

	.view-description {
		color: var(--color-text-tertiary);
		font-size: 0.95rem;
		margin: var(--space-xs) 0 0;
	}

	.settings-sections {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-xl);
		padding: var(--space-2xl);
		max-width: 100%;
	}

	@media (max-width: 1200px) {
		.settings-sections {
			grid-template-columns: 1fr;
			max-width: 720px;
		}
	}

	.settings-section {
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xl);
		padding: var(--space-xl);
		display: flex;
		flex-direction: column;
		gap: var(--space-lg);
		position: relative;
		overflow: hidden;
	}

	.settings-section::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 1px;
		background: linear-gradient(90deg, transparent, rgba(255,255,255,0.04), transparent);
	}

	.section-title {
		font-family: var(--font-display);
		font-size: 1.25rem;
		font-weight: 400;
		margin: 0;
		letter-spacing: -0.01em;
	}

	.section-description {
		color: var(--color-text-tertiary);
		font-size: 0.9rem;
		margin: 0;
		line-height: 1.5;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-sm);
	}

	.form-group label {
		font-family: var(--font-mono);
		font-size: 0.65rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--color-text-muted);
	}

	.form-group input,
	.form-group select {
		width: 100%;
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--space-md);
		color: var(--color-text-primary);
		font-size: 0.9rem;
		transition: all var(--transition-fast);
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: var(--color-border-hover);
		box-shadow: 0 0 0 3px rgba(212, 165, 116, 0.1);
	}

	.form-hint {
		font-size: 0.8rem;
		color: var(--color-text-muted);
		margin: 0;
		line-height: 1.4;
	}

	/* Theme Mode Options */
	.theme-mode-options {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--space-md);
	}

	.theme-mode-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-xs);
		padding: var(--space-lg) var(--space-md);
		background: var(--color-bg-tertiary);
		border: 2px solid var(--color-border);
		border-radius: var(--radius-lg);
		cursor: pointer;
		transition: all var(--transition-fast);
		text-align: center;
	}

	.theme-mode-option:hover {
		border-color: var(--color-border-hover);
		background: var(--color-bg-hover);
	}

	.theme-mode-option.selected {
		border-color: var(--color-accent-primary);
		background: var(--color-accent-glow);
	}

	.theme-icon {
		font-size: 1.75rem;
		line-height: 1;
	}

	.theme-label {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: var(--color-text-primary);
	}

	.theme-description {
		font-size: 0.7rem;
		color: var(--color-text-muted);
	}

	/* Accent Color Grid */
	.accent-color-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-sm);
	}

	.accent-option {
		display: flex;
		align-items: center;
		gap: var(--space-md);
		padding: var(--space-md);
		background: var(--color-bg-tertiary);
		border: 2px solid var(--color-border);
		border-radius: var(--radius-md);
		cursor: pointer;
		transition: all var(--transition-fast);
		text-align: left;
	}

	.accent-option:hover {
		border-color: var(--color-border-hover);
		background: var(--color-bg-hover);
	}

	.accent-option.selected {
		border-color: var(--accent-color);
		box-shadow: inset 0 0 0 1px var(--accent-color);
	}

	.accent-swatch {
		width: 32px;
		height: 32px;
		border-radius: var(--radius-md);
		background: var(--accent-color);
		flex-shrink: 0;
		box-shadow: var(--shadow-sm);
	}

	.accent-info {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		min-width: 0;
	}

	.accent-name {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-primary);
	}

	.accent-desc {
		font-size: 0.7rem;
		color: var(--color-text-muted);
	}

	.accent-check {
		font-size: 0.875rem;
		color: var(--accent-color);
		font-weight: 700;
	}

	/* Effect Toggle */
	.effects-options {
		display: flex;
		flex-direction: column;
		gap: var(--space-sm);
	}

	.effect-toggle {
		display: flex;
		align-items: center;
		gap: var(--space-md);
		padding: var(--space-md);
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		cursor: pointer;
		transition: all var(--transition-fast);
	}

	.effect-toggle:hover {
		border-color: var(--color-border-hover);
	}

	.effect-toggle input[type='checkbox'] {
		display: none;
	}

	.effect-toggle-track {
		width: 44px;
		height: 24px;
		background: var(--color-bg-hover);
		border-radius: var(--radius-full);
		position: relative;
		transition: background var(--transition-fast);
		flex-shrink: 0;
	}

	.effect-toggle input:checked + .effect-toggle-track {
		background: var(--color-accent-primary);
	}

	.effect-toggle-thumb {
		position: absolute;
		top: 2px;
		left: 2px;
		width: 20px;
		height: 20px;
		background: var(--color-bg-elevated);
		border-radius: 50%;
		transition: transform var(--transition-fast);
		box-shadow: var(--shadow-sm);
	}

	.effect-toggle input:checked + .effect-toggle-track .effect-toggle-thumb {
		transform: translateX(20px);
	}

	.effect-toggle-content {
		display: flex;
		flex-direction: column;
	}

	.effect-toggle-label {
		font-size: 0.9rem;
		font-weight: 500;
		color: var(--color-text-primary);
	}

	.effect-toggle-hint {
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.data-actions {
		display: flex;
		gap: var(--space-md);
	}

	.about-info {
		text-align: center;
		padding: var(--space-xl);
	}

	.app-logo {
		font-size: 4rem;
		margin-bottom: var(--space-md);
		filter: drop-shadow(0 0 30px var(--color-accent-glow));
		animation: emotional-pulse 3s ease-in-out infinite;
	}

	.about-info h3 {
		font-family: var(--font-display);
		font-size: 1.75rem;
		font-weight: 400;
		margin: 0;
		letter-spacing: -0.02em;
	}

	.version {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--color-text-muted);
		margin: var(--space-sm) 0 var(--space-lg);
	}

	.description {
		color: var(--color-text-tertiary);
		font-size: 0.9rem;
		line-height: 1.6;
		margin: 0;
		max-width: 360px;
		margin: 0 auto var(--space-lg);
	}

	.about-features {
		display: flex;
		justify-content: center;
		gap: var(--space-sm);
		flex-wrap: wrap;
	}

	.feature-badge {
		font-family: var(--font-mono);
		font-size: 0.65rem;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		padding: 0.4rem 0.75rem;
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-full);
		color: var(--color-text-secondary);
	}

	.settings-footer {
		margin-top: var(--space-xl);
		padding: var(--space-lg) var(--space-2xl);
		border-top: 1px solid var(--color-border);
	}

	.gradient-text {
		background: linear-gradient(135deg, var(--color-accent-primary), var(--color-accent-secondary));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	/* Buttons */
	.btn {
		padding: var(--space-sm) var(--space-lg);
		border-radius: var(--radius-md);
		font-size: 0.85rem;
		font-weight: 500;
		cursor: pointer;
		transition: all var(--transition-fast);
		border: 1px solid transparent;
		display: inline-flex;
		align-items: center;
		gap: var(--space-sm);
	}

	.btn-primary {
		background: var(--color-accent-primary);
		color: var(--color-bg-primary);
		border-color: var(--color-accent-primary);
	}

	.btn-primary:hover:not(:disabled) {
		background: var(--color-accent-secondary);
		border-color: var(--color-accent-secondary);
		transform: translateY(-1px);
	}

	.btn-secondary {
		background: transparent;
		color: var(--color-text-secondary);
		border-color: var(--color-border);
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--color-bg-tertiary);
		border-color: var(--color-border-hover);
		color: var(--color-text-primary);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Update Section */
	.update-status {
		display: flex;
		flex-direction: column;
		gap: var(--space-md);
	}

	.update-available,
	.update-current {
		display: flex;
		align-items: flex-start;
		gap: var(--space-md);
		padding: var(--space-lg);
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
	}

	.update-available {
		border-color: var(--color-accent-primary);
		background: var(--color-accent-glow);
	}

	.update-icon {
		font-size: 2rem;
		line-height: 1;
		flex-shrink: 0;
	}

	.update-details {
		flex-grow: 1;
	}

	.update-details h4 {
		font-family: var(--font-display);
		font-size: 1rem;
		font-weight: 400;
		margin: 0 0 var(--space-xs);
		color: var(--color-text-primary);
	}

	.update-version {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		color: var(--color-text-muted);
		margin: 0;
	}

	.update-notes {
		font-size: 0.85rem;
		color: var(--color-text-secondary);
		margin: var(--space-sm) 0 0;
		line-height: 1.5;
	}
</style>
