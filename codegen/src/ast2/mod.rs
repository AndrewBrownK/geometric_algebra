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
pub(crate) struct RawVariableDeclaration {
    pub(crate) comment: Option<Cow<'static, String>>,
    pub(crate) name: (String, usize),
    pub(crate) expr: Option<AnyExpression>
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RawVariableInvocation {
    pub(crate) decl: Arc<RawVariableDeclaration>,
}
