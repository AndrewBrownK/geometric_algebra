use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeight
/// This characterizes the flat aspect's relationship with the horizon.
pub trait FlatWeight {
    type Output;
    fn flat_weight(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static flat_weight: FlatWeightPrefixOrPostfix = FlatWeightPrefixOrPostfix;
pub struct FlatWeightPrefixOrPostfix;
impl<A: FlatWeight> std::ops::Div<A> for FlatWeightPrefixOrPostfix {
    type Output = <A as FlatWeight>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_weight()
    }
}
include!("./impls/flat_weight.rs");
