mod cli;
mod manager;
mod task;

use crate::cli::{Cli, Commands};
use crate::manager::Mngr;
use clap::Parser;

use std::fs;
use std::path::PathBuf;

fn get_tasklist_path(custom: Option<String>) -> (String, String) {
    let raw_path = custom.unwrap_or_else(|| ".tasklist".to_string());

    let path_buf = fs::canonicalize(raw_path).unwrap_or_else(|_| PathBuf::from(".tasklist"));
    let path_string = path_buf.to_string_lossy().to_string();

    let title = match path_buf.parent() {
        Some(parent) => parent
            .file_name()
            .map(|os_str| os_str.to_string_lossy().to_string())
            .unwrap_or_else(|| parent.to_string_lossy().to_string()),
        None => ".".to_string(),
    };

    (path_string, title)
}

fn main() {
    let args = Cli::parse();
    let (tasklist_path, project_title) = get_tasklist_path(args.file);

    if args.verbose {
        eprintln!("Using tasklist file: {}", tasklist_path);
    }

    let mngr = Mngr::new(tasklist_path, Some(project_title));

    let result = match args.command {
        Some(Commands::Add { description }) => mngr.add_task(description),
        Some(Commands::Update {
            id,
            status,
            description,
        }) => mngr.update_task(id, status, description),
        Some(Commands::Show { kanban }) => mngr.list_tasks(kanban),
        Some(Commands::Delete { id }) => mngr.delete_task(id),
        None => mngr.list_tasks(args.kanban), // Default: show tasks
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
