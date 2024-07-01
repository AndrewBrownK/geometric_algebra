use std::sync::Arc;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{MultiVecEnum, ConsolidateEnum, mono_grade_groups, MultiVec, num_elements};
use crate::ast2::expressions::{FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
// TODO move these items to better locations

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Integer;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Float;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Vec2;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Vec3;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Vec4;
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct MultiVector(Arc<MultiVecEnum>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionType {
    Int(Integer),
    Float(Float),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Class(MultiVector),
}



impl MultiVector {
    pub fn construct<F: Fn(BasisElement) -> FloatExpr>(&self, f: F) -> MultiVectorExpr {
        let mut outer = vec![];
        for thing in self.0.element_groups() {
            outer.push(match thing.len() {
                1 => MultiVectorGroupExpr::JustFloat(f(thing[0])),
                2 => MultiVectorGroupExpr::Vec2(Vec2Expr::Gather2(f(thing[0]), f(thing[1]))),
                3 => MultiVectorGroupExpr::Vec3(Vec3Expr::Gather3(f(thing[0]), f(thing[1]), f(thing[2]))),
                4 => MultiVectorGroupExpr::Vec4(Vec4Expr::Gather4(f(thing[0]), f(thing[1]), f(thing[2]), f(thing[3]))),
                _ => unreachable!("BasisElementGroup has len 1-4")
            });
        }
        MultiVectorExpr {
            mv_class: self.clone(),
            expr: Box::new(MultiVectorVia::Construct(outer)),
        }
    }

    pub fn grade(&self) -> Option<u32> {
        let mut grade = None;
        for el in self.0.elements() {
            let elg = el.grade();
            match grade {
                None => grade = Some(elg),
                Some(gr) => if elg != gr { return None; },
            }
        }
        grade
    }

    pub fn anti_grade(&self, anti_scalar: BasisElement) -> Option<u32> {
        let gr = self.grade()?;
        Some(anti_scalar.grade() - gr)
    }

    pub fn grades(&self) -> Grades {
        let mut grades = Grades::none;
        for el in self.0.elements() {
            grades |= el.grades();
        }
        grades
    }
}





pub trait ClassesFromRegistry {
    fn include_class(&self, mvc: &MultiVecEnum) -> bool;
}
pub struct NoParam;
impl ClassesFromRegistry for NoParam {
    fn include_class(&self, _: &MultiVecEnum) -> bool {
        false
    }
}
pub struct AnyClasses;
impl ClassesFromRegistry for AnyClasses {
    fn include_class(&self, _: &MultiVecEnum) -> bool {
        true
    }
}



/// Good for manual implementations
pub struct Specifically<const D: u8>(MultiVec<D>) where
    MultiVec<D>: ConsolidateEnum<Output=MultiVecEnum>,
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized;
impl<const D: u8> ClassesFromRegistry for Specifically<D> where
    MultiVec<D>: ConsolidateEnum<Output=MultiVecEnum>,
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {
    fn include_class(&self, mvc: &MultiVecEnum) -> bool {
        mvc.adapt_eq(&self.0)
    }
}