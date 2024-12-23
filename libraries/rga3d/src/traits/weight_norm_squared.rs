use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// WeightNormSquared
/// Intermediate result to FlatWeightNorm.
pub trait WeightNormSquared {
    fn weight_norm_squared(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct weight_norm_squared;
impl<A: WeightNormSquared> std::ops::Div<A> for weight_norm_squared {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.weight_norm_squared()
    }
}
include!("./impls/weight_norm_squared.rs");
