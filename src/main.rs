//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod fixture;
mod function;
mod print;

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use crate::fixture::is_fixture;
use crate::function::{check_function_arg_types, CheckedFunction};
use crate::print::pretty_print;
use anyhow::Result;
use rustpython_ast::Stmt;
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Python pytest file for which to check fixture types.
    #[arg(required = true)]
    file: PathBuf,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    if args.file.is_file() == false {
        println!("File does not exist!");
        exit(1);
    }

    let contents = fs::read_to_string(&args.file).expect("Must be a file");
    let mut python_ast = Suite::parse(&contents, &args.file.to_string_lossy()).expect("Error");

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
        test_cases
            .iter()
            .map(|v| CheckedFunction {
                name: String::from(v.0),
                args: check_function_arg_types(&v.1, &fixtures),
            })
            .collect(),
    );

    Ok(())
}
