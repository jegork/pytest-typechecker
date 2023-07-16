use rustpython_ast::Expr::Attribute;
use rustpython_ast::{Expr, StmtFunctionDef};

pub fn is_fixture(func: &StmtFunctionDef) -> bool {
    if func.decorator_list.len() > 0 {
        for dec in &func.decorator_list {
            match &dec.expression {
                Expr::Call(call) => match call.func.as_ref() {
                    Attribute(atr) => {
                        let name = atr.attr.as_str();

                        if name == "fixture" {
                            return true;
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
        }
    }

    false
}
