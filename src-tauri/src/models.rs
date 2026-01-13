use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Input for creating a new agent (doesn't require id, timestamps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentInput {
    pub name: String,
    pub description: String,
    pub avatar_emoji: String,
    pub personality: Personality,
    pub system_prompt: String,
    pub skills: Vec<String>,
    pub instructions: Vec<String>,
    pub tags: Vec<String>,
}

/// An Agent represents a customizable AI persona with specific skills and personality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub avatar_emoji: String,
    pub personality: Personality,
    pub system_prompt: String,
    pub skills: Vec<String>, // Skill IDs
    pub instructions: Vec<String>, // Instruction IDs
    pub tags: Vec<String>, // For organization/filtering
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub usage_count: i32,
    pub last_used_at: Option<DateTime<Utc>>,
}

impl Default for Agent {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "New Agent".to_string(),
            description: "A helpful AI assistant".to_string(),
            avatar_emoji: "🤖".to_string(),
            personality: Personality::default(),
            system_prompt: "You are a helpful AI assistant.".to_string(),
            skills: vec![],
            instructions: vec![],
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            usage_count: 0,
            last_used_at: None,
        }
    }
}

/// Personality traits that influence how the agent communicates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub tone: String,           // e.g., "friendly", "professional", "casual"
    pub verbosity: String,      // e.g., "concise", "detailed", "balanced"
    pub creativity: f32,        // 0.0 - 1.0
    pub formality: f32,         // 0.0 (casual) - 1.0 (formal)
    pub traits: Vec<String>,    // e.g., ["empathetic", "analytical", "witty"]
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            tone: "friendly".to_string(),
            verbosity: "balanced".to_string(),
            creativity: 0.5,
            formality: 0.5,
            traits: vec!["helpful".to_string(), "clear".to_string()],
        }
    }
}

/// Input for creating a new skill (doesn't require id, timestamps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkillInput {
    pub name: String,
    pub description: String,
    pub icon_emoji: String,
    pub skill_type: SkillType,
    pub definition: SkillDefinition,
    pub enabled: bool,
}

/// A Skill represents a specific capability or tool the agent can use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_emoji: String,
    pub skill_type: SkillType,
    pub definition: SkillDefinition,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "New Skill".to_string(),
            description: "A custom skill".to_string(),
            icon_emoji: "⚡".to_string(),
            skill_type: SkillType::Prompt,
            definition: SkillDefinition::Prompt {
                template: String::new(),
            },
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillType {
    Prompt,      // A prompt template
    Tool,        // A tool definition (MCP-compatible)
    Workflow,    // A multi-step workflow
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SkillDefinition {
    Prompt {
        template: String,
    },
    Tool {
        parameters: Vec<ToolParameter>,
        handler: String, // Script or command to execute
    },
    Workflow {
        steps: Vec<WorkflowStep>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub description: String,
    pub param_type: String, // "string", "number", "boolean", "array", "object"
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub action: String,
    pub inputs: serde_json::Value,
    pub outputs: Vec<String>,
}

/// Input for creating a new instruction (doesn't require id, timestamps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstructionInput {
    pub name: String,
    pub description: String,
    pub icon_emoji: String,
    pub category: InstructionCategory,
    pub content: String,
    pub priority: u8,
    pub tags: Vec<String>,
    pub enabled: bool,
}

/// An Instruction set - like CLAUDE.md but structured
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_emoji: String,
    pub category: InstructionCategory,
    pub content: String, // The actual instruction text
    pub priority: u8, // 1-10, higher = more important
    pub tags: Vec<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "New Instruction".to_string(),
            description: "Custom instruction set".to_string(),
            icon_emoji: "📋".to_string(),
            category: InstructionCategory::General,
            content: String::new(),
            priority: 5,
            tags: vec![],
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionCategory {
    General,      // General guidelines
    CodeStyle,    // Code formatting and style
    Communication, // How to communicate
    Workflow,     // Process and workflow
    Security,     // Security guidelines
    Testing,      // Testing practices
    Documentation, // Documentation standards
    Custom,       // User-defined category
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub mcp_server_port: u16,
    pub mcp_server_enabled: bool,
    pub data_directory: Option<String>,
    pub auto_start_mcp: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            mcp_server_port: 3333,
            mcp_server_enabled: false,
            data_directory: None,
            auto_start_mcp: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub mode: String, // "dark", "light", "auto"
    pub accent_color: String,
    pub emotional_ui: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mode: "dark".to_string(),
            accent_color: "#8b5cf6".to_string(), // Purple
            emotional_ui: true,
        }
    }
}

/// MCP Server status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpStatus {
    pub running: bool,
    pub port: u16,
    pub connected_clients: u32,
    pub available_tools: Vec<String>,
}
