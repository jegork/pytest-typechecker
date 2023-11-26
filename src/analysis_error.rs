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
