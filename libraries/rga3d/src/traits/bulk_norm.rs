use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// BulkNorm
/// BulkNorm for flat aspect.
pub trait BulkNorm {
    fn bulk_norm(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static bulk_norm: BulkNormPrefixOrPostfix = BulkNormPrefixOrPostfix;
pub struct BulkNormPrefixOrPostfix;
impl<A: BulkNorm> std::ops::Div<A> for BulkNormPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.bulk_norm()
    }
}
include!("./impls/bulk_norm.rs");
