use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedCenterNormSquared
/// Intermediate result to UnitizedCenterNorm.
pub trait UnitizedCenterNormSquared {
    fn unitized_center_norm_squared(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_center_norm_squared: UnitizedCenterNormSquaredPrefixOrPostfix = UnitizedCenterNormSquaredPrefixOrPostfix;
pub struct UnitizedCenterNormSquaredPrefixOrPostfix;
impl<A: UnitizedCenterNormSquared> std::ops::Div<A> for UnitizedCenterNormSquaredPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_center_norm_squared()
    }
}
include!("./impls/unitized_center_norm_squared.rs");
