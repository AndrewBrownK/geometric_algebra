use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedCenterNormSquared
/// TODO
pub trait UnitizedCenterNormSquared {
    fn unitized_center_norm_squared(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_center_norm_squared;
impl<A: UnitizedCenterNormSquared> std::ops::Div<A> for unitized_center_norm_squared {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_center_norm_squared()
    }
}
include!("./impls/unitized_center_norm_squared.rs");
