use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulkNormSquared
/// Intermediate result for FlatBulkNorm.
pub trait FlatBulkNormSquared {
    fn flat_bulk_norm_squared(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_bulk_norm_squared: FlatBulkNormSquaredPrefixOrPostfix = FlatBulkNormSquaredPrefixOrPostfix;
pub struct FlatBulkNormSquaredPrefixOrPostfix;
impl<A: FlatBulkNormSquared> std::ops::Div<A> for FlatBulkNormSquaredPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_bulk_norm_squared()
    }
}
include!("./impls/flat_bulk_norm_squared.rs");
