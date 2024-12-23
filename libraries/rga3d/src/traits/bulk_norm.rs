use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// BulkNorm
/// BulkNorm for flat aspect.
pub trait BulkNorm {
    fn bulk_norm(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct bulk_norm;
impl<A: BulkNorm> std::ops::Div<A> for bulk_norm {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.bulk_norm()
    }
}
include!("./impls/bulk_norm.rs");
