use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiAutoMorphism
/// Negate every BasisElement with an odd AntiGrade.
pub trait AntiAutoMorphism {
    fn anti_auto_morphism(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_auto_morphism: AntiAutoMorphismPrefixOrPostfix = AntiAutoMorphismPrefixOrPostfix;
pub struct AntiAutoMorphismPrefixOrPostfix;
impl<A: AntiAutoMorphism> std::ops::Div<A> for AntiAutoMorphismPrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_auto_morphism()
    }
}
include!("./impls/anti_auto_morphism.rs");
