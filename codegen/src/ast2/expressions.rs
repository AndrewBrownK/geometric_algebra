use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::sync::Arc;

use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::{BasisElementGroup, DynamicMultiVector};
use crate::ast2::{RawVariableDeclaration, RawVariableInvocation, Variable};
use crate::ast2::datatype::{ExpressionType, Float, Integer, MultiVector, Vec2, Vec3, Vec4};
use crate::ast2::operations_tracker::{TrackOperations, TraitOperationsLookup, VectoredOperationsTracker};
use crate::ast2::traits::TraitKey;

pub trait TraitResultType: Clone + Debug + Sized + Send + Sync + 'static {
    type Expr: Expression<Self>;
    #[allow(unused)]
    fn expr_10(trait_name: TraitKey, owner: MultiVector, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_10 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_10(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_10 is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_11 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_11(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_11 is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_21(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVector, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_21 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_21(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_21 is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_22(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVectorExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_22 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_22(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_22 is needed (but not supported)")
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self>;
}
impl TraitResultType for Integer {
    type Expr = IntExpr;
    fn expr_10(trait_name: TraitKey, owner: MultiVector, mv_out: Option<MultiVector>) -> IntExpr {
        assert!(mv_out.is_none(), "Confused Trait output: Expected Integer, found MultiVector");
        IntExpr::TraitInvoke10ToInt(trait_name, owner)
    }

    fn inlined_expr_10(var: Variable<Self>) -> Self::Expr {
        IntExpr::Variable(RawVariableInvocation {
            decl: var.decl.clone(),
        })
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Int(_) => Some(Self),
            _ => None,
        }
    }
}
impl TraitResultType for Float {
    type Expr = FloatExpr;

    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> FloatExpr {
        assert!(mv_out.is_none(), "Confused Trait output: Expected Float, found MultiVector");
        FloatExpr::TraitInvoke11ToFloat(trait_name, owner)
    }

    fn inlined_expr_11(var: Variable<Self>) -> Self::Expr {
        FloatExpr::Variable(RawVariableInvocation {
            decl: var.decl.clone(),
        })
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Float(_) => Some(Self),
            _ => None,
        }
    }
}
impl TraitResultType for MultiVector {
    type Expr = MultiVectorExpr;

    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect(
            "Confused Trait output: Expected MultiVector, but None provided."
        );
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke11ToClass(trait_name, owner)),
        }
    }
    fn inlined_expr_11(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation {
                decl: var.decl.clone(),
            })),
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
    fn inlined_expr_21(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation {
                decl: var.decl.clone(),
            })),
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
    fn inlined_expr_22(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation {
                decl: var.decl.clone(),
            })),
        }
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Class(mv_expr) => Some(mv_expr.mv_class.clone()),
            _ => None,
        }
    }
}


#[derive(PartialEq, Clone, Debug)]
pub enum IntExpr {
    Variable(RawVariableInvocation),
    Literal(u32),
    // e.g. Grade
    TraitInvoke10ToInt(TraitKey, MultiVector),
}
#[derive(PartialEq, Clone, Debug)]
pub enum FloatExpr {
    Variable(RawVariableInvocation),
    Literal(f32),
    AccessVec2(Box<Vec2Expr>, u8),
    AccessVec3(Box<Vec3Expr>, u8),
    AccessVec4(Box<Vec4Expr>, u8),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    AccessMultiVecFlat(MultiVectorExpr, u16),
    // e.g. UnitizedNorm
    TraitInvoke11ToFloat(TraitKey, MultiVectorExpr),
    // There is an argument to be made that this should contain at least 2 elements,
    // but for now, the consistency of handling everything in a single Vec is nice.
    // Note that this does cause panics and/or Errors if this Vec is provoked
    // when empty, so it is an important part of the simplification methods
    // to ensure no empty Vecs escape.
    // TODO:
    //  type IsNegative = bool;
    //  type IsReciprocal = bool;
    //  Product(Vec<(IsReciprocal, FloatExpr)>)
    //  Sum(Vec<(IsNegative, FloatExpr)>)
    //  then get rid of Subtract and Divide
    //  and do this for Vec2Expr, Vec3Expr, and Vec4Expr as well
    Product(Vec<FloatExpr>),
    Sum(Vec<FloatExpr>),
    Divide(Vec<FloatExpr>),
    Pow(Box<FloatExpr>, Box<FloatExpr>),
    // /* Use Pow instead of Sqrt */ Sqrt(Box<FloatExpr>),
    // TODO trig? floor? log? round? trunc? mix? step? smoothstep? fma? fract? modf?
}
#[derive(PartialEq, Clone, Debug)]
pub enum Vec2Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather2(FloatExpr, FloatExpr),
    SwizzleVec2(Box<Vec2Expr>, u8, u8),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    // There is an argument to be made that this should contain at least 2 elements,
    // but for now, the consistency of handling everything in a single Vec is nice.
    // Note that this does cause panics and/or Errors if this Vec is provoked
    // when empty, so it is an important part of the simplification methods
    // to ensure no empty Vecs escape.
    Product(Vec<Vec2Expr>),
    Sum(Vec<Vec2Expr>),
}
#[derive(PartialEq, Clone, Debug)]
pub enum Vec3Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather3(FloatExpr, FloatExpr, FloatExpr),
    SwizzleVec3(Box<Vec3Expr>, u8, u8, u8),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    // There is an argument to be made that this should contain at least 2 elements,
    // but for now, the consistency of handling everything in a single Vec is nice.
    // Note that this does cause panics and/or Errors if this Vec is provoked
    // when empty, so it is an important part of the simplification methods
    // to ensure no empty Vecs escape.
    Product(Vec<Vec3Expr>),
    Sum(Vec<Vec3Expr>),
}
#[derive(PartialEq, Clone, Debug)]
pub enum Vec4Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather4(FloatExpr, FloatExpr, FloatExpr, FloatExpr),
    SwizzleVec4(Box<Vec4Expr>, u8, u8, u8, u8),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    // There is an argument to be made that this should contain at least 2 elements,
    // but for now, the consistency of handling everything in a single Vec is nice.
    // Note that this does cause panics and/or Errors if this Vec is provoked
    // when empty, so it is an important part of the simplification methods
    // to ensure no empty Vecs escape.
    Product(Vec<Vec4Expr>),
    Sum(Vec<Vec4Expr>),
}
#[derive(PartialEq, Clone, Debug)]
pub enum MultiVectorGroupExpr {
    JustFloat(FloatExpr),
    Vec2(Vec2Expr),
    Vec3(Vec3Expr),
    Vec4(Vec4Expr)
}
#[derive(PartialEq, Clone, Debug)]
pub struct MultiVectorExpr {
    pub mv_class: MultiVector,
    pub expr: Box<MultiVectorVia>
}
#[derive(PartialEq, Clone, Debug)]
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


#[derive(Clone, Debug, PartialEq)]
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

    pub fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            AnyExpression::Int(i) => i.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Float(f) => f.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Vec2(v2) => v2.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Vec3(v3) => v3.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Vec4(v4) => v4.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Class(c) => c.substitute_variable(old.clone(), new.clone()),
        }
    }
}



pub trait Expression<ExprType>: Send + Sized {
    fn into_any_expression(self) -> AnyExpression;

    fn from_any_expression(any: AnyExpression) -> Option<Self>;
    fn strong_expression_type(&self) -> ExprType;
    fn type_from_any(any: &AnyExpression) -> Option<ExprType>;
    fn try_into_variable(&self) -> Option<Variable<ExprType>>;

