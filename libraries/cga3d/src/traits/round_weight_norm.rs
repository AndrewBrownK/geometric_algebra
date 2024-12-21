use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeightNorm
/// Weight Norm for round aspect.
pub trait RoundWeightNorm {
    fn round_weight_norm(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_weight_norm;
impl<A: RoundWeightNorm> std::ops::Div<A> for round_weight_norm {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_weight_norm()
    }
}
include!("./impls/round_weight_norm.rs");
