
use tracing::error;

use crate::parser::parse_cli_args;

mod parser;
mod print;
mod storage;


fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    match parse_cli_args() {
        Ok(_) => (),
        Err(e) => error!("{:?}", e)
    }
}

