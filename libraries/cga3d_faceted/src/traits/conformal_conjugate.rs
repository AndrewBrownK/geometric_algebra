use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ConformalConjugate
/// TODO
pub trait ConformalConjugate {
    fn conformal_conjugate(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct conformal_conjugate;
impl<A: ConformalConjugate> std::ops::Div<A> for conformal_conjugate {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.conformal_conjugate()
    }
}
include!("./impls/conformal_conjugate.rs");
