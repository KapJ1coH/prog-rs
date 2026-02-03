use chrono::{Local, NaiveDateTime};
use chrono_english::{Dialect, parse_date_string};
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use rkyv::{Archive, Deserialize, Serialize};

mod parser;
mod print;
mod storage;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// #[rkyv(
//     // This will generate a PartialEq impl between our unarchived
//     // and archived types
//     compare(PartialEq),
//     // Derives can be passed through to the generated type:
//     derive(Debug),
// )]
struct Task {
    due_date: NaiveDateTime,
    name: String,
    option: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum SortType {
    Alphabetical,
    ClosestToDeadline,
    FurthestFromDeadline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum WorkWeight {
    UltraLight,
    Light,
    Medium,
    Heavy,
    UltraHeavy,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    List,
}

#[derive(Args)]
struct AddArgs {
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            let raw_date = &args.due;

            match parse_user_date(raw_date) {
                Ok(dt) => {
                    let timestamp = dt.timestamp();
                    println!("Adding Task: '{}'", args.task);
                    println!("Due Date:  {} (Timestamp: {})", dt, timestamp);
                    println!("Work Weight: {:?}", args.weight);

                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::List => {
            todo!()
        }
    }
}

// fn main() {
//     tracing_subscriber::fmt::init();

//     let value = Task {
//         int: 42,
//         string: "hello world".to_string(),
//         option: Some(vec![1, 2, 3, 4]),
//     };

//     // Serializing is as easy as a single function call
//     let _bytes = rkyv::to_bytes::<Error>(&value).unwrap();

//     println!("Bytes: {:?}", _bytes);

//     // Or you can customize your serialization for better performance or control
//     // over resource usage
//     use rkyv::{api::high::to_bytes_with_alloc, ser::allocator::Arena};

//     let mut arena = Arena::new();
//     let bytes =
//         to_bytes_with_alloc::<_, Error>(&value, arena.acquire()).unwrap();

//     println!("Bytes: {:?}", bytes);

//     // You can use the safe API for fast zero-copy deserialization
//     let archived = rkyv::access::<ArchivedTask, Error>(&bytes[..]).unwrap();
//     assert_eq!(archived, &value);

//     println!("Archived: {:?}", archived);

//     // Or you can use the unsafe API for maximum performance
//     let archived =
//         unsafe { rkyv::access_unchecked::<ArchivedTask>(&bytes[..]) };
//     assert_eq!(archived, &value);

//     println!("Archived (unchecked): {:?}", archived);

//     // And you can always deserialize back to the original type
//     let deserialized = deserialize::<Task, Error>(archived).unwrap();
//     assert_eq!(deserialized, value);

//     println!("Deserialized: {:?}", deserialized);
// }
