use std::fmt::Debug;
use std::sync::Arc;

use crate::algebra::MultiVectorClass;
use crate::ast2::{RawVariableDeclaration, RawVariableInvocation, Variable};
use crate::ast2::basis::BasisSignature;
use crate::ast2::datatype::{ExpressionType, Float, Integer, MultiVector, Vec2, Vec3, Vec4};

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
struct TraitName {
    name: String,
}
enum ClassGroup {
    JustFloat(BasisSignature),
    Vec2(BasisSignature, BasisSignature),
    Vec3(BasisSignature, BasisSignature, BasisSignature),
    Vec4(BasisSignature, BasisSignature, BasisSignature, BasisSignature)
}

pub trait TraitResultType: Debug {
    type ExprType;
    fn into_expr_10(self, trait_name: TraitName, owner: MultiVector) -> Self::ExprType {
        panic!("into_expr_0 is not yet supported for {self:?}, but needed for {trait_name}")
    }
    fn into_expr_11(self, trait_name: TraitName, owner: MultiVectorExpr) -> Self::ExprType {
        panic!("into_expr_11 is not yet supported for {self:?}, but needed for {trait_name}")
    }
    fn into_expr_21(self, trait_name: TraitName, owner: MultiVectorExpr, other: MultiVector) -> Self::ExprType {
        panic!("into_expr_21 is not yet supported for {self:?}, but needed for {trait_name}")
    }
    fn into_expr_22(self, trait_name: TraitName, owner: MultiVectorExpr, other: MultiVectorExpr) -> Self::ExprType {
        panic!("into_expr_22 is not yet supported for {self:?}, but needed for {trait_name}")
    }

}
impl TraitResultType for Integer {
    type ExprType = IntExpr;
    fn into_expr_10(self, trait_name: TraitName, owner: MultiVector) -> IntExpr {
        IntExpr {
            via: IntExpr::TraitInvoke10ToInt(trait_name, owner),
        }
    }
}
impl TraitResultType for Float {
    type ExprType = FloatExpr;

    fn into_expr_11(self, trait_name: TraitName, owner: MultiVectorExpr) -> FloatExpr {
        FloatExpr {
            via: FloatExpr::TraitInvoke11ToFloat(trait_name, owner),
        }
    }
}
impl TraitResultType for MultiVector {
    type ExprType = MultiVectorExpr;

    fn into_expr_11(self, trait_name: TraitName, owner: MultiVectorExpr) -> MultiVectorExpr {
        MultiVectorExpr {
            // TODO result of trait
            mv_class: todo!(),
            expr: MultiVectorVia::TraitInvoke11ToClass(trait_name, *owner),
        }
    }
    fn into_expr_21(self, trait_name: TraitName, owner: MultiVectorExpr, other: MultiVector) -> MultiVectorExpr {
        MultiVectorExpr {
            // TODO result of trait
            mv_class: todo!(),
            expr: MultiVectorVia::TraitInvoke21ToClass(trait_name, *owner, other),
        }
    }
    fn into_expr_22(self, trait_name: TraitName, owner: MultiVectorExpr, other: MultiVectorExpr) -> MultiVectorExpr {
        MultiVectorExpr {
            // TODO result of trait
            mv_class: todo!(),
            expr: MultiVectorVia::TraitInvoke22ToClass(trait_name, *owner, other),
        }
    }
}

