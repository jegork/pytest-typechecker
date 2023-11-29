use crate::analysis_error::AnalysisError;
use crate::files::parsed_python_file::ParsedPythonFile;
use crate::functions::{get_fixtures_mapping, get_functions, get_test_cases};
use rustpython_parser_core::Mode;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PythonFile {
    pub content: String,
    pub filename: String,
}

impl PythonFile {
    pub fn parse(file: PythonFile) -> ParsedPythonFile {
        let parsed = rustpython_parser::parse(&file.content, Mode::Module, &file.filename);

        return match parsed {
            Err(_err) => ParsedPythonFile {
                file,
                errors: vec![AnalysisError::UnparsableFile],
                fixtures: HashMap::new(),
                test_cases: HashMap::new(),
            },
            Ok(ast) => {
                let ast = ast.as_module().unwrap().clone();
                let functions = get_functions(&ast.body);
                let fixtures = get_fixtures_mapping(&functions);
                let test_cases = get_test_cases(&functions);

                ParsedPythonFile {
                    file,
                    errors: Vec::new(),
                    fixtures,
                    test_cases,
                }
            }
        };
    }
}
