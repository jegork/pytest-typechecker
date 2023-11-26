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
                "[FIXTURE_DOES_NOT_EXIST] Fixture {} used in function {} does not exist.",
                argument_name, function_name
            ),
            AnalysisError::FixtureMissingReturnType { fixture_name } => {
                write!(f, "[FIXTURE_MISSING_RETURN_TYPE] Fixture {} missing return type.", fixture_name)
            }
            AnalysisError::IncorrectArgumentType {
                function_name,
                argument_name,
                expected_type,
                provided_type,
            } => write!(
                f,
                "[INCORRECT_ARGUMENT_TYPE] Function's {} argument {} receives a fixture of type {}, but specified type is {}. ",
                function_name, argument_name, expected_type, provided_type
            ),
            AnalysisError::MissingArgumentType {
                function_name,
                argument_name,
            } => write!(f, "[MISSING_ARGUMENT_TYPE] Function {} has no type specified for argument {}.", function_name, argument_name),
            AnalysisError::UnparsableFile => write!(f, "Impossible to parse file's AST."),
        }
    }
}
