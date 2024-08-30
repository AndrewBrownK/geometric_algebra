use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::sync::Arc;
use float_ord::FloatOrd;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::{BasisElementGroup, DynamicMultiVector};
use crate::ast2::datatype::{ExpressionType, Float, Integer, MultiVector, Vec2, Vec3, Vec4};
use crate::ast2::operations_tracker::{TrackOperations, TraitOperationsLookup, VectoredOperationsTracker};
use crate::ast2::traits::TraitKey;
use crate::ast2::{RawVariableDeclaration, RawVariableInvocation, Variable};
use crate::utility::slice_retain_mut;

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
    fn select_expr(expr: AnyExpression) -> Option<Self::Expr>;
}
impl TraitResultType for Integer {
    type Expr = IntExpr;
    fn expr_10(trait_name: TraitKey, owner: MultiVector, mv_out: Option<MultiVector>) -> IntExpr {
        assert!(mv_out.is_none(), "Confused Trait output: Expected Integer, found MultiVector");
        IntExpr::TraitInvoke10ToInt(trait_name, owner)
    }

    fn inlined_expr_10(var: Variable<Self>) -> Self::Expr {
        IntExpr::Variable(RawVariableInvocation { decl: var.decl.clone() })
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Int(_) => Some(Self),
            _ => None,
        }
    }

    fn select_expr(expr: AnyExpression) -> Option<Self::Expr> {
        match expr {
            AnyExpression::Int(e) => Some(e),
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
        FloatExpr::Variable(RawVariableInvocation { decl: var.decl.clone() })
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Float(_) => Some(Self),
            _ => None,
        }
    }

    fn select_expr(expr: AnyExpression) -> Option<Self::Expr> {
        match expr {
            AnyExpression::Float(mut e) => {
                e.simplify();
                Some(e)
            }
            _ => None,
        }
    }
}
impl TraitResultType for MultiVector {
    type Expr = MultiVectorExpr;

    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke11ToClass(trait_name, owner)),
        }
    }
    fn inlined_expr_11(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }
    fn expr_21(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVector, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke21ToClass(trait_name, owner, other)),
        }
    }
    fn inlined_expr_21(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }
    fn expr_22(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVectorExpr, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke22ToClass(trait_name, owner, other)),
        }
    }
    fn inlined_expr_22(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Class(mv_expr) => Some(mv_expr.mv_class.clone()),
            _ => None,
        }
    }

    fn select_expr(expr: AnyExpression) -> Option<Self::Expr> {
        match expr {
            AnyExpression::Class(mut e) => {
                e.simplify();
                Some(e)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntExpr {
    Variable(RawVariableInvocation),
    Literal(u32),
    // e.g. Grade
    TraitInvoke10ToInt(TraitKey, MultiVector),
}
#[derive(Clone, Debug)]
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
    Product(Vec<(FloatExpr, f32)>, f32),
    Sum(Vec<(FloatExpr, f32)>, f32),
    // TODO trig? floor? log? round? trunc? mix? step? smoothstep? fma? fract? modf?
}
#[derive(Clone, Debug)]
pub enum Vec2Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather2(FloatExpr, FloatExpr),
    SwizzleVec2(Box<Vec2Expr>, u8, u8),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    Product(Vec<(Vec2Expr, f32)>, [f32; 2]),
    Sum(Vec<(Vec2Expr, f32)>, [f32; 2]),
}
#[derive(Clone, Debug)]
pub enum Vec3Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather3(FloatExpr, FloatExpr, FloatExpr),
    SwizzleVec3(Box<Vec3Expr>, u8, u8, u8),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    Product(Vec<(Vec3Expr, f32)>, [f32; 3]),
    Sum(Vec<(Vec3Expr, f32)>, [f32; 3]),
}
#[derive(Clone, Debug)]
pub enum Vec4Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather4(FloatExpr, FloatExpr, FloatExpr, FloatExpr),
    SwizzleVec4(Box<Vec4Expr>, u8, u8, u8, u8),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    Product(Vec<(Vec4Expr, f32)>, [f32; 4]),
    Sum(Vec<(Vec4Expr, f32)>, [f32; 4]),
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiVectorGroupExpr {
    JustFloat(FloatExpr),
    Vec2(Vec2Expr),
    Vec3(Vec3Expr),
    Vec4(Vec4Expr),
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultiVectorExpr {
    pub mv_class: MultiVector,
    pub expr: Box<MultiVectorVia>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub(crate) fn deep_inline_variables(&mut self) {
        let _ = match self {
            AnyExpression::Int(e) => e.deep_inline_variables(),
            AnyExpression::Float(e) => e.deep_inline_variables(),
            AnyExpression::Vec2(e) => e.deep_inline_variables(),
            AnyExpression::Vec3(e) => e.deep_inline_variables(),
            AnyExpression::Vec4(e) => e.deep_inline_variables(),
            AnyExpression::Class(e) => e.deep_inline_variables(),
        };
    }
}

pub trait Expression<ExprType>: Send + Sized {
    fn into_any_expression(self) -> AnyExpression;

    fn from_any_expression(any: AnyExpression) -> Option<Self>;
    fn expression_type(&self) -> ExprType;
    fn type_from_any(any: &AnyExpression) -> Option<ExprType>;
    fn try_into_variable(&self) -> Option<Variable<ExprType>>;
    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>);
}

/// This helps unify Variable<MultiVector> and MultiVectorExpr
pub fn extract_multivector_expr<Expr: Expression<MultiVector>>(expr: Expr) -> MultiVectorExpr {
    match expr.into_any_expression() {
        AnyExpression::Class(mve) => mve,
        _ => unreachable!("Expression<MultiVector> will always create AnyExpression::Class"),
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

    fn expression_type(&self) -> Integer {
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
            IntExpr::Variable(v) => Some(Variable {
                expr_type: Integer,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
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

    fn expression_type(&self) -> Float {
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
            FloatExpr::Variable(v) => Some(Variable {
                expr_type: Float,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
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
            FloatExpr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            FloatExpr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
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

    fn expression_type(&self) -> Vec2 {
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
            Vec2Expr::Variable(v) => Some(Variable {
                expr_type: Vec2,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
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
            Vec2Expr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec2Expr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
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

    fn expression_type(&self) -> Vec3 {
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
            Vec3Expr::Variable(v) => Some(Variable {
                expr_type: Vec3,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
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
            Vec3Expr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec3Expr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
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

    fn expression_type(&self) -> Vec4 {
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
            Vec4Expr::Variable(v) => Some(Variable {
                expr_type: Vec4,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
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
            Vec4Expr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec4Expr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
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

    fn expression_type(&self) -> MultiVector {
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
            MultiVectorVia::Variable(v) => Some(Variable {
                expr_type: self.mv_class,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
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

    fn expression_type(&self) -> Integer {
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

    fn expression_type(&self) -> Float {
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

    fn expression_type(&self) -> Vec2 {
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

    fn expression_type(&self) -> Vec3 {
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

    fn expression_type(&self) -> Vec4 {
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
            AnyExpression::Class(MultiVectorExpr { mv_class, expr }) => {
                if let MultiVectorVia::Variable(var) = *expr {
                    Some(Variable {
                        expr_type: mv_class,
                        decl: var.decl.clone(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn expression_type(&self) -> MultiVector {
        self.expr_type.clone()
    }

    fn type_from_any(any: &AnyExpression) -> Option<MultiVector> {
        match any {
            AnyExpression::Class(MultiVectorExpr { mv_class, expr }) => {
                if let MultiVectorVia::Variable(_) = **expr {
                    Some(mv_class.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<MultiVector>> {
        Some((*self).clone())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}

impl Variable<MultiVector> {
    pub fn elements_flat(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type
            .elements()
            .into_iter()
            .enumerate()
            .map(move |(i, el)| (FloatExpr::AccessMultiVecFlat(mv_expr.clone(), i as u16), el))
    }

    pub fn groups(&self) -> impl Iterator<Item = (MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type.groups().into_iter().enumerate().map(move |(g, group)| {
            let g = g as u16;
            match group {
                BasisElementGroup::G1(a) => (MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G1(a)),
                BasisElementGroup::G2(a, b) => (MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G2(a, b)),
                BasisElementGroup::G3(a, b, c) => (MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G3(a, b, c)),
                BasisElementGroup::G4(a, b, c, d) => (MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G4(a, b, c, d)),
            }
        })
    }

    pub fn elements_by_groups(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type
            .groups()
            .into_iter()
            .enumerate()
            .map(move |(g, group)| {
                let g = g as u16;
                let mut v = vec![];
                match group {
                    BasisElementGroup::G1(a) => {
                        v.push((FloatExpr::AccessMultiVecGroup(mv_expr.clone(), g), a));
                    }
                    BasisElementGroup::G2(a, b) => {
                        v.push((FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 0), a));
                        v.push((FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 1), b));
                    }
                    BasisElementGroup::G3(a, b, c) => {
                        v.push((FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 0), a));
                        v.push((FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 1), b));
                        v.push((FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 2), c));
                    }
                    BasisElementGroup::G4(a, b, c, d) => {
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 0), a));
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 1), b));
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 2), c));
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), 3), d));
                    }
                }
                for (f, _el) in v.iter_mut() {
                    f.simplify();
                }
                v.into_iter()
            })
            .flatten()
    }
}

impl MultiVectorExpr {
    pub fn elements_flat(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
        self.mv_class
            .elements()
            .into_iter()
            .enumerate()
            .map(move |(i, el)| (FloatExpr::AccessMultiVecFlat(self.clone(), i as u16), el))
    }

    pub fn groups(&self) -> impl Iterator<Item = (MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        self.mv_class.groups().into_iter().enumerate().map(move |(g, group)| {
            let g = g as u16;
            match group {
                BasisElementGroup::G1(a) => (MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G1(a)),
                BasisElementGroup::G2(a, b) => (MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G2(a, b)),
                BasisElementGroup::G3(a, b, c) => (MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G3(a, b, c)),
                BasisElementGroup::G4(a, b, c, d) => (MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G4(a, b, c, d)),
            }
        })
    }
    pub fn elements_by_groups(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
        self.mv_class
            .groups()
            .into_iter()
            .enumerate()
            .map(move |(g, group)| {
                let g = g as u16;
                let mut v = vec![];
                match group {
                    BasisElementGroup::G1(a) => {
                        v.push((FloatExpr::AccessMultiVecGroup(self.clone(), g), a));
                    }
                    BasisElementGroup::G2(a, b) => {
                        v.push((FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(self.clone(), g)), 0), a));
                        v.push((FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(self.clone(), g)), 1), b));
                    }
                    BasisElementGroup::G3(a, b, c) => {
                        v.push((FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), 0), a));
                        v.push((FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), 1), b));
                        v.push((FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), 2), c));
                    }
                    BasisElementGroup::G4(a, b, c, d) => {
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 0), a));
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 1), b));
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 2), c));
                        v.push((FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), 3), d));
                    }
                }
                for (f, _el) in v.iter_mut() {
                    f.simplify();
                }
                v.into_iter()
            })
            .flatten()
    }
}

