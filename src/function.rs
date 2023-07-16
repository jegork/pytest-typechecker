use rustpython_ast::{Expr, StmtFunctionDef};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ArgTypeState {
    MissingType,
    IncorrectType,
    CorrectType,
}

#[derive(Debug)]
pub struct FunctionArgs {
    pub(crate) name: String,
    pub(crate) state: ArgTypeState,
}

pub struct CheckedFunction {
    pub(crate) name: String,
    pub(crate) args: Vec<FunctionArgs>,
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

pub fn check_function_arg_types(
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
