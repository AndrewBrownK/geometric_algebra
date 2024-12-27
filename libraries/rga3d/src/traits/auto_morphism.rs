use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AutoMorphism
/// Negate every BasisElement with an odd Grade. Also known as grade involution.
pub trait AutoMorphism {
    fn auto_morphism(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static auto_morphism: AutoMorphismPrefixOrPostfix = AutoMorphismPrefixOrPostfix;
pub struct AutoMorphismPrefixOrPostfix;
impl<A: AutoMorphism> std::ops::Div<A> for AutoMorphismPrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.auto_morphism()
    }
}
include!("./impls/auto_morphism.rs");
