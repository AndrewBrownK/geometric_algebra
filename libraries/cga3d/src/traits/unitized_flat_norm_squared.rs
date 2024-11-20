use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedFlatNormSquared
/// TODO
pub trait UnitizedFlatNormSquared {
    fn unitized_flat_norm_squared(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_flat_norm_squared;
impl<A: UnitizedFlatNormSquared> std::ops::Div<A> for unitized_flat_norm_squared {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_flat_norm_squared()
    }
}
include!("./impls/unitized_flat_norm_squared.rs");
