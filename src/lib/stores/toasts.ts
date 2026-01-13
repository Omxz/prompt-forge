import { writable, derived } from 'svelte/store';

export interface Toast {
	id: string;
	type: 'success' | 'error' | 'warning' | 'info';
	message: string;
	duration: number;
}

function createToastStore() {
	const { subscribe, update } = writable<Toast[]>([]);

	function addToast(type: Toast['type'], message: string, duration = 5000): string {
		const id = crypto.randomUUID();
		update((toasts) => [...toasts, { id, type, message, duration }]);

		// Auto-dismiss after duration
		if (duration > 0) {
			setTimeout(() => {
				dismiss(id);
			}, duration);
		}

		return id;
	}

	function dismiss(id: string) {
		update((toasts) => toasts.filter((t) => t.id !== id));
	}

	function dismissAll() {
		update(() => []);
	}

	return {
		subscribe,
		success: (message: string, duration?: number) => addToast('success', message, duration),
		error: (message: string, duration?: number) => addToast('error', message, duration ?? 8000),
		warning: (message: string, duration?: number) => addToast('warning', message, duration),
		info: (message: string, duration?: number) => addToast('info', message, duration),
		dismiss,
		dismissAll
	};
}

export const toasts = createToastStore();
