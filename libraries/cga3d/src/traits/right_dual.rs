use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RightDual
/// TODO
pub trait RightDual {
    type Output;
    fn right_dual(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct right_dual;
impl<A: RightDual> std::ops::Div<A> for right_dual {
    type Output = <A as RightDual>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.right_dual()
    }
}
include!("./impls/right_dual.rs");
