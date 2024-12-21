use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulk
/// This characterizes the flat aspect's relationship with the origin.
pub trait FlatBulk {
    type Output;
    fn flat_bulk(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_bulk;
impl<A: FlatBulk> std::ops::Div<A> for flat_bulk {
    type Output = <A as FlatBulk>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_bulk()
    }
}
include!("./impls/flat_bulk.rs");
