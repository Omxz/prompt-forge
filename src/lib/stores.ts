// Svelte stores for Prompt Forge - Agent/Skill/Instruction Management
import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Agent, Skill, Instruction, Settings, McpStatus, AppState, View, Theme } from './types';
import { defaultSettings } from './types';
import { toasts } from './stores/toasts';

// ============================================================================
// Theme Management
// ============================================================================

function getSystemTheme(): 'light' | 'dark' {
	if (typeof window === 'undefined') return 'light';
	return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

export function applyTheme(theme: Theme): void {
	if (typeof document === 'undefined') return;

	let resolvedMode: 'light' | 'dark';
	if (theme.mode === 'auto') {
		resolvedMode = getSystemTheme();
	} else {
		resolvedMode = theme.mode;
	}

	// Apply theme mode
	if (resolvedMode === 'dark') {
		document.documentElement.setAttribute('data-theme', 'dark');
	} else {
		document.documentElement.removeAttribute('data-theme');
	}

	// Apply accent color as CSS variable
	document.documentElement.style.setProperty('--color-accent-primary', theme.accent_color);

	// Generate accent color variants
	const hexToHSL = (hex: string): { h: number; s: number; l: number } => {
		const r = parseInt(hex.slice(1, 3), 16) / 255;
		const g = parseInt(hex.slice(3, 5), 16) / 255;
		const b = parseInt(hex.slice(5, 7), 16) / 255;
		const max = Math.max(r, g, b);
		const min = Math.min(r, g, b);
		let h = 0;
		let s = 0;
		const l = (max + min) / 2;
		if (max !== min) {
			const d = max - min;
			s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
			switch (max) {
				case r:
					h = ((g - b) / d + (g < b ? 6 : 0)) / 6;
					break;
				case g:
					h = ((b - r) / d + 2) / 6;
					break;
				case b:
					h = ((r - g) / d + 4) / 6;
					break;
			}
		}
		return { h: h * 360, s: s * 100, l: l * 100 };
	};

	const hsl = hexToHSL(theme.accent_color);
	document.documentElement.style.setProperty(
		'--color-accent-secondary',
		`hsl(${hsl.h}, ${hsl.s * 0.9}%, ${Math.min(hsl.l + 12, 80)}%)`
	);
	document.documentElement.style.setProperty(
		'--color-accent-tertiary',
		`hsl(${hsl.h}, ${hsl.s * 0.8}%, ${Math.min(hsl.l + 22, 88)}%)`
	);
	document.documentElement.style.setProperty(
		'--color-accent-muted',
		`hsl(${hsl.h}, ${hsl.s * 0.7}%, ${Math.max(hsl.l - 12, 25)}%)`
	);
	document.documentElement.style.setProperty(
		'--color-accent-glow',
		`hsla(${hsl.h}, ${hsl.s}%, ${hsl.l}%, ${resolvedMode === 'dark' ? 0.2 : 0.15})`
	);
	document.documentElement.style.setProperty(
		'--color-border-active',
		`hsla(${hsl.h}, ${hsl.s}%, ${hsl.l}%, 0.45)`
	);
	document.documentElement.style.setProperty('--color-text-accent', theme.accent_color);
}

// Listen for system theme changes
if (typeof window !== 'undefined') {
	window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
		// Re-apply theme if in auto mode
		const currentSettings = localStorage.getItem('prompt-forge-settings');
		if (currentSettings) {
			try {
				const parsed = JSON.parse(currentSettings) as Settings;
				if (parsed.theme.mode === 'auto') {
					applyTheme(parsed.theme);
				}
			} catch {
				// Ignore parsing errors
			}
		}
	});
}

// ============================================================================
// Loading States Store
// ============================================================================

export const loadingState = writable<{
	agents: boolean;
	skills: boolean;
	instructions: boolean;
	settings: boolean;
	mcp: boolean;
	exporting: boolean;
	importing: boolean;
}>({
	agents: false,
	skills: false,
	instructions: false,
	settings: false,
	mcp: false,
	exporting: false,
	importing: false
});

// ============================================================================
// Core Data Stores
// ============================================================================

