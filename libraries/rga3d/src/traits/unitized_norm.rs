use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedNorm
/// Unitized FlatNorm.
pub trait UnitizedNorm {
    fn unitized_norm(self) -> f32;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitized_norm: UnitizedNormPrefixOrPostfix = UnitizedNormPrefixOrPostfix;
pub struct UnitizedNormPrefixOrPostfix;
impl<A: UnitizedNorm> std::ops::Div<A> for UnitizedNormPrefixOrPostfix {
    type Output = f32;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_norm()
    }
}
include!("./impls/unitized_norm.rs");