    // TODO it seems this method is not used
    //  Well, ExpressionType is used. So hold off deleting this until you're sure you don't need it.
    //  I think I'll have a better idea after I've actually emitted any code.
    fn soft_expression_type(&self) -> ExpressionType;

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>);
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

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Int(i) => Some(i),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Integer {
        Integer
    }

    fn type_from_any(any: &AnyExpression) -> Option<Integer> {
        match any {
            AnyExpression::Int(_) => Some(Integer),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Integer>> {
        match self {
            IntExpr::Variable(v) => {
                Some(Variable {
                    expr_type: Integer,
                    decl: v.decl.clone(),
                })
            }
            _ => None
        }
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Int(Integer)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            IntExpr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            IntExpr::Literal(_) => {}
            IntExpr::TraitInvoke10ToInt(_, _) => {}
        }
    }
}
impl Expression<Float> for FloatExpr {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Float(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Float(f) => Some(f),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Float {
        Float
    }

    fn type_from_any(any: &AnyExpression) -> Option<Float> {
        match any {
            AnyExpression::Float(_) => Some(Float),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Float>> {
        match self {
            FloatExpr::Variable(v) => {
                Some(Variable {
                    expr_type: Float,
                    decl: v.decl.clone(),
                })
            }
            _ => None
        }
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Float(Float)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            FloatExpr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            FloatExpr::Literal(_) => {}
            FloatExpr::AccessVec2(v, _) => v.substitute_variable(old.clone(), new.clone()),
            FloatExpr::AccessVec3(v, _) => v.substitute_variable(old.clone(), new.clone()),
            FloatExpr::AccessVec4(v, _) => v.substitute_variable(old.clone(), new.clone()),
            FloatExpr::TraitInvoke11ToFloat(_, mvc) => mvc.substitute_variable(old.clone(), new.clone()),
            FloatExpr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            FloatExpr::AccessMultiVecFlat(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            FloatExpr::Product(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            FloatExpr::Sum(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            FloatExpr::Divide(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }}
            FloatExpr::Pow(f1, f2) => {
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
            }
        }
    }
}
impl Expression<Vec2> for Vec2Expr {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec2(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec2(f) => Some(f),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Vec2 {
        Vec2
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec2> {
        match any {
            AnyExpression::Vec2(_) => Some(Vec2),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec2>> {
        match self {
            Vec2Expr::Variable(v) => {
                Some(Variable {
                    expr_type: Vec2,
                    decl: v.decl.clone(),
                })
            }
            _ => None
        }
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec2(Vec2)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            Vec2Expr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            Vec2Expr::Gather1(f) => f.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::Gather2(f1, f2) => {
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
            }
            Vec2Expr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::Product(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec2Expr::Sum(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec2Expr::SwizzleVec2(v, _, _) => v.substitute_variable(old.clone(), new.clone()),
        }
    }
}
impl Expression<Vec3> for Vec3Expr {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec3(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec3(f) => Some(f),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Vec3 {
        Vec3
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec3> {
        match any {
            AnyExpression::Vec3(_) => Some(Vec3),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec3>> {
        match self {
            Vec3Expr::Variable(v) => {
                Some(Variable {
                    expr_type: Vec3,
                    decl: v.decl.clone(),
                })
            }
            _ => None
        }
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec3(Vec3)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            Vec3Expr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            Vec3Expr::Gather1(f) => f.substitute_variable(old.clone(), new.clone()),
            Vec3Expr::Gather3(f1, f2, f3) => {
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
                f3.substitute_variable(old.clone(), new.clone());
            }
            Vec3Expr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            Vec3Expr::Product(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec3Expr::Sum(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec3Expr::SwizzleVec3(v, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
        }
    }
}
impl Expression<Vec4> for Vec4Expr {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec4(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec4(f) => Some(f),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Vec4 {
        Vec4
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec4> {
        match any {
            AnyExpression::Vec4(_) => Some(Vec4),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec4>> {
        match self {
            Vec4Expr::Variable(v) => {
                Some(Variable {
                    expr_type: Vec4,
                    decl: v.decl.clone(),
                })
            }
            _ => None
        }
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec4(Vec4)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            Vec4Expr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            Vec4Expr::Gather1(f) => f.substitute_variable(old.clone(), new.clone()),
            Vec4Expr::Gather4(f1, f2, f3, f4) => {
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
                f3.substitute_variable(old.clone(), new.clone());
                f4.substitute_variable(old.clone(), new.clone());
            }
            Vec4Expr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            Vec4Expr::Product(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec4Expr::Sum(v) => {
                for v in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec4Expr::SwizzleVec4(v, _, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
        }
    }
}
impl Expression<MultiVector> for MultiVectorExpr {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Class(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Class(mv) => Some(mv),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> MultiVector {
        self.mv_class.clone()
    }

    fn type_from_any(any: &AnyExpression) -> Option<MultiVector> {
        match any {
            AnyExpression::Class(mve) => Some(mve.mv_class.clone()),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<MultiVector>> {
        match self.expr.as_ref() {
            MultiVectorVia::Variable(v) => {
                Some(Variable {
                    expr_type: self.mv_class,
                    decl: v.decl.clone(),
                })
            }
            _ => None
        }
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Class(self.strong_expression_type())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self.expr.as_mut() {
            MultiVectorVia::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            MultiVectorVia::Construct(stuff) => {
                for stuff in stuff {
                    match stuff {
                        MultiVectorGroupExpr::JustFloat(f) => f.substitute_variable(old.clone(), new.clone()),
                        MultiVectorGroupExpr::Vec2(v) => v.substitute_variable(old.clone(), new.clone()),
                        MultiVectorGroupExpr::Vec3(v) => v.substitute_variable(old.clone(), new.clone()),
                        MultiVectorGroupExpr::Vec4(v) => v.substitute_variable(old.clone(), new.clone()),
                    }
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_, a) => a.substitute_variable(old.clone(), new.clone()),
            MultiVectorVia::TraitInvoke21ToClass(_, a, _) => a.substitute_variable(old.clone(), new.clone()),
            MultiVectorVia::TraitInvoke22ToClass(_, a, b) => {
                a.substitute_variable(old.clone(), new.clone());
                b.substitute_variable(old.clone(), new.clone());
            }
        }
    }
}

impl Expression<Integer> for Variable<Integer> {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Int(IntExpr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(Variable {
                expr_type: Integer,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Integer {
        Integer
    }

    fn type_from_any(any: &AnyExpression) -> Option<Integer> {
        match any {
            AnyExpression::Int(IntExpr::Variable(_)) => Some(Integer),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Integer>> {
        Some(self.clone())
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Int(Integer)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Float> for Variable<Float> {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Float(FloatExpr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(Variable {
                expr_type: Float,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Float {
        Float
    }

    fn type_from_any(any: &AnyExpression) -> Option<Float> {
        match any {
            AnyExpression::Float(FloatExpr::Variable(_)) => Some(Float),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Float>> {
        Some(self.clone())
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Float(Float)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Vec2> for Variable<Vec2> {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec2(Vec2Expr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(Variable {
                expr_type: Vec2,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Vec2 {
        Vec2
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec2> {
        match any {
            AnyExpression::Vec2(Vec2Expr::Variable(_)) => Some(Vec2),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec2>> {
        Some(self.clone())
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec2(Vec2)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Vec3> for Variable<Vec3> {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec3(Vec3Expr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(Variable {
                expr_type: Vec3,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Vec3 {
        Vec3
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec3> {
        match any {
            AnyExpression::Vec3(Vec3Expr::Variable(_)) => Some(Vec3),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec3>> {
        Some(self.clone())
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec3(Vec3)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Vec4> for Variable<Vec4> {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec4(Vec4Expr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(Variable {
                expr_type: Vec4,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> Vec4 {
        Vec4
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec4> {
        match any {
            AnyExpression::Vec4(Vec4Expr::Variable(_)) => Some(Vec4),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec4>> {
        Some(self.clone())
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Vec4(Vec4)
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<MultiVector> for Variable<MultiVector> {

    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Class(MultiVectorExpr {
            mv_class: self.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: self.decl.clone() })),
        })
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Class(MultiVectorExpr { mv_class, expr, }) => {
                if let MultiVectorVia::Variable(var) = *expr {
                    Some(Variable {
                        expr_type: mv_class,
                        decl: var.decl.clone(),
                    })
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn strong_expression_type(&self) -> MultiVector {
        self.expr_type.clone()
    }

    fn type_from_any(any: &AnyExpression) -> Option<MultiVector> {
        match any {
            AnyExpression::Class(MultiVectorExpr { mv_class, expr, }) => {
                if let MultiVectorVia::Variable(_) = **expr {
                    Some(mv_class.clone())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<MultiVector>> {
        Some((*self).clone())
    }

    fn soft_expression_type(&self) -> ExpressionType {
        ExpressionType::Class(self.strong_expression_type())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}

impl Variable<MultiVector> {
    pub fn elements_flat(&self) -> impl Iterator<Item=(FloatExpr, BasisElement)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type.elements().into_iter().enumerate().map(move |(i, el)| {
            (FloatExpr::AccessMultiVecFlat(mv_expr.clone(), i as u16), el)
        })
    }

    pub fn groups(&self) -> impl Iterator<Item=(MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type.groups().into_iter().enumerate().map(move |(g, group)| {
            let g = g as u16;
            match group {
                BasisElementGroup::G1(a) => (
                    MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(mv_expr.clone(), g)),
                    BasisElementGroup::G1(a),
                ),
                BasisElementGroup::G2(a, b) => (
                    MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g)),
                    BasisElementGroup::G2(a, b),
                ),
                BasisElementGroup::G3(a, b, c) => (
                    MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)),
                    BasisElementGroup::G3(a, b, c),
                ),
                BasisElementGroup::G4(a, b, c, d) => (
                    MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)),
                    BasisElementGroup::G4(a, b, c, d),
                ),
            }
        })
    }

    pub fn elements_by_groups(&self) -> impl Iterator<Item=(FloatExpr, BasisElement)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type.groups().into_iter().enumerate().map(move |(g, group)| {
            let g = g as u16;
            let mut v = vec![];
            match group {
                BasisElementGroup::G1(a) => {
                    v.push((
                        FloatExpr::AccessMultiVecGroup(mv_expr.clone(), g),
                        a,
                    ));
                },
                BasisElementGroup::G2(a, b) => {
                    v.push((
                        FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 0),
                        a,
                    ));
                    v.push((
                        FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 1),
                        b,
                    ));
                },
                BasisElementGroup::G3(a, b, c) => {
                    v.push((
                        FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 0),
                        a,
                    ));
                    v.push((
                        FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 1),
                        b,
                    ));
                    v.push((
                        FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 2),
                        c,
                    ));
                },
                BasisElementGroup::G4(a, b, c, d) => {
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 0),
                        a,
                    ));
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 1),
                        b,
                    ));
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 2),
                        c,
                    ));
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 3),
                        d,
                    ));
                },
            }
            v.into_iter()
        }).flatten()
    }
}

impl MultiVectorExpr {
    pub fn elements_flat(&self) -> impl Iterator<Item=(FloatExpr, BasisElement)> + '_ {
        self.mv_class.elements().into_iter().enumerate().map(move |(i, el)| {
            (FloatExpr::AccessMultiVecFlat(self.clone(), i as u16), el)
        })
    }

    pub fn groups(&self) -> impl Iterator<Item=(MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        self.mv_class.groups().into_iter().enumerate().map(move |(g, group)| {
            let g = g as u16;
            match group {
                BasisElementGroup::G1(a) => (
                    MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(self.clone(), g)),
                    BasisElementGroup::G1(a),
                ),
                BasisElementGroup::G2(a, b) => (
                    MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(self.clone(), g)),
                    BasisElementGroup::G2(a, b),
                ),
                BasisElementGroup::G3(a, b, c) => (
                    MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(self.clone(), g)),
                    BasisElementGroup::G3(a, b, c),
                ),
                BasisElementGroup::G4(a, b, c, d) => (
                    MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(self.clone(), g)),
                    BasisElementGroup::G4(a, b, c, d),
                ),
            }
        })
    }
    pub fn elements_by_groups(&self) -> impl Iterator<Item=(FloatExpr, BasisElement)> + '_ {
        self.mv_class.groups().into_iter().enumerate().map(move |(g, group)| {
            let g = g as u16;
            let mut v = vec![];
            match group {
                BasisElementGroup::G1(a) => {
                    v.push((
                        FloatExpr::AccessMultiVecGroup(self.clone(), g),
                        a,
                    ));
                },
                BasisElementGroup::G2(a, b) => {
                    v.push((
                        FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(self.clone(), g)), 0),
                        a,
                    ));
                    v.push((
                        FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(self.clone(), g)), 1),
                        b,
                    ));
                },
                BasisElementGroup::G3(a, b, c) => {
                    v.push((
                        FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), 0),
                        a,
                    ));
                    v.push((
                        FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), 1),
                        b,
                    ));
                    v.push((
                        FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), 2),
                        c,
                    ));
                },
                BasisElementGroup::G4(a, b, c, d) => {
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 0),
                        a,
                    ));
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 1),
                        b,
                    ));
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 2),
                        c,
                    ));
                    v.push((
                        FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 3),
                        d,
                    ));
                },
            }
            v.into_iter()
        }).flatten()
    }

    // TODO get rid of this after I write "strong_inline" on traits
    pub(crate) fn distributive_by_groups<
        F1: Fn(FloatExpr, FloatExpr) -> FloatExpr,
        F2: Fn(BasisElement, BasisElement) -> BasisElement
    >(&self, other: &Self, f1: F1, f2: F2) -> DynamicMultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in self.elements_by_groups() {
            for (b, b_el) in other.elements_by_groups() {
                let a = a.clone();
                dyn_mv += (f1(a, b), f2(a_el, b_el));
            }
        }
        dyn_mv
    }
}

impl From<Variable<Float>> for FloatExpr {
    fn from(value: Variable<Float>) -> Self {
        FloatExpr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}
impl From<f32> for FloatExpr {
    fn from(value: f32) -> Self {
        FloatExpr::Literal(value)
    }
}
impl From<Variable<Vec2>> for Vec2Expr {
    fn from(value: Variable<Vec2>) -> Self {
        Vec2Expr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}
impl From<[f32; 2]> for Vec2Expr {
    fn from(value: [f32; 2]) -> Self {
        let mut v = Vec2Expr::Gather2(FloatExpr::Literal(value[0]), FloatExpr::Literal(value[1]));
        v.simplify();
        v
    }
}
impl From<Variable<Vec3>> for Vec3Expr {
    fn from(value: Variable<Vec3>) -> Self {
        Vec3Expr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}
impl From<[f32; 3]> for Vec3Expr {
    fn from(value: [f32; 3]) -> Self {
        let mut v = Vec3Expr::Gather3(FloatExpr::Literal(value[0]), FloatExpr::Literal(value[1]), FloatExpr::Literal(value[2]));
        v.simplify();
        v
    }
}
impl From<Variable<Vec4>> for Vec4Expr {
    fn from(value: Variable<Vec4>) -> Self {
        Vec4Expr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}
impl From<[f32; 4]> for Vec4Expr {
    fn from(value: [f32; 4]) -> Self {
        let mut v = Vec4Expr::Gather4(FloatExpr::Literal(value[0]), FloatExpr::Literal(value[1]), FloatExpr::Literal(value[2]), FloatExpr::Literal(value[3]));
        v.simplify();
        v
    }
}
impl From<Variable<MultiVector>> for MultiVectorExpr {
    fn from(value: Variable<MultiVector>) -> Self {
        MultiVectorExpr {
            mv_class: value.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation {
                decl: value.decl.clone(),
            })),
        }
    }
}


impl Display for FloatExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatExpr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            FloatExpr::Literal(l) => {
                write!(f, "{l}")?;
            }
            FloatExpr::AccessVec2(v, i) => todo!(),
            FloatExpr::AccessVec3(v, i) => todo!(),
            FloatExpr::AccessVec4(v, i) => todo!(),
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                let gs: Vec<_> = mv.groups().collect();
                let (grp, el) = &gs[*i as usize];
                match (grp, el) {
                    (MultiVectorGroupExpr::JustFloat(v), BasisElementGroup::G1(el)) => {
                        write!(f, "{el}({v})")?;
                    }
                    _ => unreachable!("Should not be able to access FloatExpr as MultiVecGroup \
                    unless the MultiVecGroup is just one Float"),
                }
            },
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                let gs: Vec<_> = mv.elements_flat().collect();
                let (float, el) = &gs[*i as usize];
                write!(f, "{el}({float})")?;
            }
            FloatExpr::TraitInvoke11ToFloat(_, _) => {}
            FloatExpr::Product(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
            FloatExpr::Sum(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
            FloatExpr::Divide(v) => todo!(),
            FloatExpr::Pow(_, _) => todo!(),
        }
        Ok(())
    }
}
impl Display for Vec2Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec2Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            Vec2Expr::Gather1(f0) => {
                write!(f, "[{f0}, {f0}]")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(f, "[{f0}, {f1}]")?;
            }
            Vec2Expr::SwizzleVec2(_, _, _) => {}
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                let gs: Vec<_> = mv.groups().collect();
                let (grp, el) = &gs[*i as usize];
                match (grp, el) {
                    (MultiVectorGroupExpr::Vec2(v),
                        BasisElementGroup::G2(el0, el1)) => {
                        write!(f, "({el0}, {el1})({v})")?;
                    }
                    _ => unreachable!("Should not be able to access Vec2Expr as MultiVecGroup \
                    unless the MultiVecGroup is Vec2"),
                }
            },
            Vec2Expr::Product(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
            Vec2Expr::Sum(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Display for Vec3Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec3Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            Vec3Expr::Gather1(f0) => {
                write!(f, "[{f0}, {f0}, {f0}]")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(f, "[{f0}, {f1}, {f2}]")?;
            }
            Vec3Expr::SwizzleVec3(_, _, _, _) => {}
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                let gs: Vec<_> = mv.groups().collect();
                let (grp, el) = &gs[*i as usize];
                match (grp, el) {
                    (MultiVectorGroupExpr::Vec3(v),
                        BasisElementGroup::G3(el0, el1, el2)) => {
                        write!(f, "({el0}, {el1}, {el2})({v})")?;
                    }
                    _ => unreachable!("Should not be able to access Vec3Expr as MultiVecGroup \
                    unless the MultiVecGroup is Vec3"),
                }
            },
            Vec3Expr::Product(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
            Vec3Expr::Sum(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Display for Vec4Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec4Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            Vec4Expr::Gather1(f0) => {
                write!(f, "[{f0}, {f0}, {f0}, {f0}]")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(f, "[{f0}, {f1}, {f2}, {f3}]")?;
            }
            Vec4Expr::SwizzleVec4(_, _, _, _, _) => {}
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                let gs: Vec<_> = mv.groups().collect();
                let (grp, el) = &gs[*i as usize];
                match (grp, el) {
                    (MultiVectorGroupExpr::Vec4(v),
                        BasisElementGroup::G4(el0, el1, el2, el3)) => {
                        write!(f, "({el0}, {el1}, {el2}, {el3})({v})")?;
                    }
                    _ => unreachable!("Should not be able to access Vec4Expr as MultiVecGroup \
                    unless the MultiVecGroup is Vec4"),
                }
            },
            Vec4Expr::Product(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
            Vec4Expr::Sum(v) => {
                write!(f, "(")?;
                for (i, factor) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{factor}")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Display for MultiVectorGroupExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiVectorGroupExpr::JustFloat(v) => write!(f, "{v}")?,
            MultiVectorGroupExpr::Vec2(v) => write!(f, "{v}")?,
            MultiVectorGroupExpr::Vec3(v) => write!(f, "{v}")?,
            MultiVectorGroupExpr::Vec4(v) => write!(f, "{v}")?,
        }
        Ok(())
    }
}

impl Display for MultiVectorExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.mv_class.name();
        write!(f, "{n}( ")?;
        let via = self.expr.as_ref();
        match via {
            MultiVectorVia::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            MultiVectorVia::Construct(v) => {
                let mut gs = self.mv_class.groups().into_iter();
                for (i, expr) in v.iter().enumerate() {
                    let group = gs.next().expect("zipping");
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{group}({expr})")?;
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_, _) => todo!(),
            MultiVectorVia::TraitInvoke21ToClass(_, _, _) => todo!(),
            MultiVectorVia::TraitInvoke22ToClass(_, _, _) => todo!(),
        }
        write!(f, " )")?;
        Ok(())
    }
}




// TODO convert all the validation to use asserts that only run at debug time
const DEBUG_PRINT_SIMPLIFY: bool = false;


// TODO simplification todo list
//  - impl Wedge<AntiFlectorOnOrigin> for AntiPlane
//    might be interesting to extend Simd32x3 to Simd32x4
//    should accept a float to fill in the slot
//  - impl Wedge<RoundPoint> for AntiMotorAligningOrigin
//    same as previous
//  - impl Wedge<AntiFlector> for RoundPoint
//    not sure if this can be improved, but it's sure tempting/interesting
//  - impl Wedge<AntiMotorOnOrigin> for AntiFlectorAtInfinity
//    there is a gather3 of float sum of products that would be great to simplify
//  - impl Wedge<AntiSphereOnOrigin> for MultiVector
//    another vec3 extend to vec4 situation
//  - impl Wedge<AntiCircleOnOrigin> for AntiFlectorAtInfinity
//    another vec3 extend to vec4 situation
//  - impl GeometricAntiProduct<Line> for Flector
//  - impl GeometricAntiProduct<MultiVector> for MultiVector

// TODO scanning for good places to simplify... where I left off:
//  impl Wedge<AntiCircleOnOrigin> for AntiFlectorAtInfinity

// TODO definitely want a particularly close scan of
//  geometric_anti_product.rs `for Sphere`
/*
    //  impl GeometricAntiProduct<FlatPoint> for Sphere
    /* e415, e425, e435 */
    (Simd32x3::from(-1)
        * Simd32x3::from([
            ((self.group0()[0] * other.group0()[3]) + (self.group1()[0] * other.group0()[0])),
            ((self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
            ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[2])),
        ])),
*/
impl FloatExpr {
    pub(crate) fn simplify(&mut self) {
        if DEBUG_PRINT_SIMPLIFY {
            eprintln!("{self:?}");
        }
        match self {
            FloatExpr::Variable(_) => {}
            FloatExpr::Literal(_) => {}
            FloatExpr::AccessVec2(av2, idx) => {
                av2.simplify();
                match av2.as_mut() {
                    Vec2Expr::Gather1(fe) => {
                        *self = fe.clone();
                    }
                    Vec2Expr::Gather2(fe0, fe1) => {
                        *self = [fe0, fe1][*idx as usize].clone();
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec3(av3, idx) => {
                av3.simplify();
                match av3.as_mut() {
                    Vec3Expr::Gather1(fe) => {
                        *self = fe.clone();
                    }
                    Vec3Expr::Gather3(fe0, fe1, fe2) => {
                        *self = [fe0, fe1, fe2][*idx as usize].clone();
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec4(av4, idx) => {
                av4.simplify();
                match av4.as_mut() {
                    Vec4Expr::Gather1(fe) => {
                        *self = fe.clone();
                    }
                    Vec4Expr::Gather4(fe0, fe1, fe2, fe3) => {
                        *self = [fe0, fe1, fe2, fe3][*idx as usize].clone();
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessMultiVecGroup(mve, idx) => {
                mve.simplify();
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(f) => {
                            *self = f.clone();
                            1
                        }
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 1 {
                        panic!("Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 1. {mv}")
                    }
                }
            }
            FloatExpr::AccessMultiVecFlat(mve, idx) => {
                mve.simplify();
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let mut scan_idx = 0;
                    let mut scan_group = 0;
                    while scan_group < groups.len() {
                        let i = (*idx as i32) - scan_idx;
                        if i < 0 {
                            // This can happen if the index is valid but does not simplify
                            break
                        }
                        match &mut groups[scan_group] {
                            MultiVectorGroupExpr::JustFloat(f) => {
                                if i == 0 {
                                    *self = f.clone();
                                    return
                                }
                                scan_idx += 1;
                            }
                            MultiVectorGroupExpr::Vec2(v2) => {
                                if i < 2 {
                                    match v2 {
                                        Vec2Expr::Gather1(f) => {
                                            *self = f.clone();
                                            return
                                        }
                                        Vec2Expr::Gather2(f0, f1) => {
                                            *self = [f0, f1][i as usize].clone();
                                            return
                                        }
                                        _ => {}
                                    }
                                }
                                scan_idx += 2;
                            }
                            MultiVectorGroupExpr::Vec3(v3) => {
                                if i < 3 {
                                    match v3 {
                                        Vec3Expr::Gather1(f) => {
                                            *self = f.clone();
                                            return
                                        }
                                        Vec3Expr::Gather3(f0, f1, f2) => {
                                            *self = [f0, f1, f2][i as usize].clone();
                                            return
                                        }
                                        _ => {}
                                    }
                                }
                                scan_idx += 3;
                            }
                            MultiVectorGroupExpr::Vec4(v4) => {
                                if i < 4 {
                                    match v4 {
                                        Vec4Expr::Gather1(f) => {
                                            *self = f.clone();
                                            return
                                        }
                                        Vec4Expr::Gather4(f0, f1, f2, f3) => {
                                            *self = [f0, f1, f2, f3][i as usize].clone();
                                            return
                                        }
                                        _ => {}
                                    }
                                }
                                scan_idx += 4;
                            }
                        }
                        scan_group += 1;
                    }
                }
            }
            FloatExpr::TraitInvoke11ToFloat(_, _) => {}
            FloatExpr::Product(product) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for p in  product.iter_mut() {
                    p.simplify();
                    if let FloatExpr::Literal(0.0) = p {
                        *self = FloatExpr::Literal(0.0);
                        return
                    }
                }
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                let mut coalesce = 1.0;
                let mut contained_one = false;
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let FloatExpr::Literal(f) = it {
                        contained_one |= *f == 1.0;
                        coalesce *= *f;
                        false
                    } else if let FloatExpr::Product(ref mut p) = it {
                        flatten.append(p);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let FloatExpr::Literal(f) = it {
                        contained_one |= *f == 1.0;
                        coalesce *= f;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != 1.0 {
                    product.push(FloatExpr::Literal(coalesce));
                }
                product.append(&mut flatten);
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                if product.is_empty() && contained_one {
                    *self = FloatExpr::Literal(1.0);
                    return
                }
                if product.is_empty() {
                    panic!("Problem")
                }
            }
            FloatExpr::Sum(sum) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for s in  sum.iter_mut() {
                    s.simplify();
                }
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                let mut coalesce = 0.0;
                let mut contained_zero = false;
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let FloatExpr::Literal(f) = it {
                        contained_zero = *f == 0.0;
                        coalesce += *f;
                        false
                    } else if let FloatExpr::Sum(ref mut s) = it {
                        flatten.append(s);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let FloatExpr::Literal(f) = it {
                        contained_zero = *f == 0.0;
                        coalesce += f;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != 0.0 {
                    sum.push(FloatExpr::Literal(coalesce));
                }
                sum.append(&mut flatten);
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                if sum.is_empty() && contained_zero {
                    *self = FloatExpr::Literal(0.0);
                    return
                }
                if sum.is_empty() {
                    panic!("Problem")
                }
            }
            FloatExpr::Divide(_) => {}
            FloatExpr::Pow(f1, f2) => {
                f1.simplify();
                f2.simplify();
                match f2.as_mut() {
                    FloatExpr::Literal(0.0) => {
                        *self = FloatExpr::Literal(1.0);
                        return
                    }
                    FloatExpr::Literal(1.0) => {
                        *self = f1.as_ref().clone();
                        return
                    }
                    _ => {}
                }
                match f1.as_mut() {
                    FloatExpr::Literal(0.0) => {
                        *self = FloatExpr::Literal(0.0);
                        return
                    }
                    FloatExpr::Literal(1.0) => {
                        *self = FloatExpr::Literal(1.0);
                        return
                    }
                    FloatExpr::Pow(f3, f4) => {
                        let mut new_pow = FloatExpr::Product(vec![*f2.clone(), *f4.clone()]);
                        new_pow.simplify();
                        *self = FloatExpr::Pow(f3.clone(), Box::new(new_pow));
                        return
                    }
                    _ => {}
                }
            }
        }
    }
}
impl Vec2Expr {
    pub(crate) fn simplify(&mut self) {
        if DEBUG_PRINT_SIMPLIFY {
            eprintln!("{self:?}");
        }
        match self {
            Vec2Expr::Variable(_) => {}
            Vec2Expr::Gather1(ref mut f) => {
                f.simplify();
                // Do I really want to do more here?
            }
            Vec2Expr::Gather2(ref mut f0, ref mut f1) => {
                use crate::ast2::expressions::FloatExpr::*;
                f0.simplify();
                f1.simplify();
                if f0 == f1 {
                    *self = Vec2Expr::Gather1(f0.clone());
                    return;
                }
                match (f0, f1) {
                    (
                        AccessVec2(box ref mut v2_a, 0),
                        AccessVec2(box ref mut v2_b, 1),
                    ) => {
                        if v2_a == v2_b {
                            *self = v2_a.clone();
                            return;
                        }
                    }
                    (
                        Product(ref mut float_product_0),
                        Product(ref mut float_product_1),
                    ) => {
                        if let Some(transposed) = transpose_vec2_product(
                            float_product_0, float_product_1,
                        ) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut float_sum_0),
                        Sum(ref mut float_sum_1),
                    ) => {
                        if let Some(transposed) = transpose_vec2_sum(
                            float_sum_0, float_sum_1,
                        ) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec2Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                mve.simplify();
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(v2) => {
                            *self = v2.clone();
                            2
                        },
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 2 {
                        panic!("Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 2. {mv}")
                    }
                }
            }
            Vec2Expr::Product(ref mut product) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for p in product.iter_mut() {
                    p.simplify();
                    if let Vec2Expr::Gather1(FloatExpr::Literal(0.0)) = p {
                        *self = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
                        return;
                    }
                    if let Vec2Expr::Gather2(
                        FloatExpr::Literal(0.0),
                        FloatExpr::Literal(0.0)) = p {
                        *self = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
                        return;
                    }
                }
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                let mut coalesce = [1.0, 1.0];
                let mut contained_one = false;
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let Vec2Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_one |= *f == 1.0;
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
                        contained_one |= *f0 == 1.0 && *f1 == 1.0;
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        false
                    } else if let Vec2Expr::Product(ref mut p) = it {
                        flatten.append(p);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let Vec2Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_one |= *f == 1.0;
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
                        contained_one |= *f0 == 1.0 && *f1 == 1.0;
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [1.0, 1.0] {
                    if coalesce[0] == coalesce[1] {
                        product.push(Vec2Expr::Gather1(FloatExpr::Literal(coalesce[0])))
                    } else {
                        product.push(Vec2Expr::Gather2(
                            FloatExpr::Literal(coalesce[0]),
                            FloatExpr::Literal(coalesce[1]),
                        ))
                    }
                }
                product.append(&mut flatten);
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                if product.is_empty() && contained_one {
                    *self = Vec2Expr::Gather1(FloatExpr::Literal(1.0));
                    return
                }
                if product.is_empty() {
                    panic!("Problem")
                }
            }
            Vec2Expr::Sum(ref mut sum) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for s in sum.iter_mut() {
                    s.simplify();
                }
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                let mut coalesce = [0.0, 0.0];
                let mut contained_zero = false;
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let Vec2Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_zero |= *f == 0.0;
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
                        contained_zero |= *f0 == 0.0 && *f1 == 0.0;
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        false
                    } else if let Vec2Expr::Sum(ref mut p) = it {
                        flatten.append(p);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let Vec2Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_zero |= *f == 0.0;
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
                        contained_zero |= *f0 == 0.0 && *f1 == 0.0;
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [0.0, 0.0] {
                    if coalesce[0] == coalesce[1] {
                        sum.push(Vec2Expr::Gather1(FloatExpr::Literal(coalesce[0])))
                    } else {
                        sum.push(Vec2Expr::Gather2(
                            FloatExpr::Literal(coalesce[0]),
                            FloatExpr::Literal(coalesce[1]),
                        ))
                    }
                }
                sum.append(&mut flatten);
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                if sum.is_empty() && contained_zero {
                    *self = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }
                if sum.is_empty() {
                    panic!("Problem")
                }
            }
            Vec2Expr::SwizzleVec2(_, _, _) => {
                // TODO
            }
        }
    }
}
impl Vec3Expr {
    pub(crate) fn simplify(&mut self) {
        if DEBUG_PRINT_SIMPLIFY {
            eprintln!("{self:?}");
        }
        match self {
            Vec3Expr::Variable(_) => {}
            Vec3Expr::Gather1(ref mut f) => {
                f.simplify();
                // Do I really want to do more here?
            }
            Vec3Expr::Gather3(ref mut f0, ref mut f1, ref mut f2) => {
                use crate::ast2::expressions::FloatExpr::*;
                f0.simplify();
                f1.simplify();
                f2.simplify();
                if f0 == f1 && f0 == f2 {
                    *self = Vec3Expr::Gather1(f0.clone());
                    return;
                }
                match (f0, f1, f2) {
                    (
                        AccessVec3(box ref mut v3_a, 0),
                        AccessVec3(box ref mut v3_b, 1),
                        AccessVec3(box ref mut v3_c, 2),
                    ) => {
                        if v3_a == v3_b && v3_a == v3_c {
                            *self = v3_a.clone();
                            return;
                        }
                    }
                    (
                        Product(ref mut float_product_0),
                        Product(ref mut float_product_1),
                        Product(ref mut float_product_2),
                    ) => {
                        if let Some(transposed) = transpose_vec3_product(
                            float_product_0, float_product_1, float_product_2
                        ) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut float_sum_0),
                        Sum(ref mut float_sum_1),
                        Sum(ref mut float_sum_2),
                    ) => {
                        if let Some(transposed) = transpose_vec3_sum(
                            float_sum_0, float_sum_1, float_sum_2
                        ) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec3Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                mve.simplify();
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(v3) => {
                            *self = v3.clone();
                            3
                        },
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 3 {
                        panic!("Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 3. {mv}")
                    }
                }
            }
            Vec3Expr::Product(ref mut product) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for p in product.iter_mut() {
                    p.simplify();
                    if let Vec3Expr::Gather1(FloatExpr::Literal(0.0)) = p {
                        *self = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
                        return;
                    }
                    if let Vec3Expr::Gather3(
                        FloatExpr::Literal(0.0),
                        FloatExpr::Literal(0.0),
                        FloatExpr::Literal(0.0)) = p {
                        *self = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
                        return;
                    }
                }
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                let mut coalesce = [1.0, 1.0, 1.0];
                let mut contained_one = false;
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let Vec3Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_one |= *f == 1.0;
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        coalesce[2] *= *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
                        contained_one |= *f0 == 1.0 && *f1 == 1.0 && *f2 == 1.0;
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        coalesce[2] *= *f2;
                        false
                    } else if let Vec3Expr::Product(ref mut p) = it {
                        flatten.append(p);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let Vec3Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_one |= *f == 1.0;
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        coalesce[2] *= *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
                        contained_one |= *f0 == 1.0 && *f1 == 1.0 && *f2 == 1.0;
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        coalesce[2] *= *f2;
                        false
                    } else {
                        true
                    }
                });
                product.append(&mut flatten);
                if coalesce != [1.0, 1.0, 1.0] {
                    if coalesce[0] == coalesce[1] && coalesce[1] == coalesce[2] {
                        product.push(Vec3Expr::Gather1(FloatExpr::Literal(coalesce[0])))
                    } else {
                        product.push(Vec3Expr::Gather3(
                            FloatExpr::Literal(coalesce[0]),
                            FloatExpr::Literal(coalesce[1]),
                            FloatExpr::Literal(coalesce[2]),
                        ))
                    }
                }
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                if product.is_empty() && contained_one {
                    *self = Vec3Expr::Gather1(FloatExpr::Literal(1.0));
                    return
                }
                if product.is_empty() {
                    panic!("Problem")
                }
            }
            Vec3Expr::Sum(ref mut sum) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for s in sum.iter_mut() {
                    s.simplify();
                }
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                let mut coalesce = [0.0, 0.0, 0.0];
                let mut contained_zero = false;
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let Vec3Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_zero |= *f == 0.0;
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        coalesce[2] += *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
                        contained_zero |= *f0 == 0.0 && *f1 == 0.0 && *f2 == 0.0;
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        coalesce[2] += *f2;
                        false
                    } else if let Vec3Expr::Sum(ref mut p) = it {
                        flatten.append(p);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let Vec3Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_zero |= *f == 0.0;
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        coalesce[2] += *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
                        contained_zero |= *f0 == 0.0 && *f1 == 0.0 && *f2 == 0.0;
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        coalesce[2] += *f2;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [0.0, 0.0, 0.0] {
                    if coalesce[0] == coalesce[1] && coalesce[1] == coalesce[2] {
                        sum.push(Vec3Expr::Gather1(FloatExpr::Literal(coalesce[0])))
                    } else {
                        sum.push(Vec3Expr::Gather3(
                            FloatExpr::Literal(coalesce[0]),
                            FloatExpr::Literal(coalesce[1]),
                            FloatExpr::Literal(coalesce[2]),
                        ))
                    }
                }
                sum.append(&mut flatten);
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                if sum.is_empty() && contained_zero {
                    *self = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }
                if sum.is_empty() {
                    panic!("Problem")
                }
            }
            Vec3Expr::SwizzleVec3(_, _, _, _) => {
                // TODO
            }
        }
    }
}
impl Vec4Expr {
    pub(crate) fn simplify(&mut self) {
        if DEBUG_PRINT_SIMPLIFY {
            eprintln!("{self:?}");
        }
        match self {
            Vec4Expr::Variable(_) => {}
            Vec4Expr::Gather1(f) => {
                f.simplify();
                // Do I really want to do more here?
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                use crate::ast2::expressions::FloatExpr::*;
                f0.simplify();
                f1.simplify();
                f2.simplify();
                f3.simplify();
                if f0 == f1 && f0 == f2 && f0 == f3 {
                    *self = Vec4Expr::Gather1(f0.clone());
                    return;
                }
                match (f0, f1, f2, f3) {
                    (
                        AccessVec4(v4_a, 0),
                        AccessVec4(v4_b, 1),
                        AccessVec4(v4_c, 2),
                        AccessVec4(v4_d, 3),
                    ) => {
                        if v4_a == v4_b && v4_a == v4_c && v4_a == v4_d {
                            *self = *v4_a.clone();
                            return;
                        }
                    }
                    (
                        Product(ref mut float_product_0),
                        Product(ref mut float_product_1),
                        Product(ref mut float_product_2),
                        Product(ref mut float_product_3),
                    ) => {
                        if let Some(transposed) = transpose_vec4_product(
                            float_product_0, float_product_1, float_product_2, float_product_3
                        ) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut float_sum_0),
                        Sum(ref mut float_sum_1),
                        Sum(ref mut float_sum_2),
                        Sum(ref mut float_sum_3),
                    ) => {
                        if let Some(transposed) = transpose_vec4_sum(
                            float_sum_0, float_sum_1, float_sum_2, float_sum_3
                        ) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec4Expr::AccessMultiVecGroup(mve, idx) => {
                mve.simplify();
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(v4) => {
                            *self = v4.clone();
                            4
                        },
                    };
                    if size != 4 {
                        panic!("Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 4. {mv}")
                    }
                }
            }
            Vec4Expr::Product(product) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for p in product.iter_mut() {
                    p.simplify();
                    if let Vec4Expr::Gather1(FloatExpr::Literal(0.0)) = p {
                        *self = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
                        return;
                    }
                    if let Vec4Expr::Gather4(
                        FloatExpr::Literal(0.0),
                        FloatExpr::Literal(0.0),
                        FloatExpr::Literal(0.0),
                        FloatExpr::Literal(0.0)) = p {
                        *self = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
                        return;
                    }
                }
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                let mut coalesce = [1.0, 1.0, 1.0, 1.0];
                let mut contained_one = false;
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let Vec4Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_one |= *f == 1.0;
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        coalesce[2] *= *f;
                        coalesce[3] *= *f;
                        false
                    } else if let Vec4Expr::Gather4(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2),
                        FloatExpr::Literal(f3)) = it {
                        contained_one |= *f0 == 1.0 && *f1 == 1.0 && *f2 == 1.0 && *f3 == 1.0;
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        coalesce[2] *= *f2;
                        coalesce[3] *= *f3;
                        false
                    } else if let Vec4Expr::Product(ref mut p) = it {
                        flatten.append(p);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let Vec4Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_one |= *f == 1.0;
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        coalesce[2] *= *f;
                        coalesce[3] *= *f;
                        false
                    } else if let Vec4Expr::Gather4(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2),
                        FloatExpr::Literal(f3)) = it {
                        contained_one |= *f0 == 1.0 && *f1 == 1.0 && *f2 == 1.0 && *f3 == 1.0;
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        coalesce[2] *= *f2;
                        coalesce[3] *= *f3;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [1.0, 1.0, 1.0, 1.0] {
                    if coalesce[0] == coalesce[1] && coalesce[1] == coalesce[2] && coalesce[2] == coalesce[3] {
                        product.push(Vec4Expr::Gather1(FloatExpr::Literal(coalesce[0])))
                    } else {
                        product.push(Vec4Expr::Gather4(
                            FloatExpr::Literal(coalesce[0]),
                            FloatExpr::Literal(coalesce[1]),
                            FloatExpr::Literal(coalesce[2]),
                            FloatExpr::Literal(coalesce[3]),
                        ))
                    }
                }
                product.append(&mut flatten);
                if product.len() == 1 {
                    *self = product.remove(0);
                    return
                }
                if product.is_empty() && contained_one {
                    *self = Vec4Expr::Gather1(FloatExpr::Literal(1.0));
                    return
                }
                if product.is_empty() {
                    panic!("Problem")
                }
            }
            Vec4Expr::Sum(sum) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for s in sum.iter_mut() {
                    s.simplify();
                }
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                let mut coalesce = [0.0, 0.0, 0.0, 0.0];
                let mut contained_zero = false;
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let Vec4Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_zero |= *f == 0.0;
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        coalesce[2] += *f;
                        coalesce[3] += *f;
                        false
                    } else if let Vec4Expr::Gather4(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2),
                        FloatExpr::Literal(f3)) = it {
                        contained_zero |= *f0 == 0.0 && *f1 == 0.0 && *f2 == 0.0 && *f3 == 0.0;
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        coalesce[2] += *f2;
                        coalesce[3] += *f3;
                        false
                    } else if let Vec4Expr::Sum(ref mut p) = it {
                        flatten.append(p);
                        false
                    } else {
                        true
                    }
                });
                flatten.retain(|it| {
                    if let Vec4Expr::Gather1(FloatExpr::Literal(f)) = it {
                        contained_zero |= *f == 0.0;
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        coalesce[2] += *f;
                        coalesce[3] += *f;
                        false
                    } else if let Vec4Expr::Gather4(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2),
                        FloatExpr::Literal(f3)) = it {
                        contained_zero |= *f0 == 0.0 && *f1 == 0.0 && *f2 == 0.0 && *f3 == 0.0;
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        coalesce[2] += *f2;
                        coalesce[3] += *f3;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [0.0, 0.0, 0.0, 0.0] {
                    if coalesce[0] == coalesce[1] && coalesce[1] == coalesce[2] && coalesce[2] == coalesce[3] {
                        sum.push(Vec4Expr::Gather1(FloatExpr::Literal(coalesce[0])))
                    } else {
                        sum.push(Vec4Expr::Gather4(
                            FloatExpr::Literal(coalesce[0]),
                            FloatExpr::Literal(coalesce[1]),
                            FloatExpr::Literal(coalesce[2]),
                            FloatExpr::Literal(coalesce[3]),
                        ))
                    }
                }
                sum.append(&mut flatten);
                if sum.len() == 1 {
                    *self = sum.remove(0);
                    return
                }
                if sum.is_empty() && contained_zero {
                    *self = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }
                if sum.is_empty() {
                    panic!("Problem")
                }
            }
            Vec4Expr::SwizzleVec4(_, _, _, _, _) => {
                // TODO
            }
        }
    }
}
impl MultiVectorGroupExpr {
    pub(crate) fn simplify(&mut self) {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => {
                f.simplify();
                if let FloatExpr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = f {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].clone();
                    }
                }
            }
            MultiVectorGroupExpr::Vec2(v2) => {
                v2.simplify();
                if let Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v2 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].clone();
                    }
                }
            }
            MultiVectorGroupExpr::Vec3(v3) => {
                v3.simplify();
                if let Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v3 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].clone();
                    }
                }
            }
            MultiVectorGroupExpr::Vec4(v4) => {
                v4.simplify();
                if let Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v4 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].clone();
                    }
                }
            }
        }
    }
}
impl MultiVectorExpr {
    pub(crate) fn simplify(&mut self) {
        if DEBUG_PRINT_SIMPLIFY {
            eprintln!("{self:?}");
        }
        match &mut *self.expr {
            MultiVectorVia::Variable(_) => {}
            MultiVectorVia::Construct(groups) => {
                for group in groups.iter_mut() {
                    group.simplify();
                }
                let result = groups.iter()
                    .enumerate()
                    .fold(None, |a, (b_idx, b)| {
                        let mv_b = match b {
                            MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(mv, idx)) if *idx as usize == b_idx => Some(mv),
                            MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(mv, idx)) if *idx as usize == b_idx => Some(mv),
                            MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(mv, idx)) if *idx as usize == b_idx => Some(mv),
                            MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(mv, idx)) if *idx as usize == b_idx => Some(mv),
                            _ => None,
                        };
                        if b_idx == 0 {
                            return mv_b;
                        }
                        let a = a?;
                        let b = mv_b?;
                        if a == b {
                            Some(a)
                        } else {
                            None
                        }
                    });
                if let Some(result) = result {
                    *self = result.clone();
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_, _) => {}
            MultiVectorVia::TraitInvoke21ToClass(_, _, _) => {}
            MultiVectorVia::TraitInvoke22ToClass(_, _, _) => {}
        }
    }
}


