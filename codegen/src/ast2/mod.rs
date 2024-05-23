use std::marker::PhantomData;
use std::sync::Arc;

use crate::algebra::MultiVectorClass;
use crate::ast2::datatype::{ClassesFromRegistry, DataType};
use crate::ast2::expressions::{AnyExpression, ExpressionOf};

mod basis;
mod datatype;
mod playground;
mod traits;
mod expressions;

// TODO Hamish recommends terms "intrinsic" and "extrinsic" instead of "Space" and "antispace"


type MultiVectorParam = VariableInvocation<'static, Arc<MultiVectorClass>>;
type MultiVectorVar<'var> = VariableInvocation<'var, Arc<MultiVectorClass>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VariableDeclaration<'var, DataType> {
    name: String,
    ty: DataType,
    phantom: PhantomData<&'var ()>,
    expr: Option<AnyExpression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VariableInvocation<'var, DataType> {
    var: Arc<VariableDeclaration<'var, DataType>>
}
impl<'var, DT> ExpressionOf<'var, DT> for VariableInvocation<'var, DT> where
    DT: Clone + PartialEq + Into<DataType> {
    fn get_datatype(&self) -> DataType {
        self.var.ty.clone().into()
    }

    fn ty(&self) -> DT {
        self.var.ty.clone()
    }
}


trait AssignDataType<DT: ClassesFromRegistry> {
    type Output;
    fn assign_data_type(dt: DT) -> Self::Output;
}
struct RawVariableDeclaration {
    name: String,
    ty: DataType,
    expr: Option<AnyExpression>
}
impl<'mvc, DT> From<VariableDeclaration<'_, DT>> for RawVariableDeclaration where
    DT: Clone + Into<DataType> {
    fn from(value: VariableDeclaration<'_, DT>) -> Self {
        return RawVariableDeclaration {
            name: value.name.clone(),
            ty: value.ty.clone().into(),
            expr: value.expr.map(|it| it.into()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RawVariableInvocation {
    var: String,
    ty: DataType,
}
impl<'mvc, DT: Clone + Into<DataType>> From<VariableInvocation<'_, DT>> for RawVariableInvocation {
    fn from(value: VariableInvocation<'_, DT>) -> Self {
        return RawVariableInvocation {
            var: value.var.name.clone(),
            ty: value.var.ty.clone().into(),
        }
    }
}