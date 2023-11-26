use colored::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AnalysisError {
    FixtureMissingReturnType {
        fixture_name: String,
    },
    IncorrectArgumentType {
        function_name: String,
        argument_name: String,
        expected_type: String,
        provided_type: String,
    },
    MissingArgumentType {
        function_name: String,
        argument_name: String,
    },
    FixtureDoesNotExist {
        function_name: String,
        argument_name: String,
    },
    UnparsableFile,
}

impl Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::FixtureDoesNotExist {
                function_name,
                argument_name,
            } => write!(
                f,
                "{} Fixture {} used in function {} does not exist.",
                "[FIXTURE_DOES_NOT_EXIST]".red(),
                argument_name, function_name
            ),
            AnalysisError::FixtureMissingReturnType { fixture_name } => {
                write!(f, "{} Fixture {} missing return type.", "[FIXTURE_MISSING_RETURN_TYPE]".red(), fixture_name)
            }
            AnalysisError::IncorrectArgumentType {
                function_name,
                argument_name,
                expected_type,
                provided_type,
            } => write!(
                f,
                "{} Function's {} argument {} receives a fixture of type {}, but specified type is {}.",
                "[INCORRECT_ARGUMENT_TYPE]".red(),
                function_name, argument_name, expected_type, provided_type
            ),
            AnalysisError::MissingArgumentType {
                function_name,
                argument_name,
            } => write!(f, "{} Function {} has no type specified for argument {}.", "[MISSING_ARGUMENT_TYPE]".red(), function_name, argument_name),
            AnalysisError::UnparsableFile => write!(f, "{} Impossible to parse file's AST.", "[UNPARSABLE_FILE]".red()),
        }
    }
}
