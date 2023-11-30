use rustpython_ast::{Expr, ExprAttribute, ExprCall, ExprName, StmtFunctionDef};

pub trait FunctionUtil {
    fn is_pytest_fixture(&self) -> bool;
}

impl FunctionUtil for ExprCall {
    fn is_pytest_fixture(&self) -> bool {
        match &*self.func {
            Expr::Attribute(ExprAttribute { value, attr, .. }) => {
                let is_pytest_name = if let Expr::Name(ExprName { id, .. }) = &**value {
                    id == "pytest"
                } else {
                    false
                };

                let is_pytest_attr = attr == "fixture";
                is_pytest_name && is_pytest_attr
            }
            _ => false,
        }
    }
}

impl FunctionUtil for StmtFunctionDef {
    fn is_pytest_fixture(&self) -> bool {
        self.decorator_list.iter().any(|x: &Expr| match x {
            Expr::Call(v) => v.is_pytest_fixture(),
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionUtil;
    use crate::test_utils::{get_fixture, get_function};

    #[test]
    fn assert_pytest_fixture() {
        let func = get_fixture("fixture", Vec::new(), None);

        assert!(func.as_function_def_stmt().unwrap().is_pytest_fixture());
    }

    #[test]
    fn assert_not_pytest_fixture() {
        let func = get_function("fixture", Vec::new(), None, Vec::new());

        assert!(!func.as_function_def_stmt().unwrap().is_pytest_fixture());
    }
}
