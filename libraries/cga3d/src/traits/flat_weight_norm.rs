use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
pub trait FlatWeightNorm {
    fn flat_weight_norm(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_weight_norm;
impl<A: FlatWeightNorm> std::ops::Div<A> for flat_weight_norm {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_weight_norm()
    }
}
include!("./impls/flat_weight_norm.rs");
