use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CenterNormSquared
/// Intermediate result to CenterNorm.
pub trait CenterNormSquared {
    fn center_norm_squared(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static center_norm_squared: CenterNormSquaredPrefixOrPostfix = CenterNormSquaredPrefixOrPostfix;
pub struct CenterNormSquaredPrefixOrPostfix;
impl<A: CenterNormSquared> std::ops::Div<A> for CenterNormSquaredPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.center_norm_squared()
    }
}
include!("./impls/center_norm_squared.rs");
