use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRoundNorm
/// Unitized Norm for round aspect.
pub trait UnitizedRoundNorm {
    fn unitized_round_norm(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_round_norm: UnitizedRoundNormPrefixOrPostfix = UnitizedRoundNormPrefixOrPostfix;
pub struct UnitizedRoundNormPrefixOrPostfix;
impl<A: UnitizedRoundNorm> std::ops::Div<A> for UnitizedRoundNormPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_round_norm()
    }
}
include!("./impls/unitized_round_norm.rs");
