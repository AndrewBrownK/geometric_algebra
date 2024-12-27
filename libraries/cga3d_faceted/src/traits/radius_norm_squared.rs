use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RadiusNormSquared
/// Intermediate result to RadiusNorm.
pub trait RadiusNormSquared {
    fn radius_norm_squared(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static radius_norm_squared: RadiusNormSquaredPrefixOrPostfix = RadiusNormSquaredPrefixOrPostfix;
pub struct RadiusNormSquaredPrefixOrPostfix;
impl<A: RadiusNormSquared> std::ops::Div<A> for RadiusNormSquaredPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.radius_norm_squared()
    }
}
include!("./impls/radius_norm_squared.rs");
