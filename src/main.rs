//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use clap::Parser;
use std::path::PathBuf;

use anyhow::Result;
use pytest_typechecker::check_and_parse_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Python pytest file or directory for which to check fixture types.
    #[arg(required = true, num_args(1..))]
    file: Vec<PathBuf>,

    /// Check files recursively.
    #[arg(required = false, short, long, default_value_t = false)]
    recursive: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let files = check_and_parse_file(args.file, args.recursive);
    for file in files {
        println!("File: {}", &file.file.filename);
        println!("Errors: {:#?}", file.errors);
        println!();
    }

    Ok(())
}
