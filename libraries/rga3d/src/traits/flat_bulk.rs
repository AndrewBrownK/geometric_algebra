use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulk
/// This characterizes the flat aspect's relationship with the origin.
pub trait FlatBulk {
    type Output;
    fn flat_bulk(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_bulk: FlatBulkPrefixOrPostfix = FlatBulkPrefixOrPostfix;
pub struct FlatBulkPrefixOrPostfix;
impl<A: FlatBulk> std::ops::Div<A> for FlatBulkPrefixOrPostfix {
    type Output = <A as FlatBulk>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_bulk()
    }
}
include!("./impls/flat_bulk.rs");