impl<FE: Into<FloatExpr>> Add<FE> for FloatExpr {
    type Output = FloatExpr;

    fn add(self, rhs: FE) -> Self::Output {
        let rhs = rhs.into();
        let mut s = FloatExpr::Sum(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<FE: Into<FloatExpr>> AddAssign<FE> for FloatExpr {
    fn add_assign(&mut self, rhs: FE) {
        let rhs = rhs.into();
        let mut x = FloatExpr::Literal(0.0);
        mem::swap(&mut x, self);
        *self = FloatExpr::Sum(vec![x, rhs]);
        self.simplify();
    }
}
impl<FE: Into<FloatExpr>> Mul<FE> for FloatExpr {
    type Output = FloatExpr;

    fn mul(self, rhs: FE) -> Self::Output {
        let rhs = rhs.into();
        let mut s = FloatExpr::Product(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<FE: Into<FloatExpr>> MulAssign<FE> for FloatExpr {
    fn mul_assign(&mut self, rhs: FE) {
        let rhs = rhs.into();
        let mut x = FloatExpr::Literal(1.0);
        mem::swap(&mut x, self);
        *self = FloatExpr::Product(vec![x, rhs]);
        self.simplify();
    }
}
impl Neg for FloatExpr {
    type Output = FloatExpr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(FloatExpr::Literal(-1.0));
        result.simplify();
        result
    }
}
impl Sub for FloatExpr {
    type Output = FloatExpr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for FloatExpr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}



impl<V: Into<Vec2Expr>> Add<V> for Vec2Expr {
    type Output = Vec2Expr;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec2Expr::Sum(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec2Expr>> AddAssign<V> for Vec2Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec2Expr::Sum(vec![x, rhs]);
        self.simplify();
    }
}
impl<V: Into<Vec2Expr>> Mul<V> for Vec2Expr {
    type Output = Vec2Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec2Expr::Product(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec2Expr>> MulAssign<V> for Vec2Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec2Expr::Product(vec![x, rhs]);
        self.simplify();
    }
}
impl Neg for Vec2Expr {
    type Output = Vec2Expr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(Vec2Expr::Gather1(FloatExpr::Literal(-1.0)));
        result.simplify();
        result
    }
}
impl Sub for Vec2Expr {
    type Output = Vec2Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for Vec2Expr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}



impl<V: Into<Vec3Expr>> Add<V> for Vec3Expr {
    type Output = Vec3Expr;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec3Expr::Sum(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec3Expr>> AddAssign<V> for Vec3Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec3Expr::Sum(vec![x, rhs]);
        self.simplify();
    }
}
impl<V: Into<Vec3Expr>> Mul<V> for Vec3Expr {
    type Output = Vec3Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec3Expr::Product(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec3Expr>> MulAssign<V> for Vec3Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec3Expr::Product(vec![x, rhs]);
        self.simplify();
    }
}
impl Neg for Vec3Expr {
    type Output = Vec3Expr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(Vec3Expr::Gather1(FloatExpr::Literal(-1.0)));
        result.simplify();
        result
    }
}
impl Sub for Vec3Expr {
    type Output = Vec3Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for Vec3Expr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}



impl<V: Into<Vec4Expr>> Add<V> for Vec4Expr {
    type Output = Vec4Expr;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec4Expr::Sum(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec4Expr>> AddAssign<V> for Vec4Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec4Expr::Sum(vec![x, rhs]);
        self.simplify();
    }
}
impl<V: Into<Vec4Expr>> Mul<V> for Vec4Expr {
    type Output = Vec4Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec4Expr::Product(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec4Expr>> MulAssign<V> for Vec4Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec4Expr::Product(vec![x, rhs]);
        self.simplify();
    }
}
impl Neg for Vec4Expr {
    type Output = Vec4Expr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(Vec4Expr::Gather1(FloatExpr::Literal(-1.0)));
        result.simplify();
        result
    }
}
impl Sub for Vec4Expr {
    type Output = Vec4Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for Vec4Expr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}



impl TrackOperations for AnyExpression {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            AnyExpression::Int(a) => a.count_operations(lookup),
            AnyExpression::Float(a) => a.count_operations(lookup),
            AnyExpression::Vec2(a) => a.count_operations(lookup),
            AnyExpression::Vec3(a) => a.count_operations(lookup),
            AnyExpression::Vec4(a) => a.count_operations(lookup),
            AnyExpression::Class(a) => a.count_operations(lookup),
        }
    }
}



