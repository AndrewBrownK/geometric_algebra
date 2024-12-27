use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundNorm
/// Norm for Round aspect.
pub trait RoundNorm {
    fn round_norm(self) -> MultiVector;
}
#[allow(non_upper_case_globals, dead_code)]
pub static round_norm: RoundNormPrefixOrPostfix = RoundNormPrefixOrPostfix;
pub struct RoundNormPrefixOrPostfix;
impl<A: RoundNorm> std::ops::Div<A> for RoundNormPrefixOrPostfix {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_norm()
    }
}
include!("./impls/round_norm.rs");
