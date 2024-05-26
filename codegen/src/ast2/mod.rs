use std::sync::Arc;

use crate::ast2::expressions::AnyExpression;

mod datatype;
mod playground;
mod traits;
mod expressions;

// TODO Hamish recommends terms "intrinsic" and "extrinsic" instead of "Space" and "antispace"

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable<ExprType> {
    pub expr_type: ExprType,
    pub decl: Arc<RawVariableDeclaration>,
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
