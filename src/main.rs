//TODO: finish two commands: add and update
#[allow(dead_code)]
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(Parser, Debug)]
#[command(
    version,
    name = "todolist",
    about = "A to-do list app for command line"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands, // name or description of the task
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(visible_alias = "a")]
    Add {
        #[arg(short, long)]
        description: Option<String>, // name or description of the task
    },
    #[clap(visible_alias = "u")]
    Update {
        #[arg(short, long)]
        id: i32,
        #[arg(short, long)]
        status: Status,
        #[arg(short, long)]
        description: Option<String>,
    },
    #[clap(visible_alias = "ls")]
    Show {
        #[arg(short, long)]
        kanban: bool,
    },
    #[clap(visible_alias = "rm")]
    Delete {
        #[arg(short, long)]
        id: i32,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum, Eq, Hash, PartialEq)]
enum Status {
    #[value(name = "done")]
    Done,
    #[value(name = "in_progress")]
    InProgress,
    #[value(name = "not_started")]
    NotStarted,
}

impl Status {
    pub const DONE_LABEL: &'static str = "✅ Done";
    pub const IN_PROGRESS_LABEL: &'static str = "⏳ In Progress";
    pub const NOT_STARTED_LABEL: &'static str = "🚀 Not Started";

    pub fn from_str(s: &str) -> Self {
        match s {
            Self::DONE_LABEL => Status::Done,
            Self::IN_PROGRESS_LABEL => Status::InProgress,
            Self::NOT_STARTED_LABEL => Status::NotStarted,
            _ => Status::NotStarted,
        }
    }

    pub fn as_label(&self) -> &'static str {
        match self {
            Status::Done => Self::DONE_LABEL,
            Status::InProgress => Status::IN_PROGRESS_LABEL,
            Status::NotStarted => Status::NOT_STARTED_LABEL,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Done => write!(f, "✅ Done"),
            Status::InProgress => write!(f, "⏳ In Progress"),
            Status::NotStarted => write!(f, "🚀 Not Started"),
        }
    }
}

#[derive(Debug)]
struct Item {
    id: i32,
    status: Status,
    description: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<4} | {:<14} | {:<256}",
            self.id,          // Right-align ID, 3 chars
            self.status,      // Left-align status, 15 chars
            self.description, // Description follows
        )
    }
}

impl Item {
    fn new(id: i32, status: Status, description: String) -> Item {
        Item {
            id,
            status,
            description,
        }
    }
    fn to_file_string(&self) -> String {
        format!("{},{},{}", self.id, self.status, self.description)
    }

    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "{}", self.to_file_string())
    }
}

fn add_item(description: String) {
    let tasklist = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(".tasklist")
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
    let item = Item::new(max_id + 1, Status::NotStarted, description);
    let mut writer = BufWriter::new(&tasklist);
    if !item.description.is_empty() {
        item.write_to(&mut writer).expect("Failed to write task");
    }
    writer.flush().expect("Failed to flush writer");
    println!("{} {}", "Added task:".green(), format!("{}", item).yellow());
}

fn update_item(id: i32, status: Status, description: Option<String>) {
    // read the .tasklist file
    // process each line. if the target task id is found, edit it by creating new item.
    let tasklist = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(".tasklist")
        .expect("Failed to open task list");
    let reader = BufReader::new(&tasklist);
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for line in lines.iter_mut() {
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() >= 3 && parts[0].parse::<i32>().unwrap() == id {
            let new_description = description.unwrap_or(parts[2].to_string());
            let item = Item::new(id, status, new_description);
            *line = item.to_file_string();
            println!("Updated task: {}", item);
            break; // stop iterations
        }
    }
    // write new .tasklist file
    let new_tasklist = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(".tasklist")
        .expect("Failed to open task list");
    let mut writer = BufWriter::new(&new_tasklist);
    for line in lines {
        writer
            .write_all(line.as_bytes())
            .expect("Failed to write task");
    }
    writer.flush().expect("Failed to flush writer");
}

fn list_items(kanban: bool) {
    let tasklist = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(".tasklist")
        .expect("Failed to open task list");
    let reader = BufReader::new(&tasklist);

    if !kanban {
        println!("{}", "-".repeat(64));
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() >= 3 {
                let id = parts[0].parse::<i32>().unwrap();
                let status = Status::from_str(parts[1]);
                let item = Item::new(id, status, parts[2].to_string());
                println!("{}", item);
            }
        }
    } else {
        let mut board: HashMap<Status, Vec<String>> = HashMap::new();
        for status in &[Status::NotStarted, Status::Done, Status::InProgress] {
            board.insert(*status, Vec::new());
        }
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() >= 3 {
                let status = Status::from_str(parts[1]);
                let description = parts[2].trim().to_string();
                board.get_mut(&status).unwrap().push(description);
            }
        }
        let max_len = board.values().map(|v| v.len()).max().unwrap_or(0);
        println!("{}", "-".repeat(64));
        println!(
            "{:<20} | {:<20} | {:<20}",
            Status::NOT_STARTED_LABEL,
            Status::IN_PROGRESS_LABEL,
            Status::DONE_LABEL
        );
        println!("{}", "-".repeat(64));
        for i in 0..max_len {
            for status in &[Status::NotStarted, Status::Done, Status::InProgress] {
                if let Some(tasks) = board.get(status) {
                    if i < tasks.len() {
                        print!("{:<20} | ", tasks[i]);
                    } else {
                        print!("{:20} | ", "");
                    }
                }
            }
            println!();
        }
    }
}

fn delete_item(id: i32) {
    let tasklist = OpenOptions::new()
        .read(true)
        .open(".tasklist")
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
        .open(".tasklist")
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

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Add { description } => add_item(description.unwrap()),
        Commands::Update {
            id,
            status,
            description,
        } => update_item(id, status, description),
        Commands::Show { kanban } => list_items(kanban),
        Commands::Delete { id } => delete_item(id),
    }
}
