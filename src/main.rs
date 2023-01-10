#[macro_use]
extern crate log;

mod cli;
mod lexer;
mod monomial;
mod parser;
mod polynomial;
use crate::cli::run_cli;

fn main() {
    env_logger::init();
    run_cli();
}
