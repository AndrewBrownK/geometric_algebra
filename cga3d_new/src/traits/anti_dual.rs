use crate::data::*;
use crate::simd::*;

/// AntiDual
/// TODO
pub trait AntiDual {
    type Output;
    fn anti_dual(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct anti_dual;
impl<A: AntiDual> std::ops::Div<anti_dual> for A {
    type Output = <A as AntiDual>::Output;
    fn div(self, _rhs: anti_dual) -> Self::Output {
        self.anti_dual()
    }
}
impl<A: AntiDual> std::ops::Div<A> for anti_dual {
    type Output = <A as AntiDual>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_dual()
    }
}
include!("./impls/anti_dual.rs");