impl TrackOperations for IntExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            IntExpr::Variable(_) => VectoredOperationsTracker::zero(),
            IntExpr::Literal(_) => VectoredOperationsTracker::zero(),
            IntExpr::TraitInvoke10ToInt(t, m) => lookup.trait_10_ops(t, m),
        }
    }
}
impl TrackOperations for FloatExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            FloatExpr::Variable(_) => VectoredOperationsTracker::zero(),
            FloatExpr::Literal(_) => VectoredOperationsTracker::zero(),
            FloatExpr::AccessVec2(v, _) => v.count_operations(lookup),
            FloatExpr::AccessVec3(v, _) => v.count_operations(lookup),
            FloatExpr::AccessVec4(v, _) => v.count_operations(lookup),
            FloatExpr::AccessMultiVecGroup(m, _) => {
                let mut result = m.count_operations(lookup);
                result.basis_element_struct_access = true;
                result
            },
            FloatExpr::AccessMultiVecFlat(m, _) => {
                let mut result = m.count_operations(lookup);
                result.basis_element_struct_access = true;
                result
            },
            FloatExpr::TraitInvoke11ToFloat(t, m) => {
                m.count_operations(lookup) + lookup.trait_11_ops(t, &m.mv_class)
            }
            FloatExpr::Product(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.floats.mul += v.len() - 1;
                }
                result
            }
            FloatExpr::Sum(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.floats.add_sub += v.len() - 1;
                }
                result
            }
            FloatExpr::Divide(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.floats.div += v.len() - 1;
                }
                result
            }
            FloatExpr::Pow(f1, f2) => {
                f1.count_operations(lookup) + f2.count_operations(lookup)
            }
        }
    }
}
impl TrackOperations for Vec2Expr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            Vec2Expr::Variable(_) => VectoredOperationsTracker::zero(),
            Vec2Expr::Gather1(f) => f.count_operations(lookup),
            Vec2Expr::Gather2(f0, f1) => f0.count_operations(lookup) + f1.count_operations(lookup),
            Vec2Expr::AccessMultiVecGroup(m, _) => m.count_operations(lookup),
            Vec2Expr::Product(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd2.mul += v.len() - 1;
                }
                result
            }
            Vec2Expr::Sum(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd2.add_sub += v.len() - 1;
                }
                result
            }
            Vec2Expr::SwizzleVec2(v, _, _) => v.count_operations(lookup),
        }
    }
}
impl TrackOperations for Vec3Expr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            Vec3Expr::Variable(_) => VectoredOperationsTracker::zero(),
            Vec3Expr::Gather1(f) => f.count_operations(lookup),
            Vec3Expr::Gather3(f0, f1, f2) => f0.count_operations(lookup) + f1.count_operations(lookup) + f2.count_operations(lookup),
            Vec3Expr::AccessMultiVecGroup(m, _) => m.count_operations(lookup),
            Vec3Expr::Product(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd3.mul += v.len() - 1;
                }
                result
            }
            Vec3Expr::Sum(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd3.add_sub += v.len() - 1;
                }
                result
            }
            Vec3Expr::SwizzleVec3(v, _, _, _) => v.count_operations(lookup),
        }
    }
}
impl TrackOperations for Vec4Expr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            Vec4Expr::Variable(_) => VectoredOperationsTracker::zero(),
            Vec4Expr::Gather1(f) => f.count_operations(lookup),
            Vec4Expr::Gather4(f0, f1, f2, f3) => f0.count_operations(lookup) + f1.count_operations(lookup) + f2.count_operations(lookup) + f3.count_operations(lookup),
            Vec4Expr::AccessMultiVecGroup(m, _) => m.count_operations(lookup),
            Vec4Expr::Product(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd4.mul += v.len() - 1;
                }
                result
            }
            Vec4Expr::Sum(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd4.add_sub += v.len() - 1;
                }
                result
            }
            Vec4Expr::SwizzleVec4(v, _, _, _, _) => v.count_operations(lookup),
        }
    }
}
impl TrackOperations for MultiVectorGroupExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => f.count_operations(lookup),
            MultiVectorGroupExpr::Vec2(v) => v.count_operations(lookup),
            MultiVectorGroupExpr::Vec3(v) => v.count_operations(lookup),
            MultiVectorGroupExpr::Vec4(v) => v.count_operations(lookup)
        }
    }
}
impl TrackOperations for MultiVectorExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self.expr.as_ref() {
            MultiVectorVia::Variable(_) => VectoredOperationsTracker::zero(),
            MultiVectorVia::Construct(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                result
            }
            MultiVectorVia::TraitInvoke11ToClass(t, m) => {
                m.count_operations(lookup) + lookup.trait_11_ops(t, &m.mv_class)
            }
            MultiVectorVia::TraitInvoke21ToClass(t, a, b) => {
                a.count_operations(lookup) + lookup.trait_21_ops(t, &a.mv_class, b)
            }
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                a.count_operations(lookup) + b.count_operations(lookup) + lookup.trait_22_ops(t, &a.mv_class, &b.mv_class)
            }
        }
    }
}



