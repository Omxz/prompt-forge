// Prompt Forge - A local agent/skill/instruction management UI with MCP server

mod commands;
pub mod db;
pub mod mcp_server;
mod models;
mod parser;

use commands::*;
use db::Database;
pub use mcp_server::run_mcp_server;
use std::path::PathBuf;
use std::process::Child;
use std::sync::{Arc, Mutex};

/// Application state shared across all Tauri commands
pub struct AppState {
    pub db: Arc<Database>,
    pub db_path: PathBuf,
    pub mcp_running: Mutex<bool>,
    pub mcp_process: Mutex<Option<Child>>,
}

/// Get the default database path for the application
pub fn get_db_path() -> PathBuf {
    let app_data = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.promptforge.app");

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&app_data).ok();

    app_data.join("promptforge.db")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database
    let db_path = get_db_path();
    let db = Database::open(&db_path).expect("Failed to open database");

    // Run migrations
    db.migrate().expect("Failed to run database migrations");

    // Initialize with default data if empty
    db::init_default_data(&db).expect("Failed to initialize default data");

    let app_state = AppState {
        db: Arc::new(db),
        db_path,
        mcp_running: Mutex::new(false),
        mcp_process: Mutex::new(None),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Agent commands
            create_agent,
            get_agents,
            get_agent,
            update_agent,
            delete_agent,
            import_agent_from_text,
            export_agent_to_markdown,
            // Skill commands
            create_skill,
            get_skills,
            get_skill,
            update_skill,
            delete_skill,
            // Instruction commands
            create_instruction,
            get_instructions,
            get_instruction,
            update_instruction,
            delete_instruction,
            import_instruction_from_text,
            export_instruction_to_markdown,
            // Settings commands
            get_settings,
            save_settings,
            // MCP commands
            get_mcp_status,
            start_mcp_server,
            stop_mcp_server,
            // MCP tool helpers
            apply_agent,
            get_all_enabled_instructions,
            // Export/Import commands
            export_all_data,
            import_all_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Prompt Forge");
}
