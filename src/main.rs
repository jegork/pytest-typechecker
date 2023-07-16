//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
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

fn extract_type(el: &Option<Box<Expr>>) -> &str {
    match &el {
        None => return "None",

        Some(t) => match t.as_ref() {
            Expr::Name(expr_name) => {
                return expr_name.id.as_str();
            }
            _ => {}
        },

        _ => {}
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

fn print_function_types_state(name: &String, states: Vec<FunctionArgs>) {
    let valid_args: Vec<&FunctionArgs> = states
        .iter()
        .filter(|v| v.state == ArgTypeState::CorrectType)
        .collect();

    if valid_args.len() == states.len() {
        println!("Func: {} is correct", name);
        return;
    }

    println!("Func {} is incorrect", name);
    for arg in states {
        if arg.state == ArgTypeState::MissingType {
            println!("Arg: {} is missing type", arg.name);
        } else if arg.state == ArgTypeState::IncorrectType {
            println!("Arg {} is incorrect", arg.name);
        }
    }
}

fn main() -> Result<(), ()> {
    let file = PathBuf::from("test_sample.py");
    let contents = fs::read_to_string(&file).expect("Must be a file");
    let mut python_ast = Suite::parse(&contents, &file.to_string_lossy()).expect("Error");

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

    for f in test_cases {
        print_function_types_state(&f.0, check_function_arg_types(&f.1, &fixtures))
    }
    Ok(())
}
