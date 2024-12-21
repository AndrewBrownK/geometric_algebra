use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// BulkContraction
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
pub trait BulkContraction<T> {
    type Output;
    fn bulk_contraction(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct bulk_contraction;
#[allow(non_camel_case_types)]
pub struct bulk_contraction_partial<A>(A);
impl<A: BulkContraction<B>, B> std::ops::Div<B> for bulk_contraction_partial<A> {
    type Output = <A as BulkContraction<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.bulk_contraction(rhs)
    }
}
include!("./impls/bulk_contraction.rs");
