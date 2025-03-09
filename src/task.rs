use clap::ValueEnum;
use std::fmt;
use std::io::{self, Write};
#[derive(Debug, Clone, Copy, ValueEnum, Eq, Hash, PartialEq)]
pub enum Status {
    #[value(name = "done")]
    Done,
    #[value(name = "in_progress")]
    InProgress,
    #[value(name = "not_started")]
    NotStarted,
}

#[allow(dead_code)]
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

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub status: Status,
    pub description: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<4} | {:<14} | {:<256}",
            self.id,                // Right-align ID, 3 chars
            self.status.as_label(), // Left-align status, 15 chars
            self.description,       // Description follows
        )
    }
}

impl Task {
    pub fn new(id: i32, status: Status, description: String) -> Task {
        Task {
            id,
            status,
            description,
        }
    }
    pub fn to_file_string(&self) -> String {
        format!(
            "{},{},{}",
            self.id,
            self.status.as_label(),
            self.description
        )
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "{}", self.to_file_string())
    }
}
