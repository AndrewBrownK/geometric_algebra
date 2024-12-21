use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRadiusNorm
/// Unitized radius of an object.
pub trait UnitizedRadiusNorm {
    fn unitized_radius_norm(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_radius_norm;
impl<A: UnitizedRadiusNorm> std::ops::Div<A> for unitized_radius_norm {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_radius_norm()
    }
}
include!("./impls/unitized_radius_norm.rs");
