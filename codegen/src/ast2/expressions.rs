use std::fmt::Debug;

use crate::ast2::{RawVariableInvocation, Variable};
use crate::algebra2::basis::BasisSignature;
use crate::ast2::datatype::{ExpressionType, Float, Integer, MultiVector, Vec2, Vec3, Vec4};
use crate::ast2::traits::TraitKey;

// TODO use this somehow
enum ClassGroup {
    JustFloat(BasisSignature),
    Vec2(BasisSignature, BasisSignature),
    Vec3(BasisSignature, BasisSignature, BasisSignature),
    Vec4(BasisSignature, BasisSignature, BasisSignature, BasisSignature)
}

pub trait TraitResultType: Debug + Sized {
    type ExprType;
    #[allow(unused)]
    fn expr_10(trait_name: TraitKey, owner: MultiVector, mv_out: Option<MultiVector>) -> Self::ExprType {
        panic!("into_expr_0 is needed (but not supported) for {trait_name:?}")
    }
    #[allow(unused)]
    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> Self::ExprType {
        panic!("into_expr_11 is needed (but not supported) for {trait_name:?}")
    }
    #[allow(unused)]
    fn expr_21(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVector, mv_out: Option<MultiVector>) -> Self::ExprType {
        panic!("into_expr_21 is needed (but not supported) for {trait_name:?}")
    }
    #[allow(unused)]
    fn expr_22(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVectorExpr, mv_out: Option<MultiVector>) -> Self::ExprType {
        panic!("into_expr_22 is needed (but not supported) for {trait_name:?}")
    }

}
impl TraitResultType for Integer {
    type ExprType = IntExpr;
    fn expr_10(trait_name: TraitKey, owner: MultiVector, mv_out: Option<MultiVector>) -> IntExpr {
        assert!(mv_out.is_none(), "Confused Trait output: Expected Integer, found MultiVector");
        IntExpr::TraitInvoke10ToInt(trait_name, owner)
    }
}
impl TraitResultType for Float {
    type ExprType = FloatExpr;

    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> FloatExpr {
        assert!(mv_out.is_none(), "Confused Trait output: Expected Float, found MultiVector");
        FloatExpr::TraitInvoke11ToFloat(trait_name, owner)
    }
}
impl TraitResultType for MultiVector {
    type ExprType = MultiVectorExpr;

    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect(
            "Confused Trait output: Expected MultiVector, but None provided."
        );
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke11ToClass(trait_name, owner)),
        }
    }
    fn expr_21(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVector, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect(
            "Confused Trait output: Expected MultiVector, but None provided."
        );
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke21ToClass(trait_name, owner, other)),
        }
    }
    fn expr_22(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVectorExpr, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect(
            "Confused Trait output: Expected MultiVector, but None provided."
        );
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke22ToClass(trait_name, owner, other)),
        }
    }
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum IntExpr {
    Variable(RawVariableInvocation),
    Literal(u32),
    // e.g. Grade
    TraitInvoke10ToInt(TraitKey, MultiVector),
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FloatExpr {
    Variable(RawVariableInvocation),
    Zero, One, NegOne, Two, Half,
    AccessVec2(Box<Vec2Expr>, u8),
    AccessVec3(Box<Vec3Expr>, u8),
    AccessVec4(Box<Vec4Expr>, u8),
    // e.g. UnitizedNorm
    TraitInvoke11ToFloat(TraitKey, MultiVectorExpr),
    // TODO sum of products
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Vec2Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather2(FloatExpr, FloatExpr),
    // TODO sum of products
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Vec3Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather3(FloatExpr, FloatExpr, FloatExpr),
    // TODO sum of products
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Vec4Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather4(FloatExpr, FloatExpr, FloatExpr, FloatExpr),
    // TODO sum of products
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
    pub mv_class: MultiVector,
    pub expr: Box<MultiVectorVia>
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MultiVectorVia {
    Variable(RawVariableInvocation),
    Construct(Vec<MultiVectorGroupExpr>),
    // e.g. Involutions
    TraitInvoke11ToClass(TraitKey, MultiVectorExpr),
    // e.g. Into
    TraitInvoke21ToClass(TraitKey, MultiVectorExpr, MultiVector),
    // e.g. Wedge
    TraitInvoke22ToClass(TraitKey, MultiVectorExpr, MultiVectorExpr),
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
impl AnyExpression {
    pub fn expression_type(&self) -> ExpressionType {
        match self {
            AnyExpression::Int(_) => ExpressionType::Int(Integer),
            AnyExpression::Float(_) => ExpressionType::Float(Float),
            AnyExpression::Vec2(_) => ExpressionType::Vec2(Vec2),
            AnyExpression::Vec3(_) => ExpressionType::Vec3(Vec3),
            AnyExpression::Vec4(_) => ExpressionType::Vec4(Vec4),
            AnyExpression::Class(mv) => ExpressionType::Class(mv.mv_class.clone()),
        }
    }
}



pub trait Expression<ExprType>: Send {
    fn into_any_expression(self) -> AnyExpression;
    fn strong_expression_type(&self) -> ExprType;
    fn soft_expression_type(&self) -> ExpressionType;
}

/// This helps unify Variable<MultiVector> and MultiVectorExpr
pub fn extract_multivector_expr<Expr: Expression<MultiVector>>(expr: Expr) -> MultiVectorExpr {
    match expr.into_any_expression() {
        AnyExpression::Class(mve) => mve,
        _ => unreachable!("Expression<MultiVector> will always create AnyExpression::Class")
    }
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







