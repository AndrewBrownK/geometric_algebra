use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeightNormSquared
/// TODO
pub trait RoundWeightNormSquared {
    fn round_weight_norm_squared(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_weight_norm_squared;
impl<A: RoundWeightNormSquared> std::ops::Div<A> for round_weight_norm_squared {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_weight_norm_squared()
    }
}
include!("./impls/round_weight_norm_squared.rs");
