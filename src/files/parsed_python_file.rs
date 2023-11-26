use crate::analysis_error::AnalysisError;
use crate::files::python_file::PythonFile;
use rustpython_ast::StmtFunctionDef;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParsedPythonFile {
    pub file: PythonFile,
    pub errors: Vec<AnalysisError>,
    pub fixtures: HashMap<String, StmtFunctionDef>,
    pub test_cases: HashMap<String, StmtFunctionDef>,
}

impl Display for ParsedPythonFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.errors {
            writeln!(f, "{}: {}", self.file.filename, err)?
        }

        Ok(())
    }
}
