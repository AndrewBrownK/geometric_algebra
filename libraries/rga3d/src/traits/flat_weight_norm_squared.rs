use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeightNormSquared
/// Intermediate result to FlatWeightNorm.
pub trait FlatWeightNormSquared {
    fn flat_weight_norm_squared(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_weight_norm_squared;
impl<A: FlatWeightNormSquared> std::ops::Div<A> for flat_weight_norm_squared {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_weight_norm_squared()
    }
}
include!("./impls/flat_weight_norm_squared.rs");
