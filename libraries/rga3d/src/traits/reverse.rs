use crate::data::*;
use crate::simd::*;

/// Reverse
/// TODO
pub trait Reverse {
    fn reverse(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct reverse;
impl<A: Reverse> std::ops::Div<reverse> for A {
    type Output = A;
    fn div(self, _rhs: reverse) -> Self::Output {
        self.reverse()
    }
}
impl<A: Reverse> std::ops::Div<A> for reverse {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.reverse()
    }
}
impl<A: Reverse> std::ops::DivAssign<A> for reverse {
    fn div_assign(&mut self, rhs: reverse) {
        *self = *self.reverse()
    }
}
include!("./impls/reverse.rs");
