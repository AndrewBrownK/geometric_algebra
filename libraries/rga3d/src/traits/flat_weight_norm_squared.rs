use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeightNormSquared
/// Intermediate result to FlatWeightNorm.
pub trait FlatWeightNormSquared {
    fn flat_weight_norm_squared(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_weight_norm_squared: FlatWeightNormSquaredPrefixOrPostfix = FlatWeightNormSquaredPrefixOrPostfix;
pub struct FlatWeightNormSquaredPrefixOrPostfix;
impl<A: FlatWeightNormSquared> std::ops::Div<A> for FlatWeightNormSquaredPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_weight_norm_squared()
    }
}
include!("./impls/flat_weight_norm_squared.rs");
