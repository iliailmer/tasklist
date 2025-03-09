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
    let mngr = Mngr::new(tasklist_path, Some(project_title));
    match args.command {
        Commands::Add { description } => mngr.add_task(description.unwrap()),
        Commands::Update {
            id,
            status,
            description,
        } => mngr.update_task(id, status, description),
        Commands::Show { kanban } => mngr.list_tasks(kanban),
        Commands::Delete { id } => mngr.delete_task(id),
    }
}