function createAgentsStore() {
	const store = writable<Agent[]>([]);
	const { subscribe, set } = store;

	return {
		subscribe,
		set,
		async load() {
			loadingState.update((s) => ({ ...s, agents: true }));
			try {
				const data = await invoke<Agent[]>('get_agents');
				set(data);
			} catch (error) {
				console.error('Failed to load agents:', error);
				toasts.error('Failed to load agents');
			} finally {
				loadingState.update((s) => ({ ...s, agents: false }));
			}
		},
		async create(agent: Partial<Agent>): Promise<Agent | null> {
			loadingState.update((s) => ({ ...s, agents: true }));
			try {
				const created = await invoke<Agent>('create_agent', { agent });
				store.update((list) => [...list, created]);
				toasts.success(`Agent "${created.name}" created`);
				return created;
			} catch (error) {
				console.error('Failed to create agent:', error);
				toasts.error('Failed to create agent');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, agents: false }));
			}
		},
		async update(agent: Agent): Promise<Agent | null> {
			loadingState.update((s) => ({ ...s, agents: true }));
			try {
				const updated = await invoke<Agent>('update_agent', { agent });
				store.update((list) => list.map((a) => (a.id === updated.id ? updated : a)));
				toasts.success(`Agent "${updated.name}" updated`);
				return updated;
			} catch (error) {
				console.error('Failed to update agent:', error);
				toasts.error('Failed to update agent');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, agents: false }));
			}
		},
		async delete(id: string): Promise<boolean> {
			loadingState.update((s) => ({ ...s, agents: true }));
			try {
				await invoke('delete_agent', { id });
				store.update((list) => list.filter((a) => a.id !== id));
				toasts.success('Agent deleted');
				return true;
			} catch (error) {
				console.error('Failed to delete agent:', error);
				toasts.error('Failed to delete agent');
				return false;
			} finally {
				loadingState.update((s) => ({ ...s, agents: false }));
			}
		},
		async importFromText(text: string): Promise<Agent | null> {
			loadingState.update((s) => ({ ...s, agents: true }));
			try {
				const agent = await invoke<Agent>('import_agent_from_text', { text });
				store.update((list) => [...list, agent]);
				toasts.success(`Agent "${agent.name}" imported`);
				return agent;
			} catch (error) {
				console.error('Failed to import agent:', error);
				toasts.error('Failed to import agent. Check the format.');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, agents: false }));
			}
		},
		async exportToMarkdown(id: string): Promise<string | null> {
			try {
				const markdown = await invoke<string>('export_agent_to_markdown', { id });
				toasts.success('Agent exported to clipboard');
				return markdown;
			} catch (error) {
				console.error('Failed to export agent:', error);
				toasts.error('Failed to export agent');
				return null;
			}
		}
	};
}

function createSkillsStore() {
	const store = writable<Skill[]>([]);
	const { subscribe, set } = store;

	return {
		subscribe,
		set,
		async load() {
			loadingState.update((s) => ({ ...s, skills: true }));
			try {
				const data = await invoke<Skill[]>('get_skills');
				set(data);
			} catch (error) {
				console.error('Failed to load skills:', error);
				toasts.error('Failed to load skills');
			} finally {
				loadingState.update((s) => ({ ...s, skills: false }));
			}
		},
		async create(skill: Partial<Skill>): Promise<Skill | null> {
			loadingState.update((s) => ({ ...s, skills: true }));
			try {
				const created = await invoke<Skill>('create_skill', { skill });
				store.update((list) => [...list, created]);
				toasts.success(`Skill "${created.name}" created`);
				return created;
			} catch (error) {
				console.error('Failed to create skill:', error);
				toasts.error('Failed to create skill');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, skills: false }));
			}
		},
		async update(skill: Skill): Promise<Skill | null> {
			loadingState.update((s) => ({ ...s, skills: true }));
			try {
				const updated = await invoke<Skill>('update_skill', { skill });
				store.update((list) => list.map((s) => (s.id === updated.id ? updated : s)));
				toasts.success(`Skill "${updated.name}" updated`);
				return updated;
			} catch (error) {
				console.error('Failed to update skill:', error);
				toasts.error('Failed to update skill');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, skills: false }));
			}
		},
		async delete(id: string): Promise<boolean> {
			loadingState.update((s) => ({ ...s, skills: true }));
			try {
				await invoke('delete_skill', { id });
				store.update((list) => list.filter((s) => s.id !== id));
				toasts.success('Skill deleted');
				return true;
			} catch (error) {
				console.error('Failed to delete skill:', error);
				toasts.error('Failed to delete skill');
				return false;
			} finally {
				loadingState.update((s) => ({ ...s, skills: false }));
			}
		}
	};
}