impl From<Variable<Float>> for FloatExpr {
    fn from(value: Variable<Float>) -> Self {
        FloatExpr::Variable(RawVariableInvocation { decl: value.decl.clone() })
    }
}
impl From<f32> for FloatExpr {
    fn from(value: f32) -> Self {
        FloatExpr::Literal(value)
    }
}
impl From<Variable<Vec2>> for Vec2Expr {
    fn from(value: Variable<Vec2>) -> Self {
        Vec2Expr::Variable(RawVariableInvocation { decl: value.decl.clone() })
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
        Vec3Expr::Variable(RawVariableInvocation { decl: value.decl.clone() })
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
        Vec4Expr::Variable(RawVariableInvocation { decl: value.decl.clone() })
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
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: value.decl.clone() })),
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
            FloatExpr::Literal(l) => write!(f, "{l}")?,
            FloatExpr::AccessVec2(box v, i) => write!(f, "{v}[{i}]")?,
            FloatExpr::AccessVec3(box v, i) => write!(f, "{v}[{i}]")?,
            FloatExpr::AccessVec4(box v, i) => write!(f, "{v}[{i}]")?,
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G1(be0) = mv.mv_class.groups()[*i as usize] else {
                    unreachable!(
                        "Should not be able to access FloatExpr as MultiVecGroup \
                        unless the MultiVecGroup is just one Float"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i as usize] {
                        MultiVectorGroupExpr::JustFloat(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access FloatExpr as MultiVecGroup \
                                unless the MultiVecGroup is just one Float"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}]")?
                    }
                }
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                let gs: Vec<_> = mv.elements_flat().collect();
                let (float, el) = &gs[*i as usize];
                write!(f, "{el}({float})")?;
            }
            FloatExpr::TraitInvoke11ToFloat(t, mv) => {
                let n = t.as_lower_snake();
                write!(f, "({mv} {n})")?
            }
            FloatExpr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != 1.0 {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    write!(f, "{last_factor}")?;
                }
                write!(f, ")")?;
            }
            FloatExpr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                match (*last_addend, !v.is_empty()) {
                    (fl, _) if fl == 0.0 => {}
                    (fl, false) => write!(f, "{fl}")?,
                    (fl, true) if fl > 0.0 => write!(f, " + {fl}")?,
                    (fl, true) if fl < 0.0 => {
                        let fl = -fl;
                        write!(f, " - {fl}")?;
                    }
                    _ => {}
                }
                write!(f, ")")?;
            }
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
                let BasisElementGroup::G2(be0, be1) = mv.mv_class.groups()[*i as usize] else {
                    unreachable!(
                        "Should not be able to access Vec2Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec2"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}, {be1}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}, {be1}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i as usize] {
                        MultiVectorGroupExpr::Vec2(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec2Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec2"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}, {be1}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}]")?
                    }
                }
            }
            Vec2Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != [1.0; 2] {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    let a0 = last_factor[0];
                    let a1 = last_factor[1];
                    write!(f, "[{a0}, {a1}]")?;
                }
                write!(f, ")")?;
            }
            Vec2Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                if *last_addend != [0.0; 2] {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    let a0 = last_addend[0];
                    let a1 = last_addend[1];
                    write!(f, "[{a0}, {a1}]")?;
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
                let BasisElementGroup::G3(be0, be1, be2) = mv.mv_class.groups()[*i as usize] else {
                    unreachable!(
                        "Should not be able to access Vec3Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec3"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}, {be1}, {be2}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}, {be1}, {be2}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i as usize] {
                        MultiVectorGroupExpr::Vec3(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec3Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec3"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}, {be1}, {be2}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}]")?
                    }
                }
            }
            Vec3Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != [1.0; 3] {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    let a0 = last_factor[0];
                    let a1 = last_factor[1];
                    let a2 = last_factor[2];
                    write!(f, "[{a0}, {a1}, {a2}]")?;
                }
                write!(f, ")")?;
            }
            Vec3Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                if *last_addend != [0.0; 3] {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    let a0 = last_addend[0];
                    let a1 = last_addend[1];
                    let a2 = last_addend[2];
                    write!(f, "[{a0}, {a1}, {a2}]")?;
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
                let BasisElementGroup::G4(be0, be1, be2, be3) = mv.mv_class.groups()[*i as usize] else {
                    unreachable!(
                        "Should not be able to access Vec4Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec4"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}, {be1}, {be2}, {be3}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}, {be1}, {be2}, {be3}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i as usize] {
                        MultiVectorGroupExpr::Vec4(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec4Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec4"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                }
            }
            Vec4Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != [1.0; 4] {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    let a0 = last_factor[0];
                    let a1 = last_factor[1];
                    let a2 = last_factor[2];
                    let a3 = last_factor[3];
                    write!(f, "[{a0}, {a1}, {a2}, {a3}]")?;
                }
                write!(f, ")")?;
            }
            Vec4Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                if *last_addend != [0.0; 4] {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    let a0 = last_addend[0];
                    let a1 = last_addend[1];
                    let a2 = last_addend[2];
                    let a3 = last_addend[3];
                    write!(f, "[{a0}, {a1}, {a2}, {a3}]")?;
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
            MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                let n = t.as_lower_snake();
                write!(f, "({mv} {n})")?
            }
            MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {mvb})")?
            }
            MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {mvb})")?
            }
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl IntExpr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            IntExpr::Variable(v) => {
                let Some(AnyExpression::Int(e)) = v.decl.expr.as_ref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                *self = e;
                true
            }
            IntExpr::Literal(_) => false,
            IntExpr::TraitInvoke10ToInt(_, _) => false,
        };
        // There is no int simplification yet at this time
        // if result {
        //     self.simplify();
        // }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = IntExpr::Literal(0);
        mem::swap(&mut x, self);
        x
    }
}

