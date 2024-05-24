use std::sync::Arc;

use crate::ast2::expressions::AnyExpression;

mod basis;
mod datatype;
mod playground;
mod traits;
mod expressions;

// TODO Hamish recommends terms "intrinsic" and "extrinsic" instead of "Space" and "antispace"

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Variable<'var, ExprType> {
    expr_type: ExprType,
    decl: &'var Arc<RawVariableDeclaration>,
}

#[derive(Clone, Debug, PartialEq, Eq,)]
struct RawVariableDeclaration {
    comment: Option<String>,
    name: String,
    expr: Option<AnyExpression>
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct RawVariableInvocation {
    decl: Arc<RawVariableDeclaration>,
}