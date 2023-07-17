//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod fixture;
mod function;
mod print;

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::iter::Product;
use std::path::PathBuf;
use std::process::exit;

use crate::fixture::is_fixture;
use crate::function::check_function_arg_types;
use crate::print::pretty_print;
use anyhow::Result;
use rustpython_ast::{Stmt};
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;

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
            .map(|v| check_function_arg_types(&v.1, &fixtures))
            .collect(),
    );

    Ok(())
}

fn parse_dir(file: PathBuf, recursive: bool) -> Result<()> {
    for entry in fs::read_dir(file)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            parse_file(entry_path)?
        } else if entry_path.is_dir() && recursive == true {
            return parse_dir(entry_path, recursive);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    for file in args.file {
        if file.is_dir() {
            parse_dir(file, args.recursive.clone())?
        } else if file.is_file() {
            parse_file(file)?
        } else {
            println!("File {} does not exist!", file.to_str().unwrap());
            exit(1);
        }
    }


    Ok(())
}
