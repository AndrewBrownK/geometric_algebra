use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// WeightNormSquared
/// Intermediate result to FlatWeightNorm.
pub trait WeightNormSquared {
    fn weight_norm_squared(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static weight_norm_squared: WeightNormSquaredPrefixOrPostfix = WeightNormSquaredPrefixOrPostfix;
pub struct WeightNormSquaredPrefixOrPostfix;
impl<A: WeightNormSquared> std::ops::Div<A> for WeightNormSquaredPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.weight_norm_squared()
    }
}
include!("./impls/weight_norm_squared.rs");
