//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod fixture;
mod function;
mod print;

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::fixture::is_fixture;
use crate::function::check_function_arg_types;
use crate::print::pretty_print;
use anyhow::Result;
use rustpython_ast::Stmt;
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;
use walkdir::WalkDir;

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

fn parse_file(file: PathBuf) -> Result<()> {
    let contents = fs::read_to_string(&file)?;
    let mut python_ast = Suite::parse(&contents, &file.to_string_lossy())?;

    let mut fixtures = HashMap::new();
    let mut test_cases = HashMap::new();

    for mut el in python_ast.iter_mut() {
        match el {
            Stmt::FunctionDef(v) => {
                let function_name: String = v.name.as_str().parse().unwrap();

                if function_name.starts_with("test_") {
                    test_cases.insert(function_name, v);
                    continue;
                }

                if is_fixture(&v) {
                    fixtures.insert(function_name, v);
                    continue;
                }
            }

            _ => {}
        }
    }

    pretty_print(
        file.file_name().unwrap(),
        test_cases
            .iter()
            .map(|v| check_function_arg_types(&contents, &v.1, &fixtures))
            .collect(),
    );

    Ok(())
}

fn get_files_list(provided: Vec<PathBuf>, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut total = Vec::new();
    for f in provided {
        if f.exists() == false {
            println!("File {} does not exist!", f.display());
            exit(1);
        }

        if f.is_file() {
            total.push(f.clone());
        } else if f.is_dir() {
            if recursive == false {
                for entry in fs::read_dir(f)? {
                    let entry = entry?;
                    let entry_path = entry.path();

                    if entry_path.is_file() {
                        total.push(entry_path);
                    }
                }
            } else {
                for entry in WalkDir::new(f).max_depth(10) {
                    let entry = entry?;
                    let entry_path: &Path = entry.path();

                    if entry_path.is_file() {
                        total.push(entry_path.to_path_buf());
                    }
                }
            }
        }

    }

    total.sort_unstable();
    total.dedup();

    return Ok(total);
}

fn main() -> Result<()> {
    let args = Args::parse();

    for file in get_files_list(args.file, args.recursive)? {
        parse_file(file)?
    }

    Ok(())
}
