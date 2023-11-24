use crate::files::PythonFile;
use rustpython_parser;
use rustpython_parser_core::Mode;

pub enum AnalysisError {

}

pub struct ParsedPythonFile {
    pub file: PythonFile,
    pub errors: Vec<AnalysisError>,
    ast: rustpython_ast::ModModule
}

impl ParsedPythonFile {
    pub fn new(file: PythonFile) -> ParsedPythonFile {
        let parsed = rustpython_parser::parse(&file.content, Mode::Module, &file.filename);

        return match parsed {
            Err(err) => panic!("Failed to parse Python AST: {}", err),
            Ok(ast) => ParsedPythonFile {file, errors: Vec::new(), ast: ast.as_module().unwrap().to_owned()}
        }
    }

    pub fn print_ast(&self) {
        println!("Printing AST for file: {}", self.file.filename);

        for el in &self.ast.body {
            println!("{:#?}", el);
        }
    }
}

pub fn parse_python_files(files: Vec<PythonFile>) -> Vec<ParsedPythonFile> {
    files.into_iter().map(ParsedPythonFile::new).collect()
}