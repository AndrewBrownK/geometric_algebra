use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRadiusNorm
/// Unitized radius of an object.
pub trait UnitizedRadiusNorm {
    fn unitized_radius_norm(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_radius_norm: UnitizedRadiusNormPrefixOrPostfix = UnitizedRadiusNormPrefixOrPostfix;
pub struct UnitizedRadiusNormPrefixOrPostfix;
impl<A: UnitizedRadiusNorm> std::ops::Div<A> for UnitizedRadiusNormPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_radius_norm()
    }
}
include!("./impls/unitized_radius_norm.rs");
