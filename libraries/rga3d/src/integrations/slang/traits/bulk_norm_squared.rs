using data::*;

/// BulkNormSquared
/// Intermediate result for FlatBulkNorm.
pub trait BulkNormSquared {
    fn bulk_norm_squared(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct bulk_norm_squared;
impl<A: BulkNormSquared> std::ops::Div<A> for bulk_norm_squared {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.bulk_norm_squared()
    }
}
__include ./impls/bulk_norm_squared;
