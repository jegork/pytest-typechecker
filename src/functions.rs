use std::collections::HashMap;

use rustpython_ast::{ArgWithDefault, Stmt, StmtFunctionDef};

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

pub fn get_argument_annotation(arg: &ArgWithDefault) -> Option<String> {
    Some(arg.def.annotation.clone()?.as_name_expr()?.id.to_string())
}

pub fn get_return_annotation(func: &StmtFunctionDef) -> Option<String> {
    match &func.returns {
        Some(returns) => Some(returns.as_name_expr()?.id.to_string()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rustpython_ast::{
        Arg, ArgWithDefault, Arguments, EmptyRange, Expr, ExprAttribute, ExprCall, ExprContext,
        ExprName, Identifier, Stmt, StmtFunctionDef, TextSize,
    };
    use rustpython_parser_vendored::text_size::TextRange;

    use crate::functions::{
        get_fixtures_mapping, get_functions, get_return_annotation, get_test_cases,
    };

    fn get_mock_text_sizes() -> (TextSize, TextSize) {
        (TextSize::new(0), TextSize::new(1))
    }

    fn get_mock_text_range() -> TextRange {
        let (start, end) = get_mock_text_sizes();
        TextRange::new(start, end)
    }

    fn get_mock_empty_range<T>() -> EmptyRange<T> {
        let (start, end) = get_mock_text_sizes();
        EmptyRange::new(start, end)
    }

    fn get_function_args(
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

    fn get_function(
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

    fn get_test_case(name: &str, args: Vec<ArgWithDefault>, returns: Option<Box<Expr>>) -> Stmt {
        get_function(name, args, returns, Vec::new())
    }

    fn get_fixture(name: &str, args: Vec<ArgWithDefault>, returns: Option<Box<Expr>>) -> Stmt {
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

    fn create_functions() -> Vec<Stmt> {
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
