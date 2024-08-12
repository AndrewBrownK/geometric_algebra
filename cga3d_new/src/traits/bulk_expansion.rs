use crate::data::*;
use crate::simd::*;

///
pub trait BulkExpansion<T> {
    fn bulk_expansion(self, other: T) -> Self;
}
pub trait InfixBulkExpansion {}
#[allow(non_camel_case_types)]
pub struct bulk_expansion;
#[allow(non_camel_case_types)]
pub struct bulk_expansion_partial<A>(A);
impl<A: InfixBulkExpansion> std::ops::Div<bulk_expansion> for A {
    type Output = bulk_expansion_partial<A>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl<A: BulkExpansion<B>, B> std::ops::Div<B> for bulk_expansion_partial<A> {
    type Output = A;
    fn div(self, rhs: B) -> Self::Output {
        self.0.bulk_expansion(rhs)
    }
}
include!("./impls/bulk_expansion.rs");
