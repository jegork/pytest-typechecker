//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use anyhow::Result;
use prettytable::{row, Table};
use rustpython_ast::Expr::Attribute;
use rustpython_ast::{Expr, Stmt, StmtFunctionDef};
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;

fn is_fixture(func: &StmtFunctionDef) -> bool {
    if func.decorator_list.len() > 0 {
        for dec in &func.decorator_list {
            match &dec.expression {
                Expr::Call(call) => match call.func.as_ref() {
                    Attribute(atr) => {
                        let name = atr.attr.as_str();

                        if name == "fixture" {
                            return true;
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
        }
    }

    false
}

#[derive(Debug, PartialEq, Eq)]
enum ArgTypeState {
    MissingType,
    IncorrectType,
    CorrectType,
}

#[derive(Debug)]
struct FunctionArgs {
    name: String,
    state: ArgTypeState,
}

struct CheckedFunction {
    name: String,
    args: Vec<FunctionArgs>,
}

fn extract_type(el: &Option<Box<Expr>>) -> &str {
    match &el {
        None => return "None",

        Some(t) => match t.as_ref() {
            Expr::Name(expr_name) => {
                return expr_name.id.as_str();
            }
            _ => {}
        },
    }

    return "undefined";
}

fn check_function_arg_types(
    func: &StmtFunctionDef,
    fixtures: &HashMap<String, &mut StmtFunctionDef>,
) -> Vec<FunctionArgs> {
    let mut checked_args = Vec::new();

    for el in func.args.args.iter() {
        let arg_name = el.def.arg.as_str();

        match fixtures.get(arg_name) {
            Some(fixture) => {
                if el.def.annotation == None {
                    checked_args.push(FunctionArgs {
                        name: String::from(arg_name),
                        state: ArgTypeState::MissingType,
                    })
                } else if extract_type(&el.def.annotation) != extract_type(&fixture.returns) {
                    checked_args.push(FunctionArgs {
                        name: String::from(arg_name),
                        state: ArgTypeState::IncorrectType,
                    })
                } else {
                    checked_args.push(FunctionArgs {
                        name: String::from(arg_name),
                        state: ArgTypeState::CorrectType,
                    })
                }
            }

            None => {}
        }
    }

    checked_args
}

fn pretty_print(funcs: Vec<CheckedFunction>) {
    let mut table = Table::new();

    table.set_titles(row!["Name", "Invalid args"]);

    for f in funcs {
        let s: Vec<String> = f
            .args
            .iter()
            .filter(|v| v.state != ArgTypeState::CorrectType)
            .map(|v| {
                return if v.state == ArgTypeState::IncorrectType {
                    format!("{}: incorrect type", v.name)
                } else {
                    format!("{}: missing type", v.name)
                };
            })
            .collect();
        table.add_row(row![f.name, s.join("\n")]);
    }

    if table.len() > 0 {
        table.printstd();
        exit(1);
    } else {
        println!("All types are correct!");
    }
}

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