fn transpose_vec2_product(
    float_product_0: &mut Vec<FloatExpr>,
    float_product_1: &mut Vec<FloatExpr>,
) -> Option<Vec2Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Product
    let mut coalesce_product_literals = [1.0, 1.0];
    let mut vec2_product = vec![];
    float_product_0.retain_mut(|it0| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|it1| {
            if pulling_out_factor {
                return true
            }
            pulling_out_factor = vec2_product_extract(
                &mut vec2_product, &mut coalesce_product_literals,
                it0, it1,
            );
            !pulling_out_factor
        });
        !pulling_out_factor
    });
    if coalesce_product_literals != [1.0, 1.0] {
        if coalesce_product_literals[0] == coalesce_product_literals[1] {
            vec2_product.push(Vec2Expr::Gather1(Literal(coalesce_product_literals[0])));
        } else {
            vec2_product.push(Vec2Expr::Gather2(
                Literal(coalesce_product_literals[0]),
                Literal(coalesce_product_literals[1]),
            ));
        }
    }
    if vec2_product.is_empty() {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.clone())
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.clone())
    };
    if keep_remaining {
        vec2_product.push(Vec2Expr::Gather2(p0, p1));
    }
    let mut result = Vec2Expr::Product(vec2_product);
    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec2_product_extract(
    vec2_product: &mut Vec<Vec2Expr>,
    coalesce_product_literals: &mut [f32; 2],
    f0: &mut FloatExpr,
    f1: &mut FloatExpr,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = f0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false
    }
    if f0 == f1 {
        vec2_product.push(Vec2Expr::Gather1(f0.clone()));
        return true
    }
    return match (f0, f1) {
        (
            AccessVec2(box v0, 0),
            AccessVec2(box v1, 1),
        ) if v0 == v1 => {
            vec2_product.push(v0.clone());
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
        ) if v0 == v1 => {
            vec2_product.push(Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1));
            true
        }
        (
            Sum(v0),
            Sum(v1),
        ) => {
            let Some(transposed) = transpose_vec2_sum(v0, v1) else { return false };
            vec2_product.push(transposed);
            true
        }
        _ => false
    }
}

