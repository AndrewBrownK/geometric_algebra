use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// BulkExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static bulk_expansion: BulkExpansionInfix = BulkExpansionInfix;
pub struct BulkExpansionInfix;
pub struct BulkExpansionInfixPartial<A>(A);
impl<A: BulkExpansion<B>, B> std::ops::Div<B> for BulkExpansionInfixPartial<A> {
    type Output = <A as BulkExpansion<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.bulk_expansion(rhs)
    }
}
include!("./impls/bulk_expansion.rs");
