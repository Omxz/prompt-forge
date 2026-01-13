// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check for --mcp flag to run as MCP server
    if args.iter().any(|arg| arg == "--mcp" || arg == "-m") {
        // Parse --db-path argument
        let db_path = args
            .iter()
            .position(|arg| arg == "--db-path")
            .and_then(|i| args.get(i + 1))
            .map(PathBuf::from)
            .unwrap_or_else(|| prompt_forge_lib::get_db_path());

        prompt_forge_lib::run_mcp_server(db_path);
    } else {
        prompt_forge_lib::run();
    }
}
