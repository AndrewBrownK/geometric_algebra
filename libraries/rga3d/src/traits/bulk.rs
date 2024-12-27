use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Bulk
/// This characterizes the flat aspect's relationship with the origin.
pub trait Bulk {
    type Output;
    fn bulk(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static bulk: BulkPrefixOrPostfix = BulkPrefixOrPostfix;
pub struct BulkPrefixOrPostfix;
impl<A: Bulk> std::ops::Div<A> for BulkPrefixOrPostfix {
    type Output = <A as Bulk>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.bulk()
    }
}
include!("./impls/bulk.rs");
