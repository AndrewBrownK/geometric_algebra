use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulkNormSquared
/// TODO
pub trait FlatBulkNormSquared {
    fn flat_bulk_norm_squared(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_bulk_norm_squared;
impl<A: FlatBulkNormSquared> std::ops::Div<A> for flat_bulk_norm_squared {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_bulk_norm_squared()
    }
}
include!("./impls/flat_bulk_norm_squared.rs");
