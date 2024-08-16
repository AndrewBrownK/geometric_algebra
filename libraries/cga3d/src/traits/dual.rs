use crate::data::*;
use crate::simd::*;

/// Dual
/// TODO
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct dual;
impl<A: Dual> std::ops::Div<dual> for A {
    type Output = <A as Dual>::Output;
    fn div(self, _rhs: dual) -> Self::Output {
        self.dual()
    }
}
impl<A: Dual> std::ops::Div<A> for dual {
    type Output = <A as Dual>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.dual()
    }
}
include!("./impls/dual.rs");
