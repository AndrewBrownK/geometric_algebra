
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

    pub(crate) fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
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

    pub(crate) fn maybe_variable(&self) -> Option<&RawVariableInvocation> {
        match self {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(v),
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(v),
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(v),
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(v),
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(v),
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => Some(v),
            _ => None,
        }
    }

    // /// Check if this expression is zero, assuming it is already simplified
    // pub(crate) fn is_zero(&self) -> bool {
    //     match self {
    //         AnyExpression::Int(i) => i.is_zero(),
    //         AnyExpression::Float(f) => f.is_zero(),
    //         AnyExpression::Vec2(v) => v.is_zero(),
    //         AnyExpression::Vec3(v) => v.is_zero(),
    //         AnyExpression::Vec4(v) => v.is_zero(),
    //         AnyExpression::Class(c) => c.is_zero(),
    //     }
    // }
}



/// This helps unify Variable<MultiVector> and MultiVectorExpr
pub fn extract_multivector_expr<Expr: Expression<MultiVector>>(expr: Expr) -> MultiVectorExpr {
    match expr.into_any_expression() {
        AnyExpression::Class(mve) => mve,
        _ => unreachable!("Expression<MultiVector> will always create AnyExpression::Class"),
    }
}

/// This helps unify Variable<Float> and FloatExpr
pub fn extract_float_expr<Expr: Expression<Float>>(expr: Expr) -> FloatExpr {
    match expr.into_any_expression() {
        AnyExpression::Float(f) => f,
        _ => unreachable!("Expression<Float> will always create AnyExpression::Float"),
    }
}

/// This helps unify Variable<Float> and FloatExpr
pub fn extract_integer_expr<Expr: Expression<Integer>>(expr: Expr) -> IntExpr {
    match expr.into_any_expression() {
        AnyExpression::Int(i) => i,
        _ => unreachable!("Expression<Integer> will always create AnyExpression::Int"),
    }
}


impl Variable<MultiVector> {
    // pub fn elements_flat(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
    //     let mv_expr: MultiVectorExpr = self.clone().into();
    //     self.expr_type
    //         .elements()
    //         .into_iter()
    //         .enumerate()
    //         .map(move |(i, el)| (FloatExpr::AccessMultiVecFlat(mv_expr.clone(), i as u16), el))
    // }

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

    pub fn elements(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
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
    // pub fn elements_flat(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
    //     self.mv_class
    //         .elements()
    //         .into_iter()
    //         .enumerate()
    //         .map(move |(i, el)| (FloatExpr::AccessMultiVecFlat(self.clone(), i as u16), el))
    // }

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
    pub fn elements(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
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


impl IntExpr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            IntExpr::Variable(v) => {
                let Some(lock) = v.decl.expr.as_ref() else { return false };
                let lock = lock.read();
                let AnyExpression::Int(e) = lock.deref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                drop(lock);
                *self = e;
                true
            }
            IntExpr::Literal(_) => false,
            IntExpr::TraitInvoke10ToInt(_, _) => false,
        };
        if result {
            self.simplify_nuanced(true, false, false, false);
        }
        result
    }

    #[allow(unused)]
    fn take_as_owned(&mut self) -> Self {
        let mut x = IntExpr::Literal(0);
        mem::swap(&mut x, self);
        x
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            IntExpr::Literal(0) => true,
            _ => false,
        }
    }
}