impl FloatExpr {
    pub(crate) fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            FloatExpr::Variable(v) => {
                let Some(AnyExpression::Float(e)) = v.decl.expr.as_ref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                *self = e;
                true
            }
            FloatExpr::Literal(_) => false,
            FloatExpr::AccessVec2(v, _) => v.deep_inline_variables(),
            FloatExpr::AccessVec3(v, _) => v.deep_inline_variables(),
            FloatExpr::AccessVec4(v, _) => v.deep_inline_variables(),
            FloatExpr::AccessMultiVecGroup(mv, _) => mv.deep_inline_variables(),
            FloatExpr::AccessMultiVecFlat(mv, _) => mv.deep_inline_variables(),
            FloatExpr::TraitInvoke11ToFloat(_, _) => false,
            FloatExpr::Product(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
            FloatExpr::Sum(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
        };
        if result {
            self.simplify_nuanced(true);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = FloatExpr::Literal(0.0);
        mem::swap(&mut x, self);
        x
    }
}
impl Vec2Expr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            Vec2Expr::Variable(v) => {
                let Some(AnyExpression::Vec2(e)) = v.decl.expr.as_ref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                *self = e;
                true
            }
            Vec2Expr::Gather1(e) => e.deep_inline_variables(),
            Vec2Expr::Gather2(e0, e1) => {
                let mut result = false;
                result |= e0.deep_inline_variables();
                result |= e1.deep_inline_variables();
                result
            }
            Vec2Expr::SwizzleVec2(v, _, _) => v.deep_inline_variables(),
            Vec2Expr::AccessMultiVecGroup(mv, _) => mv.deep_inline_variables(),
            Vec2Expr::Product(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
            Vec2Expr::Sum(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
        };
        if result {
            self.simplify();
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
}
impl Vec3Expr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            Vec3Expr::Variable(v) => {
                let Some(AnyExpression::Vec3(e)) = v.decl.expr.as_ref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                *self = e;
                true
            }
            Vec3Expr::Gather1(e) => e.deep_inline_variables(),
            Vec3Expr::Gather3(e0, e1, e2) => {
                let mut result = false;
                result |= e0.deep_inline_variables();
                result |= e1.deep_inline_variables();
                result |= e2.deep_inline_variables();
                result
            }
            Vec3Expr::SwizzleVec3(v, _, _, _) => v.deep_inline_variables(),
            Vec3Expr::AccessMultiVecGroup(mv, _) => mv.deep_inline_variables(),
            Vec3Expr::Product(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
            Vec3Expr::Sum(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
        };
        if result {
            self.simplify_nuanced(true);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
}
impl Vec4Expr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            Vec4Expr::Variable(v) => {
                let Some(AnyExpression::Vec4(e)) = v.decl.expr.as_ref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                *self = e;
                true
            }
            Vec4Expr::Gather1(e) => e.deep_inline_variables(),
            Vec4Expr::Gather4(e0, e1, e2, e3) => {
                let mut result = false;
                result |= e0.deep_inline_variables();
                result |= e1.deep_inline_variables();
                result |= e2.deep_inline_variables();
                result |= e3.deep_inline_variables();
                result
            }
            Vec4Expr::SwizzleVec4(v, _, _, _, _) => v.deep_inline_variables(),
            Vec4Expr::AccessMultiVecGroup(mv, _) => mv.deep_inline_variables(),
            Vec4Expr::Product(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
            Vec4Expr::Sum(v, _) => {
                let mut result = false;
                for (e, _) in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
        };
        if result {
            self.simplify_nuanced(true);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
}
impl MultiVectorGroupExpr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            MultiVectorGroupExpr::JustFloat(v) => v.deep_inline_variables(),
            MultiVectorGroupExpr::Vec2(v) => v.deep_inline_variables(),
            MultiVectorGroupExpr::Vec3(v) => v.deep_inline_variables(),
            MultiVectorGroupExpr::Vec4(v) => v.deep_inline_variables(),
        };
        if result {
            self.simplify_nuanced(true);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = MultiVectorGroupExpr::JustFloat(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
}
impl MultiVectorExpr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self.expr.as_mut() {
            MultiVectorVia::Variable(v) => {
                let Some(AnyExpression::Class(e)) = v.decl.expr.as_ref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                *self = e;
                true
            }
            MultiVectorVia::Construct(v) => {
                let mut result = false;
                for e in v.iter_mut() {
                    result |= e.deep_inline_variables();
                }
                result
            }
            MultiVectorVia::TraitInvoke11ToClass(_, _) => false,
            MultiVectorVia::TraitInvoke21ToClass(_, _, _) => false,
            MultiVectorVia::TraitInvoke22ToClass(_, _, _) => false,
        };
        if result {
            self.simplify_nuanced(true);
        }
        result
    }
}
trait TakeAsOwned {
    fn take_as_owned(&mut self) -> Self;
}
impl<T> TakeAsOwned for Vec<T> {
    fn take_as_owned(&mut self) -> Self {
        let mut x = vec![];
        mem::swap(&mut x, self);
        x
    }
}

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
//  impl GeometricAntiProduct<FlatPoint> for Sphere
// e415, e425, e435 */
// (Simd32x3::from(-1)
// Simd32x3::from([
// ((self.group0()[0] * other.group0()[3]) + (self.group1()[0] * other.group0()[0])),
// ((self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
// ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[2])),
// ])),

trait SortVecDespiteF32 {
    fn sort_with_f32(&mut self);
}
impl<Expr: Ord> SortVecDespiteF32 for Vec<(Expr, f32)> {
    fn sort_with_f32(&mut self) {
        self.sort_by(|(a_expr, a_f32), (b_expr, b_f32)| {
            a_expr.cmp(b_expr).then_with(|| {
                FloatOrd(*a_f32).cmp(&FloatOrd(*b_f32))
            })
        });
    }
}


impl FloatExpr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool) {
        match self {
            FloatExpr::Variable(_) => {}
            FloatExpr::Literal(_) => {}
            FloatExpr::AccessVec2(av2, idx) => {
                if !insides_already_done {
                    av2.simplify();
                }
                match av2.as_mut() {
                    Vec2Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec2Expr::Gather2(fe0, fe1) => {
                        *self = [fe0, fe1][*idx as usize].take_as_owned();
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec3(av3, idx) => {
                if !insides_already_done {
                    av3.simplify();
                }
                match av3.as_mut() {
                    Vec3Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec3Expr::Gather3(fe0, fe1, fe2) => {
                        *self = [fe0, fe1, fe2][*idx as usize].take_as_owned();
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec4(av4, idx) => {
                if !insides_already_done {
                    av4.simplify();
                }
                match av4.as_mut() {
                    Vec4Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec4Expr::Gather4(fe0, fe1, fe2, fe3) => {
                        *self = [fe0, fe1, fe2, fe3][*idx as usize].take_as_owned();
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessMultiVecGroup(mve, idx) => {
                if !insides_already_done {
                    mve.simplify();
                }
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(f) => {
                            *self = f.take_as_owned();
                            1
                        }
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 1 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 1. {mv}"
                        )
                    }
                }
            }
            FloatExpr::AccessMultiVecFlat(mve, idx) => {
                if !insides_already_done {
                    mve.simplify();
                }
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let mut scan_idx = 0;
                    let mut scan_group = 0;
                    while scan_group < groups.len() {
                        let i = (*idx as i32) - scan_idx;
                        if i < 0 {
                            // This can happen if the index is valid but does not simplify
                            break;
                        }
                        match &mut groups[scan_group] {
                            MultiVectorGroupExpr::JustFloat(f) => {
                                if i == 0 {
                                    *self = f.take_as_owned();
                                    return;
                                }
                                scan_idx += 1;
                            }
                            MultiVectorGroupExpr::Vec2(v2) => {
                                if i < 2 {
                                    match v2 {
                                        Vec2Expr::Gather1(f) => {
                                            *self = f.take_as_owned();
                                            return;
                                        }
                                        Vec2Expr::Gather2(f0, f1) => {
                                            *self = [f0, f1][i as usize].take_as_owned();
                                            return;
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
                                            *self = f.take_as_owned();
                                            return;
                                        }
                                        Vec3Expr::Gather3(f0, f1, f2) => {
                                            *self = [f0, f1, f2][i as usize].take_as_owned();
                                            return;
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
                                            *self = f.take_as_owned();
                                            return;
                                        }
                                        Vec4Expr::Gather4(f0, f1, f2, f3) => {
                                            *self = [f0, f1, f2, f3][i as usize].take_as_owned();
                                            return;
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
            FloatExpr::Product(product, last_factor) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.simplify();
                    }
                }
                if product.len() == 1 && *last_factor == 1.0 {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    FloatExpr::Literal(f) => {
                        *last_factor *= f32::powf(*f, *exponent);
                        false
                    }
                    FloatExpr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        *last_factor *= *another_factor;
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    FloatExpr::Literal(f) => {
                        *last_factor *= f32::powf(*f, *exponent);
                        false
                    }
                    _ => true,
                });
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);

                if product.len() == 1 && product[0].1 == 1.0 {
                    if let FloatExpr::Sum(sum, last_addend) = &mut product[0].0 {
                        *last_addend *= *last_factor;
                        for (_addend, sum_factor) in sum.iter_mut() {
                            *sum_factor *= *last_factor;
                        }
                        *last_factor = 1.0;
                    }
                    if *last_factor == 1.0 {
                        // _exponent is 1.0 (we just checked it a few lines ago)
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.is_empty() {
                    *self = FloatExpr::Literal(*last_factor);
                }
            }
            FloatExpr::Sum(sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.simplify();
                    }
                }
                if sum.len() == 1 && *last_addend == 0.0 {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        *self = FloatExpr::Product(vec![(addend, 1.0)], factor);
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    FloatExpr::Literal(f) => {
                        *last_addend += *f * *factor;
                        false
                    }
                    FloatExpr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        *last_addend += *another_addend;
                        false
                    }
                    FloatExpr::Product(p, last_factor) => {
                        *factor *= *last_factor;
                        *last_factor = 1.0;
                        if p.len() == 1 {
                            if p[0].1 == 1.0 {
                                *addend = p.remove(0).0;
                            }
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    FloatExpr::Literal(f) => {
                        *last_addend += *f * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == 0.0 {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        *self = FloatExpr::Product(vec![(addend, 1.0)], factor);
                    };
                }
                if sum.is_empty() {
                    *self = FloatExpr::Literal(*last_addend);
                }
            }
        }
    }
}
impl Vec2Expr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool) {
        match self {
            Vec2Expr::Variable(_) => {}
            Vec2Expr::Gather1(ref mut f) => {
                if !insides_already_done {
                    f.simplify();
                }
                // Do I really want to do more here?
            }
            Vec2Expr::Gather2(ref mut f0, ref mut f1) => {
                use crate::ast2::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.simplify();
                    f1.simplify();
                }
                if f0 == f1 {
                    *self = Vec2Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1) {
                    (AccessVec2(box ref mut v2_a, 0), AccessVec2(box ref mut v2_b, 1)) if v2_a == v2_b => {
                        *self = v2_a.take_as_owned();
                        return;
                    }
                    (Product(ref mut float_product_0, lits_0), Product(ref mut float_product_1, lits_1)) => {
                        let lits = [*lits_0, *lits_1];
                        if let Some(transposed) = transpose_vec2_product(float_product_0, float_product_1, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut float_sum_0, lits_0), Sum(ref mut float_sum_1, lits_1)) => {
                        let lits = [*lits_0, *lits_1];
                        if let Some(transposed) = transpose_vec2_sum(float_sum_0, float_sum_1, lits) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec2Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                if !insides_already_done {
                    mve.simplify();
                }
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(v2) => {
                            *self = v2.take_as_owned();
                            2
                        }
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 2 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 2. {mv}"
                        )
                    }
                }
            }
            Vec2Expr::Product(ref mut product, last_factor) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.simplify();
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 2] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        false
                    }
                    Vec2Expr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        last_factor[0] *= another_factor[0];
                        last_factor[1] *= another_factor[1];
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        false
                    }
                    _ => true,
                });
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);

                if product.len() == 1 && *last_factor == [1.0; 2] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    *self = if f0 == f1 { Vec2Expr::Gather1(f0) } else { Vec2Expr::Gather2(f0, f1) };
                }
            }
            Vec2Expr::Sum(ref mut sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.simplify();
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 2] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        *self = Vec2Expr::Product(vec![(addend, 1.0)], [factor, factor]);
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        false
                    }
                    Vec2Expr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        last_addend[0] += another_addend[0];
                        last_addend[1] += another_addend[1];
                        false
                    }
                    Vec2Expr::Product(p, last_factor) => {
                        if last_factor[0] == last_factor[1] {
                            *factor *= last_factor[0];
                            *last_factor = [1.0; 2];
                        }
                        if p.len() == 1 && p[0].1 == 1.0 && *last_factor == [1.0; 2] {
                            *addend = p.remove(0).0;
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == [0.0; 2] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let gather = [factor, factor];
                        *self = Vec2Expr::Product(vec![(addend, 1.0)], gather);
                    };
                }
                if sum.is_empty() {
                    let f0 = FloatExpr::Literal(last_addend[0]);
                    let f1 = FloatExpr::Literal(last_addend[1]);
                    *self = if f0 == f1 { Vec2Expr::Gather1(f0) } else { Vec2Expr::Gather2(f0, f1) };
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
        self.simplify_nuanced(false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool) {
        match self {
            Vec3Expr::Variable(_) => {}
            Vec3Expr::Gather1(ref mut f) => {
                if !insides_already_done {
                    f.simplify();
                }
                // Do I really want to do more here?
            }
            Vec3Expr::Gather3(ref mut f0, ref mut f1, ref mut f2) => {
                use crate::ast2::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.simplify();
                    f1.simplify();
                    f2.simplify();
                }
                if f0 == f1 && f0 == f2 {
                    *self = Vec3Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1, f2) {
                    (AccessVec3(box ref mut v3_a, 0), AccessVec3(box ref mut v3_b, 1), AccessVec3(box ref mut v3_c, 2)) => {
                        if v3_a == v3_b && v3_a == v3_c {
                            *self = v3_a.take_as_owned();
                            return;
                        }
                    }
                    (Product(ref mut float_product_0, lits_0), Product(ref mut float_product_1, lits_1), Product(ref mut float_product_2, lits_2)) => {
                        let lits = [*lits_0, *lits_1, *lits_2];
                        if let Some(transposed) = transpose_vec3_product(float_product_0, float_product_1, float_product_2, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut float_sum_0, lits_0), Sum(ref mut float_sum_1, lits_1), Sum(ref mut float_sum_2, lits_2)) => {
                        let lits = [*lits_0, *lits_1, *lits_2];
                        if let Some(transposed) = transpose_vec3_sum(float_sum_0, float_sum_1, float_sum_2, lits) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec3Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                if !insides_already_done {
                    mve.simplify();
                }
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(v3) => {
                            *self = v3.take_as_owned();
                            3
                        }
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 3 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 3. {mv}"
                        )
                    }
                }
            }
            Vec3Expr::Product(ref mut product, last_factor) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.simplify();
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 3] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        false
                    }
                    Vec3Expr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        last_factor[0] *= another_factor[0];
                        last_factor[1] *= another_factor[1];
                        last_factor[2] *= another_factor[2];
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        false
                    }
                    _ => true,
                });
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);

                if product.len() == 1 && *last_factor == [1.0; 3] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    let f2 = FloatExpr::Literal(last_factor[2]);
                    *self = if f0 == f1 && f1 == f2 { Vec3Expr::Gather1(f0) } else { Vec3Expr::Gather3(f0, f1, f2) };
                }
            }
            Vec3Expr::Sum(ref mut sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.simplify();
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 3] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        *self = Vec3Expr::Product(vec![(addend, 1.0)], [factor, factor, factor]);
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        false
                    }
                    Vec3Expr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        last_addend[0] += another_addend[0];
                        last_addend[1] += another_addend[1];
                        last_addend[2] += another_addend[2];
                        false
                    }
                    Vec3Expr::Product(p, last_factor) => {
                        if last_factor[0] == last_factor[1] && last_factor[1] == last_factor[2] {
                            *factor *= last_factor[0];
                            *last_factor = [1.0; 3];
                        }
                        if p.len() == 1 && p[0].1 == 1.0 && *last_factor == [1.0; 3] {
                            *addend = p.remove(0).0;
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == [0.0; 3] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let gather = [factor, factor, factor];
                        *self = Vec3Expr::Product(vec![(addend, 1.0)], gather);
                    };
                }
                if sum.is_empty() {
                    let f0 = FloatExpr::Literal(last_addend[0]);
                    let f1 = FloatExpr::Literal(last_addend[1]);
                    let f2 = FloatExpr::Literal(last_addend[2]);
                    *self = if f0 == f1 && f1 == f2 { Vec3Expr::Gather1(f0) } else { Vec3Expr::Gather3(f0, f1, f2) };
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
        self.simplify_nuanced(false);
    }

    // TODO see impl Dual for Circle
    fn simplify_nuanced(&mut self, insides_already_done: bool) {
        match self {
            Vec4Expr::Variable(_) => {}
            Vec4Expr::Gather1(f) => {
                if !insides_already_done {
                    f.simplify();
                }
                // Do I really want to do more here?
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                use crate::ast2::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.simplify();
                    f1.simplify();
                    f2.simplify();
                    f3.simplify();
                }
                if f0 == f1 && f0 == f2 && f0 == f3 {
                    *self = Vec4Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1, f2, f3) {
                    (AccessVec4(box v4_a, 0), AccessVec4(box v4_b, 1), AccessVec4(box v4_c, 2), AccessVec4(box v4_d, 3)) => {
                        if v4_a == v4_b && v4_a == v4_c && v4_a == v4_d {
                            *self = v4_a.take_as_owned();
                            return;
                        }
                    }
                    (
                        Product(ref mut float_product_0, lits_0),
                        Product(ref mut float_product_1, lits_1),
                        Product(ref mut float_product_2, lits_2),
                        Product(ref mut float_product_3, lits_3),
                    ) => {
                        let lits = [*lits_0, *lits_1, *lits_2, *lits_3];
                        if let Some(transposed) = transpose_vec4_product(float_product_0, float_product_1, float_product_2, float_product_3, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut float_sum_0, lits_0), Sum(ref mut float_sum_1, lits_1), Sum(ref mut float_sum_2, lits_2), Sum(ref mut float_sum_3, lits_3)) => {
                        let lits = [*lits_0, *lits_1, *lits_2, *lits_3];
                        if let Some(transposed) = transpose_vec4_sum(float_sum_0, float_sum_1, float_sum_2, float_sum_3, lits) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec4Expr::AccessMultiVecGroup(mve, idx) => {
                if !insides_already_done {
                    mve.simplify();
                }
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(v4) => {
                            *self = v4.take_as_owned();
                            4
                        }
                    };
                    if size != 4 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 4. {mv}"
                        )
                    }
                }
            }
            Vec4Expr::Product(product, last_factor) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.simplify();
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 4] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        last_factor[3] *= powf;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        last_factor[3] *= f32::powf(*f3, *exponent);
                        false
                    }
                    Vec4Expr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        last_factor[0] *= another_factor[0];
                        last_factor[1] *= another_factor[1];
                        last_factor[2] *= another_factor[2];
                        last_factor[3] *= another_factor[3];
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        last_factor[3] *= powf;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        last_factor[3] *= f32::powf(*f3, *exponent);
                        false
                    }
                    _ => true,
                });
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);

                if product.len() == 1 && *last_factor == [1.0; 4] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    let f2 = FloatExpr::Literal(last_factor[2]);
                    let f3 = FloatExpr::Literal(last_factor[3]);
                    *self = if f0 == f1 && f1 == f2 && f2 == f3 {
                        Vec4Expr::Gather1(f0)
                    } else {
                        Vec4Expr::Gather4(f0, f1, f2, f3)
                    };
                }
            }
            Vec4Expr::Sum(sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.simplify();
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 4] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        *self = Vec4Expr::Product(vec![(addend, 1.0)], [factor, factor, factor, factor]);
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        last_addend[3] += *f * *factor;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        last_addend[3] += *f3 * *factor;
                        false
                    }
                    Vec4Expr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        last_addend[0] += another_addend[0];
                        last_addend[1] += another_addend[1];
                        last_addend[2] += another_addend[2];
                        last_addend[3] += another_addend[3];
                        false
                    }
                    Vec4Expr::Product(p, last_factor) => {
                        if last_factor[0] == last_factor[1] && last_factor[1] == last_factor[2] && last_factor[2] == last_factor[3] {
                            *factor *= last_factor[0];
                            *last_factor = [1.0; 4];
                        }
                        if p.len() == 1 && p[0].1 == 1.0 && *last_factor == [1.0; 4] {
                            *addend = p.remove(0).0;
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        last_addend[3] += *f * *factor;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        last_addend[3] += *f3 * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == [0.0; 4] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let gather = [factor, factor, factor, factor];
                        *self = Vec4Expr::Product(vec![(addend, 1.0)], gather);
                    };
                }
                if sum.is_empty() {
                    let f0 = FloatExpr::Literal(last_addend[0]);
                    let f1 = FloatExpr::Literal(last_addend[1]);
                    let f2 = FloatExpr::Literal(last_addend[2]);
                    let f3 = FloatExpr::Literal(last_addend[3]);
                    *self = if f0 == f1 && f1 == f2 && f2 == f3 {
                        Vec4Expr::Gather1(f0)
                    } else {
                        Vec4Expr::Gather4(f0, f1, f2, f3)
                    };
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
        self.simplify_nuanced(false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool) {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => {
                if !insides_already_done {
                    f.simplify();
                }
                if let FloatExpr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = f {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                        return;
                    }
                }
                if let FloatExpr::AccessMultiVecFlat(MultiVectorExpr { expr, mv_class: _ }, idx) = f {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        let mut target = *idx;
                        for ge in v.iter_mut() {
                            match (target, ge) {
                                (0, MultiVectorGroupExpr::JustFloat(fe)) => {
                                    *self = MultiVectorGroupExpr::JustFloat(fe.take_as_owned());
                                    return;
                                }
                                (_, MultiVectorGroupExpr::JustFloat(_)) => {
                                    target -= 1;
                                }
                                (_, MultiVectorGroupExpr::Vec2(_)) => {
                                    if target < 2 {
                                        return;
                                    }
                                    target -= 2;
                                }
                                (_, MultiVectorGroupExpr::Vec3(_)) => {
                                    if target < 3 {
                                        return;
                                    }
                                    target -= 3;
                                }
                                (_, MultiVectorGroupExpr::Vec4(_)) => {
                                    if target < 4 {
                                        return;
                                    }
                                    target -= 4;
                                }
                            }
                        }
                    }
                }
            }
            MultiVectorGroupExpr::Vec2(v2) => {
                if !insides_already_done {
                    v2.simplify();
                }
                if let Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v2 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                    }
                }
            }
            MultiVectorGroupExpr::Vec3(v3) => {
                if !insides_already_done {
                    v3.simplify();
                }
                if let Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v3 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                    }
                }
            }
            MultiVectorGroupExpr::Vec4(v4) => {
                if !insides_already_done {
                    v4.simplify();
                }
                if let Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v4 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                    }
                }
            }
        }
    }
}
impl MultiVectorExpr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool) {
        match &mut *self.expr {
            MultiVectorVia::Variable(_) => {}
            MultiVectorVia::Construct(groups) => {
                for group in groups.iter_mut() {
                    if !insides_already_done {
                        group.simplify();
                    }
                }
                let result = groups.iter_mut().enumerate().fold(None, |a, (b_idx, b)| {
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
                    // Any chance of take_as_owned for MultiVectorExpr? Not trivial.
                    *self = result.clone();
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_, _) => {}
            MultiVectorVia::TraitInvoke21ToClass(_, _, _) => {}
            MultiVectorVia::TraitInvoke22ToClass(_, _, _) => {}
        }
    }
}

impl PartialEq for FloatExpr {
    fn eq(&self, other: &Self) -> bool {
        use FloatExpr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a == b,
            (Literal(a), Literal(b)) => FloatOrd(*a) == FloatOrd(*b),
            (AccessVec2(a, ai), AccessVec2(b, bi)) => a == b && ai == bi,
            (AccessVec3(a, ai), AccessVec3(b, bi)) => a == b && ai == bi,
            (AccessVec4(a, ai), AccessVec4(b, bi)) => a == b && ai == bi,
            (AccessMultiVecGroup(a, ai), AccessMultiVecGroup(b, bi)) => a == b && ai == bi,
            (AccessMultiVecFlat(a, ai), AccessMultiVecFlat(b, bi)) => a == b && ai == bi,
            (TraitInvoke11ToFloat(ak, a), TraitInvoke11ToFloat(bk, b)) => ak == bk && a == b,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(*al) != FloatOrd(*bl) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                return true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(*al) != FloatOrd(*bl) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                return true
            }
            _ => false,
        }
    }
}
impl Eq for FloatExpr {}
impl PartialOrd for FloatExpr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for FloatExpr {
    fn cmp(&self, other: &Self) -> Ordering {
        use FloatExpr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(b),
            (Literal(a), Literal(b)) => FloatOrd(*a).cmp(&FloatOrd(*b)),
            (AccessVec2(a, ai), AccessVec2(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessVec3(a, ai), AccessVec3(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessVec4(a, ai), AccessVec4(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessMultiVecGroup(a, ai), AccessMultiVecGroup(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessMultiVecFlat(a, ai), AccessMultiVecFlat(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (TraitInvoke11ToFloat(ak, a), TraitInvoke11ToFloat(bk, b)) => ak.cmp(bk).then_with(|| a.cmp(b)),
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(*al).cmp(&FloatOrd(*bl));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(*al).cmp(&FloatOrd(*bl));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            }
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Literal(_), _) => Ordering::Less,
            (_, Literal(_)) => Ordering::Greater,
            (AccessVec2(_, _), _) => Ordering::Less,
            (_, AccessVec2(_, _)) => Ordering::Greater,
            (AccessVec3(_, _), _) => Ordering::Less,
            (_, AccessVec3(_, _)) => Ordering::Greater,
            (AccessVec4(_, _), _) => Ordering::Less,
            (_, AccessVec4(_, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (AccessMultiVecFlat(_, _), _) => Ordering::Less,
            (_, AccessMultiVecFlat(_, _)) => Ordering::Greater,
            (TraitInvoke11ToFloat(_, _), _) => Ordering::Less,
            (_, TraitInvoke11ToFloat(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
        }
    }
}

impl PartialEq for Vec2Expr {
    fn eq(&self, other: &Self) -> bool {
        use Vec2Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.eq(&b),
            (Gather1(a), Gather1(b)) => a.eq(&b),
            (Gather2(a0, a1), Gather2(b0, b1)) => a0 == b0 && a1 == b1,
            (SwizzleVec2(av, a0, a1), SwizzleVec2(bv, b0, b1)) => av == bv && a0 == b0 && a1 == b1,
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => amv == bmv && ai == bi,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                return true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                return true
            }
            _ => false
        }
    }
}
impl Eq for Vec2Expr {}
impl PartialOrd for Vec2Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for Vec2Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        use Vec2Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(&b),
            (Gather1(a), Gather1(b)) => a.cmp(&b),
            (Gather2(a0, a1), Gather2(b0, b1)) => {
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                return a1.cmp(b1)
            },
            (SwizzleVec2(av, a0, a1), SwizzleVec2(bv, b0, b1)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                return a1.cmp(b1)
            },
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => {
                let c = amv.cmp(bmv);
                if c != Ordering::Equal { return c }
                return ai.cmp(bi)
            }
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            },
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Gather1(_), _) => Ordering::Less,
            (_, Gather1(_)) => Ordering::Greater,
            (Gather2(_, _), _) => Ordering::Less,
            (_, Gather2(_, _)) => Ordering::Greater,
            (SwizzleVec2(_, _, _), _) => Ordering::Less,
            (_, SwizzleVec2(_, _, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
        }
    }
}

impl PartialEq for Vec3Expr {
    fn eq(&self, other: &Self) -> bool {
        use Vec3Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.eq(&b),
            (Gather1(a), Gather1(b)) => a.eq(&b),
            (Gather3(a0, a1, a2), Gather3(b0, b1, b2)) => a0 == b0 && a1 == b1 && a2 == b2,
            (SwizzleVec3(av, a0, a1, a2), SwizzleVec3(bv, b0, b1, b2)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2,
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => amv == bmv && ai == bi,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                return true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                return true
            }
            _ => false
        }
    }
}

impl Eq for Vec3Expr {}
impl PartialOrd for Vec3Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for Vec3Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        use Vec3Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(&b),
            (Gather1(a), Gather1(b)) => a.cmp(&b),
            (Gather3(a0, a1, a2), Gather3(b0, b1, b2)) => {
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                return a2.cmp(b2)
            },
            (SwizzleVec3(av, a0, a1, a2), SwizzleVec3(bv, b0, b1, b2)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                return a2.cmp(b2)
            },
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => {
                let c = amv.cmp(bmv);
                if c != Ordering::Equal { return c }
                return ai.cmp(bi)
            }
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            },
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Gather1(_), _) => Ordering::Less,
            (_, Gather1(_)) => Ordering::Greater,
            (Gather3(_, _, _), _) => Ordering::Less,
            (_, Gather3(_, _, _)) => Ordering::Greater,
            (SwizzleVec3(_, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec3(_, _, _, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
        }
    }
}

