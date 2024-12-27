use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// WeightNorm
/// Weight Norm for flat aspect.
pub trait WeightNorm {
    fn weight_norm(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static weight_norm: WeightNormPrefixOrPostfix = WeightNormPrefixOrPostfix;
pub struct WeightNormPrefixOrPostfix;
impl<A: WeightNorm> std::ops::Div<A> for WeightNormPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.weight_norm()
    }
}
include!("./impls/weight_norm.rs");
