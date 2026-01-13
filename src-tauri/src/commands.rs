use crate::db::ExportData;
use crate::models::*;
use crate::parser;
use crate::AppState;
use chrono::Utc;
use std::process::{Command, Stdio};
use tauri::State;
use uuid::Uuid;

// ============================================================================
// Agent Commands
// ============================================================================

#[tauri::command]
pub fn create_agent(state: State<'_, AppState>, agent: CreateAgentInput) -> Result<Agent, String> {
    let agent = Agent {
        id: Uuid::new_v4().to_string(),
        name: agent.name,
        description: agent.description,
        avatar_emoji: agent.avatar_emoji,
        personality: agent.personality,
        system_prompt: agent.system_prompt,
        skills: agent.skills,
        instructions: agent.instructions,
        tags: agent.tags,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    state
        .db
        .insert_agent(&agent)
        .map_err(|e| format!("Failed to create agent: {}", e))?;

    Ok(agent)
}

#[tauri::command]
pub fn get_agents(state: State<'_, AppState>) -> Result<Vec<Agent>, String> {
    state
        .db
        .get_all_agents()
        .map_err(|e| format!("Failed to get agents: {}", e))
}

#[tauri::command]
pub fn get_agent(state: State<'_, AppState>, id: String) -> Result<Option<Agent>, String> {
    state
        .db
        .get_agent(&id)
        .map_err(|e| format!("Failed to get agent: {}", e))
}

#[tauri::command]
pub fn update_agent(state: State<'_, AppState>, agent: Agent) -> Result<Agent, String> {
    let mut agent = agent;
    agent.updated_at = Utc::now();

    state
        .db
        .update_agent(&agent)
        .map_err(|e| format!("Failed to update agent: {}", e))?;

    Ok(agent)
}

#[tauri::command]
pub fn delete_agent(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .db
        .delete_agent(&id)
        .map_err(|e| format!("Failed to delete agent: {}", e))
}

#[tauri::command]
pub fn import_agent_from_text(state: State<'_, AppState>, text: String) -> Result<Agent, String> {
    let mut agent = parser::parse_agent_from_markdown(&text)?;
    agent.id = Uuid::new_v4().to_string();
    agent.created_at = Utc::now();
    agent.updated_at = Utc::now();

    state
        .db
        .insert_agent(&agent)
        .map_err(|e| format!("Failed to import agent: {}", e))?;

    Ok(agent)
}

#[tauri::command]
pub fn export_agent_to_markdown(state: State<'_, AppState>, id: String) -> Result<String, String> {
    let agent = state
        .db
        .get_agent(&id)
        .map_err(|e| format!("Failed to get agent: {}", e))?
        .ok_or_else(|| "Agent not found".to_string())?;

    Ok(parser::export_agent_to_markdown_text(&agent))
}

// ============================================================================
// Skill Commands
// ============================================================================

#[tauri::command]
pub fn create_skill(state: State<'_, AppState>, skill: CreateSkillInput) -> Result<Skill, String> {
    let skill = Skill {
        id: Uuid::new_v4().to_string(),
        name: skill.name,
        description: skill.description,
        icon_emoji: skill.icon_emoji,
        skill_type: skill.skill_type,
        definition: skill.definition,
        enabled: skill.enabled,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    state
        .db
        .insert_skill(&skill)
        .map_err(|e| format!("Failed to create skill: {}", e))?;

    Ok(skill)
}

#[tauri::command]
pub fn get_skills(state: State<'_, AppState>) -> Result<Vec<Skill>, String> {
    state
        .db
        .get_all_skills()
        .map_err(|e| format!("Failed to get skills: {}", e))
}

#[tauri::command]
pub fn get_skill(state: State<'_, AppState>, id: String) -> Result<Option<Skill>, String> {
    state
        .db
        .get_skill(&id)
        .map_err(|e| format!("Failed to get skill: {}", e))
}

#[tauri::command]
pub fn update_skill(state: State<'_, AppState>, skill: Skill) -> Result<Skill, String> {
    let mut skill = skill;
    skill.updated_at = Utc::now();

    state
        .db
        .update_skill(&skill)
        .map_err(|e| format!("Failed to update skill: {}", e))?;

    Ok(skill)
}

#[tauri::command]
pub fn delete_skill(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .db
        .delete_skill(&id)
        .map_err(|e| format!("Failed to delete skill: {}", e))
}

// ============================================================================
// Instruction Commands
// ============================================================================

#[tauri::command]
pub fn create_instruction(
    state: State<'_, AppState>,
    instruction: CreateInstructionInput,
) -> Result<Instruction, String> {
    let instruction = Instruction {
        id: Uuid::new_v4().to_string(),
        name: instruction.name,
        description: instruction.description,
        icon_emoji: instruction.icon_emoji,
        category: instruction.category,
        content: instruction.content,
        priority: instruction.priority,
        tags: instruction.tags,
        enabled: instruction.enabled,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    state
        .db
        .insert_instruction(&instruction)
        .map_err(|e| format!("Failed to create instruction: {}", e))?;

    Ok(instruction)
}

#[tauri::command]
pub fn get_instructions(state: State<'_, AppState>) -> Result<Vec<Instruction>, String> {
    state
        .db
        .get_all_instructions()
        .map_err(|e| format!("Failed to get instructions: {}", e))
}

#[tauri::command]
pub fn get_instruction(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<Instruction>, String> {
    state
        .db
        .get_instruction(&id)
        .map_err(|e| format!("Failed to get instruction: {}", e))
}

#[tauri::command]
pub fn update_instruction(
    state: State<'_, AppState>,
    instruction: Instruction,
) -> Result<Instruction, String> {
    let mut instruction = instruction;
    instruction.updated_at = Utc::now();

    state
        .db
        .update_instruction(&instruction)
        .map_err(|e| format!("Failed to update instruction: {}", e))?;

    Ok(instruction)
}

#[tauri::command]
pub fn delete_instruction(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .db
        .delete_instruction(&id)
        .map_err(|e| format!("Failed to delete instruction: {}", e))
}

#[tauri::command]
pub fn import_instruction_from_text(
    state: State<'_, AppState>,
    text: String,
) -> Result<Instruction, String> {
    let mut instruction = parser::parse_instruction_from_markdown(&text)?;
    instruction.id = Uuid::new_v4().to_string();
    instruction.created_at = Utc::now();
    instruction.updated_at = Utc::now();

    state
        .db
        .insert_instruction(&instruction)
        .map_err(|e| format!("Failed to import instruction: {}", e))?;

    Ok(instruction)
}

#[tauri::command]
pub fn export_instruction_to_markdown(
    state: State<'_, AppState>,
    id: String,
) -> Result<String, String> {
    let instruction = state
        .db
        .get_instruction(&id)
        .map_err(|e| format!("Failed to get instruction: {}", e))?
        .ok_or_else(|| "Instruction not found".to_string())?;

    Ok(parser::export_instruction_to_markdown_text(&instruction))
}

// ============================================================================
// Settings Commands
// ============================================================================

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    let mut settings = state
        .db
        .get_settings()
        .map_err(|e| format!("Failed to get settings: {}", e))?;

    // Update runtime state
    settings.mcp_server_enabled = *state.mcp_running.lock().unwrap();

    Ok(settings)
}

#[tauri::command]
pub fn save_settings(state: State<'_, AppState>, settings: Settings) -> Result<Settings, String> {
    state
        .db
        .save_settings(&settings)
        .map_err(|e| format!("Failed to save settings: {}", e))?;

    Ok(settings)
}

// ============================================================================
// MCP Server Commands
// ============================================================================

#[tauri::command]
pub fn get_mcp_status(state: State<'_, AppState>) -> Result<McpStatus, String> {
    let running = *state.mcp_running.lock().map_err(|e| e.to_string())?;

    // Check if process is still alive
    let actually_running = if running {
        let mut mcp_process = state.mcp_process.lock().map_err(|e| e.to_string())?;
        if let Some(ref mut child) = *mcp_process {
            match child.try_wait() {
                Ok(None) => true,       // Still running
                Ok(Some(_)) => {
                    // Process exited
                    *state.mcp_running.lock().unwrap() = false;
                    *mcp_process = None;
                    false
                }
                Err(_) => false,
            }
        } else {
            false
        }
    } else {
        false
    };

    let settings = state.db.get_settings().unwrap_or_default();
    let agents = state.db.get_all_agents().unwrap_or_default();
    let skills = state.db.get_all_skills().unwrap_or_default();

    let mut available_tools = vec![
        "list_agents".to_string(),
        "get_agent".to_string(),
        "list_skills".to_string(),
        "get_skill".to_string(),
        "get_instructions".to_string(),
        "apply_agent".to_string(),
    ];

    // Add agent-specific tools
    for agent in agents.iter() {
        available_tools.push(format!(
            "agent:{}",
            agent.name.to_lowercase().replace(' ', "_")
        ));
    }

    // Add skill-specific tools
    for skill in skills.iter() {
        available_tools.push(format!(
            "skill:{}",
            skill.name.to_lowercase().replace(' ', "_")
        ));
    }

    Ok(McpStatus {
        running: actually_running,
        port: settings.mcp_server_port,
        connected_clients: 0,
        available_tools,
    })
}

#[tauri::command]
pub fn start_mcp_server(state: State<'_, AppState>) -> Result<McpStatus, String> {
    let mut mcp_process = state.mcp_process.lock().map_err(|e| e.to_string())?;

    if mcp_process.is_some() {
        return Err("MCP server is already running".to_string());
    }

    // Get path to current executable
    let exe_path = std::env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;

    // Spawn the MCP server as a child process
    let child = Command::new(&exe_path)
        .arg("--mcp")
        .arg("--db-path")
        .arg(&state.db_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start MCP server: {}", e))?;

    *mcp_process = Some(child);
    *state.mcp_running.lock().unwrap() = true;

    drop(mcp_process);
    get_mcp_status(state)
}

#[tauri::command]
pub fn stop_mcp_server(state: State<'_, AppState>) -> Result<McpStatus, String> {
    let mut mcp_process = state.mcp_process.lock().map_err(|e| e.to_string())?;

    if let Some(mut child) = mcp_process.take() {
        // Try graceful shutdown first
        let _ = child.kill();
        let _ = child.wait();
    }

    *state.mcp_running.lock().unwrap() = false;

    drop(mcp_process);
    get_mcp_status(state)
}

// ============================================================================
// MCP Tool Handlers (called by MCP server)
// ============================================================================

/// Get the full configuration for an agent to "become" that persona
#[tauri::command]
pub fn apply_agent(state: State<'_, AppState>, agent_name: String) -> Result<String, String> {
    let agents = state
        .db
        .get_all_agents()
        .map_err(|e| format!("Failed to get agents: {}", e))?;
    let skills = state
        .db
        .get_all_skills()
        .map_err(|e| format!("Failed to get skills: {}", e))?;
    let instructions = state
        .db
        .get_all_instructions()
        .map_err(|e| format!("Failed to get instructions: {}", e))?;

    let agent = agents
        .iter()
        .find(|a| a.name.to_lowercase() == agent_name.to_lowercase())
        .ok_or_else(|| format!("Agent '{}' not found", agent_name))?;

    // Build the full system prompt from agent + attached skills + attached instructions
    let mut full_prompt = agent.system_prompt.clone();

    // Add personality context
    full_prompt.push_str(&format!(
        "\n\n## Personality\n- Tone: {}\n- Verbosity: {}\n- Traits: {}",
        agent.personality.tone,
        agent.personality.verbosity,
        agent.personality.traits.join(", ")
    ));

    // Add attached skills
    let agent_skills: Vec<_> = skills
        .iter()
        .filter(|s| agent.skills.contains(&s.id) && s.enabled)
        .collect();

    if !agent_skills.is_empty() {
        full_prompt.push_str("\n\n## Available Skills\n");
        for skill in agent_skills {
            full_prompt.push_str(&format!("\n### {}\n{}\n", skill.name, skill.description));
            if let SkillDefinition::Prompt { template } = &skill.definition {
                full_prompt.push_str(&format!("Template: {}\n", template));
            }
        }
    }

    // Add attached instructions
    let agent_instructions: Vec<_> = instructions
        .iter()
        .filter(|i| agent.instructions.contains(&i.id) && i.enabled)
        .collect();

    if !agent_instructions.is_empty() {
        full_prompt.push_str("\n\n## Instructions\n");
        for instruction in agent_instructions {
            full_prompt.push_str(&format!("\n{}\n", instruction.content));
        }
    }

    Ok(full_prompt)
}

/// Get all enabled instructions combined
#[tauri::command]
pub fn get_all_enabled_instructions(state: State<'_, AppState>) -> Result<String, String> {
    let instructions = state
        .db
        .get_all_instructions()
        .map_err(|e| format!("Failed to get instructions: {}", e))?;

    let mut sorted: Vec<_> = instructions.iter().filter(|i| i.enabled).collect();

    // Sort by priority (higher first)
    sorted.sort_by(|a, b| b.priority.cmp(&a.priority));

    let combined = sorted
        .iter()
        .map(|i| format!("## {}\n{}", i.name, i.content))
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    Ok(combined)
}

// ============================================================================
// Export/Import Commands
// ============================================================================

#[tauri::command]
pub fn export_all_data(state: State<'_, AppState>) -> Result<ExportData, String> {
    state
        .db
        .export_all()
        .map_err(|e| format!("Failed to export data: {}", e))
}

#[tauri::command]
pub fn import_all_data(state: State<'_, AppState>, data: ExportData) -> Result<(), String> {
    state
        .db
        .import_all(&data)
        .map_err(|e| format!("Failed to import data: {}", e))
}
