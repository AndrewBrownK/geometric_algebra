use crate::data::*;
use crate::simd::*;

/// Inverse
/// TODO
pub trait Inverse {
    fn inverse(self) -> Scalar;
}
#[allow(non_camel_case_types)]
pub struct inverse;
impl<A: Inverse> std::ops::Div<inverse> for A {
    type Output = Scalar;
    fn div(self, _rhs: inverse) -> Self::Output {
        self.inverse()
    }
}
impl<A: Inverse> std::ops::Div<A> for inverse {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.inverse()
    }
}
include!("./impls/inverse.rs");
