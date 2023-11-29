use crate::analysis_error::AnalysisError;
use parsed_python_file::ParsedPythonFile;
use python_file::PythonFile;
use rustpython_ast::{ArgWithDefault, StmtFunctionDef};
use std::{collections::HashMap, fs, path::PathBuf};
pub mod parsed_python_file;
pub mod python_file;

fn get_files_in_a_directory(dir: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in dir.read_dir().unwrap() {
        files.push(entry.unwrap().path())
    }

    files
}

pub fn read_file(file: &PathBuf) -> PythonFile {
    let filename = file
        .as_os_str()
        .to_str()
        .expect("Invalid filename.")
        .to_string();
    let content: String = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => panic!("Unable to read file."),
    };

    PythonFile { content, filename }
}

pub fn get_files_list(provided: Vec<PathBuf>, recursive: bool) -> Vec<PathBuf> {
    provided
        .into_iter()
        .flat_map(|p| {
            if !p.exists() {
                let filename = p.file_name().unwrap().to_str().unwrap();
                panic!("File {} does not exists.", filename)
            } else if recursive && p.is_dir() {
                get_files_list(get_files_in_a_directory(p), recursive)
            } else if p.is_file() {
                vec![p]
            } else {
                [].to_vec()
            }
        })
        .collect()
}

pub fn get_argument_annotation(arg: &ArgWithDefault) -> Option<String> {
    Some(arg.def.annotation.clone()?.as_name_expr()?.id.to_string())
}

pub fn get_return_annotation(func: &StmtFunctionDef) -> Option<String> {
    match &func.returns {
        Some(returns) => Some(returns.as_name_expr()?.id.to_string()),
        None => None,
    }
}

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

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    #[test]
    fn assert_files_list() {
        let base_dir = PathBuf::from("./python-examples");
        let output = get_files_list(vec![base_dir], true);
        let filenames: HashSet<&str> = output
            .iter()
            .map(|p| p.as_os_str().to_str().unwrap())
            .collect();

        assert_eq!(
            filenames,
            HashSet::from([
                "./python-examples/test_sample.py",
                "./python-examples/folder/test_empty.py",
                "./python-examples/test_sample_complex.py",
            ])
        )
    }
}
