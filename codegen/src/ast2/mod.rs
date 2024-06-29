use std::borrow::Cow;
use std::sync::Arc;

use crate::ast2::expressions::AnyExpression;

pub mod datatype;
mod playground;
pub mod traits;
mod expressions;
mod impls;
// TODO Hamish recommends terms "intrinsic" and "extrinsic" instead of "Space" and "antispace"

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable<ExprType> {
    pub expr_type: ExprType,
    decl: Arc<RawVariableDeclaration>,
}

#[derive(Clone, Debug, PartialEq, Eq,)]
struct RawVariableDeclaration {
    comment: Option<Cow<'static, String>>,
    name: (String, usize),
    expr: Option<AnyExpression>
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct RawVariableInvocation {
    decl: Arc<RawVariableDeclaration>,
}
