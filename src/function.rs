use rustpython_ast::{Expr, StmtFunctionDef};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ArgTypeState {
    MissingType {
        name: String,
    },
    IncorrectType {
        name: String,
        expected: String,
        provided: String,
    },
    CorrectType {
        name: String,
    },
}

pub struct CheckedFunction {
    pub(crate) name: String,
    pub(crate) args: Vec<ArgTypeState>,
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
) -> CheckedFunction {
    let mut checked_args = Vec::new();

    for el in func.args.args.iter() {
        let arg_name = el.def.arg.as_str();

        match fixtures.get(arg_name) {
            Some(fixture) => {
                if el.def.annotation == None {
                    checked_args.push(ArgTypeState::MissingType {
                        name: String::from(arg_name),
                    });
                } else if extract_type(&el.def.annotation) != extract_type(&fixture.returns) {
                    checked_args.push(ArgTypeState::IncorrectType {
                        name: String::from(arg_name),
                        expected: String::from(extract_type(&fixture.returns)),
                        provided: String::from(extract_type(&el.def.annotation)),
                    })
                } else {
                    checked_args.push(ArgTypeState::CorrectType {
                        name: String::from(arg_name),
                    });
                }
            }

            None => {}
        }
    }

    CheckedFunction {
        name: func.name.as_str().parse().unwrap(),
        args: checked_args,
    }
}
