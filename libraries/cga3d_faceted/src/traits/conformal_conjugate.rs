use crate::data::*;
use crate::simd::*;

/// ConformalConjugate
/// TODO
pub trait ConformalConjugate {
    fn conformal_conjugate(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct conformal_conjugate;
impl<A: ConformalConjugate> std::ops::Div<conformal_conjugate> for A {
    type Output = A;
    fn div(self, _rhs: conformal_conjugate) -> Self::Output {
        self.conformal_conjugate()
    }
}
impl<A: ConformalConjugate> std::ops::Div<A> for conformal_conjugate {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.conformal_conjugate()
    }
}
impl<A: ConformalConjugate> std::ops::DivAssign<A> for conformal_conjugate {
    fn div_assign(&mut self, rhs: conformal_conjugate) {
        *self = *self.conformal_conjugate()
    }
}
include!("./impls/conformal_conjugate.rs");
