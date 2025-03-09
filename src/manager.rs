use colored::Colorize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::task::{Status, Task};
#[derive(Debug)]
pub struct Mngr {
    tasklist_path: String,
    title: Option<String>,
}

impl Mngr {
    pub fn new(tasklist_path: String, title: Option<String>) -> Self {
        Self {
            tasklist_path,
            title,
        }
    }

    pub fn add_task(&self, description: String) {
        let tasklist = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&self.tasklist_path)
            .expect("Failed to open task list");

        let reader = BufReader::new(&tasklist);
        let mut max_id = 0;
        for line in reader.lines() {
            if let Some(id_str) = line.unwrap().split(",").next() {
                if let Ok(id) = id_str.parse::<i32>() {
                    max_id = max_id.max(id);
                }
            }
        }
        let task = Task::new(max_id + 1, Status::NotStarted, description);
        let mut writer = BufWriter::new(&tasklist);
        if !task.description.is_empty() {
            task.write_to(&mut writer).expect("Failed to write task");
        }
        writer.flush().expect("Failed to flush writer");
        println!("{} {}", "Added task:".green(), format!("{}", task).yellow());
    }

    pub fn update_task(&self, id: i32, status: Status, description: Option<String>) {
        // read the .tasklist file
        // process each line. if the target task id is found, edit it by creating new task.
        let tasklist = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&self.tasklist_path)
            .expect("Failed to open task list");
        let reader = BufReader::new(&tasklist);
        let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        for line in lines.iter_mut() {
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() >= 3 && parts[0].parse::<i32>().unwrap() == id {
                let new_description = description.unwrap_or(parts[2].to_string());
                let task = Task::new(id, status, new_description);
                *line = task.to_file_string();
                println!("Updated task: {}", task);
                break; // stop iterations
            }
        }
        // write new .tasklist file
        let new_tasklist = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.tasklist_path)
            .expect("Failed to open task list");
        let mut writer = BufWriter::new(&new_tasklist);
        for line in lines {
            writer
                .write_all(line.as_bytes())
                .expect("Failed to write task");
        }
        writer.flush().expect("Failed to flush writer");
    }

    pub fn list_tasks(&self, kanban: bool) {
        let tasklist = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&self.tasklist_path)
            .expect("Failed to open task list");
        let reader = BufReader::new(&tasklist);

        if !kanban {
            println!(
                "Project: {}",
                self.title.as_ref().unwrap_or(&String::from("My Tasks"))
            );
            println!("{}", "-".repeat(64));
            for line in reader.lines() {
                let line = line.unwrap();
                let parts: Vec<&str> = line.split(",").collect();
                if parts.len() >= 3 {
                    let id = parts[0].parse::<i32>().unwrap();
                    let status = Status::from_str(parts[1]);
                    let task = Task::new(id, status, parts[2].to_string());
                    println!("{}", task);
                }
            }
        } else {
            let mut board: HashMap<Status, Vec<(i32, String)>> = HashMap::new();
            for status in &[Status::NotStarted, Status::Done, Status::InProgress] {
                board.insert(*status, Vec::new());
            }
            for line in reader.lines() {
                let line = line.unwrap();
                let parts: Vec<&str> = line.split(",").collect();
                if parts.len() >= 3 {
                    let id = parts[0].parse().unwrap();
                    let status = Status::from_str(parts[1]);
                    let description = parts[2].trim().to_string();
                    board.get_mut(&status).unwrap().push((id, description));
                }
            }
            let max_len = board.values().map(|v| v.len()).max().unwrap_or(0);
            println!(
                "Project: {}",
                self.title.as_ref().unwrap_or(&String::from("My Tasks"))
            );
            println!("{}", "-".repeat(64));
            println!(
                "{:<20} | {:<20} | {:<20}",
                Status::NOT_STARTED_LABEL,
                Status::IN_PROGRESS_LABEL,
                Status::DONE_LABEL
            );
            println!("{}", "-".repeat(64));
            for i in 0..max_len {
                for status in &[Status::NotStarted, Status::InProgress, Status::Done] {
                    if let Some(tasks) = board.get(status) {
                        if i < tasks.len() {
                            print!("{} {:<20} | ", tasks[i].0, tasks[i].1);
                        } else {
                            print!("{:20} | ", "");
                        }
                    }
                }
                println!();
            }
        }
    }

    pub fn delete_task(&self, id: i32) {
        let tasklist = OpenOptions::new()
            .read(true)
            .open(&self.tasklist_path)
            .expect("Failed to open task list");
        let reader = BufReader::new(&tasklist);
        let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let initial_len = lines.len();
        lines.retain(|l| {
            let parts: Vec<&str> = l.split(",").collect();
            parts[0].parse::<i32>().unwrap() != id
        });
        let new_tasklist = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.tasklist_path)
            .expect("Failed to open task list");
        let mut writer = BufWriter::new(&new_tasklist);
        for line in lines.iter_mut() {
            writer
                .write_all(line.as_bytes())
                .expect("Failed to write task");
            writer.write_all(b"\n").expect("Failed to write newline");
        }
        writer.flush().expect("Failed to flush writer");
        if initial_len > lines.len() {
            println!("{}", format!("Deleted task with ID {}", id).yellow());
        } else {
            println!("{}", format!("No task found with ID {}", id).red());
        }
    }
}