impl PartialEq for Vec4Expr {
    fn eq(&self, other: &Self) -> bool {
        use Vec4Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.eq(&b),
            (Gather1(a), Gather1(b)) => a.eq(&b),
            (Gather4(a0, a1, a2, a3), Gather4(b0, b1, b2, b3)) => a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3,
            (SwizzleVec4(av, a0, a1, a2, a3), SwizzleVec4(bv, b0, b1, b2, b3)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3,
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => amv == bmv && ai == bi,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if FloatOrd(al[3]) != FloatOrd(bl[3]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                return true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if FloatOrd(al[3]) != FloatOrd(bl[3]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                return true
            },
            _ => false
        }
    }
}
impl Eq for Vec4Expr {}
impl PartialOrd for Vec4Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for Vec4Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        use Vec4Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(&b),
            (Gather1(a), Gather1(b)) => a.cmp(&b),
            (Gather4(a0, a1, a2, a3), Gather4(b0, b1, b2, b3)) => {
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                let c = a2.cmp(b2);
                if c != Ordering::Equal { return c }
                return a3.cmp(b3)
            },
            (SwizzleVec4(av, a0, a1, a2, a3), SwizzleVec4(bv, b0, b1, b2, b3)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                let c = a2.cmp(b2);
                if c != Ordering::Equal { return c }
                return a3.cmp(b3)
            },
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => {
                let c = amv.cmp(bmv);
                if c != Ordering::Equal { return c }
                return ai.cmp(bi)
            }
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[3]).cmp(&FloatOrd(bl[3]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[3]).cmp(&FloatOrd(bl[3]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                return Ordering::Equal
            },
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Gather1(_), _) => Ordering::Less,
            (_, Gather1(_)) => Ordering::Greater,
            (Gather4(_, _, _, _), _) => Ordering::Less,
            (_, Gather4(_, _, _, _)) => Ordering::Greater,
            (SwizzleVec4(_, _, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec4(_, _, _, _, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
        }
    }
}


impl<FE: Into<FloatExpr>> Add<FE> for FloatExpr {
    type Output = FloatExpr;

    fn add(self, rhs: FE) -> Self::Output {
        let rhs = rhs.into();
        let mut s = FloatExpr::Sum(vec![(self, 1.0), (rhs, 1.0)], 0.0);
        s.simplify();
        s
    }
}
impl<FE: Into<FloatExpr>> AddAssign<FE> for FloatExpr {
    fn add_assign(&mut self, rhs: FE) {
        let rhs = rhs.into();
        let mut x = FloatExpr::Literal(0.0);
        mem::swap(&mut x, self);
        *self = FloatExpr::Sum(vec![(x, 1.0), (rhs, 1.0)], 0.0);
        self.simplify();
    }
}
impl<FE: Into<FloatExpr>> Mul<FE> for FloatExpr {
    type Output = FloatExpr;

    fn mul(self, rhs: FE) -> Self::Output {
        let rhs = rhs.into();
        let mut s = FloatExpr::Product(vec![(self, 1.0), (rhs, 1.0)], 1.0);
        s.simplify();
        s
    }
}
impl<FE: Into<FloatExpr>> MulAssign<FE> for FloatExpr {
    fn mul_assign(&mut self, rhs: FE) {
        let rhs = rhs.into();
        let mut x = FloatExpr::Literal(1.0);
        mem::swap(&mut x, self);
        *self = FloatExpr::Product(vec![(x, 1.0), (rhs, 1.0)], 1.0);
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
        let mut s = Vec2Expr::Sum(vec![(self, 1.0), (rhs, 1.0)], [0.0; 2]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec2Expr>> AddAssign<V> for Vec2Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec2Expr::Sum(vec![(x, 1.0), (rhs, 1.0)], [0.0; 2]);
        self.simplify();
    }
}
impl<V: Into<Vec2Expr>> Mul<V> for Vec2Expr {
    type Output = Vec2Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec2Expr::Product(vec![(self, 1.0), (rhs, 1.0)], [1.0; 2]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec2Expr>> MulAssign<V> for Vec2Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec2Expr::Product(vec![(x, 1.0), (rhs, 1.0)], [1.0; 2]);
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
        let mut s = Vec3Expr::Sum(vec![(self, 1.0), (rhs, 1.0)], [0.0; 3]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec3Expr>> AddAssign<V> for Vec3Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec3Expr::Sum(vec![(x, 1.0), (rhs, 1.0)], [0.0; 3]);
        self.simplify();
    }
}
impl<V: Into<Vec3Expr>> Mul<V> for Vec3Expr {
    type Output = Vec3Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec3Expr::Product(vec![(self, 1.0), (rhs, 1.0)], [1.0; 3]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec3Expr>> MulAssign<V> for Vec3Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec3Expr::Product(vec![(x, 1.0), (rhs, 1.0)], [1.0; 3]);
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
        let mut s = Vec4Expr::Sum(vec![(self, 1.0), (rhs, 1.0)], [0.0; 4]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec4Expr>> AddAssign<V> for Vec4Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec4Expr::Sum(vec![(x, 1.0), (rhs, 1.0)], [0.0; 4]);
        self.simplify();
    }
}
impl<V: Into<Vec4Expr>> Mul<V> for Vec4Expr {
    type Output = Vec4Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec4Expr::Product(vec![(self, 1.0), (rhs, 1.0)], [1.0; 4]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec4Expr>> MulAssign<V> for Vec4Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec4Expr::Product(vec![(x, 1.0), (rhs, 1.0)], [1.0; 4]);
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
            }
            FloatExpr::AccessMultiVecFlat(m, _) => {
                let mut result = m.count_operations(lookup);
                result.basis_element_struct_access = true;
                result
            }
            FloatExpr::TraitInvoke11ToFloat(t, m) => m.count_operations(lookup) + lookup.trait_11_ops(t, &m.mv_class),
            FloatExpr::Product(v, last_factor) => {
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.floats.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.floats.div += 1;
                        }
                        _ => {
                            result.floats.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != 1.0 {
                    result.floats.mul += 1;
                }
                result
            }
            FloatExpr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.floats.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.floats.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != 0.0 {
                    result.floats.add_sub += 1;
                }
                result
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
            Vec2Expr::Product(v, last_factor) => {
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.simd2.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.simd2.div += 1;
                        }
                        _ => {
                            result.simd2.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != [1.0; 2] {
                    result.simd2.mul += 1;
                }
                result
            }
            Vec2Expr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.simd2.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd2.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != [0.0; 2] {
                    result.simd2.add_sub += 1;
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
            Vec3Expr::Product(v, last_factor) => {
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.simd3.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.simd3.div += 1;
                        }
                        _ => {
                            result.simd3.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != [1.0; 3] {
                    result.simd3.mul += 1;
                }
                result
            }
            Vec3Expr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.simd3.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd3.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != [0.0; 3] {
                    result.simd3.add_sub += 1;
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
            Vec4Expr::Product(v, last_factor) => {
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.simd4.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.simd4.div += 1;
                        }
                        _ => {
                            result.simd4.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != [1.0; 4] {
                    result.simd4.mul += 1;
                }
                result
            }
            Vec4Expr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.simd4.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd4.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != [0.0; 4] {
                    result.simd4.add_sub += 1;
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
            MultiVectorGroupExpr::Vec4(v) => v.count_operations(lookup),
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
            MultiVectorVia::TraitInvoke11ToClass(t, m) => m.count_operations(lookup) + lookup.trait_11_ops(t, &m.mv_class),
            MultiVectorVia::TraitInvoke21ToClass(t, a, b) => a.count_operations(lookup) + lookup.trait_21_ops(t, &a.mv_class, b),
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => a.count_operations(lookup) + b.count_operations(lookup) + lookup.trait_22_ops(t, &a.mv_class, &b.mv_class),
        }
    }
}

fn transpose_vec2_product(float_product_0: &mut Vec<(FloatExpr, f32)>, float_product_1: &mut Vec<(FloatExpr, f32)>, mut coalesce_product_literal: [f32; 2]) -> Option<Vec2Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Product
    let mut vec2_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            pulling_out_factor = vec2_product_extract(&mut vec2_product, &mut coalesce_product_literal, e0, f0, e1, f1);
            !pulling_out_factor
        });
        !pulling_out_factor
    });

    if vec2_product.is_empty() && coalesce_product_literal == [1.0; 2] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.take_as_owned(), 1.0)
    };
    if keep_remaining {
        vec2_product.push((Vec2Expr::Gather2(p0, p1), 1.0));
    }
    let mut result = Vec2Expr::Product(vec2_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec2_product_extract(
    vec2_product: &mut Vec<(Vec2Expr, f32)>,
    coalesce_product_literals: &mut [f32; 2],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && f0 == f1 {
        vec2_product.push((Vec2Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1) {
        (AccessVec2(box v0, 0), AccessVec2(box v1, 1)) if v0 == v1 && f0 == f1 => {
            vec2_product.push((v0.clone(), *f0));
            true
        }
        (AccessVec2(box v0, i0), AccessVec2(box v1, i1)) if v0 == v1 && f0 == f1 => {
            vec2_product.push((Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), *f0));
            true
        }
        (Sum(v0, a0), Sum(v1, a1)) if f0 == f1 => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_sum(v0, v1, a) else { return false };
            vec2_product.push((transposed, *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec2_sum(float_sum_0: &mut Vec<(FloatExpr, f32)>, float_sum_1: &mut Vec<(FloatExpr, f32)>, mut coalesce_sum_literal: [f32; 2]) -> Option<Vec2Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Sum
    let mut vec2_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            pulling_out_addend = vec2_sum_extract(&mut vec2_sum, &mut coalesce_sum_literal, e0, f0, e1, f1);
            !pulling_out_addend
        });
        !pulling_out_addend
    });

    if vec2_sum.is_empty() && coalesce_sum_literal == [0.0; 2] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.take_as_owned(), 0.0)
    };
    if keep_remaining {
        vec2_sum.push((Vec2Expr::Gather2(p0, p1), 1.0));
    }
    let mut result = Vec2Expr::Sum(vec2_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec2_sum_extract(vec2_sum: &mut Vec<(Vec2Expr, f32)>, coalesce_sum_literals: &mut [f32; 2], e0: &mut FloatExpr, f0: &mut f32, e1: &mut FloatExpr, f1: &mut f32) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && f0 == f1 {
        vec2_sum.push((Vec2Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1) {
        (AccessVec2(box v0, 0), AccessVec2(box v1, 1)) if v0 == v1 && f0 == f1 => {
            vec2_sum.push((v0.clone(), *f0));
            true
        }
        (AccessVec2(box v0, i0), AccessVec2(box v1, i1)) if v0 == v1 && f0 == f1 => {
            vec2_sum.push((Vec2Expr::SwizzleVec2(Box::new(v0.clone()), *i0, *i1), *f0));
            true
        }
        (Product(v0, a0), Product(v1, a1)) if f0 == f1 => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_product(v0, v1, a) else { return false };
            vec2_sum.push((transposed, *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec3_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Product
    let mut vec3_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            float_product_2.retain_mut(|(e2, f2)| {
                if pulling_out_factor {
                    return true;
                }
                pulling_out_factor = vec3_product_extract(&mut vec3_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2);
                !pulling_out_factor
            });
            !pulling_out_factor
        });
        !pulling_out_factor
    });

    if vec3_product.is_empty() && coalesce_product_literal == [1.0; 3] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.take_as_owned(), 1.0)
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_2.take_as_owned(), 1.0)
    };
    if keep_remaining {
        vec3_product.push((Vec3Expr::Gather3(p0, p1, p2), 1.0));
    }
    let mut result = Vec3Expr::Product(vec3_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec3_product_extract(
    vec3_product: &mut Vec<(Vec3Expr, f32)>,
    coalesce_product_literals: &mut [f32; 3],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 1.0 {
            coalesce_product_literals[2] *= f32::powf(*f, *f2);
            *f = 1.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && f0 == f1 && f1 == f2 {
        vec3_product.push((Vec3Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2) {
        (AccessVec3(box v0, 0), AccessVec3(box v1, 1), AccessVec3(box v2, 2)) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_product.push((v0.clone(), *f0));
            true
        }
        (AccessVec3(box v0, i0), AccessVec3(box v1, i1), AccessVec3(box v2, i2)) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_product.push((Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2), *f0));
            true
        }
        (Sum(v0, a0), Sum(v1, a1), Sum(v2, a2)) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = transpose_vec3_sum(v0, v1, v2, a) else { return false };
            vec3_product.push((transposed, *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec3_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Sum
    let mut vec3_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            float_sum_2.retain_mut(|(e2, f2)| {
                if pulling_out_addend {
                    return true;
                }
                pulling_out_addend = vec3_sum_extract(&mut vec3_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2);
                !pulling_out_addend
            });
            !pulling_out_addend
        });
        !pulling_out_addend
    });

    if vec3_sum.is_empty() && coalesce_sum_literal == [0.0; 3] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.take_as_owned(), 0.0)
    };
    let p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_2.take_as_owned(), 0.0)
    };
    if keep_remaining {
        vec3_sum.push((Vec3Expr::Gather3(p0, p1, p2), 1.0));
    }
    let mut result = Vec3Expr::Sum(vec3_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec3_sum_extract(
    vec3_sum: &mut Vec<(Vec3Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 3],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f * *f2;
            *f = 0.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && f0 == f1 && f1 == f2 {
        vec3_sum.push((Vec3Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2) {
        (AccessVec3(box v0, 0), AccessVec3(box v1, 1), AccessVec3(box v2, 2)) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_sum.push((v0.clone(), *f0));
            true
        }
        (AccessVec3(box v0, i0), AccessVec3(box v1, i1), AccessVec3(box v2, i2)) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_sum.push((Vec3Expr::SwizzleVec3(Box::new(v0.clone()), *i0, *i1, *i2), *f0));
            true
        }
        (Product(v0, a0), Product(v1, a1), Product(v2, a2)) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = transpose_vec3_product(v0, v1, v2, a) else { return false };
            vec3_sum.push((transposed, *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec4_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    float_product_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Product
    let mut vec4_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            float_product_2.retain_mut(|(e2, f2)| {
                if pulling_out_factor {
                    return true;
                }
                float_product_3.retain_mut(|(e3, f3)| {
                    if pulling_out_factor {
                        return true;
                    }
                    pulling_out_factor = vec4_product_extract(&mut vec4_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                    !pulling_out_factor
                });
                !pulling_out_factor
            });
            !pulling_out_factor
        });
        !pulling_out_factor
    });

    if vec4_product.is_empty() && coalesce_product_literal == [1.0; 4] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_1.take_as_owned(), 1.0)
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_2.take_as_owned(), 1.0)
    };
    let p3 = if float_product_3.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        Product(float_product_3.take_as_owned(), 1.0)
    };
    if keep_remaining {
        vec4_product.push((Vec4Expr::Gather4(p0, p1, p2, p3), 1.0));
    }
    let mut result = Vec4Expr::Product(vec4_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec4_product_extract(
    vec4_product: &mut Vec<(Vec4Expr, f32)>,
    coalesce_product_literals: &mut [f32; 4],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
    e3: &mut FloatExpr,
    f3: &mut f32,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 1.0 {
            coalesce_product_literals[2] *= f32::powf(*f, *f2);
            *f = 1.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e3 {
        if *f != 1.0 {
            coalesce_product_literals[3] *= f32::powf(*f, *f3);
            *f = 1.0;
            *f3 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && e2 == e3 && f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_product.push((Vec4Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2, e3) {
        (AccessVec4(box v0, 0), AccessVec4(box v1, 1), AccessVec4(box v2, 2), AccessVec4(box v3, 3)) if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 => {
            vec4_product.push((v0.clone(), *f0));
            true
        }
        (AccessVec4(box v0, i0), AccessVec4(box v1, i1), AccessVec4(box v2, i2), AccessVec4(box v3, i3))
            if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 =>
        {
            vec4_product.push((Vec4Expr::SwizzleVec4(Box::new(v0.clone()), *i0, *i1, *i2, *i3), *f0));
            true
        }
        (Sum(v0, a0), Sum(v1, a1), Sum(v2, a2), Sum(v3, a3)) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = transpose_vec4_sum(v0, v1, v2, v3, a) else { return false };
            vec4_product.push((transposed, *f0));
            true
        }
        _ => false,
    };
}

fn transpose_vec4_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    float_sum_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast2::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Sum
    let mut vec4_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            float_sum_2.retain_mut(|(e2, f2)| {
                if pulling_out_addend {
                    return true;
                }
                float_sum_3.retain_mut(|(e3, f3)| {
                    if pulling_out_addend {
                        return true;
                    }
                    pulling_out_addend = vec4_sum_extract(&mut vec4_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                    !pulling_out_addend
                });
                !pulling_out_addend
            });
            !pulling_out_addend
        });
        !pulling_out_addend
    });

    if vec4_sum.is_empty() && coalesce_sum_literal == [0.0; 4] {
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_1.take_as_owned(), 0.0)
    };
    let p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_2.take_as_owned(), 0.0)
    };
    let p3 = if float_sum_3.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        Sum(float_sum_3.take_as_owned(), 0.0)
    };
    if keep_remaining {
        vec4_sum.push((Vec4Expr::Gather4(p0, p1, p2, p3), 1.0));
    }
    let mut result = Vec4Expr::Sum(vec4_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.simplify();
    Some(result)
}

