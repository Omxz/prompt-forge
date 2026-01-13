//! Database module for Prompt Forge
//! Provides SQLite-backed persistence for agents, skills, instructions, and settings.

use crate::models::*;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::Path;
use std::sync::Mutex;

/// Database wrapper that provides thread-safe access to SQLite
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Open or create a database at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Run database migrations
    pub fn migrate(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;

        // Check if usage_count column exists before running migration 002
        let has_usage_count: bool = conn
            .prepare("SELECT COUNT(*) FROM pragma_table_info('agents') WHERE name='usage_count'")?
            .query_row([], |row| {
                let count: i32 = row.get(0)?;
                Ok(count > 0)
            })?;

        // Only run migration if columns don't exist
        if !has_usage_count {
            conn.execute_batch(include_str!("../migrations/002_add_usage_tracking.sql"))?;
        }

        Ok(())
    }

    /// Check if the database has any data (for first-run detection)
    pub fn is_empty(&self) -> SqliteResult<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM agents", [], |row| row.get(0))?;
        Ok(count == 0)
    }

    // ========================================================================
    // Agent Operations
    // ========================================================================

    pub fn insert_agent(&self, agent: &Agent) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO agents (id, name, description, avatar_emoji, personality_json,
             system_prompt, skills_json, instructions_json, tags_json, created_at, updated_at, usage_count, last_used_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                agent.id,
                agent.name,
                agent.description,
                agent.avatar_emoji,
                serde_json::to_string(&agent.personality).unwrap(),
                agent.system_prompt,
                serde_json::to_string(&agent.skills).unwrap(),
                serde_json::to_string(&agent.instructions).unwrap(),
                serde_json::to_string(&agent.tags).unwrap(),
                agent.created_at.to_rfc3339(),
                agent.updated_at.to_rfc3339(),
                agent.usage_count,
                agent.last_used_at.map(|dt| dt.to_rfc3339()),
            ],
        )?;
        Ok(())
    }

    pub fn get_all_agents(&self) -> SqliteResult<Vec<Agent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, avatar_emoji, personality_json, system_prompt,
             skills_json, instructions_json, tags_json, created_at, updated_at, usage_count, last_used_at FROM agents
             ORDER BY usage_count DESC",
        )?;

        let agents = stmt
            .query_map([], |row| {
                Ok(Agent {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    avatar_emoji: row.get(3)?,
                    personality: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    system_prompt: row.get(5)?,
                    skills: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                    instructions: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                    tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    usage_count: row.get(11)?,
                    last_used_at: row.get::<_, Option<String>>(12)?
                        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&Utc)),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(agents)
    }

    pub fn get_agent(&self, id: &str) -> SqliteResult<Option<Agent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, avatar_emoji, personality_json, system_prompt,
             skills_json, instructions_json, tags_json, created_at, updated_at, usage_count, last_used_at
             FROM agents WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Agent {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                avatar_emoji: row.get(3)?,
                personality: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                system_prompt: row.get(5)?,
                skills: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                instructions: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                usage_count: row.get(11)?,
                last_used_at: row.get::<_, Option<String>>(12)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn update_agent(&self, agent: &Agent) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE agents SET name = ?2, description = ?3, avatar_emoji = ?4,
             personality_json = ?5, system_prompt = ?6, skills_json = ?7,
             instructions_json = ?8, tags_json = ?9, updated_at = ?10, usage_count = ?11, last_used_at = ?12 WHERE id = ?1",
            params![
                agent.id,
                agent.name,
                agent.description,
                agent.avatar_emoji,
                serde_json::to_string(&agent.personality).unwrap(),
                agent.system_prompt,
                serde_json::to_string(&agent.skills).unwrap(),
                serde_json::to_string(&agent.instructions).unwrap(),
                serde_json::to_string(&agent.tags).unwrap(),
                agent.updated_at.to_rfc3339(),
                agent.usage_count,
                agent.last_used_at.map(|dt| dt.to_rfc3339()),
            ],
        )?;
        Ok(())
    }

    pub fn delete_agent(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM agents WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn record_agent_usage(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE agents SET usage_count = usage_count + 1, last_used_at = ?2 WHERE id = ?1",
            params![id, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    // ========================================================================
    // Skill Operations
    // ========================================================================

    pub fn insert_skill(&self, skill: &Skill) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO skills (id, name, description, icon_emoji, skill_type,
             definition_json, enabled, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                skill.id,
                skill.name,
                skill.description,
                skill.icon_emoji,
                skill_type_to_string(&skill.skill_type),
                serde_json::to_string(&skill.definition).unwrap(),
                skill.enabled,
                skill.created_at.to_rfc3339(),
                skill.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_all_skills(&self) -> SqliteResult<Vec<Skill>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, icon_emoji, skill_type, definition_json,
             enabled, created_at, updated_at FROM skills",
        )?;

        let skills = stmt
            .query_map([], |row| {
                Ok(Skill {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    icon_emoji: row.get(3)?,
                    skill_type: string_to_skill_type(&row.get::<_, String>(4)?),
                    definition: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_else(
                        |_| SkillDefinition::Prompt {
                            template: String::new(),
                        },
                    ),
                    enabled: row.get(6)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(skills)
    }

    pub fn get_skill(&self, id: &str) -> SqliteResult<Option<Skill>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, icon_emoji, skill_type, definition_json,
             enabled, created_at, updated_at FROM skills WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Skill {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                icon_emoji: row.get(3)?,
                skill_type: string_to_skill_type(&row.get::<_, String>(4)?),
                definition: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_else(|_| {
                    SkillDefinition::Prompt {
                        template: String::new(),
                    }
                }),
                enabled: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn update_skill(&self, skill: &Skill) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE skills SET name = ?2, description = ?3, icon_emoji = ?4,
             skill_type = ?5, definition_json = ?6, enabled = ?7, updated_at = ?8 WHERE id = ?1",
            params![
                skill.id,
                skill.name,
                skill.description,
                skill.icon_emoji,
                skill_type_to_string(&skill.skill_type),
                serde_json::to_string(&skill.definition).unwrap(),
                skill.enabled,
                skill.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_skill(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM skills WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ========================================================================
    // Instruction Operations
    // ========================================================================

    pub fn insert_instruction(&self, instruction: &Instruction) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO instructions (id, name, description, icon_emoji, category,
             content, priority, tags_json, enabled, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                instruction.id,
                instruction.name,
                instruction.description,
                instruction.icon_emoji,
                category_to_string(&instruction.category),
                instruction.content,
                instruction.priority,
                serde_json::to_string(&instruction.tags).unwrap(),
                instruction.enabled,
                instruction.created_at.to_rfc3339(),
                instruction.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_all_instructions(&self) -> SqliteResult<Vec<Instruction>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, icon_emoji, category, content, priority,
             tags_json, enabled, created_at, updated_at FROM instructions",
        )?;

        let instructions = stmt
            .query_map([], |row| {
                Ok(Instruction {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    icon_emoji: row.get(3)?,
                    category: string_to_category(&row.get::<_, String>(4)?),
                    content: row.get(5)?,
                    priority: row.get(6)?,
                    tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                    enabled: row.get(8)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(instructions)
    }

    pub fn get_instruction(&self, id: &str) -> SqliteResult<Option<Instruction>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, icon_emoji, category, content, priority,
             tags_json, enabled, created_at, updated_at FROM instructions WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Instruction {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                icon_emoji: row.get(3)?,
                category: string_to_category(&row.get::<_, String>(4)?),
                content: row.get(5)?,
                priority: row.get(6)?,
                tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                enabled: row.get(8)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn update_instruction(&self, instruction: &Instruction) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE instructions SET name = ?2, description = ?3, icon_emoji = ?4,
             category = ?5, content = ?6, priority = ?7, tags_json = ?8, enabled = ?9,
             updated_at = ?10 WHERE id = ?1",
            params![
                instruction.id,
                instruction.name,
                instruction.description,
                instruction.icon_emoji,
                category_to_string(&instruction.category),
                instruction.content,
                instruction.priority,
                serde_json::to_string(&instruction.tags).unwrap(),
                instruction.enabled,
                instruction.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_instruction(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM instructions WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ========================================================================
    // Settings Operations
    // ========================================================================

    pub fn get_settings(&self) -> SqliteResult<Settings> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT theme_mode, theme_accent_color, theme_emotional_ui, mcp_server_port,
             auto_start_mcp, data_directory FROM settings WHERE id = 1",
        )?;

        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(Settings {
                theme: Theme {
                    mode: row.get(0)?,
                    accent_color: row.get(1)?,
                    emotional_ui: row.get(2)?,
                },
                mcp_server_port: row.get(3)?,
                mcp_server_enabled: false, // Runtime state, not persisted
                auto_start_mcp: row.get(4)?,
                data_directory: row.get(5)?,
            })
        } else {
            Ok(Settings::default())
        }
    }

    pub fn save_settings(&self, settings: &Settings) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE settings SET theme_mode = ?1, theme_accent_color = ?2,
             theme_emotional_ui = ?3, mcp_server_port = ?4, auto_start_mcp = ?5,
             data_directory = ?6 WHERE id = 1",
            params![
                settings.theme.mode,
                settings.theme.accent_color,
                settings.theme.emotional_ui,
                settings.mcp_server_port,
                settings.auto_start_mcp,
                settings.data_directory,
            ],
        )?;
        Ok(())
    }

    // ========================================================================
    // Export/Import Operations
    // ========================================================================

    pub fn export_all(&self) -> SqliteResult<ExportData> {
        Ok(ExportData {
            agents: self.get_all_agents()?,
            skills: self.get_all_skills()?,
            instructions: self.get_all_instructions()?,
            settings: self.get_settings()?,
            exported_at: Utc::now(),
            version: "1.0".to_string(),
        })
    }

    pub fn import_all(&self, data: &ExportData) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();

        // Clear existing data
        conn.execute("DELETE FROM agents", [])?;
        conn.execute("DELETE FROM skills", [])?;
        conn.execute("DELETE FROM instructions", [])?;

        drop(conn); // Release lock before calling other methods

        // Import agents
        for agent in &data.agents {
            self.insert_agent(agent)?;
        }

        // Import skills
        for skill in &data.skills {
            self.insert_skill(skill)?;
        }

        // Import instructions
        for instruction in &data.instructions {
            self.insert_instruction(instruction)?;
        }

        // Import settings
        self.save_settings(&data.settings)?;

        Ok(())
    }
}

// ============================================================================
// Export Data Structure
// ============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportData {
    pub agents: Vec<Agent>,
    pub skills: Vec<Skill>,
    pub instructions: Vec<Instruction>,
    pub settings: Settings,
    pub exported_at: DateTime<Utc>,
    pub version: String,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn skill_type_to_string(st: &SkillType) -> &'static str {
    match st {
        SkillType::Prompt => "prompt",
        SkillType::Tool => "tool",
        SkillType::Workflow => "workflow",
    }
}

fn string_to_skill_type(s: &str) -> SkillType {
    match s {
        "tool" => SkillType::Tool,
        "workflow" => SkillType::Workflow,
        _ => SkillType::Prompt,
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

fn string_to_category(s: &str) -> InstructionCategory {
    match s {
        "code_style" => InstructionCategory::CodeStyle,
        "communication" => InstructionCategory::Communication,
        "workflow" => InstructionCategory::Workflow,
        "security" => InstructionCategory::Security,
        "testing" => InstructionCategory::Testing,
        "documentation" => InstructionCategory::Documentation,
        "custom" => InstructionCategory::Custom,
        _ => InstructionCategory::General,
    }
}

// ============================================================================
// Default Data Initialization
// ============================================================================

pub fn create_default_agent() -> Agent {
    Agent {
        id: "default".to_string(),
        name: "Claude Assistant".to_string(),
        description: "The default Claude assistant - helpful, harmless, and honest.".to_string(),
        avatar_emoji: "🧠".to_string(),
        personality: Personality {
            tone: "friendly".to_string(),
            verbosity: "balanced".to_string(),
            creativity: 0.7,
            formality: 0.5,
            traits: vec![
                "helpful".to_string(),
                "thoughtful".to_string(),
                "clear".to_string(),
            ],
        },
        system_prompt: "You are Claude, an AI assistant made by Anthropic. You are helpful, harmless, and honest. You aim to be direct and concise while being warm and personable.".to_string(),
        skills: vec![],
        instructions: vec![],
        tags: vec!["default".to_string()],
        created_at: Utc::now(),
        updated_at: Utc::now(),
        usage_count: 0,
        last_used_at: None,
    }
}

pub fn create_default_skills() -> Vec<Skill> {
    vec![
        Skill {
            id: "code-review".to_string(),
            name: "Code Review".to_string(),
            description: "Perform thorough code reviews with constructive feedback".to_string(),
            icon_emoji: "🔍".to_string(),
            skill_type: SkillType::Prompt,
            definition: SkillDefinition::Prompt {
                template: "Review the following code for:\n- Bugs and potential issues\n- Performance optimizations\n- Code style and best practices\n- Security concerns\n\nProvide specific, actionable feedback.".to_string(),
            },
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Skill {
            id: "explain-code".to_string(),
            name: "Explain Code".to_string(),
            description: "Explain code in clear, simple terms".to_string(),
            icon_emoji: "📚".to_string(),
            skill_type: SkillType::Prompt,
            definition: SkillDefinition::Prompt {
                template: "Explain this code step by step:\n1. What does it do overall?\n2. Break down each important section\n3. Highlight any clever or tricky parts\n4. Suggest improvements if applicable".to_string(),
            },
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ]
}

pub fn create_default_instructions() -> Vec<Instruction> {
    vec![
        Instruction {
            id: "code-style".to_string(),
            name: "Code Style Guidelines".to_string(),
            description: "Standard code formatting and style rules".to_string(),
            icon_emoji: "📏".to_string(),
            category: InstructionCategory::CodeStyle,
            content: r#"# Code Style Guidelines

- Use meaningful variable and function names
- Keep functions small and focused (max 20-30 lines)
- Add comments for complex logic, not obvious code
- Follow the language's official style guide
- Use consistent indentation (spaces preferred)
- Group related code together
- Avoid deep nesting (max 3 levels)"#
                .to_string(),
            priority: 7,
            tags: vec!["code".to_string(), "style".to_string()],
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Instruction {
            id: "communication".to_string(),
            name: "Communication Style".to_string(),
            description: "How to communicate responses".to_string(),
            icon_emoji: "💬".to_string(),
            category: InstructionCategory::Communication,
            content: r#"# Communication Style

- Be direct and concise
- Start with the answer, then explain
- Use code examples when helpful
- Format responses with markdown
- Break complex topics into steps
- Acknowledge uncertainty honestly
- Ask clarifying questions when needed"#
                .to_string(),
            priority: 8,
            tags: vec!["communication".to_string()],
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ]
}

/// Initialize the database with default data if it's empty
pub fn init_default_data(db: &Database) -> SqliteResult<()> {
    if db.is_empty()? {
        // Insert default agent
        db.insert_agent(&create_default_agent())?;

        // Insert default skills
        for skill in create_default_skills() {
            db.insert_skill(&skill)?;
        }

        // Insert default instructions
        for instruction in create_default_instructions() {
            db.insert_instruction(&instruction)?;
        }
    }
    Ok(())
}
