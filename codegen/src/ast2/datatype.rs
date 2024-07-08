use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{MultiVec};
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

const DUMMY_ANTI_SCALAR: BasisElement = BasisElement::const_from(BasisSignature::from_bits_retain(u16::MAX))
    .with_name("DUMMY_ANTI_SCALAR_SHOULD_NOT_BE_EXPOSED", false);

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
// Solution: unsafe pointer casting to and from MultiVec<DUMMY_ANTI_SCALAR>
#[derive(Clone, Copy, Debug, Hash)]
pub struct MultiVector {
    anti_scalar: BasisElement,
    multi_vec: &'static MultiVec<DUMMY_ANTI_SCALAR>
}
impl PartialEq for MultiVector {
    fn eq(&self, other: &Self) -> bool {
        self.anti_scalar == other.anti_scalar && std::ptr::eq(self.multi_vec, other.multi_vec)
    }
}
impl Eq for MultiVector {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionType {
    Int(Integer),
    Float(Float),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Class(MultiVector),
}

// SAFETY:
// MultiVector should not use methods on the multi_vec field that depend on
// a correct AntiScalar type. Currently, this is only enforced by manual
// inspection and diligence. If you want a method that changes behavior depending
// on an accurate AntiScalar, then accept an AntiScalar as a type parameter of the method,
// and use the implementation of From or Into to convert multi_vec to the correct type first.
impl MultiVector {
    pub fn construct<F: Fn(BasisElement) -> FloatExpr>(&self, f: F) -> MultiVectorExpr {
        let mut outer = vec![];
        let groups = self.multi_vec.groups();
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
        for el in self.multi_vec.elements() {
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
        for el in self.multi_vec.elements() {
            grades |= el.grades();
        }
        grades
    }
}

impl<const AntiScalar: BasisElement> From<&'static MultiVec<AntiScalar>> for MultiVector {
    fn from(value: &'static MultiVec<AntiScalar>) -> Self {
        // SAFETY:
        // We erase the real AntiScalar in order to prevent generic type parameter infection.
        // We record the correct AntiScalar in order to safely (use unsafe to) restore the correct
        // type in Option<&'static MultiVec<AntiScalar>>::from(MultiVector).
        let multi_vec = unsafe {
            let correct_pointer: *const MultiVec<AntiScalar> = value as *const MultiVec<AntiScalar>;
            let obscure_pointer: *const MultiVec<DUMMY_ANTI_SCALAR> = correct_pointer.cast::<MultiVec<DUMMY_ANTI_SCALAR>>();
            &*(obscure_pointer)
        };
        Self {
            anti_scalar: AntiScalar,
            multi_vec,
        }
    }
}
impl<const AntiScalar: BasisElement> From<MultiVector> for Option<&'static MultiVec<AntiScalar>> {
    fn from(value: MultiVector) -> Self {
        if value.anti_scalar != AntiScalar {
            return None
        }
        // SAFETY:
        // The original AntiScalar that this MultiVector was created with was stored in
        // anti_scalar. So by checking that the type parameter of this method matches that
        // recorded type, we can know we are casting to the correct type.
        let mv = unsafe {
            let obscure_pointer: *const MultiVec<DUMMY_ANTI_SCALAR> = value.multi_vec as *const MultiVec<DUMMY_ANTI_SCALAR>;
            let correct_pointer: *const MultiVec<AntiScalar> = obscure_pointer.cast::<MultiVec<AntiScalar>>();
            &*(correct_pointer)
        };
        Some(mv)
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