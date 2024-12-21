use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulk
/// This is the aspect of a round object that characterizes the carrier's relationship with the origin.
pub trait RoundBulk {
    type Output;
    fn round_bulk(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_bulk;
impl<A: RoundBulk> std::ops::Div<A> for round_bulk {
    type Output = <A as RoundBulk>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_bulk()
    }
}
include!("./impls/round_bulk.rs");
