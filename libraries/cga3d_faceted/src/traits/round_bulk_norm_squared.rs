use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulkNormSquared
/// Intermediate result to RoundBulkNorm.
pub trait RoundBulkNormSquared {
    fn round_bulk_norm_squared(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static round_bulk_norm_squared: RoundBulkNormSquaredPrefixOrPostfix = RoundBulkNormSquaredPrefixOrPostfix;
pub struct RoundBulkNormSquaredPrefixOrPostfix;
impl<A: RoundBulkNormSquared> std::ops::Div<A> for RoundBulkNormSquaredPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_bulk_norm_squared()
    }
}
include!("./impls/round_bulk_norm_squared.rs");
