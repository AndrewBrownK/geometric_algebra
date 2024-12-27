use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedFlatNormSquared
/// Intermediate result to UnitizedFlatNorm.
pub trait UnitizedFlatNormSquared {
    fn unitized_flat_norm_squared(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_flat_norm_squared: UnitizedFlatNormSquaredPrefixOrPostfix = UnitizedFlatNormSquaredPrefixOrPostfix;
pub struct UnitizedFlatNormSquaredPrefixOrPostfix;
impl<A: UnitizedFlatNormSquared> std::ops::Div<A> for UnitizedFlatNormSquaredPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_flat_norm_squared()
    }
}
include!("./impls/unitized_flat_norm_squared.rs");
