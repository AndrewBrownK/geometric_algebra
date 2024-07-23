use std::fmt::Debug;
use std::mem;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::sync::Arc;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::BasisElementGroup;
use crate::ast2::{RawVariableDeclaration, RawVariableInvocation, Variable};
use crate::ast2::datatype::{ExpressionType, Float, Integer, MultiVector, Vec2, Vec3, Vec4};
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
    Product(Vec<FloatExpr>),
    Sum(Vec<FloatExpr>)
    // TODO divide, sqrt, pow, etc
}
#[derive(PartialEq, Clone, Debug)]
pub enum Vec2Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather2(FloatExpr, FloatExpr),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    Product(Vec<Vec2Expr>),
    Sum(Vec<Vec2Expr>),
    // TODO divide, sqrt, pow, etc
}
#[derive(PartialEq, Clone, Debug)]
pub enum Vec3Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather3(FloatExpr, FloatExpr, FloatExpr),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    Product(Vec<Vec3Expr>),
    Sum(Vec<Vec3Expr>),
    // TODO divide, sqrt, pow, etc
}
#[derive(PartialEq, Clone, Debug)]
pub enum Vec4Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather4(FloatExpr, FloatExpr, FloatExpr, FloatExpr),
    AccessMultiVecGroup(MultiVectorExpr, u16),
    Product(Vec<Vec4Expr>),
    Sum(Vec<Vec4Expr>),
    // TODO divide, sqrt, pow, etc
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

// TODO this but for other variable types
impl Variable<MultiVector> {
    pub fn expr(self) -> MultiVectorExpr {
        MultiVectorExpr {
            mv_class: self.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation {
                decl: self.decl.clone(),
            })),
        }
    }
}

impl Variable<MultiVector> {
    pub fn elements(&self) -> impl Iterator<Item=(FloatExpr, BasisElement)> + '_ {
        let mv_expr = self.clone().expr();
        self.expr_type.elements().into_iter().enumerate().map(move |(i, el)| {
            (FloatExpr::AccessMultiVecFlat(mv_expr.clone(), i as u16), el)
        })
    }

    // TODO still not exactly sure how I feel about this, but we'll see.
    pub fn groups(&self) -> impl Iterator<Item=(MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        let mv_expr = self.clone().expr();
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
}

