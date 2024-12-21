use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AutoMorphism
/// Negate every BasisElement with an odd Grade. Also known as grade involution.
pub trait AutoMorphism {
    fn auto_morphism(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct auto_morphism;
impl<A: AutoMorphism> std::ops::Div<A> for auto_morphism {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.auto_morphism()
    }
}
include!("./impls/auto_morphism.rs");
