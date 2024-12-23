use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulkNorm
/// BulkNorm for flat aspect.
pub trait FlatBulkNorm {
    fn flat_bulk_norm(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_bulk_norm;
impl<A: FlatBulkNorm> std::ops::Div<A> for flat_bulk_norm {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_bulk_norm()
    }
}
include!("./impls/flat_bulk_norm.rs");