function createInstructionsStore() {
	const store = writable<Instruction[]>([]);
	const { subscribe, set } = store;

	return {
		subscribe,
		set,
		async load() {
			loadingState.update((s) => ({ ...s, instructions: true }));
			try {
				const data = await invoke<Instruction[]>('get_instructions');
				set(data);
			} catch (error) {
				console.error('Failed to load instructions:', error);
				toasts.error('Failed to load instructions');
			} finally {
				loadingState.update((s) => ({ ...s, instructions: false }));
			}
		},
		async create(instruction: Partial<Instruction>): Promise<Instruction | null> {
			loadingState.update((s) => ({ ...s, instructions: true }));
			try {
				const created = await invoke<Instruction>('create_instruction', { instruction });
				store.update((list) => [...list, created]);
				toasts.success(`Instruction "${created.name}" created`);
				return created;
			} catch (error) {
				console.error('Failed to create instruction:', error);
				toasts.error('Failed to create instruction');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, instructions: false }));
			}
		},
		async update(instruction: Instruction): Promise<Instruction | null> {
			loadingState.update((s) => ({ ...s, instructions: true }));
			try {
				const updated = await invoke<Instruction>('update_instruction', { instruction });
				store.update((list) => list.map((i) => (i.id === updated.id ? updated : i)));
				toasts.success(`Instruction "${updated.name}" updated`);
				return updated;
			} catch (error) {
				console.error('Failed to update instruction:', error);
				toasts.error('Failed to update instruction');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, instructions: false }));
			}
		},
		async delete(id: string): Promise<boolean> {
			loadingState.update((s) => ({ ...s, instructions: true }));
			try {
				await invoke('delete_instruction', { id });
				store.update((list) => list.filter((i) => i.id !== id));
				toasts.success('Instruction deleted');
				return true;
			} catch (error) {
				console.error('Failed to delete instruction:', error);
				toasts.error('Failed to delete instruction');
				return false;
			} finally {
				loadingState.update((s) => ({ ...s, instructions: false }));
			}
		},
		async importFromText(text: string): Promise<Instruction | null> {
			loadingState.update((s) => ({ ...s, instructions: true }));
			try {
				const instruction = await invoke<Instruction>('import_instruction_from_text', { text });
				store.update((list) => [...list, instruction]);
				toasts.success(`Instruction "${instruction.name}" imported`);
				return instruction;
			} catch (error) {
				console.error('Failed to import instruction:', error);
				toasts.error('Failed to import instruction. Check the format.');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, instructions: false }));
			}
		},
		async exportToMarkdown(id: string): Promise<string | null> {
			try {
				const markdown = await invoke<string>('export_instruction_to_markdown', { id });
				toasts.success('Instruction exported to clipboard');
				return markdown;
			} catch (error) {
				console.error('Failed to export instruction:', error);
				toasts.error('Failed to export instruction');
				return null;
			}
		}
	};
}

