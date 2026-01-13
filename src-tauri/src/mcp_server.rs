// MCP Server implementation using STDIO transport
// This module handles JSON-RPC 2.0 communication with MCP clients (like Claude Code)

use crate::db::Database;
use crate::models::{Agent, Instruction, InstructionCategory, Skill, SkillDefinition};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

// ============================================================================
// JSON-RPC 2.0 Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

// ============================================================================
// MCP Protocol Types
// ============================================================================

#[derive(Debug, Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

#[derive(Debug, Serialize)]
pub struct ServerCapabilities {
    pub tools: ToolsCapability,
    pub resources: ResourcesCapability,
}

#[derive(Debug, Serialize)]
pub struct ToolsCapability {
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Serialize)]
pub struct ResourcesCapability {
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
    pub subscribe: bool,
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[derive(Debug, Serialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Serialize)]
pub struct ToolResult {
    pub content: Vec<ToolContent>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ResourceContent {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
}

// ============================================================================
// MCP Server State
// ============================================================================

pub struct McpServer {
    db_path: PathBuf,
    agents: Vec<Agent>,
    skills: Vec<Skill>,
    instructions: Vec<Instruction>,
}

impl McpServer {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            agents: Vec::new(),
            skills: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn load_data(&mut self) -> Result<(), String> {
        // Load data from SQLite database
        let db = Database::open(&self.db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        self.agents = db
            .get_all_agents()
            .map_err(|e| format!("Failed to load agents: {}", e))?;
        self.skills = db
            .get_all_skills()
            .map_err(|e| format!("Failed to load skills: {}", e))?;
        self.instructions = db
            .get_all_instructions()
            .map_err(|e| format!("Failed to load instructions: {}", e))?;

        eprintln!(
            "Loaded {} agents, {} skills, {} instructions from database",
            self.agents.len(),
            self.skills.len(),
            self.instructions.len()
        );

        Ok(())
    }

    /// Run the MCP server (STDIO mode)
    pub fn run(&mut self) -> io::Result<()> {
        if let Err(e) = self.load_data() {
            eprintln!("Warning: Failed to load data from database: {}", e);
            eprintln!("MCP server will start with empty data");
        }

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        eprintln!("Prompt Forge MCP Server started");
        eprintln!("Database path: {:?}", self.db_path);

        for line in stdin.lock().lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            // Parse the JSON-RPC request
            match serde_json::from_str::<JsonRpcRequest>(&line) {
                Ok(request) => {
                    // Notifications (no id) should not receive responses
                    let is_notification = request.id.is_none();
                    let response = self.handle_request(request);

                    if !is_notification {
                        let response_json = serde_json::to_string(&response).unwrap();
                        writeln!(stdout, "{}", response_json)?;
                        stdout.flush()?;
                    }
                }
                Err(e) => {
                    let error_response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32700,
                            message: format!("Parse error: {}", e),
                            data: None,
                        }),
                    };
                    let response_json = serde_json::to_string(&error_response).unwrap();
                    writeln!(stdout, "{}", response_json)?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }

    fn handle_request(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        eprintln!("Received method: {}", request.method);

        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(),
            "initialized" => Ok(json!({})),
            "tools/list" => self.handle_tools_list(),
            "tools/call" => self.handle_tools_call(request.params),
            "resources/list" => self.handle_resources_list(),
            "resources/read" => self.handle_resources_read(request.params),
            "ping" => Ok(json!({})),
            // Reload data from database on request
            "notifications/reload" => {
                let _ = self.load_data();
                Ok(json!({"reloaded": true}))
            }
            _ => Err(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
        };

        match result {
            Ok(result) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(result),
                error: None,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(error),
            },
        }
    }

