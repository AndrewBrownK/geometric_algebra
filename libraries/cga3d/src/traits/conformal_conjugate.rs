use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ConformalConjugate
/// The conformal conjugate negates the flat elements of an object, and is useful in calculating the center norm of the object.
pub trait ConformalConjugate {
    fn conformal_conjugate(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static conformal_conjugate: ConformalConjugatePrefixOrPostfix = ConformalConjugatePrefixOrPostfix;
pub struct ConformalConjugatePrefixOrPostfix;
impl<A: ConformalConjugate> std::ops::Div<A> for ConformalConjugatePrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.conformal_conjugate()
    }
}
include!("./impls/conformal_conjugate.rs");
