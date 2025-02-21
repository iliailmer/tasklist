use clap::Parser;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    description: String, // name or description of the task
}

#[derive(Debug)]
enum Status {
    Done,
    InProgress,
    NotStarted,
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
    status: Status,
    description: String,
}
impl Item {
    fn new(status: Status, description: String) -> Item {
        Item {
            status,
            description,
        }
    }
}

fn main() {
    let args = Args::parse();
    let tasklist = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(".tasklist")
        .expect("Failed to open task list");
    let reader = BufReader::new(&tasklist);
    let item = Item::new(Status::NotStarted, args.description);
    let mut writer = BufWriter::new(&tasklist);
    if item.description.len() > 0 {
        writeln!(writer, "{},{}", item.status, item.description).expect("Failed to write task");
    }
    writer.flush().expect("Failed to flush writer");
}
