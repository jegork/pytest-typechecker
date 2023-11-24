use rustpython_ast::StmtFunctionDef;
use std::collections::HashMap;
use crate::analysis_error::AnalysisError;
use crate::files::python_file::PythonFile;

#[derive(Debug)]
pub struct ParsedPythonFile {
    pub file: PythonFile,
    pub errors: Vec<AnalysisError>,
    pub fixtures: HashMap<String, StmtFunctionDef>,
    pub test_cases: HashMap<String, StmtFunctionDef>,
}