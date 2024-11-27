impl From<Variable<Integer>> for IntExpr {
    fn from(value: Variable<Integer>) -> Self {
        IntExpr::Variable(RawVariableInvocation { decl: value.decl.clone() })
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