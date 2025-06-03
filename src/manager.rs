use crate::task::{Status, Task};
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use tabled::{Table, settings::Style};
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

    pub fn add_task(&self, description: String) -> Result<(), Error> {
        let tasklist = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&self.tasklist_path)
            .map_err(|e| {
                Error::new(
                    e.kind(),
                    format!("Could not open {}: {}", self.tasklist_path, e),
                )
            })?;
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
            task.write_to(&mut writer).map_err(|e| {
                Error::new(
                    e.kind(),
                    format!(
                        "Could not write description to {}: {}",
                        self.tasklist_path, e
                    ),
                )
            })?;
        }
        writer.flush().expect("Failed to flush writer");
        println!("{} {}", "Added task:".green(), format!("{}", task).yellow());
        Ok(())
    }

    pub fn update_task(
        &self,
        id: i32,
        status: Status,
        description: Option<String>,
    ) -> Result<(), Error> {
        // read the .tasklist file
        // process each line. if the target task id is found, edit it by creating new task.
        let tasklist = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&self.tasklist_path)
            .map_err(|e| {
                Error::new(
                    e.kind(),
                    format!("Could not open task list {}: {}", self.tasklist_path, e),
                )
            })?;
        let reader = BufReader::new(&tasklist);
        let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        for line in lines.iter_mut() {
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() >= 3 && parts[0].parse::<i32>().unwrap() == id {
                let new_description = String::from(description.as_deref().unwrap_or(parts[2]));
                let task = Task::new(id, status, new_description);
                *line = task.to_file_string();
                println!(
                    "{} {}",
                    "Updated task:".green(),
                    format!("{}", task).yellow()
                );
            } else {
                continue;
            }
        }
        // write new .tasklist file
        let new_tasklist = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.tasklist_path)
            .map_err(|e| {
                Error::new(
                    e.kind(),
                    format!("Could not update task list {}: {}", self.tasklist_path, e),
                )
            })?;
        let mut writer = BufWriter::new(&new_tasklist);
        for line in lines {
            writer.write_all(line.as_bytes())?;
        }
        writer.flush()?;

        Ok(())
    }

    pub fn list_tasks(&self) -> Result<(), Error> {
        let tasklist = OpenOptions::new()
            .read(true)
            .open(&self.tasklist_path)
            .map_err(|e| {
                Error::new(
                    e.kind(),
                    format!("Could not update task list {}: {}", self.tasklist_path, e),
                )
            })?;
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
                let task = Task::new(id, status, parts[2].to_string());
                tasks.push(task);
            }
        }
        let builder = Table::builder(tasks).index().column(0).name(None);
        let mut table = builder.build();
        table.with(Style::modern());
        println!("{}", table);
        Ok(())
    }

    pub fn delete_task(&self, id: i32) -> Result<(), Error> {
        let tasklist = OpenOptions::new().read(true).open(&self.tasklist_path)?;
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
            .open(&self.tasklist_path)?;
        let mut writer = BufWriter::new(&new_tasklist);
        for line in lines.iter_mut() {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;
        if initial_len > lines.len() {
            println!("{}", format!("Deleted task with ID {}", id).yellow());
        } else {
            println!("{}", format!("No task found with ID {}", id).red());
        }
        Ok(())
    }
}
