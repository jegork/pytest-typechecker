#[derive(Debug)]
pub enum AnalysisError {
    FixtureMissingReturnType {
        fixture_name: String,
    },
    IncorrectArgumentType {
        test_case_name: String,
        argument_name: String,
        expected_type: String,
        provided_type: String,
    },
    MissingArgumentType {
        test_case_name: String,
        argument_name: String,
    },
    FixtureDoesNotExist {
        test_case_name: String,
        argument_name: String,
    },
    UnparsableFile,
}