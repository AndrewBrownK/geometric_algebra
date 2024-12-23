use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedNorm
/// Unitized FlatNorm.
pub trait UnitizedNorm {
    fn unitized_norm(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_norm;
impl<A: UnitizedNorm> std::ops::Div<A> for unitized_norm {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_norm()
    }
}
include!("./impls/unitized_norm.rs");
