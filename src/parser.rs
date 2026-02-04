use chrono::{Local, NaiveDateTime};
use chrono_english::{Dialect, parse_date_string};
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use crate::{print::print_tasks, storage::TaskStorage};

#[derive(Debug)]
pub enum ParseError {
    DateParseError(String),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Task {
    pub due_date: NaiveDateTime,
    pub name: String,
    pub weight: WorkWeight,
}

impl Task {
    pub fn new(due_date: NaiveDateTime, name: String, weight: WorkWeight) -> Self {
        Self { due_date, name, weight }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortType {
    Alphabetical,
    ClosestToDeadline,
    FurthestFromDeadline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Deserialize, Serialize)]
pub enum WorkWeight {
    UltraLight,
    Light,
    Medium,
    Heavy,
    UltraHeavy,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddArgs),
    List,
    Reset,
}

#[derive(Args)]
pub struct AddArgs {
    /// "392 exam")
    #[arg(index = 1)]
    task: String,

    /// Natural language due date (e.g. "tomorrow", "in 2 days", "Jan 19")
    #[arg(index = 2)]
    due: String,

    /// WorkWeight, amount of work basically
    #[arg(index = 3, value_enum, default_value_t = WorkWeight::Medium)]
    weight: WorkWeight,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A strict, fast task tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn parse_user_date(input: &str) -> Result<NaiveDateTime, String> {
    let now = Local::now();

    parse_date_string(input, now, Dialect::Uk)
        .map(|dt| dt.naive_local())
        .map_err(|_| format!("Could not understand date: '{}'", input))
}

pub fn parse_cli_args() -> Result<(), ParseError>{
    let cli = Cli::parse();
    let mut storage = TaskStorage::new();
    storage.load().expect("Could not load tasks");
    

    match &cli.command {
        Commands::Add(args) => {
            let task = parse_new_task(&args.task, &args.due, args.weight)?;
            storage.add_task(task);
            storage.sort(SortType::ClosestToDeadline);
            storage.store().expect("Could not save tasks");
            print_tasks(&storage.tasks);
            Ok(())
        }
        Commands::List => {
            print_tasks(&storage.tasks);
            Ok(())
        }
        Commands::Reset => {
            storage.clear().expect("Could not clear tasks");
            Ok(())
        },
    }
}

#[instrument]
fn parse_new_task(task: &str, due: &str, weight: WorkWeight) -> Result<Task, ParseError> {
    match parse_user_date(due) {
        Ok(dt) => {
            let timestamp = dt.timestamp();
            debug!("Adding Task: '{}'", task);
            debug!("Due Date:  {} (Timestamp: {})", dt, timestamp);
            debug!("Work Weight: {:?}", weight);
            Ok(Task::new(dt, task.to_owned(), weight))
        }
        Err(e) => Err(ParseError::DateParseError(format!("{:?}", e)))
    }
}
