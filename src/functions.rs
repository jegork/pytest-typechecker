use std::collections::HashMap;

use rustpython_ast::{ArgWithDefault, Expr, ExprSubscript, Stmt, StmtFunctionDef};

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

pub fn get_test_cases(functions: &[StmtFunctionDef]) -> HashMap<String, StmtFunctionDef> {
    let mut mapping = HashMap::new();

    for f in functions {
        let name = f.name.to_string();
        if !f.is_pytest_fixture() && name.starts_with("test_") {
            mapping.insert(name, f.clone());
        }
    }

    mapping
}

fn unfold_subscript(expr: &ExprSubscript) -> Option<String> {
    let value = expr.value.as_name_expr()?.id.to_string();

    match *expr.slice {
        Expr::Name(ref expr) => Some(format!("{}[{}]", value, expr.id)),
        Expr::Subscript(ref expr) => {
            Some(format!("{}[{}]", value, unfold_subscript(expr).unwrap()))
        }
        Expr::Tuple(ref expr) => {
            let tuple: Vec<String> = expr
                .elts
                .iter()
                .map(|v| v.as_name_expr().unwrap().id.to_string())
                .collect();

            Some(format!("{}[{}]", value, tuple.join(", ")))
        }
        _ => panic!("Incorrect type {:#?}", expr.slice),
    }
}

pub fn get_annotation(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Name(v) => Some(v.id.to_string()),
        Expr::Subscript(v) => unfold_subscript(v),
        _ => None,
    }
}

pub fn get_argument_annotation(arg: &ArgWithDefault) -> Option<String> {
    get_annotation(arg.def.annotation.as_deref()?)
}

pub fn get_return_annotation(func: &StmtFunctionDef) -> Option<String> {
    get_annotation(func.returns.as_deref()?)
}

#[cfg(test)]
mod tests {
    use rustpython_ast::StmtFunctionDef;

    use crate::test_utils::*;
    use std::collections::HashMap;

    use crate::functions::{
        get_fixtures_mapping, get_functions, get_return_annotation, get_test_cases,
    };

    #[test]
    fn assert_get_functions_empty() {
        assert_eq!(get_functions(&[]), &[])
    }

    #[test]
    fn assert_get_functions() {
        let functions = create_functions();
        let expected: Vec<StmtFunctionDef> = functions
            .iter()
            .map(|f| f.as_function_def_stmt().unwrap().to_owned())
            .collect();

        assert_eq!(get_functions(&functions), expected)
    }

    #[test]
    fn assert_get_fixtures_mapping() {
        let functions = get_functions(&create_functions());

        let expected = HashMap::from([("fixture_1".to_string(), functions[1].clone())]);

        assert_eq!(get_fixtures_mapping(&functions), expected);
    }

    #[test]
    fn assert_get_test_cases() {
        let functions = get_functions(&create_functions());
        let expected = HashMap::from([("test_case_1".to_string(), functions[0].clone())]);

        assert_eq!(get_test_cases(&functions), expected);
    }

    #[test]
    fn assert_get_return_annotation() {
        let functions = get_functions(&create_functions());
        let expected = vec![None, Some("int".to_string())];

        assert_eq!(
            functions
                .iter()
                .map(get_return_annotation)
                .collect::<Vec<Option<String>>>(),
            expected
        );
    }
}
