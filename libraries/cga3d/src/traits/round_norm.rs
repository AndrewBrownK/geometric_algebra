use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundNorm
/// Norm for Round aspect.
pub trait RoundNorm {
    fn round_norm(self) -> MultiVector;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_norm;
impl<A: RoundNorm> std::ops::Div<A> for round_norm {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_norm()
    }
}
include!("./impls/round_norm.rs");
