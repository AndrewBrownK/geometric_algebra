use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeightNormSquared
/// Intermediate result for RoundWeight.
pub trait RoundWeightNormSquared {
    fn round_weight_norm_squared(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static round_weight_norm_squared: RoundWeightNormSquaredPrefixOrPostfix = RoundWeightNormSquaredPrefixOrPostfix;
pub struct RoundWeightNormSquaredPrefixOrPostfix;
impl<A: RoundWeightNormSquared> std::ops::Div<A> for RoundWeightNormSquaredPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_weight_norm_squared()
    }
}
include!("./impls/round_weight_norm_squared.rs");