    fn handle_initialize(&self) -> Result<Value, JsonRpcError> {
        Ok(json!(InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: ToolsCapability { list_changed: false },
                resources: ResourcesCapability {
                    list_changed: false,
                    subscribe: false
                },
            },
            server_info: ServerInfo {
                name: "prompt-forge".to_string(),
                version: "0.1.0".to_string(),
            },
        }))
    }

    fn handle_tools_list(&self) -> Result<Value, JsonRpcError> {
        let tools = vec![
            Tool {
                name: "get_agent".to_string(),
                description: "Get a Prompt Forge agent's full configuration including system prompt, personality, and attached skills/instructions".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "agent_id": {
                            "type": "string",
                            "description": "The ID of the agent to retrieve. Use 'default' for the default agent."
                        }
                    },
                    "required": ["agent_id"]
                }),
            },
            Tool {
                name: "list_agents".to_string(),
                description: "List all available Prompt Forge agents".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Tool {
                name: "get_instructions".to_string(),
                description: "Get all enabled instructions/guidelines from Prompt Forge".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "Optional category filter: general, code_style, communication, workflow, security, testing, documentation, custom"
                        }
                    }
                }),
            },
            Tool {
                name: "get_skill".to_string(),
                description: "Get a specific skill's full configuration and prompt template. Use the skill name (e.g., 'code-review', 'frontend-design') or ID.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "skill_id": {
                            "type": "string",
                            "description": "The ID or name of the skill to retrieve (e.g., 'code-review', 'explain-code', 'frontend-design')"
                        }
                    },
                    "required": ["skill_id"]
                }),
            },
            Tool {
                name: "list_skills".to_string(),
                description: "List all available skills".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Tool {
                name: "apply_agent".to_string(),
                description: "Apply an agent's configuration - returns the full system prompt with all attached skills and instructions combined".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "agent_id": {
                            "type": "string",
                            "description": "The ID of the agent to apply"
                        }
                    },
                    "required": ["agent_id"]
                }),
            },
        ];

        Ok(json!({ "tools": tools }))
    }

    fn handle_tools_call(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or(JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let tool_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or(JsonRpcError {
                code: -32602,
                message: "Missing tool name".to_string(),
                data: None,
            })?;

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        let result = match tool_name {
            "get_agent" => self.tool_get_agent(&arguments),
            "list_agents" => self.tool_list_agents(),
            "get_instructions" => self.tool_get_instructions(&arguments),
            "get_skill" => self.tool_get_skill(&arguments),
            "list_skills" => self.tool_list_skills(),
            "apply_agent" => self.tool_apply_agent(&arguments),
            _ => Err(format!("Unknown tool: {}", tool_name)),
        };

        match result {
            Ok(text) => Ok(json!(ToolResult {
                content: vec![ToolContent {
                    content_type: "text".to_string(),
                    text,
                }],
                is_error: None,
            })),
            Err(error) => Ok(json!(ToolResult {
                content: vec![ToolContent {
                    content_type: "text".to_string(),
                    text: error,
                }],
                is_error: Some(true),
            })),
        }
    }

    fn handle_resources_list(&self) -> Result<Value, JsonRpcError> {
        let mut resources = Vec::new();

        // Expose agents as resources
        for agent in &self.agents {
            resources.push(Resource {
                uri: format!("prompt-forge://agents/{}", agent.id),
                name: agent.name.clone(),
                description: agent.description.clone(),
                mime_type: "application/json".to_string(),
            });
        }

        // Expose instructions as a combined resource
        resources.push(Resource {
            uri: "prompt-forge://instructions/all".to_string(),
            name: "All Instructions".to_string(),
            description: "All enabled instructions combined".to_string(),
            mime_type: "text/markdown".to_string(),
        });

        Ok(json!({ "resources": resources }))
    }

    fn handle_resources_read(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or(JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let uri = params
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or(JsonRpcError {
                code: -32602,
                message: "Missing uri".to_string(),
                data: None,
            })?;

        if uri == "prompt-forge://instructions/all" {
            let content = self.get_all_instructions_markdown();
            return Ok(json!({
                "contents": [ResourceContent {
                    uri: uri.to_string(),
                    mime_type: "text/markdown".to_string(),
                    text: content,
                }]
            }));
        }

        if uri.starts_with("prompt-forge://agents/") {
            let agent_id = uri.strip_prefix("prompt-forge://agents/").unwrap();
            if let Some(agent) = self.agents.iter().find(|a| a.id == agent_id) {
                let content = serde_json::to_string_pretty(agent).unwrap();
                return Ok(json!({
                    "contents": [ResourceContent {
                        uri: uri.to_string(),
                        mime_type: "application/json".to_string(),
                        text: content,
                    }]
                }));
            }
        }

        Err(JsonRpcError {
            code: -32602,
            message: format!("Resource not found: {}", uri),
            data: None,
        })
    }

    // ========================================================================
    // Tool Implementations
    // ========================================================================

    fn tool_get_agent(&self, args: &Value) -> Result<String, String> {
        let agent_id = args
            .get("agent_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing agent_id")?;

        // Try to find by ID first, then by name (case-insensitive)
        let agent = self
            .agents
            .iter()
            .find(|a| a.id == agent_id)
            .or_else(|| {
                let name_lower = agent_id.to_lowercase();
                self.agents.iter().find(|a| a.name.to_lowercase() == name_lower)
            })
            .ok_or(format!("Agent not found: '{}'. Use list_agents to see available agents.", agent_id))?;

        Ok(serde_json::to_string_pretty(agent).unwrap())
    }

    fn tool_list_agents(&self) -> Result<String, String> {
        let summary: Vec<_> = self
            .agents
            .iter()
            .map(|a| {
                json!({
                    "id": a.id,
                    "name": a.name,
                    "description": a.description,
                    "emoji": a.avatar_emoji
                })
            })
            .collect();

        Ok(serde_json::to_string_pretty(&summary).unwrap())
    }

    fn tool_get_instructions(&self, args: &Value) -> Result<String, String> {
        let category_filter = args.get("category").and_then(|v| v.as_str());

        let filtered: Vec<_> = self
            .instructions
            .iter()
            .filter(|i| i.enabled)
            .filter(|i| {
                if let Some(cat) = category_filter {
                    category_to_string(&i.category) == cat.to_lowercase()
                } else {
                    true
                }
            })
            .collect();

        if filtered.is_empty() {
            return Ok("No instructions found.".to_string());
        }

        let mut output = String::new();
        for instruction in filtered {
            output.push_str(&format!(
                "## {} {} (Priority: {})\n",
                instruction.icon_emoji, instruction.name, instruction.priority
            ));
            output.push_str(&format!(
                "Category: {}\n\n",
                category_to_string(&instruction.category)
            ));
            output.push_str(&instruction.content);
            output.push_str("\n\n---\n\n");
        }

        Ok(output)
    }

    fn tool_get_skill(&self, args: &Value) -> Result<String, String> {
        let skill_id = args
            .get("skill_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing skill_id")?;

        // Try to find by ID first, then by name (case-insensitive)
        let skill = self
            .skills
            .iter()
            .find(|s| s.id == skill_id)
            .or_else(|| {
                let name_lower = skill_id.to_lowercase().replace(' ', "-").replace('_', "-");
                self.skills.iter().find(|s| {
                    s.name.to_lowercase().replace(' ', "-").replace('_', "-") == name_lower
                })
            })
            .ok_or(format!("Skill not found: '{}'. Use list_skills to see available skills.", skill_id))?;

        Ok(serde_json::to_string_pretty(skill).unwrap())
    }

    fn tool_list_skills(&self) -> Result<String, String> {
        if self.skills.is_empty() {
            return Ok("No skills configured.".to_string());
        }

        let summary: Vec<_> = self
            .skills
            .iter()
            .map(|s| {
                json!({
                    "id": s.id,
                    "name": s.name,
                    "description": s.description,
                    "emoji": s.icon_emoji,
                    "enabled": s.enabled
                })
            })
            .collect();

        Ok(serde_json::to_string_pretty(&summary).unwrap())
    }

    fn tool_apply_agent(&self, args: &Value) -> Result<String, String> {
        let agent_id = args
            .get("agent_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing agent_id")?;

        // Try to find by ID first, then by name (case-insensitive)
        let agent = self
            .agents
            .iter()
            .find(|a| a.id == agent_id)
            .or_else(|| {
                let name_lower = agent_id.to_lowercase();
                self.agents.iter().find(|a| a.name.to_lowercase() == name_lower)
            })
            .ok_or(format!("Agent not found: '{}'. Use list_agents to see available agents.", agent_id))?;

        let mut full_prompt = String::new();

        // Add agent's system prompt
        full_prompt.push_str("# Agent Configuration\n\n");
        full_prompt.push_str(&format!(
            "**Agent:** {} {}\n\n",
            agent.avatar_emoji, agent.name
        ));
        full_prompt.push_str(&format!(
            "**Tone:** {} | **Verbosity:** {}\n\n",
            agent.personality.tone, agent.personality.verbosity
        ));

        if !agent.personality.traits.is_empty() {
            full_prompt.push_str(&format!(
                "**Traits:** {}\n\n",
                agent.personality.traits.join(", ")
            ));
        }

        full_prompt.push_str("## System Prompt\n\n");
        full_prompt.push_str(&agent.system_prompt);
        full_prompt.push_str("\n\n");

        // Add attached skills
        if !agent.skills.is_empty() {
            full_prompt.push_str("## Attached Skills\n\n");
            for skill_id in &agent.skills {
                if let Some(skill) = self.skills.iter().find(|s| s.id == *skill_id && s.enabled) {
                    full_prompt.push_str(&format!("### {} {}\n", skill.icon_emoji, skill.name));
                    if let SkillDefinition::Prompt { template } = &skill.definition {
                        full_prompt.push_str(template);
                        full_prompt.push_str("\n\n");
                    }
                }
            }
        }

        // Add attached instructions
        if !agent.instructions.is_empty() {
            full_prompt.push_str("## Instructions\n\n");
            for instruction_id in &agent.instructions {
                if let Some(instruction) = self
                    .instructions
                    .iter()
                    .find(|i| i.id == *instruction_id && i.enabled)
                {
                    full_prompt.push_str(&format!(
                        "### {} {}\n",
                        instruction.icon_emoji, instruction.name
                    ));
                    full_prompt.push_str(&instruction.content);
                    full_prompt.push_str("\n\n");
                }
            }
        }

        // Add all enabled global instructions
        let global_instructions: Vec<_> = self
            .instructions
            .iter()
            .filter(|i| i.enabled && !agent.instructions.contains(&i.id))
            .collect();

        if !global_instructions.is_empty() {
            full_prompt.push_str("## Global Instructions\n\n");
            for instruction in global_instructions {
                full_prompt.push_str(&format!(
                    "### {} {} ({})\n",
                    instruction.icon_emoji,
                    instruction.name,
                    category_to_string(&instruction.category)
                ));
                full_prompt.push_str(&instruction.content);
                full_prompt.push_str("\n\n");
            }
        }

        Ok(full_prompt)
    }

    fn get_all_instructions_markdown(&self) -> String {
        let enabled: Vec<_> = self.instructions.iter().filter(|i| i.enabled).collect();

        if enabled.is_empty() {
            return "No instructions enabled.".to_string();
        }

        let mut output = String::from("# Prompt Forge Instructions\n\n");

        for instruction in enabled {
            output.push_str(&format!(
                "## {} {} (Priority: {})\n",
                instruction.icon_emoji, instruction.name, instruction.priority
            ));
            output.push_str(&format!(
                "*Category: {}*\n\n",
                category_to_string(&instruction.category)
            ));
            output.push_str(&instruction.content);
            output.push_str("\n\n---\n\n");
        }

        output
    }
}

fn category_to_string(cat: &InstructionCategory) -> &'static str {
    match cat {
        InstructionCategory::General => "general",
        InstructionCategory::CodeStyle => "code_style",
        InstructionCategory::Communication => "communication",
        InstructionCategory::Workflow => "workflow",
        InstructionCategory::Security => "security",
        InstructionCategory::Testing => "testing",
        InstructionCategory::Documentation => "documentation",
        InstructionCategory::Custom => "custom",
    }
}

/// Entry point for MCP mode
pub fn run_mcp_server(db_path: PathBuf) {
    let mut server = McpServer::new(db_path);
    if let Err(e) = server.run() {
        eprintln!("MCP Server error: {}", e);
        std::process::exit(1);
    }
}
