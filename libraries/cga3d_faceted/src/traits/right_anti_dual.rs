use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RightAntiDual
/// TODO
pub trait RightAntiDual {
    type Output;
    fn right_anti_dual(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct right_anti_dual;
impl<A: RightAntiDual> std::ops::Div<A> for right_anti_dual {
    type Output = <A as RightAntiDual>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.right_anti_dual()
    }
}
include!("./impls/right_anti_dual.rs");
