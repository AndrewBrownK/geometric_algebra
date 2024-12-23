use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// NormSquared
/// Intermediate result to FlatNorm.
pub trait NormSquared {
    fn norm_squared(self) -> DualNum;
}
#[allow(non_camel_case_types, dead_code)]
pub struct norm_squared;
impl<A: NormSquared> std::ops::Div<A> for norm_squared {
    type Output = DualNum;
    fn div(self, rhs: A) -> Self::Output {
        rhs.norm_squared()
    }
}
include!("./impls/norm_squared.rs");
