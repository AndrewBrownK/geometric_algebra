use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRadiusNormSquared
/// Intermediate result to UnitizedRadiusNorm.
pub trait UnitizedRadiusNormSquared {
    fn unitized_radius_norm_squared(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_radius_norm_squared: UnitizedRadiusNormSquaredPrefixOrPostfix = UnitizedRadiusNormSquaredPrefixOrPostfix;
pub struct UnitizedRadiusNormSquaredPrefixOrPostfix;
impl<A: UnitizedRadiusNormSquared> std::ops::Div<A> for UnitizedRadiusNormSquaredPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_radius_norm_squared()
    }
}
include!("./impls/unitized_radius_norm_squared.rs");
