//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod files;
mod functions;
mod nodes;
mod analysis_error;
use crate::files::check_file;
use clap::Parser;
use files::{get_files_list, read_file, parsed_python_file::ParsedPythonFile, python_file::PythonFile};
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

    let files: Vec<ParsedPythonFile> = get_files_list(args.file, args.recursive)
        .iter()
        .map(read_file)
        .map(PythonFile::parse)
        .map(|f| {
            let errors = check_file(&f);
            ParsedPythonFile { errors, ..f }
        })
        .collect();

    for file in files {
        println!("File: {}", &file.file.filename);
        println!("Errors: {:#?}", check_file(&file));
        println!();
    }

    Ok(())
}
