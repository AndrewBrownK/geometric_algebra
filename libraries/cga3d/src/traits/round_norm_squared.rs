use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundNormSquared
/// Intermediate result for RoundNorm.
pub trait RoundNormSquared {
    fn round_norm_squared(self) -> MultiVector;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_norm_squared;
impl<A: RoundNormSquared> std::ops::Div<A> for round_norm_squared {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_norm_squared()
    }
}
include!("./impls/round_norm_squared.rs");
