use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CenterNormSquared
/// Intermediate result to CenterNorm.
pub trait CenterNormSquared {
    fn center_norm_squared(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct center_norm_squared;
impl<A: CenterNormSquared> std::ops::Div<A> for center_norm_squared {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.center_norm_squared()
    }
}
include!("./impls/center_norm_squared.rs");