fn transpose_vec2_sum(
    float_sum_0: &mut Vec<FloatExpr>,
    float_sum_1: &mut Vec<FloatExpr>,
) -> Option<Vec2Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Sum
    let mut vec2_sum = vec![];
    let mut coalesce_sum_literal = [0.0, 0.0];
    float_sum_0.retain_mut(|it0| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|it1| {
            if pulling_out_addend {
                return true
            }
            pulling_out_addend = vec2_sum_extract(
                &mut vec2_sum, &mut coalesce_sum_literal,
                it0, it1
            );
            !pulling_out_addend
        });
        !pulling_out_addend
    });
    if coalesce_sum_literal != [0.0, 0.0] {
        if coalesce_sum_literal[0] == coalesce_sum_literal[1] {
            vec2_sum.push(Vec2Expr::Gather1(Literal(coalesce_sum_literal[0])));
        } else {
            vec2_sum.push(Vec2Expr::Gather2(
                Literal(coalesce_sum_literal[0]),
                Literal(coalesce_sum_literal[1]),
            ));
        }
    }

    if vec2_sum.is_empty() {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.clone())
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.clone())
    };
    if keep_remaining {
        vec2_sum.push(Vec2Expr::Gather2(p0, p1));
    }
    let mut result = Vec2Expr::Sum(vec2_sum);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec2_sum_extract(
    vec2_sum: &mut Vec<Vec2Expr>,
    coalesce_sum_literals: &mut [f32; 2],
    f0: &mut FloatExpr,
    f1: &mut FloatExpr,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = f0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false
    }
    if f0 == f1 {
        vec2_sum.push(Vec2Expr::Gather1(f0.clone()));
        return true
    }
    return match (f0, f1) {
        (
            AccessVec2(box v0, 0),
            AccessVec2(box v1, 1),
        ) if v0 == v1 => {
            vec2_sum.push(v0.clone());
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
        ) if v0 == v1 => {
            vec2_sum.push(Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1));
            true
        }
        (
            Product(v0),
            Product(v1),
        ) => {
            let Some(transposed) = transpose_vec2_product(v0, v1) else { return false };
            vec2_sum.push(transposed);
            true
        }
        _ => false
    }
}



