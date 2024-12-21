use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiAutoMorphism
/// Negate every BasisElement with an odd AntiGrade.
pub trait AntiAutoMorphism {
    fn anti_auto_morphism(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_auto_morphism;
impl<A: AntiAutoMorphism> std::ops::Div<A> for anti_auto_morphism {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_auto_morphism()
    }
}
include!("./impls/anti_auto_morphism.rs");
