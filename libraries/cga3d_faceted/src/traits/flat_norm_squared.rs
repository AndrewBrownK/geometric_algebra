use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatNormSquared
/// Intermediate result to FlatNorm.
pub trait FlatNormSquared {
    fn flat_norm_squared(self) -> MultiVector;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_norm_squared: FlatNormSquaredPrefixOrPostfix = FlatNormSquaredPrefixOrPostfix;
pub struct FlatNormSquaredPrefixOrPostfix;
impl<A: FlatNormSquared> std::ops::Div<A> for FlatNormSquaredPrefixOrPostfix {
    type Output = MultiVector;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_norm_squared()
    }
}
include!("./impls/flat_norm_squared.rs");
