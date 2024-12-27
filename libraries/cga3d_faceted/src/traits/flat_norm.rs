use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatNorm
/// Norm for flat aspect.
pub trait FlatNorm {
    fn flat_norm(self) -> MultiVector;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_norm: FlatNormPrefixOrPostfix = FlatNormPrefixOrPostfix;
pub struct FlatNormPrefixOrPostfix;
impl<A: FlatNorm> std::ops::Div<A> for FlatNormPrefixOrPostfix {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_norm()
    }
}
include!("./impls/flat_norm.rs");
