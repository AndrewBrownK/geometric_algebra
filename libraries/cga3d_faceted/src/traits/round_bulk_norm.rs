use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulkNorm
/// Bulk Norm for round aspect.
pub trait RoundBulkNorm {
    fn round_bulk_norm(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static round_bulk_norm: RoundBulkNormPrefixOrPostfix = RoundBulkNormPrefixOrPostfix;
pub struct RoundBulkNormPrefixOrPostfix;
impl<A: RoundBulkNorm> std::ops::Div<A> for RoundBulkNormPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_bulk_norm()
    }
}
include!("./impls/round_bulk_norm.rs");
