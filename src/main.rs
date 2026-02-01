mod utils;
mod formats;
mod models;
mod cli;

use std::env;
use crate::cli::app::{execute, parse_options};

fn main() {
    let env:Vec<String> = env::args().collect();
    execute(parse_options(&env));
}
