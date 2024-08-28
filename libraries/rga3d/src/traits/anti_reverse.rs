use crate::data::*;
use crate::simd::*;

/// AntiReverse
/// TODO
pub trait AntiReverse {
    fn anti_reverse(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct anti_reverse;
impl<A: AntiReverse> std::ops::Div<anti_reverse> for A {
    type Output = A;
    fn div(self, _rhs: anti_reverse) -> Self::Output {
        self.anti_reverse()
    }
}
impl<A: AntiReverse> std::ops::Div<A> for anti_reverse {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_reverse()
    }
}
impl<A: AntiReverse> std::ops::DivAssign<A> for anti_reverse {
    fn div_assign(&mut self, rhs: anti_reverse) {
        *self = *self.anti_reverse()
    }
}
include!("./impls/anti_reverse.rs");
