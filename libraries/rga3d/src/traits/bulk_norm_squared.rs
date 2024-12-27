use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// BulkNormSquared
/// Intermediate result for FlatBulkNorm.
pub trait BulkNormSquared {
    fn bulk_norm_squared(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static bulk_norm_squared: BulkNormSquaredPrefixOrPostfix = BulkNormSquaredPrefixOrPostfix;
pub struct BulkNormSquaredPrefixOrPostfix;
impl<A: BulkNormSquared> std::ops::Div<A> for BulkNormSquaredPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.bulk_norm_squared()
    }
}
include!("./impls/bulk_norm_squared.rs");
