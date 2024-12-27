use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundNormSquared
/// Intermediate result for RoundNorm.
pub trait RoundNormSquared {
    fn round_norm_squared(self) -> MultiVector;
}
#[allow(non_upper_case_globals, dead_code)]
pub static round_norm_squared: RoundNormSquaredPrefixOrPostfix = RoundNormSquaredPrefixOrPostfix;
pub struct RoundNormSquaredPrefixOrPostfix;
impl<A: RoundNormSquared> std::ops::Div<A> for RoundNormSquaredPrefixOrPostfix {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_norm_squared()
    }
}
include!("./impls/round_norm_squared.rs");
