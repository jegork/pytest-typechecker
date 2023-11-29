use crate::{
    analysis_error::AnalysisError,
    files::parsed_python_file::ParsedPythonFile,
    functions::{get_argument_annotation, get_return_annotation},
};
use rustpython_ast::StmtFunctionDef;
use std::collections::HashMap;

pub fn check_function_arguments(
    func: &StmtFunctionDef,
    fixtures: &HashMap<String, StmtFunctionDef>,
) -> Vec<AnalysisError> {
    let function_name = &func.name;

    let mut errors: Vec<AnalysisError> = Vec::new();

    for arg in func.args.args.iter() {
        let arg_name = arg.def.arg.to_string();
        let arg_annotation = get_argument_annotation(arg);

        match arg_annotation {
            Some(arg_annotation) => {
                let fixture = fixtures.get(&arg_name);
                match fixture {
                    Some(fixture) => {
                        if let Some(fixture_annotation) = get_return_annotation(fixture) {
                            if fixture_annotation != arg_annotation {
                                errors.push(AnalysisError::IncorrectArgumentType {
                                    function_name: function_name.to_string(),
                                    argument_name: arg_name,
                                    expected_type: fixture_annotation,
                                    provided_type: arg_annotation,
                                })
                            }
                        }
                    }
                    None => errors.push(AnalysisError::FixtureDoesNotExist {
                        function_name: function_name.to_string(),
                        argument_name: arg_name,
                    }),
                }
            }
            None => errors.push(AnalysisError::MissingArgumentType {
                function_name: function_name.to_string(),
                argument_name: arg_name,
            }),
        }
    }
    errors
}

pub fn check_file(file: &ParsedPythonFile) -> Vec<AnalysisError> {
    let mut errors = Vec::new();

    for (fixture_name, func) in file.fixtures.iter() {
        if get_return_annotation(func).is_none() {
            errors.push(AnalysisError::FixtureMissingReturnType {
                fixture_name: fixture_name.clone(),
            })
        }
        errors.extend(check_function_arguments(func, &file.fixtures))
    }

    for (_test_case_name, func) in file.test_cases.iter() {
        errors.extend(check_function_arguments(func, &file.fixtures))
    }

    errors
}
