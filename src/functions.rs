use std::collections::HashMap;

use rustpython_ast::{Stmt, StmtFunctionDef};

use crate::nodes::FunctionUtil;

pub fn get_functions(stmts: &[Stmt]) -> Vec<StmtFunctionDef> {
    stmts
        .iter()
        .filter_map(|s| match s {
            rustpython_ast::Stmt::FunctionDef(val) => Some(val.to_owned()),
            _ => None,
        })
        .collect()
}

pub fn get_fixtures_mapping(functions: &[StmtFunctionDef]) -> HashMap<String, StmtFunctionDef> {
    let mut mapping = HashMap::new();

    for f in functions {
        if f.is_pytest_fixture() {
            let name = f.name.to_string();
            mapping.insert(name, f.clone());
        }
    }

    mapping
}

pub fn get_test_cases(functions: &Vec<StmtFunctionDef>) -> HashMap<String, StmtFunctionDef> {
    let mut mapping = HashMap::new();

    for f in functions {
        let name = f.name.to_string();
        if !f.is_pytest_fixture() && name.starts_with("test_") {
            mapping.insert(name, f.clone());
        }
    }

    mapping
}
