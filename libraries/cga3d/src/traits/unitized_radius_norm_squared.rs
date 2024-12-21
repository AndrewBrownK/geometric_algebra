use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRadiusNormSquared
/// Intermediate result to UnitizedRadiusNorm.
pub trait UnitizedRadiusNormSquared {
    fn unitized_radius_norm_squared(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_radius_norm_squared;
impl<A: UnitizedRadiusNormSquared> std::ops::Div<A> for unitized_radius_norm_squared {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_radius_norm_squared()
    }
}
include!("./impls/unitized_radius_norm_squared.rs");
