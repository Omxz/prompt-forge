# Prompt Forge - Development Guidelines

This document provides context for Claude Code when working on this codebase.

## Project Overview

Prompt Forge is a Tauri 2 desktop application for managing AI agents, skills, and instructions. It provides an MCP (Model Context Protocol) server that connects to Claude Code.

## Architecture

### Frontend (SvelteKit 5 + TypeScript)

- **Location**: `src/`
- **Framework**: SvelteKit 5 with Svelte 5 runes syntax
- **Styling**: CSS variables in `src/app.css`, component-scoped styles
- **State**: Svelte stores in `src/lib/stores.ts`
- **Types**: TypeScript definitions in `src/lib/types.ts`

### Backend (Rust + Tauri 2)

- **Location**: `src-tauri/`
- **Database**: SQLite via rusqlite (`src-tauri/src/db.rs`)
- **MCP Server**: STDIO-based MCP implementation (`src-tauri/src/mcp.rs`)
- **Commands**: Tauri IPC commands in `src-tauri/src/main.rs`

## Code Style

### Frontend

- Use Svelte 5 runes syntax (`$state`, `$derived`, `$effect`) for new components
- Keep components in `src/lib/components/`
- Use CSS variables from `app.css` for colors, spacing, typography
- Prefer `onclick` over `on:click` (Svelte 5 syntax)

### Backend

- Use `Result<T, String>` for Tauri command return types
- Database operations go in `db.rs`
- MCP tool handlers go in `mcp.rs`
- Keep `main.rs` focused on Tauri setup and command definitions

## Theme System

The app supports light/dark themes via CSS variables:

- Light theme: Default in `:root`
- Dark theme: `[data-theme="dark"]` selector
- Theme applied via `applyTheme()` in `stores.ts`
- Accent colors are dynamically generated from user selection

## Key Files

- `src/app.css` - Global styles and CSS variables
- `src/lib/stores.ts` - Svelte stores and theme management
- `src/lib/types.ts` - TypeScript interfaces
- `src/lib/components/SettingsView.svelte` - Settings UI including appearance
- `src-tauri/src/db.rs` - Database operations
- `src-tauri/src/mcp.rs` - MCP server implementation

## Commands

```bash
npm run dev          # Start SvelteKit dev server
npm run tauri dev    # Start full Tauri app in dev mode
npm run build        # Build SvelteKit
npm run tauri build  # Build production app
npm run check        # TypeScript/Svelte type checking
```

## Testing Changes

1. Run `npm run check` to verify TypeScript
2. Run `npm run build` to verify the build succeeds
3. Test in `npm run tauri dev` for full functionality

## MCP Tools

The MCP server exposes these tools to Claude Code:

- `list_agents` - List all agents
- `get_agent` - Get agent by ID
- `apply_agent` - Get full agent configuration with skills/instructions
- `list_skills` - List all skills
- `get_skill` - Get skill by ID or name
- `get_instructions` - Get enabled instructions (optional category filter)
