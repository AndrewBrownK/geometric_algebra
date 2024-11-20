use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulkNorm
/// TODO
pub trait RoundBulkNorm {
    fn round_bulk_norm(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_bulk_norm;
impl<A: RoundBulkNorm> std::ops::Div<A> for round_bulk_norm {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_bulk_norm()
    }
}
include!("./impls/round_bulk_norm.rs");