fn transpose_vec3_product(
    float_product_0: &mut Vec<FloatExpr>,
    float_product_1: &mut Vec<FloatExpr>,
    float_product_2: &mut Vec<FloatExpr>,
) -> Option<Vec3Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Product
    let mut coalesce_product_literals = [1.0, 1.0, 1.0];
    let mut vec3_product = vec![];
    float_product_0.retain_mut(|it0| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|it1| {
            if pulling_out_factor {
                return true
            }
            float_product_2.retain_mut(|it2| {
                if pulling_out_factor {
                    return true
                }
                pulling_out_factor = vec3_product_extract(
                    &mut vec3_product, &mut coalesce_product_literals,
                    it0, it1, it2,
                );
                !pulling_out_factor
            });
            !pulling_out_factor
        });
        !pulling_out_factor
    });
    if coalesce_product_literals != [1.0, 1.0, 1.0] {
        if coalesce_product_literals[0] == coalesce_product_literals[1]
            && coalesce_product_literals[1] == coalesce_product_literals[2] {
            vec3_product.push(Vec3Expr::Gather1(Literal(coalesce_product_literals[0])));
        } else {
            vec3_product.push(Vec3Expr::Gather3(
                Literal(coalesce_product_literals[0]),
                Literal(coalesce_product_literals[1]),
                Literal(coalesce_product_literals[2]),
            ));
        }
    }
    if vec3_product.is_empty() {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.clone())
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.clone())
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_2.clone())
    };
    if keep_remaining {
        vec3_product.push(Vec3Expr::Gather3(p0, p1, p2));
    }
    let mut result = Vec3Expr::Product(vec3_product);
    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec3_product_extract(
    vec3_product: &mut Vec<Vec3Expr>,
    coalesce_product_literals: &mut [f32; 3],
    f0: &mut FloatExpr,
    f1: &mut FloatExpr,
    f2: &mut FloatExpr,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = f0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f2 {
        if *f != 1.0 {
            coalesce_product_literals[2] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false
    }
    if f0 == f1 && f1 == f2 {
        vec3_product.push(Vec3Expr::Gather1(f0.clone()));
        return true
    }
    return match (f0, f1, f2) {
        (
            AccessVec3(box v0, 0),
            AccessVec3(box v1, 1),
            AccessVec3(box v2, 2),
        ) if v0 == v1 && v1 == v2 => {
            vec3_product.push(v0.clone());
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
        ) if v0 == v1 && v1 == v2 => {
            vec3_product.push(Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2));
            true
        }
        (
            Sum(v0),
            Sum(v1),
            Sum(v2),
        ) => {
            // Is this actually a correct/desired transposition that doesn't
            //  change the meaning of the math? It's hard to keep track of.
            // ...after examining, but not testing yet...
            // I'm fairly confident it is right.
            // It basically transforms...
            // Gather3( (a + b)(c + d), (e + f)(g + h), (i + j)(k + l) )
            // where a-l are FloatExpr, into...
            // ( [aei] + [bfj] )( [cgk] + [dhl] )
            // where each [xyz] is a Vec3Expr.
            //
            // Tracking the transposition can get confusing, but at no point are order of operations
            // mixed up. If transpose_vec3_sum returns nothing, then none of its arguments are
            // mutated. If it returns something, then it serves as a total replacement, cloning
            // anything necessary. It performs its own "leftover" Gather3 on anything that doesn't
            // transpose, but the `transposed` variable itself will be a Vec3Expr::Sum.
            let Some(transposed) = transpose_vec3_sum(v0, v1, v2) else { return false };
            vec3_product.push(transposed);
            true
        }
        _ => false
    }
}

