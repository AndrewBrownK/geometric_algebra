use std::borrow::Cow;
use std::sync::Arc;

use crate::ast2::expressions::AnyExpression;

pub mod datatype;
pub mod traits;
pub mod expressions;
pub mod impls;
mod operations_tracker;

#[derive(PartialEq, Clone, Debug)]
pub struct Variable<ExprType> {
    pub expr_type: ExprType,
    decl: Arc<RawVariableDeclaration>,
}

// TODO see if this can be used generally, or if I need a more formal solution
impl<ExprType> Variable<ExprType> {
    // For quick testing purposes
    pub(crate) fn quick_var(name: &str, e: ExprType) -> Self {
        Variable {
            expr_type: e,
            decl: Arc::new(RawVariableDeclaration {
                comment: None,
                name: (name.to_string(), 0),
                expr: None,
            }),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RawVariableDeclaration {
    pub(crate) comment: Option<Cow<'static, String>>,
    pub(crate) name: (String, usize),
    pub(crate) expr: Option<AnyExpression>
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RawVariableInvocation {
    pub(crate) decl: Arc<RawVariableDeclaration>,
}
