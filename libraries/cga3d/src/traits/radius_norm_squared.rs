use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RadiusNormSquared
/// TODO
pub trait RadiusNormSquared {
    fn radius_norm_squared(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct radius_norm_squared;
impl<A: RadiusNormSquared> std::ops::Div<A> for radius_norm_squared {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.radius_norm_squared()
    }
}
include!("./impls/radius_norm_squared.rs");
