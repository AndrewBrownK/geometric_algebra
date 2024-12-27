use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Conjugation
/// This composes the reverse and grade involution (automorphism).
pub trait Conjugation {
    fn conjugation(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static conjugation: ConjugationPrefixOrPostfix = ConjugationPrefixOrPostfix;
pub struct ConjugationPrefixOrPostfix;
impl<A: Conjugation> std::ops::Div<A> for ConjugationPrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.conjugation()
    }
}
include!("./impls/conjugation.rs");
