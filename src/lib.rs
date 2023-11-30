pub mod analysis_error;
pub mod check;
pub mod files;
pub mod functions;
pub mod nodes;
use std::path::PathBuf;

use check::check_file;

use crate::files::{parsed_python_file::ParsedPythonFile, python_file::PythonFile, read_file};

pub fn check_and_parse_file<'a, I: Iterator>(files: I) -> Vec<ParsedPythonFile>
where
    I: Iterator<Item = &'a PathBuf>,
{
    files
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
                AnalysisError::IncorrectArgumentType {
                    function_name: "test_hello_6".to_string(),
                    argument_name: "sample_nested_list".to_string(),
                    expected_type: "List[List[int]]".to_string(),
                    provided_type: "List[List]".to_string(),
                },
                AnalysisError::IncorrectArgumentType {
                    function_name: "test_hello_5".to_string(),
                    argument_name: "sample_nested_dict".to_string(),
                    expected_type: "List[List[Dict[int, str]]]".to_string(),
                    provided_type: "Dict".to_string(),
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
        let files = check_and_parse_file([path].iter());
        let expected_value = get_errors_for_file(filepath);

        let provided_set: HashSet<&AnalysisError> = HashSet::from_iter(files[0].errors.iter());
        let expected_set: HashSet<&AnalysisError> = HashSet::from_iter(expected_value.iter());

        assert_eq!(provided_set, expected_set)
    }
}

#[cfg(test)]
pub(crate) mod test_utils {

    use rustpython_ast::{
        Arg, ArgWithDefault, Arguments, EmptyRange, Expr, ExprAttribute, ExprCall, ExprContext,
        ExprName, Identifier, Stmt, StmtFunctionDef, TextSize,
    };
    use rustpython_parser_vendored::text_size::TextRange;

    pub fn get_mock_text_sizes() -> (TextSize, TextSize) {
        (TextSize::new(0), TextSize::new(1))
    }

    pub fn get_mock_text_range() -> TextRange {
        let (start, end) = get_mock_text_sizes();
        TextRange::new(start, end)
    }

    pub fn get_mock_empty_range<T>() -> EmptyRange<T> {
        let (start, end) = get_mock_text_sizes();
        EmptyRange::new(start, end)
    }

    pub fn get_function_args(
        arg: Identifier,
        annotation: Option<Box<Expr<TextRange>>>,
    ) -> ArgWithDefault<TextRange> {
        ArgWithDefault {
            range: get_mock_empty_range(),
            def: Arg {
                range: get_mock_text_range(),
                arg,
                annotation,
                type_comment: None,
            },
            default: None,
        }
    }

    pub fn get_function(
        name: &str,
        args: Vec<ArgWithDefault>,
        returns: Option<Box<Expr>>,
        decorator_list: Vec<Expr>,
    ) -> Stmt {
        Stmt::FunctionDef(StmtFunctionDef {
            range: get_mock_text_range(),
            name: Identifier::new(name),
            args: Box::new(Arguments {
                range: get_mock_empty_range(),
                posonlyargs: Vec::new(),
                args,
                vararg: None,
                kwonlyargs: Vec::new(),
                kwarg: None,
            }),
            body: Vec::new(),
            decorator_list,
            returns,
            type_comment: None,
            type_params: Vec::new(),
        })
    }

    pub fn get_test_case(
        name: &str,
        args: Vec<ArgWithDefault>,
        returns: Option<Box<Expr>>,
    ) -> Stmt {
        get_function(name, args, returns, Vec::new())
    }

    pub fn get_fixture(name: &str, args: Vec<ArgWithDefault>, returns: Option<Box<Expr>>) -> Stmt {
        let func = Box::new(Expr::Attribute(ExprAttribute {
            range: get_mock_text_range(),
            value: Box::new(Expr::Name(ExprName {
                range: get_mock_text_range(),
                id: Identifier::new("pytest"),
                ctx: ExprContext::Load {},
            })),
            attr: Identifier::new("fixture"),
            ctx: ExprContext::Load {},
        }));

        let fixture = Expr::Call(ExprCall {
            range: get_mock_text_range(),
            func,
            args: Vec::new(),
            keywords: Vec::new(),
        });
        get_function(name, args, returns, vec![fixture])
    }

    pub fn create_functions() -> Vec<Stmt> {
        vec![
            get_test_case(
                "test_case_1",
                vec![get_function_args(
                    Identifier::new("fixture1"),
                    Some(Box::new(Expr::Name(ExprName {
                        range: get_mock_text_range(),
                        id: Identifier::new("int"),
                        ctx: ExprContext::Load {},
                    }))),
                )],
                None,
            ),
            get_fixture(
                "fixture_1",
                Vec::new(),
                Some(Box::new(Expr::Name(ExprName {
                    range: get_mock_text_range(),
                    id: Identifier::new("int"),
                    ctx: ExprContext::Load {},
                }))),
            ),
        ]
    }
}
