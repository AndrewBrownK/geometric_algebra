use std::borrow::Cow;
use std::cmp::Ordering;
use std::sync::Arc;

use crate::ast2::expressions::AnyExpression;

pub mod datatype;
pub mod expressions;
pub mod impls;
mod operations_tracker;
pub mod traits;

#[derive(Clone, Debug)]
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
impl<ExprType> PartialEq for Variable<ExprType> where ExprType: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.expr_type == other.expr_type && self.decl == other.decl
    }
}
impl<ExprType> Eq for Variable<ExprType> where ExprType: Eq {}
impl<ExprType> PartialOrd for Variable<ExprType> where ExprType: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl<ExprType> Ord for Variable<ExprType> where ExprType: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.expr_type.cmp(&other.expr_type).then_with(|| {
            self.decl.cmp(&other.decl)
        })
    }
}


#[derive(Clone, Debug)]
pub(crate) struct RawVariableDeclaration {
    pub(crate) comment: Option<Cow<'static, String>>,
    pub(crate) name: (String, usize),
    pub(crate) expr: Option<AnyExpression>,
}
impl PartialEq for RawVariableDeclaration {
    fn eq(&self, other: &Self) -> bool {
        let result = std::ptr::eq(self, other);
        if !result {
            return self.name == other.name && self.comment == other.comment && self.expr == other.expr
        }
        result
    }
}
impl Eq for RawVariableDeclaration {}
impl PartialOrd for RawVariableDeclaration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for RawVariableDeclaration {
    fn cmp(&self, other: &Self) -> Ordering {
        if std::ptr::eq(self, other) {
            return Ordering::Equal
        }
        self.name.cmp(&other.name).then_with(|| {
            self.expr.cmp(&other.expr).then_with(|| {
                self.comment.cmp(&other.comment)
            })
        })
    }
}



#[derive(Clone, Debug)]
pub(crate) struct RawVariableInvocation {
    pub(crate) decl: Arc<RawVariableDeclaration>,
}
impl PartialEq for RawVariableInvocation {
    fn eq(&self, other: &Self) -> bool {
        self.decl == other.decl
    }
}
impl Eq for RawVariableInvocation {}
impl PartialOrd for RawVariableInvocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.decl.partial_cmp(&other.decl)
    }
}
impl Ord for RawVariableInvocation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}