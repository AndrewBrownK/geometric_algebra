use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeightNorm
/// Weight Norm for round aspect.
pub trait RoundWeightNorm {
    fn round_weight_norm(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static round_weight_norm: RoundWeightNormPrefixOrPostfix = RoundWeightNormPrefixOrPostfix;
pub struct RoundWeightNormPrefixOrPostfix;
impl<A: RoundWeightNorm> std::ops::Div<A> for RoundWeightNormPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_weight_norm()
    }
}
include!("./impls/round_weight_norm.rs");
