use crate::task::Status;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(
    version,
    name = "tasklist",
    about = "A to-do list app for command line"
)]
pub struct Cli {
    #[arg(short, long, global = true, help = "Path to .tasklist file.")]
    pub file: Option<String>, // Path to custom tasklist file

    #[command(subcommand)]
    pub command: Commands, // name or description of the task
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Add a new task")]
    #[clap(visible_alias = "a")]
    Add {
        #[arg(short, long)]
        description: Option<String>, // name or description of the task
    },
    #[command(about = "Update an existing task")]
    #[clap(visible_alias = "u")]
    Update {
        #[arg(short, long, help = "ID (index) of the task to update")]
        id: i32,
        #[arg(short, long, help = "New task status")]
        status: Status,
        #[arg(short, long, help = "New description")]
        description: Option<String>,
    },
    #[command(about = "View tasks")]
    #[clap(visible_alias = "ls")]
    Show {},
    #[command(about = "Delete task")]
    #[clap(visible_alias = "rm")]
    Delete {
        #[arg(short, long, help = "ID of task being deleted")]
        id: i32,
    },
}