function createSettingsStore() {
	const store = writable<Settings>(defaultSettings);
	const { subscribe, set } = store;

	return {
		subscribe,
		set,
		async load() {
			loadingState.update((s) => ({ ...s, settings: true }));
			try {
				const data = await invoke<Settings>('get_settings');
				set(data);
				// Apply theme on load
				applyTheme(data.theme);
			} catch (error) {
				console.error('Failed to load settings:', error);
				toasts.error('Failed to load settings');
				// Apply default theme on error
				applyTheme(defaultSettings.theme);
			} finally {
				loadingState.update((s) => ({ ...s, settings: false }));
			}
		},
		async save(newSettings: Settings): Promise<Settings | null> {
			loadingState.update((s) => ({ ...s, settings: true }));
			try {
				const saved = await invoke<Settings>('save_settings', { settings: newSettings });
				set(saved);
				// Apply theme after save
				applyTheme(saved.theme);
				toasts.success('Settings saved');
				return saved;
			} catch (error) {
				console.error('Failed to save settings:', error);
				toasts.error('Failed to save settings');
				return null;
			} finally {
				loadingState.update((s) => ({ ...s, settings: false }));
			}
		}
	};
}

// Export stores
export const agents = createAgentsStore();
export const skills = createSkillsStore();
export const instructions = createInstructionsStore();
export const settings = createSettingsStore();

export const mcpStatus = writable<McpStatus>({
	running: false,
	port: 3333,
	connected_clients: 0,
	available_tools: []
});

// ============================================================================
// UI State Store
// ============================================================================

export const appState = writable<AppState>({
	currentView: 'agents',
	selectedAgentId: null,
	selectedSkillId: null,
	selectedInstructionId: null,
	sidebarCollapsed: false
});

// ============================================================================
// Derived Stores
// ============================================================================

export const selectedAgent = derived([agents, appState], ([$agents, $appState]) =>
	$agents.find((a) => a.id === $appState.selectedAgentId) || null
);

export const selectedSkill = derived([skills, appState], ([$skills, $appState]) =>
	$skills.find((s) => s.id === $appState.selectedSkillId) || null
);

export const selectedInstruction = derived(
	[instructions, appState],
	([$instructions, $appState]) =>
		$instructions.find((i) => i.id === $appState.selectedInstructionId) || null
);

export const enabledInstructions = derived(instructions, ($instructions) =>
	$instructions.filter((i) => i.enabled)
);

export const isLoading = derived(loadingState, ($loadingState) =>
	Object.values($loadingState).some((v) => v)
);

// ============================================================================
// Standalone Action Functions (for convenience)
// ============================================================================

export async function createInstruction(
	instruction: Partial<Instruction>
): Promise<Instruction | null> {
	return instructions.create(instruction);
}

export async function updateInstruction(instruction: Instruction): Promise<Instruction | null> {
	return instructions.update(instruction);
}

export async function deleteInstruction(id: string): Promise<boolean> {
	return instructions.delete(id);
}

export async function importInstructionFromText(text: string): Promise<Instruction | null> {
	return instructions.importFromText(text);
}

export async function exportInstructionToMarkdown(id: string): Promise<string | null> {
	return instructions.exportToMarkdown(id);
}

// ============================================================================
// MCP Server Actions
// ============================================================================

export async function loadMcpStatus(): Promise<void> {
	try {
		const status = await invoke<McpStatus>('get_mcp_status');
		mcpStatus.set(status);
	} catch (error) {
		console.error('Failed to load MCP status:', error);
	}
}

export async function startMcpServer(): Promise<McpStatus | null> {
	loadingState.update((s) => ({ ...s, mcp: true }));
	try {
		const status = await invoke<McpStatus>('start_mcp_server');
		mcpStatus.set(status);
		toasts.success('MCP server started');
		return status;
	} catch (error) {
		console.error('Failed to start MCP server:', error);
		toasts.error('Failed to start MCP server');
		return null;
	} finally {
		loadingState.update((s) => ({ ...s, mcp: false }));
	}
}

export async function stopMcpServer(): Promise<McpStatus | null> {
	loadingState.update((s) => ({ ...s, mcp: true }));
	try {
		const status = await invoke<McpStatus>('stop_mcp_server');
		mcpStatus.set(status);
		toasts.success('MCP server stopped');
		return status;
	} catch (error) {
		console.error('Failed to stop MCP server:', error);
		toasts.error('Failed to stop MCP server');
		return null;
	} finally {
		loadingState.update((s) => ({ ...s, mcp: false }));
	}
}

// ============================================================================
// Export/Import All Data
// ============================================================================

export interface ExportData {
	agents: Agent[];
	skills: Skill[];
	instructions: Instruction[];
	settings: Settings;
	exported_at: string;
	version: string;
}

