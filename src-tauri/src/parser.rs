use crate::models::{Agent, Instruction, InstructionCategory, Personality, Skill, SkillDefinition, SkillType};
use chrono::Utc;
use regex::Regex;

/// Parse agent configuration from markdown text
/// Supports formats like claude.md files or custom agent definitions
pub fn parse_agent_from_markdown(text: &str) -> Result<Agent, String> {
    let mut agent = Agent::default();

    // Try to parse as YAML frontmatter first
    if text.trim().starts_with("---") {
        if let Some(yaml_end) = text[3..].find("---") {
            let yaml_content = &text[3..yaml_end + 3];
            if let Ok(parsed) = serde_yaml::from_str::<serde_yaml::Value>(yaml_content) {
                return parse_agent_from_yaml_value(&parsed, text);
            }
        }
    }

    // Parse as markdown sections
    let lines: Vec<&str> = text.lines().collect();
    let mut current_section = String::new();
    let mut section_content = String::new();

    for line in lines {
        if line.starts_with("# ") {
            // Main title - agent name
            agent.name = line[2..].trim().to_string();
        } else if line.starts_with("## ") {
            // Save previous section
            if !current_section.is_empty() {
                apply_section_to_agent(&mut agent, &current_section, &section_content);
            }
            current_section = line[3..].trim().to_lowercase();
            section_content = String::new();
        } else if line.starts_with("### ") {
            // Subsection handling
            section_content.push_str(line);
            section_content.push('\n');
        } else {
            section_content.push_str(line);
            section_content.push('\n');
        }
    }

    // Apply last section
    if !current_section.is_empty() {
        apply_section_to_agent(&mut agent, &current_section, &section_content);
    }

    // If no explicit system prompt, use the entire text
    if agent.system_prompt.is_empty() || agent.system_prompt == "You are a helpful AI assistant." {
        agent.system_prompt = text.to_string();
    }

    agent.updated_at = Utc::now();
    Ok(agent)
}

fn apply_section_to_agent(agent: &mut Agent, section: &str, content: &str) {
    let content = content.trim();

    match section {
        "description" | "about" | "overview" => {
            agent.description = content.to_string();
        }
        "personality" | "character" | "traits" => {
            parse_personality_section(agent, content);
        }
        "system prompt" | "instructions" | "prompt" | "system" => {
            agent.system_prompt = content.to_string();
        }
        "skills" | "capabilities" | "abilities" => {
            // Parse skill references
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("- ") || line.starts_with("* ") {
                    // This would be a skill name/reference
                    // Skills are managed separately, but we note them
                }
            }
        }
        "model" | "settings" | "configuration" => {
            parse_settings_section(agent, content);
        }
        "avatar" | "emoji" | "icon" => {
            // Extract emoji from content
            let emoji_re = Regex::new(r"[\p{Emoji}]").unwrap();
            if let Some(m) = emoji_re.find(content) {
                agent.avatar_emoji = m.as_str().to_string();
            }
        }
        _ => {
            // Unknown section - append to system prompt
            if !content.is_empty() {
                agent.system_prompt.push_str("\n\n");
                agent.system_prompt.push_str(&format!("## {}\n", section));
                agent.system_prompt.push_str(content);
            }
        }
    }
}

fn parse_personality_section(agent: &mut Agent, content: &str) {
    let mut personality = Personality::default();

    for line in content.lines() {
        let line = line.trim().to_lowercase();

        // Parse key-value pairs
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "tone" => personality.tone = value.to_string(),
                "verbosity" => personality.verbosity = value.to_string(),
                "creativity" => {
                    if let Ok(v) = value.parse::<f32>() {
                        personality.creativity = v.clamp(0.0, 1.0);
                    }
                }
                "formality" => {
                    if let Ok(v) = value.parse::<f32>() {
                        personality.formality = v.clamp(0.0, 1.0);
                    }
                }
                _ => {}
            }
        }

        // Parse trait lists
        if line.starts_with("- ") || line.starts_with("* ") {
            let trait_name = line[2..].trim().to_string();
            if !trait_name.is_empty() {
                personality.traits.push(trait_name);
            }
        }
    }

    agent.personality = personality;
}

fn parse_settings_section(_agent: &mut Agent, _content: &str) {
    // Settings section now only contains tags
    // Model settings removed as this is a management UI, not a chat client
}

