use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedFlatNorm
/// Unitized FlatNorm.
pub trait UnitizedFlatNorm {
    fn unitized_flat_norm(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_flat_norm: UnitizedFlatNormPrefixOrPostfix = UnitizedFlatNormPrefixOrPostfix;
pub struct UnitizedFlatNormPrefixOrPostfix;
impl<A: UnitizedFlatNorm> std::ops::Div<A> for UnitizedFlatNormPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_flat_norm()
    }
}
include!("./impls/unitized_flat_norm.rs");
