use crate::analysis_error::AnalysisError;
use parsed_python_file::ParsedPythonFile;
use python_file::PythonFile;
use rustpython_ast::{ArgWithDefault, Expr};
use std::{fs, path::PathBuf};
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

pub fn get_return_annotation(arg: &Expr) -> Option<String> {
    Some(arg.as_name_expr()?.id.to_string())
}

pub fn check_file(file: &ParsedPythonFile) -> Vec<AnalysisError> {
    let mut errors = Vec::new();

    for (fixture_name, func) in file.fixtures.iter() {
        let annotation = match &func.returns {
            Some(returns) => get_return_annotation(returns),
            None => None,
        };

        match annotation {
            Some(_v) => {}
            None => errors.push(AnalysisError::FixtureMissingReturnType {
                fixture_name: fixture_name.clone(),
            }),
        }
    }

    for (test_case_name, func) in file.test_cases.iter() {
        for arg in func.args.args.iter() {
            let arg_name = arg.def.arg.to_string();
            let arg_annotation = get_argument_annotation(arg);

            match arg_annotation {
                Some(arg_annotation) => {
                    let fixture = file.fixtures.get(&arg_name);
                    match fixture {
                        Some(fixture) => {
                            if let Some(returns) = &fixture.returns {
                                if let Some(fixture_annotation) = get_return_annotation(returns) {
                                    if fixture_annotation != arg_annotation {
                                        errors.push(AnalysisError::IncorrectArgumentType {
                                            test_case_name: test_case_name.clone(),
                                            argument_name: arg_name,
                                            expected_type: fixture_annotation,
                                            provided_type: arg_annotation,
                                        })
                                    }
                                }
                            }
                        }
                        None => errors.push(AnalysisError::FixtureDoesNotExist {
                            test_case_name: test_case_name.clone(),
                            argument_name: arg_name,
                        }),
                    }
                }
                None => errors.push(AnalysisError::MissingArgumentType {
                    test_case_name: test_case_name.clone(),
                    argument_name: arg_name,
                }),
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn assert_files_list() {
        let base_dir = PathBuf::from("./python-examples");
        let output = get_files_list(vec![base_dir], true);
        let filenames: Vec<&str> = output
            .iter()
            .map(|p| p.as_os_str().to_str().unwrap())
            .collect();

        assert_eq!(
            filenames,
            vec![
                "./python-examples/test_sample.py",
                "./python-examples/folder/test_empty.py",
                "./python-examples/test_sample_complex.py",
            ]
        )
    }
}
