use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatNormSquared
/// TODO
pub trait FlatNormSquared {
    fn flat_norm_squared(self) -> MultiVector;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_norm_squared;
impl<A: FlatNormSquared> std::ops::Div<A> for flat_norm_squared {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_norm_squared()
    }
}
include!("./impls/flat_norm_squared.rs");
