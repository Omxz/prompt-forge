-- Prompt Forge Database Schema
-- Version: 001_initial_schema

-- Agents table: stores AI agent configurations
CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    avatar_emoji TEXT DEFAULT '🤖',
    personality_json TEXT NOT NULL DEFAULT '{}',
    system_prompt TEXT DEFAULT '',
    skills_json TEXT DEFAULT '[]',
    instructions_json TEXT DEFAULT '[]',
    tags_json TEXT DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Skills table: stores reusable skills (prompt templates, tools, workflows)
CREATE TABLE IF NOT EXISTS skills (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    icon_emoji TEXT DEFAULT '⚡',
    skill_type TEXT NOT NULL CHECK (skill_type IN ('prompt', 'tool', 'workflow')),
    definition_json TEXT NOT NULL DEFAULT '{}',
    enabled INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Instructions table: stores coding guidelines and rules
CREATE TABLE IF NOT EXISTS instructions (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    icon_emoji TEXT DEFAULT '📋',
    category TEXT NOT NULL DEFAULT 'general',
    content TEXT NOT NULL DEFAULT '',
    priority INTEGER DEFAULT 5 CHECK (priority >= 1 AND priority <= 10),
    tags_json TEXT DEFAULT '[]',
    enabled INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Settings table: single row for app configuration
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    theme_mode TEXT DEFAULT 'dark',
    theme_accent_color TEXT DEFAULT '#8b5cf6',
    theme_emotional_ui INTEGER DEFAULT 1,
    mcp_server_port INTEGER DEFAULT 3333,
    auto_start_mcp INTEGER DEFAULT 0,
    data_directory TEXT
);

-- Insert default settings row if not exists
INSERT OR IGNORE INTO settings (id, theme_mode, theme_accent_color, theme_emotional_ui, mcp_server_port, auto_start_mcp)
VALUES (1, 'dark', '#8b5cf6', 1, 3333, 0);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_agents_name ON agents(name);
CREATE INDEX IF NOT EXISTS idx_skills_name ON skills(name);
CREATE INDEX IF NOT EXISTS idx_skills_enabled ON skills(enabled);
CREATE INDEX IF NOT EXISTS idx_instructions_category ON instructions(category);
CREATE INDEX IF NOT EXISTS idx_instructions_priority ON instructions(priority);
CREATE INDEX IF NOT EXISTS idx_instructions_enabled ON instructions(enabled);
