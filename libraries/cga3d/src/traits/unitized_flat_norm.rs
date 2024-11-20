use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedFlatNorm
/// TODO
pub trait UnitizedFlatNorm {
    fn unitized_flat_norm(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_flat_norm;
impl<A: UnitizedFlatNorm> std::ops::Div<A> for unitized_flat_norm {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_flat_norm()
    }
}
include!("./impls/unitized_flat_norm.rs");
