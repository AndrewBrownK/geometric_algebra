use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRoundNormSquared
/// Intermediate result to UnitizedRoundNorm.
pub trait UnitizedRoundNormSquared {
    fn unitized_round_norm_squared(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_round_norm_squared: UnitizedRoundNormSquaredPrefixOrPostfix = UnitizedRoundNormSquaredPrefixOrPostfix;
pub struct UnitizedRoundNormSquaredPrefixOrPostfix;
impl<A: UnitizedRoundNormSquared> std::ops::Div<A> for UnitizedRoundNormSquaredPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_round_norm_squared()
    }
}
include!("./impls/unitized_round_norm_squared.rs");