fn parse_agent_from_yaml_value(yaml: &serde_yaml::Value, full_text: &str) -> Result<Agent, String> {
    let mut agent = Agent::default();

    if let serde_yaml::Value::Mapping(map) = yaml {
        for (key, value) in map {
            if let serde_yaml::Value::String(key_str) = key {
                match key_str.as_str() {
                    "name" => {
                        if let serde_yaml::Value::String(v) = value {
                            agent.name = v.clone();
                        }
                    }
                    "description" => {
                        if let serde_yaml::Value::String(v) = value {
                            agent.description = v.clone();
                        }
                    }
                    "avatar" | "emoji" => {
                        if let serde_yaml::Value::String(v) = value {
                            agent.avatar_emoji = v.clone();
                        }
                    }
                    "tags" => {
                        if let serde_yaml::Value::Sequence(tags) = value {
                            agent.tags = tags
                                .iter()
                                .filter_map(|t| {
                                    if let serde_yaml::Value::String(s) = t {
                                        Some(s.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                        }
                    }
                    "personality" => {
                        if let serde_yaml::Value::Mapping(p) = value {
                            parse_yaml_personality(&mut agent.personality, p);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Extract content after YAML frontmatter as system prompt
    if let Some(yaml_end) = full_text[3..].find("---") {
        let content_start = yaml_end + 6; // Skip "---\n"
        if content_start < full_text.len() {
            agent.system_prompt = full_text[content_start..].trim().to_string();
        }
    }

    agent.updated_at = Utc::now();
    Ok(agent)
}

fn parse_yaml_personality(personality: &mut Personality, map: &serde_yaml::Mapping) {
    for (key, value) in map {
        if let serde_yaml::Value::String(key_str) = key {
            match key_str.as_str() {
                "tone" => {
                    if let serde_yaml::Value::String(v) = value {
                        personality.tone = v.clone();
                    }
                }
                "verbosity" => {
                    if let serde_yaml::Value::String(v) = value {
                        personality.verbosity = v.clone();
                    }
                }
                "creativity" => {
                    if let serde_yaml::Value::Number(v) = value {
                        personality.creativity = v.as_f64().unwrap_or(0.5) as f32;
                    }
                }
                "formality" => {
                    if let serde_yaml::Value::Number(v) = value {
                        personality.formality = v.as_f64().unwrap_or(0.5) as f32;
                    }
                }
                "traits" => {
                    if let serde_yaml::Value::Sequence(traits) = value {
                        personality.traits = traits
                            .iter()
                            .filter_map(|t| {
                                if let serde_yaml::Value::String(s) = t {
                                    Some(s.clone())
                                } else {
                                    None
                                }
                            })
                            .collect();
                    }
                }
                _ => {}
            }
        }
    }
}

/// Export agent to markdown format
pub fn export_agent_to_markdown_text(agent: &Agent) -> String {
    let mut output = String::new();

    // YAML frontmatter
    output.push_str("---\n");
    output.push_str(&format!("name: \"{}\"\n", agent.name));
    output.push_str(&format!("description: \"{}\"\n", agent.description));
    output.push_str(&format!("avatar: \"{}\"\n", agent.avatar_emoji));
    if !agent.tags.is_empty() {
        output.push_str("tags:\n");
        for tag in &agent.tags {
            output.push_str(&format!("  - \"{}\"\n", tag));
        }
    }
    output.push_str("personality:\n");
    output.push_str(&format!("  tone: \"{}\"\n", agent.personality.tone));
    output.push_str(&format!("  verbosity: \"{}\"\n", agent.personality.verbosity));
    output.push_str(&format!("  creativity: {}\n", agent.personality.creativity));
    output.push_str(&format!("  formality: {}\n", agent.personality.formality));
    output.push_str("  traits:\n");
    for trait_name in &agent.personality.traits {
        output.push_str(&format!("    - \"{}\"\n", trait_name));
    }
    output.push_str("---\n\n");

    // System prompt as main content
    output.push_str(&agent.system_prompt);

    output
}

// ============================================================================
// Instruction Parsing
// ============================================================================

/// Parse instruction from markdown text
pub fn parse_instruction_from_markdown(text: &str) -> Result<Instruction, String> {
    let mut instruction = Instruction::default();

    // Try to parse as YAML frontmatter first
    if text.trim().starts_with("---") {
        if let Some(yaml_end) = text[3..].find("---") {
            let yaml_content = &text[3..yaml_end + 3];
            if let Ok(parsed) = serde_yaml::from_str::<serde_yaml::Value>(yaml_content) {
                return parse_instruction_from_yaml_value(&parsed, text);
            }
        }
    }

    // Parse as markdown sections
    let lines: Vec<&str> = text.lines().collect();
    let mut content_lines = Vec::new();

    for line in lines {
        if line.starts_with("# ") {
            instruction.name = line[2..].trim().to_string();
        } else {
            content_lines.push(line);
        }
    }

    instruction.content = content_lines.join("\n").trim().to_string();
    instruction.updated_at = Utc::now();
    Ok(instruction)
}

fn parse_instruction_from_yaml_value(yaml: &serde_yaml::Value, full_text: &str) -> Result<Instruction, String> {
    let mut instruction = Instruction::default();

    if let serde_yaml::Value::Mapping(map) = yaml {
        for (key, value) in map {
            if let serde_yaml::Value::String(key_str) = key {
                match key_str.as_str() {
                    "name" => {
                        if let serde_yaml::Value::String(v) = value {
                            instruction.name = v.clone();
                        }
                    }
                    "description" => {
                        if let serde_yaml::Value::String(v) = value {
                            instruction.description = v.clone();
                        }
                    }
                    "icon" | "emoji" => {
                        if let serde_yaml::Value::String(v) = value {
                            instruction.icon_emoji = v.clone();
                        }
                    }
                    "category" => {
                        if let serde_yaml::Value::String(v) = value {
                            instruction.category = match v.to_lowercase().as_str() {
                                "code_style" | "codestyle" => InstructionCategory::CodeStyle,
                                "communication" => InstructionCategory::Communication,
                                "workflow" => InstructionCategory::Workflow,
                                "security" => InstructionCategory::Security,
                                "testing" => InstructionCategory::Testing,
                                "documentation" => InstructionCategory::Documentation,
                                "custom" => InstructionCategory::Custom,
                                _ => InstructionCategory::General,
                            };
                        }
                    }
                    "priority" => {
                        if let serde_yaml::Value::Number(v) = value {
                            instruction.priority = v.as_u64().unwrap_or(5) as u8;
                        }
                    }
                    "tags" => {
                        if let serde_yaml::Value::Sequence(tags) = value {
                            instruction.tags = tags
                                .iter()
                                .filter_map(|t| {
                                    if let serde_yaml::Value::String(s) = t {
                                        Some(s.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                        }
                    }
                    "enabled" => {
                        if let serde_yaml::Value::Bool(v) = value {
                            instruction.enabled = *v;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Extract content after YAML frontmatter
    if let Some(yaml_end) = full_text[3..].find("---") {
        let content_start = yaml_end + 6;
        if content_start < full_text.len() {
            instruction.content = full_text[content_start..].trim().to_string();
        }
    }

    instruction.updated_at = Utc::now();
    Ok(instruction)
}

/// Export instruction to markdown format
pub fn export_instruction_to_markdown_text(instruction: &Instruction) -> String {
    let mut output = String::new();

    // YAML frontmatter
    output.push_str("---\n");
    output.push_str(&format!("name: \"{}\"\n", instruction.name));
    output.push_str(&format!("description: \"{}\"\n", instruction.description));
    output.push_str(&format!("icon: \"{}\"\n", instruction.icon_emoji));
    output.push_str(&format!("category: \"{:?}\"\n", instruction.category));
    output.push_str(&format!("priority: {}\n", instruction.priority));
    output.push_str(&format!("enabled: {}\n", instruction.enabled));
    if !instruction.tags.is_empty() {
        output.push_str("tags:\n");
        for tag in &instruction.tags {
            output.push_str(&format!("  - \"{}\"\n", tag));
        }
    }
    output.push_str("---\n\n");

    // Content
    output.push_str(&instruction.content);

    output
}

/// Parse skill from YAML or JSON text
pub fn parse_skill_from_text(text: &str) -> Result<Skill, String> {
    // Try YAML first
    if let Ok(skill) = serde_yaml::from_str::<Skill>(text) {
        return Ok(skill);
    }

    // Try JSON
    if let Ok(skill) = serde_json::from_str::<Skill>(text) {
        return Ok(skill);
    }

    // Try to parse as simple markdown tool definition
    parse_skill_from_markdown(text)
}

fn parse_skill_from_markdown(text: &str) -> Result<Skill, String> {
    let mut skill = Skill::default();
    let lines: Vec<&str> = text.lines().collect();

    for line in lines {
        let line = line.trim();

        if line.starts_with("# ") {
            skill.name = line[2..].trim().to_string();
        } else if line.starts_with("**Description:**") || line.starts_with("Description:") {
            skill.description = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("**Type:**") || line.starts_with("Type:") {
            let type_str = line.split(':').nth(1).unwrap_or("prompt").trim().to_lowercase();
            skill.skill_type = match type_str.as_str() {
                "tool" => SkillType::Tool,
                "workflow" => SkillType::Workflow,
                _ => SkillType::Prompt,
            };
        }
    }

    // Use remaining content as template for prompt skills
    if matches!(skill.skill_type, SkillType::Prompt) {
        skill.definition = SkillDefinition::Prompt {
            template: text.to_string(),
        };
    }

    skill.updated_at = Utc::now();
    Ok(skill)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_markdown_agent() {
        let md = r#"# Code Assistant

## Description
A helpful coding assistant that excels at writing clean code.

## Personality
Tone: professional
Verbosity: concise
- analytical
- precise

## System Prompt
You are a senior software engineer. Write clean, well-documented code.
"#;

        let agent = parse_agent_from_markdown(md).unwrap();
        assert_eq!(agent.name, "Code Assistant");
        assert!(agent.description.contains("coding assistant"));
    }

    #[test]
    fn test_parse_yaml_frontmatter_agent() {
        let md = r#"---
name: "Creative Writer"
avatar: "✍️"
model: "claude-sonnet-4-20250514"
temperature: 0.9
personality:
  tone: "imaginative"
  creativity: 0.9
  traits:
    - creative
    - eloquent
---

You are a creative writer with a gift for vivid storytelling.
"#;

        let agent = parse_agent_from_markdown(md).unwrap();
        assert_eq!(agent.name, "Creative Writer");
        assert_eq!(agent.avatar_emoji, "✍️");
        assert_eq!(agent.temperature, 0.9);
    }
}
