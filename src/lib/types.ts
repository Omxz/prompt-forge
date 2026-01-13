// Type definitions for Prompt Forge - Agent/Skill/Instruction Management

export interface Personality {
  tone: string;
  verbosity: string;
  creativity: number;
  formality: number;
  traits: string[];
}

export interface Agent {
  id: string;
  name: string;
  description: string;
  avatar_emoji: string;
  personality: Personality;
  system_prompt: string;
  skills: string[];       // Skill IDs
  instructions: string[]; // Instruction IDs
  tags: string[];
  created_at: string;
  updated_at: string;
}

export type SkillType = 'prompt' | 'tool' | 'workflow';

export interface ToolParameter {
  name: string;
  description: string;
  param_type: string;
  required: boolean;
  default?: any;
}

export interface WorkflowStep {
  id: string;
  name: string;
  action: string;
  inputs: Record<string, any>;
  outputs: string[];
}

export type SkillDefinition =
  | { type: 'prompt'; template: string }
  | { type: 'tool'; parameters: ToolParameter[]; handler: string }
  | { type: 'workflow'; steps: WorkflowStep[] };

export interface Skill {
  id: string;
  name: string;
  description: string;
  icon_emoji: string;
  skill_type: SkillType;
  definition: SkillDefinition;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

export type InstructionCategory =
  | 'general'
  | 'code_style'
  | 'communication'
  | 'workflow'
  | 'security'
  | 'testing'
  | 'documentation'
  | 'custom';

export interface Instruction {
  id: string;
  name: string;
  description: string;
  icon_emoji: string;
  category: InstructionCategory;
  content: string;
  priority: number;
  tags: string[];
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface Theme {
  mode: 'dark' | 'light' | 'auto';
  accent_color: string;
  emotional_ui: boolean;
}

export interface Settings {
  theme: Theme;
  mcp_server_port: number;
  mcp_server_enabled: boolean;
  data_directory?: string;
  auto_start_mcp: boolean;
}

export interface McpStatus {
  running: boolean;
  port: number;
  connected_clients: number;
  available_tools: string[];
}

// View state types
export type View = 'agents' | 'skills' | 'instructions' | 'settings';

export interface AppState {
  currentView: View;
  selectedAgentId: string | null;
  selectedSkillId: string | null;
  selectedInstructionId: string | null;
  sidebarCollapsed: boolean;
}

// Default values
export const defaultPersonality: Personality = {
  tone: 'friendly',
  verbosity: 'balanced',
  creativity: 0.5,
  formality: 0.5,
  traits: ['helpful', 'clear']
};

export const defaultAgent: Partial<Agent> = {
  name: 'New Agent',
  description: '',
  avatar_emoji: '🤖',
  personality: defaultPersonality,
  system_prompt: '',
  skills: [],
  instructions: [],
  tags: []
};

export const defaultSkill: Partial<Skill> = {
  name: 'New Skill',
  description: '',
  icon_emoji: '⚡',
  skill_type: 'prompt',
  definition: { type: 'prompt', template: '' },
  enabled: true
};

export const defaultInstruction: Partial<Instruction> = {
  name: 'New Instruction',
  description: '',
  icon_emoji: '📋',
  category: 'general',
  content: '',
  priority: 5,
  tags: [],
  enabled: true
};

export const defaultSettings: Settings = {
  theme: {
    mode: 'light',
    accent_color: '#c4704b',
    emotional_ui: true
  },
  mcp_server_port: 3333,
  mcp_server_enabled: false,
  auto_start_mcp: false
};
