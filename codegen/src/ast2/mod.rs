use std::borrow::Cow;
use std::sync::Arc;

use crate::ast2::expressions::AnyExpression;

pub mod datatype;
mod playground;
pub mod traits;
pub mod expressions;
pub mod impls;
// TODO Hamish recommends terms "intrinsic" and "extrinsic" instead of "Space" and "antispace"

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Variable<'var, ExprType> {
    pub expr_type: ExprType,
    decl: &'var Arc<RawVariableDeclaration>,
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
