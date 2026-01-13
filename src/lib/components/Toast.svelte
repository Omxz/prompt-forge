<script lang="ts">
	import { toasts, type Toast } from '$lib/stores/toasts';
	import { fly, fade } from 'svelte/transition';

	function getIcon(type: Toast['type']): string {
		switch (type) {
			case 'success':
				return '✓';
			case 'error':
				return '✕';
			case 'warning':
				return '⚠';
			case 'info':
				return 'ℹ';
		}
	}
</script>

<div class="toast-container">
	{#each $toasts as toast (toast.id)}
		<div
			class="toast toast-{toast.type}"
			in:fly={{ y: -20, duration: 200 }}
			out:fade={{ duration: 150 }}
			role="alert"
		>
			<span class="toast-icon">{getIcon(toast.type)}</span>
			<span class="toast-message">{toast.message}</span>
			<button class="toast-dismiss" on:click={() => toasts.dismiss(toast.id)} aria-label="Dismiss">
				×
			</button>
		</div>
	{/each}
</div>

<style>
	.toast-container {
		position: fixed;
		top: var(--space-lg);
		right: var(--space-lg);
		z-index: 9999;
		display: flex;
		flex-direction: column;
		gap: var(--space-sm);
		max-width: 400px;
		pointer-events: none;
	}

	.toast {
		display: flex;
		align-items: center;
		gap: var(--space-md);
		padding: var(--space-md) var(--space-lg);
		border-radius: var(--radius-lg);
		background: var(--color-bg-elevated);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-lg);
		pointer-events: auto;
		backdrop-filter: blur(12px);
		animation: slideIn 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	.toast-success {
		border-left: 3px solid var(--color-success);
	}

	.toast-success .toast-icon {
		color: var(--color-success);
		text-shadow: 0 0 8px var(--color-success);
	}

	.toast-error {
		border-left: 3px solid var(--color-error);
	}

	.toast-error .toast-icon {
		color: var(--color-error);
		text-shadow: 0 0 8px var(--color-error);
	}

	.toast-warning {
		border-left: 3px solid var(--color-warning);
	}

	.toast-warning .toast-icon {
		color: var(--color-warning);
		text-shadow: 0 0 8px var(--color-warning);
	}

	.toast-info {
		border-left: 3px solid var(--color-accent-primary);
	}

	.toast-info .toast-icon {
		color: var(--color-accent-primary);
		text-shadow: 0 0 8px var(--color-accent-glow);
	}

	.toast-icon {
		font-size: 1.125rem;
		font-weight: bold;
		flex-shrink: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.toast-message {
		flex: 1;
		color: var(--color-text-primary);
		font-size: 0.875rem;
		line-height: 1.4;
	}

	.toast-dismiss {
		background: none;
		border: none;
		color: var(--color-text-muted);
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0;
		line-height: 1;
		opacity: 0.5;
		transition: all var(--transition-fast);
		flex-shrink: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-sm);
	}

	.toast-dismiss:hover {
		opacity: 1;
		color: var(--color-text-primary);
		background: var(--color-bg-tertiary);
	}
</style>
