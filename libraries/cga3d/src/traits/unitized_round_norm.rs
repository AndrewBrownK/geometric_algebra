use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRoundNorm
/// TODO
pub trait UnitizedRoundNorm {
    fn unitized_round_norm(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_round_norm;
impl<A: UnitizedRoundNorm> std::ops::Div<A> for unitized_round_norm {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_round_norm()
    }
}
include!("./impls/unitized_round_norm.rs");
