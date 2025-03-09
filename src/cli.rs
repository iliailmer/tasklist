use crate::task::Status;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(
    version,
    name = "todolist",
    about = "A to-do list app for command line"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands, // name or description of the task
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
