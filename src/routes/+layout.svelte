<script lang="ts">
	import '../app.css';
	import Toast from '$lib/components/Toast.svelte';
	import { settings, applyTheme } from '$lib/stores';
	import { onMount } from 'svelte';

	// Apply theme immediately on mount
	onMount(() => {
		applyTheme($settings.theme);
	});

	// Reactive theme application when settings change
	$: if ($settings.theme) {
		applyTheme($settings.theme);
	}
</script>

{#if $settings.theme.emotional_ui}
	<div class="ambient-bg">
		<div class="ambient-orb ambient-orb-1"></div>
		<div class="ambient-orb ambient-orb-2"></div>
	</div>
{/if}

<Toast />

<slot />
