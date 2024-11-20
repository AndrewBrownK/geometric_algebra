use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulkNormSquared
/// TODO
pub trait RoundBulkNormSquared {
    fn round_bulk_norm_squared(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_bulk_norm_squared;
impl<A: RoundBulkNormSquared> std::ops::Div<A> for round_bulk_norm_squared {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_bulk_norm_squared()
    }
}
include!("./impls/round_bulk_norm_squared.rs");