pub enum TraitResult<'authentic> {
    Int(&'authentic Integer),
    Float(&'authentic Float),
    OwnerClass(&'authentic MultiVector),
    OtherClass(&'authentic MultiVector),
    AnyClass(&'authentic MultiVector),
}



#[derive(PartialEq, Eq, Clone, Debug)]
pub enum IntExpr {
    Variable(RawVariableInvocation),
    Literal(u32),
    // e.g. Grade
    TraitInvoke10ToInt(TraitName, MultiVector),
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FloatExpr {
    Variable(RawVariableInvocation),
    Zero, One, NegOne, Two, Half,
    AccessVec2(Vec2Expr, u8),
    AccessVec3(Vec3Expr, u8),
    AccessVec4(Vec4Expr, u8),
    // e.g. UnitizedNorm
    TraitInvoke11ToFloat(TraitName, MultiVectorExpr),
    // TODO sum of o products
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Vec2Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather2(FloatExpr, FloatExpr),
    // TODO sum of o products
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Vec3Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather3(FloatExpr, FloatExpr, FloatExpr),
    // TODO sum of o products
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Vec4Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather4(FloatExpr, FloatExpr, FloatExpr, FloatExpr),
    // TODO sum of o products
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MultiVectorGroupExpr {
    JustFloat(FloatExpr),
    Vec2(Vec2Expr),
    Vec3(Vec3Expr),
    Vec4(Vec4Expr)
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MultiVectorExpr {
    mv_class: MultiVector,
    expr: Box<MultiVectorVia>
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MultiVectorVia {
    Variable(RawVariableInvocation),
    Construct(Vec<MultiVectorGroupExpr>),
    // e.g. Involutions
    TraitInvoke11ToClass(TraitName, MultiVectorExpr),
    // e.g. Into
    TraitInvoke21ToClass(TraitName, MultiVectorExpr, MultiVector),
    // e.g. Wedge
    TraitInvoke22ToClass(TraitName, MultiVectorExpr, MultiVectorExpr),
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnyExpression {
    Int(IntExpr),
    Float(FloatExpr),
    Vec2(Vec2Expr),
    Vec3(Vec3Expr),
    Vec4(Vec4Expr),
    Class(MultiVectorExpr),
}


pub trait Expression<ExprType> {
    fn into_any_expression(self) -> AnyExpression;
    fn strong_expression_type(&self) -> ExprType;
    fn soft_expression_type(&self) -> ExpressionType;
}
impl Expression<Integer> for IntExpr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Int(self)
    }

    fn strong_expression_type(&self) -> Integer {
        Integer
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Int(Integer)
    }
}
impl Expression<Float> for FloatExpr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Float(self)
    }

    fn strong_expression_type(&self) -> Float {
        Float
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Float(Float)
    }
}
impl Expression<Vec2> for Vec2Expr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec2(self)
    }

    fn strong_expression_type(&self) -> Vec2 {
        Vec2
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec2(Vec2)
    }
}
impl Expression<Vec3> for Vec3Expr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec3(self)
    }

    fn strong_expression_type(&self) -> Vec3 {
        Vec3
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec3(Vec3)
    }
}
impl Expression<Vec4> for Vec4Expr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec4(self)
    }

    fn strong_expression_type(&self) -> Vec4 {
        Vec4
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec4(Vec4)
    }
}
impl Expression<MultiVector> for MultiVectorExpr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Class(self)
    }

    fn strong_expression_type(&self) -> MultiVector {
        self.mv_class.clone()
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Class(self.strong_expression_type())
    }
}

impl Expression<Integer> for Variable<Integer> {
    fn into_any_expression(self) -> AnyExpression {
        let decl = self.decl.clone();
        AnyExpression::Int(IntExpr::Variable(RawVariableInvocation { decl }))
    }

    fn strong_expression_type(&self) -> Integer {
        Integer
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Int(Integer)
    }
}
impl Expression<Float> for Variable<Float> {
    fn into_any_expression(self) -> AnyExpression {
        let decl = self.decl.clone();
        AnyExpression::Float(FloatExpr::Variable(RawVariableInvocation { decl }))
    }

    fn strong_expression_type(&self) -> Float {
        Float
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Float(Float)
    }
}
impl Expression<Vec2> for Variable<Vec2> {
    fn into_any_expression(self) -> AnyExpression {
        let decl = self.decl.clone();
        AnyExpression::Vec2(Vec2Expr::Variable(RawVariableInvocation { decl }))
    }

    fn strong_expression_type(&self) -> Vec2 {
        Vec2
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec2(Vec2)
    }
}
impl Expression<Vec3> for Variable<Vec3> {
    fn into_any_expression(self) -> AnyExpression {
        let decl = self.decl.clone();
        AnyExpression::Vec3(Vec3Expr::Variable(RawVariableInvocation { decl }))
    }

    fn strong_expression_type(&self) -> Vec3 {
        Vec3
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec3(Vec3)
    }
}
impl Expression<Vec4> for Variable<Vec4> {
    fn into_any_expression(self) -> AnyExpression {
        let decl = self.decl.clone();
        AnyExpression::Vec4(Vec4Expr::Variable(RawVariableInvocation { decl }))
    }

    fn strong_expression_type(&self) -> Vec4 {
        Vec4
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec4(Vec4)
    }
}
impl Expression<MultiVector> for Variable<MultiVector> {
    fn into_any_expression(self) -> AnyExpression {
        let decl = self.decl.clone();
        AnyExpression::Class(MultiVectorExpr {
            mv_class: self.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl })),
        })
    }

    fn strong_expression_type(&self) -> MultiVector {
        self.expr_type.clone()
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Class(self.strong_expression_type())
    }
}








