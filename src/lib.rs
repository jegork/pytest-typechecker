pub mod analysis_error;
pub mod files;
pub mod functions;
pub mod nodes;
use std::path::PathBuf;

use crate::files::{
    check_file, get_files_list, parsed_python_file::ParsedPythonFile, python_file::PythonFile,
    read_file,
};

pub fn check_and_parse_file(file: &[PathBuf], recursive: bool) -> Vec<ParsedPythonFile> {
    get_files_list(file.to_vec(), recursive)
        .iter()
        .map(read_file)
        .map(PythonFile::parse)
        .map(|f| {
            let errors = check_file(&f);
            ParsedPythonFile { errors, ..f }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use std::{collections::HashSet, path::PathBuf};
    use test_case::test_case;

    use crate::{analysis_error::AnalysisError, check_and_parse_file};

    fn get_errors_for_file(filepath: &str) -> Vec<AnalysisError> {
        match filepath {
            "./python-examples/test_sample_complex.py" => vec![
                AnalysisError::FixtureMissingReturnType {
                    fixture_name: "sample_nested_dict".to_owned(),
                },
                AnalysisError::FixtureMissingReturnType {
                    fixture_name: "sample_nested_list".to_owned(),
                },
                AnalysisError::FixtureMissingReturnType {
                    fixture_name: "sample_list".to_owned(),
                },
                AnalysisError::MissingArgumentType {
                    function_name: "test_hello_6".to_owned(),
                    argument_name: "sample_nested_list".to_owned(),
                },
                AnalysisError::MissingArgumentType {
                    function_name: "test_hello_3".to_owned(),
                    argument_name: "sample_list".to_owned(),
                },
            ],
            "./python-examples/test_sample.py" => vec![
                AnalysisError::FixtureMissingReturnType {
                    fixture_name: "sample_missing_return_type".to_owned(),
                },
                AnalysisError::MissingArgumentType {
                    function_name: "test_hello".to_owned(),
                    argument_name: "sample_string_2".to_owned(),
                },
                AnalysisError::MissingArgumentType {
                    function_name: "sample_string_2".to_owned(),
                    argument_name: "sample_string".to_owned(),
                },
                AnalysisError::IncorrectArgumentType {
                    function_name: "sample_string_3".to_owned(),
                    argument_name: "sample_string".to_owned(),
                    expected_type: "str".to_owned(),
                    provided_type: "int".to_owned(),
                },
                AnalysisError::IncorrectArgumentType {
                    function_name: "test_hello".to_owned(),
                    argument_name: "sample_string".to_owned(),
                    expected_type: "str".to_owned(),
                    provided_type: "int".to_owned(),
                },
            ],
            "./python-examples/folder/test_empty.py" => vec![],
            _ => panic!("Invalid filename."),
        }
    }

    #[test_case( "./python-examples/test_sample_complex.py" ; "for ./python-examples/test_sample_complex.py")]
    #[test_case( "./python-examples/test_sample.py" ; "for ./python-examples/test_sample.py")]
    #[test_case( "./python-examples/folder/test_empty.py" ; "for ./python-examples/folder/test_empty.py")]
    fn assert_check_file(filepath: &str) {
        let path = PathBuf::from(filepath);
        let files = check_and_parse_file(&[path], false);
        let expected_value = get_errors_for_file(filepath);

        let provided_set: HashSet<&AnalysisError> = HashSet::from_iter(files[0].errors.iter());
        let expected_set: HashSet<&AnalysisError> = HashSet::from_iter(expected_value.iter());

        assert_eq!(provided_set, expected_set)
    }
}
