

impl Debug for MultiVectorExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.mv_class.name();
        write!(f, "{n}(")?;
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
                    use BasisElementGroup::*;
                    use MultiVectorGroupExpr::*;
                    match (group, expr) {
                        (G1(be0), JustFloat(f0)) => {
                            write!(f, "/* {be0} */ {f0:?}")?;
                        }
                        (G2(be0, be1), Vec2(v2)) => {
                            write!(f, "/* {be0}, {be1} */ {v2:?}")?;
                        }
                        (G3(be0, be1, be2), Vec3(v3)) => {
                            write!(f, "/* {be0}, {be1}, {be2} */ {v3:?}")?;
                        }
                        (G4(be0, be1, be2, be3), Vec4(v4)) => {
                            write!(f, "/* {be0}, {be1}, {be2}, {be3} */ {v4:?}")?;
                        }
                        _ => unreachable!("mv construction groups must match")
                    }
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
            MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {i})")?
            }
            MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {fe})")?
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}