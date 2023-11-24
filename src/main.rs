//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod files;
mod parser;
use clap::Parser;
use files::{get_files_list, read_file, PythonFile};
use parser::parse_python_files;
use std::path::PathBuf;

use anyhow::Result;

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

    let files: Vec<PythonFile> = get_files_list(args.file, args.recursive).iter().map(read_file).collect();

    let parsed = parse_python_files(files);

    parsed[1].print_ast();
    Ok(())
}
