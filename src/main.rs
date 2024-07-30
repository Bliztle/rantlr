//! # This is a header
//! Some more text
mod ast;
mod error;
mod internal;

use std::{fs, path::PathBuf};

use clap::Parser;

use crate::internal::{lexer::lexer::tokenize, parser::functional::parse};

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args = Cli::parse();
    let content = fs::read_to_string(args.file).expect("Failed to read file");
    println!("{content:?}");

    let tokens = tokenize(&content).expect("Failed to tokenize");
    println!("{tokens:?}");

    let parse_tree = parse(tokens).expect("Failed to parse tree");
    println!("{parse_tree:?}");
}
