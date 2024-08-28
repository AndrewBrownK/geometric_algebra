use crate::data::*;
use crate::simd::*;

/// AntiFixImpl
/// TODO
pub trait AntiFixImpl {
    fn anti_fix_impl(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct anti_fix_impl;
impl<A: AntiFixImpl> std::ops::Div<anti_fix_impl> for A {
    type Output = A;
    fn div(self, _rhs: anti_fix_impl) -> Self::Output {
        self.anti_fix_impl()
    }
}
impl<A: AntiFixImpl> std::ops::Div<A> for anti_fix_impl {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_fix_impl()
    }
}
impl<A: AntiFixImpl> std::ops::DivAssign<A> for anti_fix_impl {
    fn div_assign(&mut self, rhs: anti_fix_impl) {
        *self = *self.anti_fix_impl()
    }
}
include!("./impls/anti_fix_impl.rs");
