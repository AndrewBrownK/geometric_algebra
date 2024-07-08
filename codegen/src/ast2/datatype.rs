use std::sync::Arc;

use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::elements::e12345;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{MultiVec, MultiVecTrait};
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

// Should we infect a generic AntiScalar parameter across the AST?
//
// So... the way this works is.... we have TraitImpl_10/11/21/22. Right now these accept a
// generic AntiScalar at the method level, instead of the trait level. The reason that this is
// nice is that we can find the TraitImpl_10/11/21/22 implementation without having to specify
// an AntiScalar, thus allowing the compiler to find and use the NameTrait, and declare the
// static Elaborated<Traits> without depending on an AntiScalar.
//
// To be clear, it wouldn't be so bad to move the AntiScalar to the trait-level instead of
// method-level on TraitImpl_10/11/21/22, but ONLY IF the detection/use of NameTrait weren't
// impacted, and no AntiScalar had to be specified on the static Elaborated<Traits>.
//
// Then again... there's something to be said for the "purity" of a trait definition that works
// for all possible AntiScalars, instead of allowing you to define it such that it only works
// with specific AntiScalars. This is especially true since we allow such huge flexibility of
// AntiScalar specification, it would be stupid for something to work on e1234 but not e0123.
//
// So with that in mind, I think the preferred approach (with respect to TraitImpl_10/11/21/22) is
// to leave the AntiScalar at the method-level and not the trait-level. This brings us to our next
// problem then. When implementing TraitImpl_10/11/21/22, you specify the associated type "Output".
// Obviously, "MultiVector" (our struct that we are commenting on right here) is a common output
// type. And MultiVector holds a reference to a MultiVec, but MultiVecs cannot be specified without
// and AntiScalar (generic or otherwise). So if we infect MultiVector with an AntiScalar too, then
// we have no way to specify the associated output type of TraitImpl_10/11/21/22 without also
// infecting those traits. In other words, we have to erase the AntiScalar right here.
//
// Solution: MultiVecTrait
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct MultiVector(&'static dyn MultiVecTrait);

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
        let groups = self.0.groups();
        let mut i = 0;
        while i < groups.len() {
            let thing = groups.get(i).clone().into_vec();
            outer.push(match thing.len() {
                1 => MultiVectorGroupExpr::JustFloat(f(thing[0])),
                2 => MultiVectorGroupExpr::Vec2(Vec2Expr::Gather2(f(thing[0]), f(thing[1]))),
                3 => MultiVectorGroupExpr::Vec3(Vec3Expr::Gather3(f(thing[0]), f(thing[1]), f(thing[2]))),
                4 => MultiVectorGroupExpr::Vec4(Vec4Expr::Gather4(f(thing[0]), f(thing[1]), f(thing[2]), f(thing[3]))),
                _ => unreachable!("BasisElementGroup has len 1-4")
            });
            i += 1;
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
    fn include_class(&self, mvc: &MultiVector) -> bool;
}
pub struct NoParam;
impl ClassesFromRegistry for NoParam {
    fn include_class(&self, _: &MultiVector) -> bool {
        false
    }
}
pub struct AnyClasses;
impl ClassesFromRegistry for AnyClasses {
    fn include_class(&self, _: &MultiVector) -> bool {
        true
    }
}



/// Good for manual implementations
pub struct Specifically(MultiVector);

impl ClassesFromRegistry for Specifically {
    fn include_class(&self, mvc: &MultiVector) -> bool {
        mvc == &self.0.clone()
    }
}