fn transpose_vec3_sum(
    float_sum_0: &mut Vec<FloatExpr>,
    float_sum_1: &mut Vec<FloatExpr>,
    float_sum_2: &mut Vec<FloatExpr>,
) -> Option<Vec3Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Sum
    let mut vec3_sum = vec![];
    let mut coalesce_sum_literal = [0.0, 0.0, 0.0];
    float_sum_0.retain_mut(|it0| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|it1| {
            if pulling_out_addend {
                return true
            }
            float_sum_2.retain_mut(|it2| {
                if pulling_out_addend {
                    return true
                }
                pulling_out_addend = vec3_sum_extract(
                    &mut vec3_sum, &mut coalesce_sum_literal,
                    it0, it1, it2
                );
                !pulling_out_addend
            });
            !pulling_out_addend
        });
        !pulling_out_addend
    });
    if coalesce_sum_literal != [0.0, 0.0, 0.0] {
        if coalesce_sum_literal[0] == coalesce_sum_literal[1]
            && coalesce_sum_literal[1] == coalesce_sum_literal[2] {
            vec3_sum.push(Vec3Expr::Gather1(Literal(coalesce_sum_literal[0])));
        } else {
            vec3_sum.push(Vec3Expr::Gather3(
                Literal(coalesce_sum_literal[0]),
                Literal(coalesce_sum_literal[1]),
                Literal(coalesce_sum_literal[2]),
            ));
        }
    }

    if vec3_sum.is_empty() {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.clone())
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.clone())
    };
    let p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_2.clone())
    };
    if keep_remaining {
        vec3_sum.push(Vec3Expr::Gather3(p0, p1, p2));
    }
    let mut result = Vec3Expr::Sum(vec3_sum);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec3_sum_extract(
    vec3_sum: &mut Vec<Vec3Expr>,
    coalesce_sum_literals: &mut [f32; 3],
    f0: &mut FloatExpr,
    f1: &mut FloatExpr,
    f2: &mut FloatExpr,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = f0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false
    }
    if f0 == f1 && f1 == f2 {
        vec3_sum.push(Vec3Expr::Gather1(f0.clone()));
        return true
    }
    return match (f0, f1, f2) {
        (
            AccessVec3(box v0, 0),
            AccessVec3(box v1, 1),
            AccessVec3(box v2, 2),
        ) if v0 == v1 && v1 == v2 => {
            vec3_sum.push(v0.clone());
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
        ) if v0 == v1 && v1 == v2 => {
            vec3_sum.push(Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2));
            true
        }
        (
            Product(v0),
            Product(v1),
            Product(v2),
        ) => {
            let Some(transposed) = transpose_vec3_product(v0, v1, v2) else { return false };
            vec3_sum.push(transposed);
            true
        }
        _ => false
    }
}



fn transpose_vec4_product(
    float_product_0: &mut Vec<FloatExpr>,
    float_product_1: &mut Vec<FloatExpr>,
    float_product_2: &mut Vec<FloatExpr>,
    float_product_3: &mut Vec<FloatExpr>,
) -> Option<Vec4Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Product
    let mut coalesce_product_literals = [1.0, 1.0, 1.0, 1.0];
    let mut vec4_product = vec![];
    float_product_0.retain_mut(|it0| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|it1| {
            if pulling_out_factor {
                return true
            }
            float_product_2.retain_mut(|it2| {
                if pulling_out_factor {
                    return true
                }
                float_product_3.retain_mut(|it3| {
                    if pulling_out_factor {
                        return true
                    }
                    pulling_out_factor = vec4_product_extract(
                        &mut vec4_product, &mut coalesce_product_literals,
                        it0, it1, it2, it3
                    );
                    !pulling_out_factor
                });
                !pulling_out_factor
            });
            !pulling_out_factor
        });
        !pulling_out_factor
    });
    if coalesce_product_literals != [1.0, 1.0, 1.0, 1.0] {
        if coalesce_product_literals[0] == coalesce_product_literals[1]
            && coalesce_product_literals[1] == coalesce_product_literals[2]
            && coalesce_product_literals[2] == coalesce_product_literals[3] {
            vec4_product.push(Vec4Expr::Gather1(Literal(coalesce_product_literals[0])));
        } else {
            vec4_product.push(Vec4Expr::Gather4(
                Literal(coalesce_product_literals[0]),
                Literal(coalesce_product_literals[1]),
                Literal(coalesce_product_literals[2]),
                Literal(coalesce_product_literals[3]),
            ));
        }
    }
    if vec4_product.is_empty() {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.clone())
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.clone())
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_2.clone())
    };
    let p3 = if float_product_3.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_3.clone())
    };
    if keep_remaining {
        vec4_product.push(Vec4Expr::Gather4(p0, p1, p2, p3));
    }
    let mut result = Vec4Expr::Product(vec4_product);
    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec4_product_extract(
    vec4_product: &mut Vec<Vec4Expr>,
    coalesce_product_literals: &mut [f32; 4],
    f0: &mut FloatExpr,
    f1: &mut FloatExpr,
    f2: &mut FloatExpr,
    f3: &mut FloatExpr,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = f0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f2 {
        if *f != 1.0 {
            coalesce_product_literals[2] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f3 {
        if *f != 1.0 {
            coalesce_product_literals[3] *= *f;
            *f = 1.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false
    }
    if f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_product.push(Vec4Expr::Gather1(f0.clone()));
        return true
    }
    return match (f0, f1, f2, f3) {
        (
            AccessVec4(box v0, 0),
            AccessVec4(box v1, 1),
            AccessVec4(box v2, 2),
            AccessVec4(box v3, 3),
        ) if v0 == v1 && v1 == v2 && v2 == v3 => {
            vec4_product.push(v0.clone());
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3),
        ) if v0 == v1 && v1 == v2 && v2 == v3 => {
            vec4_product.push(Vec4Expr::SwizzleVec4(Box::new(v0.clone()), *i0, *i1, *i2, *i3));
            true
        }
        (
            Sum(v0),
            Sum(v1),
            Sum(v2),
            Sum(v3),
        ) => {
            let Some(transposed) = transpose_vec4_sum(v0, v1, v2, v3) else { return false };
            vec4_product.push(transposed);
            true
        }
        _ => false
    }
}

fn transpose_vec4_sum(
    float_sum_0: &mut Vec<FloatExpr>,
    float_sum_1: &mut Vec<FloatExpr>,
    float_sum_2: &mut Vec<FloatExpr>,
    float_sum_3: &mut Vec<FloatExpr>,
) -> Option<Vec4Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Sum
    let mut vec4_sum = vec![];
    let mut coalesce_sum_literal = [0.0, 0.0, 0.0, 0.0];
    float_sum_0.retain_mut(|it0| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|it1| {
            if pulling_out_addend {
                return true
            }
            float_sum_2.retain_mut(|it2| {
                if pulling_out_addend {
                    return true
                }
                float_sum_3.retain_mut(|it3| {
                    if pulling_out_addend {
                        return true
                    }
                    pulling_out_addend = vec4_sum_extract(
                        &mut vec4_sum, &mut coalesce_sum_literal,
                        it0, it1, it2, it3
                    );
                    !pulling_out_addend
                });
                !pulling_out_addend
            });
            !pulling_out_addend
        });
        !pulling_out_addend
    });
    if coalesce_sum_literal != [0.0, 0.0, 0.0, 0.0] {
        if coalesce_sum_literal[0] == coalesce_sum_literal[1]
            && coalesce_sum_literal[1] == coalesce_sum_literal[2]
            && coalesce_sum_literal[2] == coalesce_sum_literal[3] {
            vec4_sum.push(Vec4Expr::Gather1(Literal(coalesce_sum_literal[0])));
        } else {
            vec4_sum.push(Vec4Expr::Gather4(
                Literal(coalesce_sum_literal[0]),
                Literal(coalesce_sum_literal[1]),
                Literal(coalesce_sum_literal[2]),
                Literal(coalesce_sum_literal[3]),
            ));
        }
    }

    if vec4_sum.is_empty() {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.clone())
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.clone())
    };
    let p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_2.clone())
    };
    let p3 = if float_sum_3.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_3.clone())
    };
    if keep_remaining {
        vec4_sum.push(Vec4Expr::Gather4(p0, p1, p2, p3));
    }
    let mut result = Vec4Expr::Sum(vec4_sum);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec4_sum_extract(
    vec4_sum: &mut Vec<Vec4Expr>,
    coalesce_sum_literals: &mut [f32; 4],
    f0: &mut FloatExpr,
    f1: &mut FloatExpr,
    f2: &mut FloatExpr,
    f3: &mut FloatExpr,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = f0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = f3 {
        if *f != 0.0 {
            coalesce_sum_literals[3] += *f;
            *f = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false
    }
    if f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_sum.push(Vec4Expr::Gather1(f0.clone()));
        return true
    }
    return match (f0, f1, f2, f3) {
        (
            AccessVec4(box v0, 0),
            AccessVec4(box v1, 1),
            AccessVec4(box v2, 2),
            AccessVec4(box v3, 3),
        ) if v0 == v1 && v1 == v2 && v2 == v3 => {
            vec4_sum.push(v0.clone());
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3),
        ) if v0 == v1 && v1 == v2 && v2 == v3 => {
            vec4_sum.push(Vec4Expr::SwizzleVec4(Box::new(v0.clone()), *i0, *i1, *i2, *i3));
            true
        }
        (
            Product(v0),
            Product(v1),
            Product(v2),
            Product(v3),
        ) => {
            let Some(transposed) = transpose_vec4_product(v0, v1, v2, v3) else { return false };
            vec4_sum.push(transposed);
            true
        }
        _ => false
    }
}