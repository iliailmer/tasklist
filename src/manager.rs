use crate::task::{Status, Task};
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use tabled::{Table, settings::Style};
use textwrap::fill;
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
            task.write_to(&mut writer)
                .expect("Failed to write task: empty description");
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
                let new_description =
                    String::from(description.unwrap_or(parts[2].to_string()).as_str());
                let task = Task::new(id, status, new_description);
                *line = task.to_file_string();
                println!(
                    "{} {}",
                    "Updated task:".green(),
                    format!("{}", task).yellow()
                );
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

    pub fn list_tasks(&self) {
        let tasklist = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&self.tasklist_path)
            .expect("Failed to open task list");
        let reader = BufReader::new(&tasklist);
        println!(
            "Project: {}",
            self.title.as_ref().unwrap_or(&String::from("My Tasks"))
        );
        let mut tasks: Vec<Task> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() >= 3 {
                let id = parts[0].parse::<i32>().unwrap();
                let status = Status::from_str(parts[1]);
                let task = Task::new(id, status, fill(parts[2].to_string().as_str(), 50));
                // println!("{}", task);
                tasks.push(task);
            }
        }
        let builder = Table::builder(tasks).index().column(0).name(None);
        let mut table = builder.build(); //Table::new(tasks).to_string();
        table.with(Style::modern());
        println!("{}", table);
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
