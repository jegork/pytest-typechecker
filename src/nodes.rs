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
