mod cli;
mod manager;
mod task;

use crate::cli::{Cli, Commands};
use crate::manager::Mngr;
use clap::Parser;

use std::path::PathBuf;

fn get_tasklist_path(custom: Option<String>) -> (String, String) {
    if let Some(path) = custom {
        let path_buf = PathBuf::from(path);
        let path_string = path_buf.to_string_lossy().to_string();
        let title = match path_buf.parent() {
            Some(parent) => parent
                .file_name()
                .map(|os_str| os_str.to_string_lossy().to_string())
                .unwrap_or_else(|| parent.to_string_lossy().to_string()),
            None => ".".to_string(),
        };
        (path_string, title)
    } else if PathBuf::from(".tasklist").exists() {
        let path_buf = PathBuf::from(".tasklist");
        let path_string = path_buf.to_string_lossy().to_string();
        let title = match path_buf.parent() {
            Some(parent) => parent
                .file_name()
                .map(|os_str| os_str.to_string_lossy().to_string())
                .unwrap_or_else(|| parent.to_string_lossy().to_string()),
            None => String::from("."),
        };
        (path_string, title)
    } else {
        let path_buf = home::home_dir().unwrap().join(".tasklist");
        let title = String::from("My Tasks");
        (path_buf.to_string_lossy().to_string(), title)
    }
}

fn main() {
    let args = Cli::parse();
    let (tasklist_path, project_title) = get_tasklist_path(args.file);
    println!("{}", project_title);
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
