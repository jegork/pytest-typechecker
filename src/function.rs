use rustpython_ast::Expr::Name;
use rustpython_ast::{Expr, Ranged, StmtFunctionDef};
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
        line: usize
    },
    CorrectType {
        name: String,
    },
    Error {
        name: String,
        msg: String,
        line: usize
    },
}

pub struct CheckedFunction {
    pub(crate) name: String,
    pub(crate) args: Vec<ArgTypeState>,
}

fn get_line_number(value: &String, at: usize) -> usize {
    return bytecount::count(&value[..at].as_bytes(), b'\n') + 1;
}

fn extract_type(el: &Expr) -> Option<String> {
    match el {
        Expr::Name(expr_name) => {
            return Some(String::from(expr_name.id.as_str()));
        }
        Expr::Tuple(expr_tuple) => {
            let tuple: Vec<String> = expr_tuple
                .elts
                .iter()
                .filter_map(|x| extract_type(x))
                .collect();
            return Some(tuple.join(", "));
        }
        Expr::Subscript(expr_subscript) => {
            if let Name(expr_name) = &expr_subscript.value.as_ref() {
                return Some(format!(
                    "{}[{}]",
                    expr_name.id,
                    extract_type(&expr_subscript.slice).unwrap()
                ));
            }
        }
        _ => {
            panic!("Unsupported type! {:#?}", el)
        }
    }

    return None;
}

pub fn check_function_arg_types(
    file_contents: &String,
    func: &StmtFunctionDef,
    fixtures: &HashMap<String, &mut StmtFunctionDef>,
) -> CheckedFunction {
    let checked_args = func
        .args
        .args
        .iter()
        .filter_map(|el| {
            let arg_name = el.def.arg.as_str();

            match fixtures.get(arg_name) {
                Some(fixture) => match (&fixture.returns, &el.def.annotation) {
                    (Some(fixture_return_value), Some(annotation_value)) => {
                        let provided = extract_type(annotation_value);
                        let expected = extract_type(fixture_return_value);

                        match (expected, provided) {
                            (Some(expected_value), Some(provided_value)) => {
                                if expected_value != provided_value {
                                    Some(ArgTypeState::IncorrectType {
                                        name: String::from(arg_name),
                                        expected: expected_value,
                                        provided: provided_value,
                                        line: get_line_number(file_contents, annotation_value.start().to_usize())
                                    })
                                } else {
                                    Some(ArgTypeState::CorrectType {
                                        name: String::from(arg_name),
                                    })
                                }
                            }
                            _ => panic!("Unsupported type"),
                        }
                    }
                    (None, Some(annotation_value)) => Some(ArgTypeState::Error {
                        msg: format!("Fixture {} is missing return type!", fixture.name.as_str()),
                        name: String::from(arg_name),
                        line: get_line_number(file_contents, annotation_value.start().to_usize())
                    }),

                    (Some(_), None) => Some(ArgTypeState::MissingType {
                        name: String::from(arg_name),
                    }),
                    (None, None) => None,
                },
                None => None,
            }
        })
        .collect();

    CheckedFunction {
        name: func.name.as_str().parse().unwrap(),
        args: checked_args,
    }
}