impl MultiVectorExpr {
    pub fn elements(&self) -> impl Iterator<Item=(FloatExpr, BasisElement)> + '_ {
        self.mv_class.elements().into_iter().enumerate().map(|(i, el)| {
            (FloatExpr::AccessMultiVecFlat(self.clone(), i as u16), el)
        })
    }

    // TODO still not exactly sure how I feel about this, but we'll see.
    pub fn groups(&self) -> impl Iterator<Item=(MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        let mv_expr = self.clone();
        self.mv_class.groups().into_iter().enumerate().map(move |(g, group)| {
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
}

impl From<Variable<Float>> for FloatExpr {
    fn from(value: Variable<Float>) -> Self {
        FloatExpr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}
impl From<Variable<Vec2>> for Vec2Expr {
    fn from(value: Variable<Vec2>) -> Self {
        Vec2Expr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}
impl From<Variable<Vec3>> for Vec3Expr {
    fn from(value: Variable<Vec3>) -> Self {
        Vec3Expr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}
impl From<Variable<Vec4>> for Vec4Expr {
    fn from(value: Variable<Vec4>) -> Self {
        Vec4Expr::Variable(RawVariableInvocation {
            decl: value.decl.clone(),
        })
    }
}


impl FloatExpr {
    fn simplify(&mut self) {
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
                for p in  product.iter_mut() {
                    p.simplify();
                    if let FloatExpr::Literal(0.0) = p {
                        *self = FloatExpr::Literal(0.0);
                        return
                    }
                }
                let mut coalesce = 1.0;
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let FloatExpr::Literal(f) = it {
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
                }
            }
            FloatExpr::Sum(sum) => {
                for s in  sum.iter_mut() {
                    s.simplify();
                }
                let mut coalesce = 0.0;
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let FloatExpr::Literal(f) = it {
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
                }
            }
        }
    }
}
impl Vec2Expr {
    fn simplify(&mut self) {
        match self {
            Vec2Expr::Variable(_) => {}
            Vec2Expr::Gather1(f) => {
                f.simplify();
                // Do I really want to do more here?
            }
            Vec2Expr::Gather2(f0, f1) => {
                f0.simplify();
                f1.simplify();
                if f0 == f1 {
                    *self = Vec2Expr::Gather1(f0.clone());
                    return;
                }
                if let (FloatExpr::AccessVec2(v2_a, 0), FloatExpr::AccessVec2(v2_b, 1)) = (f0, f1) {
                    if v2_a == v2_b {
                        *self = *v2_a.clone();
                        return;
                    }
                }
            }
            Vec2Expr::AccessMultiVecGroup(mve, idx) => {
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
            Vec2Expr::Product(product) => {
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
                let mut coalesce = [1.0, 1.0];
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let Vec2Expr::Gather1(FloatExpr::Literal(f)) = it {
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
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
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [1.0, 1.0] {
                    product.push(Vec2Expr::Gather2(
                        FloatExpr::Literal(coalesce[0]),
                        FloatExpr::Literal(coalesce[1]),
                    ))
                }
                product.append(&mut flatten);
                if product.len() == 1 {
                    *self = product.remove(0);
                }
            }
            Vec2Expr::Sum(sum) => {
                for s in sum.iter_mut() {
                    s.simplify();
                }
                let mut coalesce = [0.0, 0.0];
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let Vec2Expr::Gather1(FloatExpr::Literal(f)) = it {
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
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
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        false
                    } else if let Vec2Expr::Gather2(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1)) = it {
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [0.0, 0.0] {
                    sum.push(Vec2Expr::Gather2(
                        FloatExpr::Literal(coalesce[0]),
                        FloatExpr::Literal(coalesce[1]),
                    ))
                }
                sum.append(&mut flatten);
                if sum.len() == 1 {
                    *self = sum.remove(0);
                }
            }
        }
    }
}
impl Vec3Expr {
    fn simplify(&mut self) {
        match self {
            Vec3Expr::Variable(_) => {}
            Vec3Expr::Gather1(f) => {
                f.simplify();
                // Do I really want to do more here?
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                f0.simplify();
                f1.simplify();
                f2.simplify();
                if f0 == f1 && f0 == f2 {
                    *self = Vec3Expr::Gather1(f0.clone());
                    return;
                }
                if let (
                    FloatExpr::AccessVec3(v3_a, 0),
                    FloatExpr::AccessVec3(v3_b, 1),
                    FloatExpr::AccessVec3(v3_c, 2),
                ) = (f0, f1, f2) {
                    if v3_a == v3_b && v3_a == v3_c {
                        *self = *v3_a.clone();
                        return;
                    }
                }
            }
            Vec3Expr::AccessMultiVecGroup(mve, idx) => {
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
            Vec3Expr::Product(product) => {
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
                let mut coalesce = [1.0, 1.0, 1.0];
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let Vec3Expr::Gather1(FloatExpr::Literal(f)) = it {
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        coalesce[2] *= *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
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
                        coalesce[0] *= *f;
                        coalesce[1] *= *f;
                        coalesce[2] *= *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
                        coalesce[0] *= *f0;
                        coalesce[1] *= *f1;
                        coalesce[2] *= *f2;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [1.0, 1.0, 1.0] {
                    product.push(Vec3Expr::Gather3(
                        FloatExpr::Literal(coalesce[0]),
                        FloatExpr::Literal(coalesce[1]),
                        FloatExpr::Literal(coalesce[2]),
                    ))
                }
                product.append(&mut flatten);
                if product.len() == 1 {
                    *self = product.remove(0);
                }
            }
            Vec3Expr::Sum(sum) => {
                for s in sum.iter_mut() {
                    s.simplify();
                }
                let mut coalesce = [0.0, 0.0, 0.0];
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let Vec3Expr::Gather1(FloatExpr::Literal(f)) = it {
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        coalesce[2] += *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
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
                        coalesce[0] += *f;
                        coalesce[1] += *f;
                        coalesce[2] += *f;
                        false
                    } else if let Vec3Expr::Gather3(
                        FloatExpr::Literal(f0),
                        FloatExpr::Literal(f1),
                        FloatExpr::Literal(f2)) = it {
                        coalesce[0] += *f0;
                        coalesce[1] += *f1;
                        coalesce[2] += *f2;
                        false
                    } else {
                        true
                    }
                });
                if coalesce != [0.0, 0.0, 0.0] {
                    sum.push(Vec3Expr::Gather3(
                        FloatExpr::Literal(coalesce[0]),
                        FloatExpr::Literal(coalesce[1]),
                        FloatExpr::Literal(coalesce[2]),
                    ))
                }
                sum.append(&mut flatten);
                if sum.len() == 1 {
                    *self = sum.remove(0);
                }
            }
        }
    }
}
impl Vec4Expr {
    fn simplify(&mut self) {
        match self {
            Vec4Expr::Variable(_) => {}
            Vec4Expr::Gather1(f) => {
                f.simplify();
                // Do I really want to do more here?
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                f0.simplify();
                f1.simplify();
                f2.simplify();
                f3.simplify();
                if f0 == f1 && f0 == f2 && f0 == f3 {
                    *self = Vec4Expr::Gather1(f0.clone());
                    return;
                }
                if let (
                    FloatExpr::AccessVec4(v4_a, 0),
                    FloatExpr::AccessVec4(v4_b, 1),
                    FloatExpr::AccessVec4(v4_c, 2),
                    FloatExpr::AccessVec4(v4_d, 3),
                ) = (f0, f1, f2, f3) {
                    if v4_a == v4_b && v4_a == v4_c && v4_a == v4_d {
                        *self = *v4_a.clone();
                        return;
                    }
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
                let mut coalesce = [1.0, 1.0, 1.0, 1.0];
                let mut flatten = vec![];
                product.retain_mut(|it| {
                    if let Vec4Expr::Gather1(FloatExpr::Literal(f)) = it {
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
                    product.push(Vec4Expr::Gather4(
                        FloatExpr::Literal(coalesce[0]),
                        FloatExpr::Literal(coalesce[1]),
                        FloatExpr::Literal(coalesce[2]),
                        FloatExpr::Literal(coalesce[3]),
                    ))
                }
                product.append(&mut flatten);
                if product.len() == 1 {
                    *self = product.remove(0);
                }
            }
            Vec4Expr::Sum(sum) => {
                for s in sum.iter_mut() {
                    s.simplify();
                }
                let mut coalesce = [0.0, 0.0, 0.0, 0.0];
                let mut flatten = vec![];
                sum.retain_mut(|it| {
                    if let Vec4Expr::Gather1(FloatExpr::Literal(f)) = it {
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
                    sum.push(Vec4Expr::Gather4(
                        FloatExpr::Literal(coalesce[0]),
                        FloatExpr::Literal(coalesce[1]),
                        FloatExpr::Literal(coalesce[2]),
                        FloatExpr::Literal(coalesce[3]),
                    ))
                }
                sum.append(&mut flatten);
                if sum.len() == 1 {
                    *self = sum.remove(0);
                }
            }
        }
    }
}
impl MultiVectorGroupExpr {
    fn simplify(&mut self) {
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
    fn simplify(&mut self) {
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


// TODO implement arithmetic for things that can turn INTO FloatExpr, not FloatExpr directly. UGH.
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



impl Add<Vec2Expr> for Vec2Expr {
    type Output = Vec2Expr;

    fn add(self, rhs: Vec2Expr) -> Self::Output {
        let mut s = Vec2Expr::Sum(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl AddAssign<Vec2Expr> for Vec2Expr {
    fn add_assign(&mut self, rhs: Vec2Expr) {
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec2Expr::Sum(vec![x, rhs]);
        self.simplify();
    }
}
impl Mul<Vec2Expr> for Vec2Expr {
    type Output = Vec2Expr;

    fn mul(self, rhs: Vec2Expr) -> Self::Output {
        let mut s = Vec2Expr::Product(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl MulAssign<Vec2Expr> for Vec2Expr {
    fn mul_assign(&mut self, rhs: Vec2Expr) {
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



impl Add<Vec3Expr> for Vec3Expr {
    type Output = Vec3Expr;

    fn add(self, rhs: Vec3Expr) -> Self::Output {
        let mut s = Vec3Expr::Sum(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl AddAssign<Vec3Expr> for Vec3Expr {
    fn add_assign(&mut self, rhs: Vec3Expr) {
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec3Expr::Sum(vec![x, rhs]);
        self.simplify();
    }
}
impl Mul<Vec3Expr> for Vec3Expr {
    type Output = Vec3Expr;

    fn mul(self, rhs: Vec3Expr) -> Self::Output {
        let mut s = Vec3Expr::Product(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl MulAssign<Vec3Expr> for Vec3Expr {
    fn mul_assign(&mut self, rhs: Vec3Expr) {
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



impl Add<Vec4Expr> for Vec4Expr {
    type Output = Vec4Expr;

    fn add(self, rhs: Vec4Expr) -> Self::Output {
        let mut s = Vec4Expr::Sum(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl AddAssign<Vec4Expr> for Vec4Expr {
    fn add_assign(&mut self, rhs: Vec4Expr) {
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec4Expr::Sum(vec![x, rhs]);
        self.simplify();
    }
}
impl Mul<Vec4Expr> for Vec4Expr {
    type Output = Vec4Expr;

    fn mul(self, rhs: Vec4Expr) -> Self::Output {
        let mut s = Vec4Expr::Product(vec![self, rhs]);
        s.simplify();
        s
    }
}
impl MulAssign<Vec4Expr> for Vec4Expr {
    fn mul_assign(&mut self, rhs: Vec4Expr) {
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