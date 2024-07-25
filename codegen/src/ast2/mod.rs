use std::borrow::Cow;
use std::sync::Arc;

use crate::ast2::expressions::AnyExpression;

pub mod datatype;
mod playground;
pub mod traits;
pub mod expressions;
pub mod impls;
mod operations_tracker;

#[derive(PartialEq, Clone, Debug)]
pub struct Variable<ExprType> {
    pub expr_type: ExprType,
    decl: Arc<RawVariableDeclaration>,
}

#[derive(Clone, Debug, PartialEq)]
struct RawVariableDeclaration {
    comment: Option<Cow<'static, String>>,
    name: (String, usize),
    expr: Option<AnyExpression>
}
#[derive(Clone, Debug, PartialEq)]
struct RawVariableInvocation {
    decl: Arc<RawVariableDeclaration>,
}