fn vec4_sum_extract(
    vec4_sum: &mut Vec<(Vec4Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 4],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
    e3: &mut FloatExpr,
    f3: &mut f32,
) -> bool {
    use crate::ast2::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f * *f2;
            *f = 0.0;
            *f2 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e3 {
        if *f != 0.0 {
            coalesce_sum_literals[3] += *f * *f3;
            *f = 0.0;
            *f3 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && e1 == e2 && e2 == e3 && f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_sum.push((Vec4Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    return match (e0, e1, e2, e3) {
        (AccessVec4(box v0, 0), AccessVec4(box v1, 1), AccessVec4(box v2, 2), AccessVec4(box v3, 3)) if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 => {
            vec4_sum.push((v0.clone(), *f0));
            true
        }
        (AccessVec4(box v0, i0), AccessVec4(box v1, i1), AccessVec4(box v2, i2), AccessVec4(box v3, i3))
            if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 =>
        {
            vec4_sum.push((Vec4Expr::SwizzleVec4(Box::new(v0.clone()), *i0, *i1, *i2, *i3), *f0));
            true
        }
        (Product(v0, a0), Product(v1, a1), Product(v2, a2), Product(v3, a3)) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = transpose_vec4_product(v0, v1, v2, v3, a) else { return false };
            vec4_sum.push((transposed, *f0));
            true
        }
        _ => false,
    };
}
