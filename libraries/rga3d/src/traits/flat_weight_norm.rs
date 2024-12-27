use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
pub trait FlatWeightNorm {
    fn flat_weight_norm(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_weight_norm: FlatWeightNormPrefixOrPostfix = FlatWeightNormPrefixOrPostfix;
pub struct FlatWeightNormPrefixOrPostfix;
impl<A: FlatWeightNorm> std::ops::Div<A> for FlatWeightNormPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_weight_norm()
    }
}
include!("./impls/flat_weight_norm.rs");
