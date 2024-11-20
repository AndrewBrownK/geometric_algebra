use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedCenterNorm
/// TODO
pub trait UnitizedCenterNorm {
    fn unitized_center_norm(self) -> f32;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_center_norm;
impl<A: UnitizedCenterNorm> std::ops::Div<A> for unitized_center_norm {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_center_norm()
    }
}
include!("./impls/unitized_center_norm.rs");