impl FloatExpr {
    pub(crate) fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            FloatExpr::Variable(v) => {
                let Some(lock) = v.decl.expr.as_ref() else { return false };
                let lock = lock.read();
                let AnyExpression::Float(e) = lock.deref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                drop(lock);
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
            FloatExpr::Exp(a, b, _) => {
                let mut result = a.deep_inline_variables();
                if let Some(b) = b {
                    result |= b.deep_inline_variables();
                }
                result
            }
            FloatExpr::FromInt(a) => a.deep_inline_variables(),
        };
        if result {
            self.simplify_nuanced(true, false, true, false, false);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = FloatExpr::Literal(0.0);
        mem::swap(&mut x, self);
        x
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            FloatExpr::Literal(0.0) => true,
            _ => false,
        }
    }
}
impl Vec2Expr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            Vec2Expr::Variable(v) => {
                let Some(lock) = v.decl.expr.as_ref() else { return false };
                let lock = lock.read();
                let AnyExpression::Vec2(e) = lock.deref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                drop(lock);
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
            Vec2Expr::Truncate3to2(box v) => v.deep_inline_variables(),
            Vec2Expr::Truncate4to2(box v) => v.deep_inline_variables(),
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
            self.simplify_nuanced(true, false, false, false);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }

    fn take_part_as_owned(&mut self, idx: u8) -> FloatExpr {
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        return match x {
            Vec2Expr::Variable(_) => FloatExpr::AccessVec2(Box::new(x), idx),
            Vec2Expr::Gather1(f) => f,
            Vec2Expr::Gather2(f0, f1) => match idx {
                0 => f0, 1 => f1, _ => panic!("{idx} does not fit in Vec2 for take_part_as_owned")
            },
            Vec2Expr::AccessMultiVecGroup(mve, g_idx) => match *mve.expr {
                MultiVectorVia::Construct(mut groups) => groups[g_idx as usize].take_part_as_owned(idx),
                _ => {
                    let mut flat_idx: u16 = 0;
                    for (scan_g_idx, (_, g)) in mve.groups().enumerate() {
                        if scan_g_idx == g_idx as usize {
                            flat_idx = flat_idx + idx as u16;
                            break
                        }
                        flat_idx = flat_idx + g.simd_width() as u16;
                    }
                    FloatExpr::AccessMultiVecFlat(mve, flat_idx)
                },
            }
            Vec2Expr::Product(v_factors, v_lits) => {
                let mut f_factors = vec![];
                for mut v_factor in v_factors {
                    f_factors.push((v_factor.0.take_part_as_owned(idx), v_factor.1));
                }
                let f_lit = v_lits[idx as usize];
                FloatExpr::Product(f_factors, f_lit)
            }
            Vec2Expr::Sum(v_addends, v_lits) => {
                let mut f_addends = vec![];
                for mut v_addend in v_addends {
                    f_addends.push((v_addend.0.take_part_as_owned(idx), v_addend.1));
                }
                let f_lit = v_lits[idx as usize];
                FloatExpr::Sum(f_addends, f_lit)
            }
            Vec2Expr::SwizzleVec2(box mut v, x, y) => v.take_part_as_owned([x, y][idx as usize]),
            Vec2Expr::Truncate3to2(box mut v3) => v3.take_part_as_owned(idx),
            Vec2Expr::Truncate4to2(box mut v4) => v4.take_part_as_owned(idx),
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            Vec2Expr::Gather1(f) => f.is_zero(),
            _ => false,
        }
    }
}
impl Vec3Expr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            Vec3Expr::Variable(v) => {
                let Some(lock) = v.decl.expr.as_ref() else { return false };
                let lock = lock.read();
                let AnyExpression::Vec3(e) = lock.deref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                drop(lock);
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
            Vec3Expr::Extend2to3(v2, f1) => {
                let mut result = false;
                result |= v2.deep_inline_variables();
                result |= f1.deep_inline_variables();
                result
            }
            Vec3Expr::Truncate4to3(box v) => v.deep_inline_variables(),
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
            self.simplify_nuanced(true, false, false, false);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
    fn take_part_as_owned(&mut self, idx: u8) -> FloatExpr {
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        return match x {
            Vec3Expr::Variable(_) => FloatExpr::AccessVec3(Box::new(x), idx),
            Vec3Expr::Gather1(f) => f,
            Vec3Expr::Gather3(f0, f1, f2) => match idx {
                0 => f0, 1 => f1, 2 => f2, _ => panic!("{idx} does not fit in Vec3 for take_part_as_owned")
            },
            Vec3Expr::AccessMultiVecGroup(mve, g_idx) => match *mve.expr {
                MultiVectorVia::Construct(mut groups) => groups[g_idx as usize].take_part_as_owned(idx),
                _ => {
                    let mut flat_idx: u16 = 0;
                    for (scan_g_idx, (_, g)) in mve.groups().enumerate() {
                        if scan_g_idx == g_idx as usize {
                            flat_idx = flat_idx + idx as u16;
                            break
                        }
                        flat_idx = flat_idx + g.simd_width() as u16;
                    }
                    FloatExpr::AccessMultiVecFlat(mve, flat_idx)
                },
            }
            Vec3Expr::Product(v_factors, v_lits) => {
                let mut f_factors = vec![];
                for mut v_factor in v_factors {
                    f_factors.push((v_factor.0.take_part_as_owned(idx), v_factor.1));
                }
                let f_lit = v_lits[idx as usize];
                FloatExpr::Product(f_factors, f_lit)
            }
            Vec3Expr::Sum(v_addends, v_lits) => {
                let mut f_addends = vec![];
                for mut v_addend in v_addends {
                    f_addends.push((v_addend.0.take_part_as_owned(idx), v_addend.1));
                }
                let f_lit = v_lits[idx as usize];
                FloatExpr::Sum(f_addends, f_lit)
            }
            Vec3Expr::SwizzleVec3(box mut v, x, y, z) => v.take_part_as_owned([x, y, z][idx as usize]),
            Vec3Expr::Truncate4to3(box mut v4) => v4.take_part_as_owned(idx),
            Vec3Expr::Extend2to3(mut v2, f) => match idx {
                0 | 1 => v2.take_part_as_owned(idx),
                2 => f,
                _ => panic!("{idx} does not fit in Vec3 for take_part_as_owned")
            }
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            Vec3Expr::Gather1(f) => f.is_zero(),
            _ => false,
        }
    }
}
impl Vec4Expr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self {
            Vec4Expr::Variable(v) => {
                let Some(lock) = v.decl.expr.as_ref() else { return false };
                let lock = lock.read();
                let AnyExpression::Vec4(e) = lock.deref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                drop(lock);
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
            Vec4Expr::Extend2to4(v2, f1, f2) => {
                let mut result = false;
                result |= v2.deep_inline_variables();
                result |= f1.deep_inline_variables();
                result |= f2.deep_inline_variables();
                result
            }
            Vec4Expr::Extend3to4(v2, f1) => {
                let mut result = false;
                result |= v2.deep_inline_variables();
                result |= f1.deep_inline_variables();
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
            self.simplify_nuanced(true, false, false, false);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
    fn take_part_as_owned(&mut self, idx: u8) -> FloatExpr {
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        return match x {
            Vec4Expr::Variable(_) => FloatExpr::AccessVec4(Box::new(x), idx),
            Vec4Expr::Gather1(f) => f,
            Vec4Expr::Gather4(f0, f1, f2, f3) => match idx {
                0 => f0, 1 => f1, 2 => f2, 3 => f3, _ => panic!("{idx} does not fit in Vec4 for take_part_as_owned")
            },
            Vec4Expr::AccessMultiVecGroup(mve, g_idx) => match *mve.expr {
                MultiVectorVia::Construct(mut groups) => groups[g_idx as usize].take_part_as_owned(idx),
                _ => {
                    let mut flat_idx: u16 = 0;
                    for (scan_g_idx, (_, g)) in mve.groups().enumerate() {
                        if scan_g_idx == g_idx as usize {
                            flat_idx = flat_idx + idx as u16;
                            break
                        }
                        flat_idx = flat_idx + g.simd_width() as u16;
                    }
                    FloatExpr::AccessMultiVecFlat(mve, flat_idx)
                },
            }
            Vec4Expr::Product(v_factors, v_lits) => {
                let mut f_factors = vec![];
                for mut v_factor in v_factors {
                    f_factors.push((v_factor.0.take_part_as_owned(idx), v_factor.1));
                }
                let f_lit = v_lits[idx as usize];
                FloatExpr::Product(f_factors, f_lit)
            }
            Vec4Expr::Sum(v_addends, v_lits) => {
                let mut f_addends = vec![];
                for mut v_addend in v_addends {
                    f_addends.push((v_addend.0.take_part_as_owned(idx), v_addend.1));
                }
                let f_lit = v_lits[idx as usize];
                FloatExpr::Sum(f_addends, f_lit)
            }
            Vec4Expr::SwizzleVec4(box mut v, x, y, z, w) => v.take_part_as_owned([x, y, z, w][idx as usize]),
            Vec4Expr::Extend2to4(mut v2, z, w) => match idx {
                0 | 1 => v2.take_part_as_owned(idx),
                2 => z,
                3 => w,
                _ => panic!("{idx} does not fit in Vec4 for take_part_as_owned")
            }
            Vec4Expr::Extend3to4(mut v3, f) => match idx {
                0 | 1 | 2 => v3.take_part_as_owned(idx),
                3 => f,
                _ => panic!("{idx} does not fit in Vec4 for take_part_as_owned")
            }
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            Vec4Expr::Gather1(f) => f.is_zero(),
            _ => false,
        }
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
            self.simplify_nuanced(true, false, false, false);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = MultiVectorGroupExpr::JustFloat(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
    fn take_part_as_owned(&mut self, idx: u8) -> FloatExpr {
        let mut x = MultiVectorGroupExpr::JustFloat(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        return match x {
            MultiVectorGroupExpr::JustFloat(f) => f,
            MultiVectorGroupExpr::Vec2(mut v) => v.take_part_as_owned(idx),
            MultiVectorGroupExpr::Vec3(mut v) => v.take_part_as_owned(idx),
            MultiVectorGroupExpr::Vec4(mut v) => v.take_part_as_owned(idx),
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => f.is_zero(),
            MultiVectorGroupExpr::Vec2(v) => v.is_zero(),
            MultiVectorGroupExpr::Vec3(v) => v.is_zero(),
            MultiVectorGroupExpr::Vec4(v) => v.is_zero(),
        }
    }
}
impl MultiVectorExpr {
    fn deep_inline_variables(&mut self) -> bool {
        let result = match self.expr.as_mut() {
            MultiVectorVia::Variable(v) => {
                let Some(lock) = v.decl.expr.as_ref() else { return false };
                let lock = lock.read();
                let AnyExpression::Class(e) = lock.deref() else { return false };
                let mut e = e.clone();
                e.deep_inline_variables();
                drop(lock);
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
            MultiVectorVia::TraitInvoke12iToClass(_, _, _) => false,
            MultiVectorVia::TraitInvoke12fToClass(_, _, _) => false,
        };
        if result {
            self.simplify_nuanced(true, false, false, false);
        }
        result
    }

    fn take_as_owned(&mut self) -> Self {
        let mut x = MultiVectorExpr {
            mv_class: self.mv_class,
            expr: Box::new(MultiVectorVia::Construct(vec![]))
        };
        mem::swap(&mut x, self);
        x
    }

    /// Check if this expression is zero, assuming it is already simplified
    pub(crate) fn is_zero(&self) -> bool {
        match self.expr.as_ref() {
            MultiVectorVia::Construct(gs) => gs.iter().all(|it| it.is_zero()),
            _ => false,
        }
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