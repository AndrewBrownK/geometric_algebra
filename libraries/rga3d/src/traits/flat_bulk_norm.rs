use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulkNorm
/// BulkNorm for flat aspect.
pub trait FlatBulkNorm {
    fn flat_bulk_norm(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_bulk_norm: FlatBulkNormPrefixOrPostfix = FlatBulkNormPrefixOrPostfix;
pub struct FlatBulkNormPrefixOrPostfix;
impl<A: FlatBulkNorm> std::ops::Div<A> for FlatBulkNormPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_bulk_norm()
    }
}
include!("./impls/flat_bulk_norm.rs");
