use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// BulkExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct bulk_expansion;
#[allow(non_camel_case_types)]
pub struct bulk_expansion_partial<A>(A);
impl<A: BulkExpansion<B>, B> std::ops::Div<B> for bulk_expansion_partial<A> {
    type Output = <A as BulkExpansion<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.bulk_expansion(rhs)
    }
}
include!("./impls/bulk_expansion.rs");
