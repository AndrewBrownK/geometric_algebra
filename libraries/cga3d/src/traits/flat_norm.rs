use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatNorm
/// Norm for flat aspect.
pub trait FlatNorm {
    fn flat_norm(self) -> MultiVector;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_norm;
impl<A: FlatNorm> std::ops::Div<A> for flat_norm {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_norm()
    }
}
include!("./impls/flat_norm.rs");