export async function exportAllData(): Promise<ExportData | null> {
	loadingState.update((s) => ({ ...s, exporting: true }));
	try {
		const data = await invoke<ExportData>('export_all_data');
		toasts.success('Data exported successfully');
		return data;
	} catch (error) {
		console.error('Failed to export data:', error);
		toasts.error('Failed to export data');
		return null;
	} finally {
		loadingState.update((s) => ({ ...s, exporting: false }));
	}
}

export async function importAllData(data: ExportData): Promise<boolean> {
	loadingState.update((s) => ({ ...s, importing: true }));
	try {
		await invoke('import_all_data', { data });
		// Reload all data
		await initializeApp();
		toasts.success('Data imported successfully');
		return true;
	} catch (error) {
		console.error('Failed to import data:', error);
		toasts.error('Failed to import data');
		return false;
	} finally {
		loadingState.update((s) => ({ ...s, importing: false }));
	}
}

// ============================================================================
// App Initialization
// ============================================================================

export async function initializeApp(): Promise<void> {
	await Promise.all([
		agents.load(),
		skills.load(),
		instructions.load(),
		settings.load(),
		loadMcpStatus()
	]);
}

// ============================================================================
// UI State Actions
// ============================================================================

export function setView(view: View): void {
	appState.update((state) => ({ ...state, currentView: view }));
}

export function selectAgent(id: string | null): void {
	appState.update((state) => ({ ...state, selectedAgentId: id }));
}

export function selectSkill(id: string | null): void {
	appState.update((state) => ({ ...state, selectedSkillId: id }));
}

export function selectInstruction(id: string | null): void {
	appState.update((state) => ({ ...state, selectedInstructionId: id }));
}

export function toggleSidebar(): void {
	appState.update((state) => ({ ...state, sidebarCollapsed: !state.sidebarCollapsed }));
}

// ============================================================================
// Auto-Update Functions
// ============================================================================

export interface UpdateInfo {
	available: boolean;
	version?: string;
	currentVersion: string;
	body?: string;
	date?: string;
}

export const updateInfo = writable<UpdateInfo | null>(null);
export const checkingForUpdate = writable(false);
export const installingUpdate = writable(false);

export async function checkForUpdates(): Promise<UpdateInfo | null> {
	checkingForUpdate.set(true);
	try {
		const { check } = await import('@tauri-apps/plugin-updater');
		const { getVersion } = await import('@tauri-apps/api/app');

		const currentVersion = await getVersion();
		const update = await check();

		if (update) {
			const info: UpdateInfo = {
				available: update.available,
				version: update.version,
				currentVersion,
				body: update.body,
				date: update.date
			};
			updateInfo.set(info);
			return info;
		} else {
			const info: UpdateInfo = {
				available: false,
				currentVersion
			};
			updateInfo.set(info);
			return info;
		}
	} catch (error) {
		console.error('Failed to check for updates:', error);
		toasts.error('Failed to check for updates');
		return null;
	} finally {
		checkingForUpdate.set(false);
	}
}

export async function installUpdate(): Promise<boolean> {
	installingUpdate.set(true);
	try {
		const { check } = await import('@tauri-apps/plugin-updater');
		const { relaunch } = await import('@tauri-apps/plugin-process');

		const update = await check();

		if (update?.available) {
			toasts.info('Downloading update...');

			await update.downloadAndInstall((progress) => {
				if (progress.event === 'Started') {
					toasts.info(`Downloading update (${progress.data.contentLength} bytes)`);
				} else if (progress.event === 'Progress') {
					const percent = Math.round((progress.data.chunkLength / progress.data.contentLength) * 100);
					console.log(`Download progress: ${percent}%`);
				}
			});

			toasts.success('Update installed! Restarting...');

			// Relaunch the app
			await relaunch();
			return true;
		} else {
			toasts.info('No update available');
			return false;
		}
	} catch (error) {
		console.error('Failed to install update:', error);
		toasts.error('Failed to install update');
		return false;
	} finally {
		installingUpdate.set(false);
	}
}
