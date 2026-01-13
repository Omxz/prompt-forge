# Prompt Forge Installation Guide

Prompt Forge is a desktop application for managing AI agents, skills, and instructions. It connects to Claude Code via MCP (Model Context Protocol).

## Download

Download the latest release for your platform from the [Releases page](https://github.com/Omxz/prompt-forge/releases).

- **macOS**: `Prompt-Forge_x.x.x_aarch64.dmg` (Apple Silicon) or `Prompt-Forge_x.x.x_x64.dmg` (Intel)
- **Windows**: `Prompt-Forge_x.x.x_x64-setup.exe` or `Prompt-Forge_x.x.x_x64.msi`
- **Linux**: `prompt-forge_x.x.x_amd64.deb` (Debian/Ubuntu) or `prompt-forge-x.x.x.AppImage`

## Installation

### macOS

1. Download the `.dmg` file
2. Open the DMG and drag "Prompt Forge" to your Applications folder
3. **First launch**: Right-click the app and select "Open" (required for unsigned apps)
4. Click "Open" in the security dialog

### Windows

1. Download the `.exe` or `.msi` installer
2. Run the installer and follow the prompts
3. Prompt Forge will be installed to `C:\Program Files\Prompt Forge`

### Linux

**Debian/Ubuntu (.deb)**:
```bash
sudo dpkg -i prompt-forge_x.x.x_amd64.deb
```

**AppImage**:
```bash
chmod +x prompt-forge-x.x.x.AppImage
./prompt-forge-x.x.x.AppImage
```

## Connect to Claude Code

To use your Prompt Forge agents and instructions with Claude Code, add the following to your Claude Code configuration.

### Configuration File Location

- **Claude Code**: `~/.claude.json` (create if it doesn't exist)
- **VS Code with GitHub Copilot**: Add to VS Code `settings.json`

### macOS Configuration

Add to `~/.claude.json`:
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

### Windows Configuration

Add to your Claude configuration:
```json
{
  "mcpServers": {
    "prompt-forge": {
      "command": "C:\\Program Files\\Prompt Forge\\prompt-forge.exe",
      "args": ["--mcp"]
    }
  }
}
```

### Linux Configuration

Add to `~/.claude.json`:
```json
{
  "mcpServers": {
    "prompt-forge": {
      "command": "/usr/bin/prompt-forge",
      "args": ["--mcp"]
    }
  }
}
```

## Usage

### Using Prompt Forge App

1. **Launch Prompt Forge** from your Applications
2. **Create Agents**: Define AI personas with custom system prompts and personalities
3. **Define Skills**: Create reusable prompt templates
4. **Set Instructions**: Add coding guidelines (like CLAUDE.md files)
5. **Attach to Agents**: Link skills and instructions to your agents

### Using with Claude Code

Once configured, you can use Prompt Forge tools in Claude Code:

```
# List all your agents
Use the list_agents tool from prompt-forge

# Apply an agent's configuration
Use the apply_agent tool with agent_id="your-agent-id"

# Get your coding instructions
Use the get_instructions tool from prompt-forge
```

### Available MCP Tools

| Tool | Description |
|------|-------------|
| `list_agents` | List all configured agents |
| `get_agent` | Get a specific agent's full configuration |
| `apply_agent` | Apply an agent (returns full system prompt with skills & instructions) |
| `list_skills` | List all available skills |
| `get_skill` | Get a specific skill's details |
| `get_instructions` | Get all enabled instructions (optionally filter by category) |

## Data Management

### Export/Import

- Go to **Settings > Data Management** in Prompt Forge
- **Export All Data**: Creates a JSON backup of all agents, skills, and instructions
- **Import Data**: Restore from a backup file

### Share with Team

1. Export your data from Prompt Forge
2. Share the JSON file with colleagues
3. They can import it into their Prompt Forge instance

## Troubleshooting

### App Won't Open (macOS)

If you see "Prompt Forge cannot be opened because it is from an unidentified developer":
1. Open System Settings > Privacy & Security
2. Scroll down to find the blocked app message
3. Click "Open Anyway"

### MCP Server Not Connecting

1. Ensure Prompt Forge is installed in the expected location
2. Verify the path in your MCP configuration matches your installation
3. Try restarting Claude Code after updating the configuration
4. Check that the `--mcp` argument is included

### Data Location

Your data is stored in:
- **macOS**: `~/Library/Application Support/com.promptforge.app/promptforge.db`
- **Windows**: `%APPDATA%\com.promptforge.app\promptforge.db`
- **Linux**: `~/.local/share/com.promptforge.app/promptforge.db`

## Building from Source

If you want to build Prompt Forge yourself:

```bash
# Prerequisites: Node.js 18+, Rust 1.70+

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

## Support

- **Issues**: [GitHub Issues](https://github.com/Omxz/prompt-forge/issues)
- **Documentation**: Check the Settings page in Prompt Forge for integration instructions
