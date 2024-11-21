use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Conjugation
/// TODO
pub trait Conjugation {
    fn conjugation(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct conjugation;
impl<A: Conjugation> std::ops::Div<A> for conjugation {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.conjugation()
    }
}
include!("./impls/conjugation.rs");
