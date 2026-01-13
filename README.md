# Prompt Forge

A desktop application for crafting and managing AI agents, skills, and instructions. Connect to Claude Code via MCP (Model Context Protocol) to use your custom prompts and personas.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey.svg)

## Features

- **Agents** - Create AI personas with custom system prompts, personalities, and attached skills
- **Skills** - Define reusable prompt templates that can be invoked via slash commands
- **Instructions** - Set coding guidelines and rules (like CLAUDE.md files) that persist across sessions
- **MCP Integration** - Connect directly to Claude Code via Model Context Protocol
- **Import/Export** - Share your configurations with teammates via JSON export

## Screenshots

*Coming soon*

## Installation

See [INSTALLATION.md](INSTALLATION.md) for detailed installation instructions.

### Quick Start

1. Download the latest release for your platform
2. Install the application
3. Add the MCP configuration to `~/.claude.json`:

```json
{
  "mcpServers": {
    "prompt-forge": {
      "command": "/Applications/Prompt Forge.app/Contents/MacOS/prompt-forge",
      "args": ["--mcp"]
    }
  }
}
```

4. Restart Claude Code to load the MCP server

## Usage

### In Prompt Forge

1. **Create Agents** - Define personas with system prompts and personalities
2. **Add Skills** - Create prompt templates for common tasks
3. **Set Instructions** - Add coding guidelines and best practices
4. **Link Everything** - Attach skills and instructions to your agents

### In Claude Code

Once configured, use the MCP tools:

```
# List your agents
Use list_agents from prompt-forge

# Apply an agent configuration
Use apply_agent with agent_id="your-agent-id"

# Get coding instructions
Use get_instructions from prompt-forge
```

### Available MCP Tools

| Tool | Description |
|------|-------------|
| `list_agents` | List all configured agents |
| `get_agent` | Get a specific agent's full configuration |
| `apply_agent` | Apply an agent (returns full system prompt) |
| `list_skills` | List all available skills |
| `get_skill` | Get a specific skill's prompt template |
| `get_instructions` | Get enabled instructions (filter by category) |

## Development

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Platform-specific dependencies (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### Setup

```bash
# Clone the repository
git clone https://github.com/Omxz/prompt-forge.git
cd prompt-forge

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure

```
prompt-forge/
├── src/                    # SvelteKit frontend
│   ├── lib/
│   │   ├── components/     # Svelte components
│   │   ├── stores.ts       # State management
│   │   └── types.ts        # TypeScript types
│   ├── routes/             # SvelteKit routes
│   └── app.css             # Global styles
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Application entry
│   │   ├── db.rs           # SQLite database
│   │   └── mcp.rs          # MCP server
│   └── Cargo.toml
└── package.json
```

## Tech Stack

- **Frontend**: SvelteKit 5, TypeScript, Vite
- **Backend**: Rust, Tauri 2
- **Database**: SQLite
- **Protocol**: MCP (Model Context Protocol)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

- **Issues**: [GitHub Issues](https://github.com/Omxz/prompt-forge/issues)
- **Documentation**: See [INSTALLATION.md](INSTALLATION.md) for setup help
