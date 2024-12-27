use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedCenterNorm
/// Unitized distance from origin to center of round object.
pub trait UnitizedCenterNorm {
    fn unitized_center_norm(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_center_norm: UnitizedCenterNormPrefixOrPostfix = UnitizedCenterNormPrefixOrPostfix;
pub struct UnitizedCenterNormPrefixOrPostfix;
impl<A: UnitizedCenterNorm> std::ops::Div<A> for UnitizedCenterNormPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_center_norm()
    }
}
include!("./impls/unitized_center_norm.rs");
