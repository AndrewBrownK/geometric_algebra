use crate::data::*;
use crate::simd::*;

/// AntiInverse
/// TODO
pub trait AntiInverse {
    fn anti_inverse(self) -> AntiScalar;
}
#[allow(non_camel_case_types)]
pub struct anti_inverse;
impl<A: AntiInverse> std::ops::Div<anti_inverse> for A {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_inverse) -> Self::Output {
        self.anti_inverse()
    }
}
impl<A: AntiInverse> std::ops::Div<A> for anti_inverse {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_inverse()
    }
}
include!("./impls/anti_inverse.rs");
