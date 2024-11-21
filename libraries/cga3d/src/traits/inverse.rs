use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Inverse
/// TODO
pub trait Inverse {
    fn inverse(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct inverse;
impl<A: Inverse> std::ops::Div<A> for inverse {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.inverse()
    }
}
include!("./impls/inverse.rs");
