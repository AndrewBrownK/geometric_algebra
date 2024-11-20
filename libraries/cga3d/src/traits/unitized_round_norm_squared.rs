use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRoundNormSquared
/// TODO
pub trait UnitizedRoundNormSquared {
    fn unitized_round_norm_squared(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_round_norm_squared;
impl<A: UnitizedRoundNormSquared> std::ops::Div<A> for unitized_round_norm_squared {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_round_norm_squared()
    }
}
include!("./impls/unitized_round_norm_squared.rs");
