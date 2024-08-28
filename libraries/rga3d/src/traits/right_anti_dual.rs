use crate::data::*;
use crate::simd::*;

/// RightAntiDual
/// TODO
pub trait RightAntiDual {
    type Output;
    fn right_anti_dual(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct right_anti_dual;
impl<A: RightAntiDual> std::ops::Div<right_anti_dual> for A {
    type Output = <A as RightAntiDual>::Output;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl<A: RightAntiDual> std::ops::Div<A> for right_anti_dual {
    type Output = <A as RightAntiDual>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.right_anti_dual()
    }
}
include!("./impls/right_anti_dual.rs");
