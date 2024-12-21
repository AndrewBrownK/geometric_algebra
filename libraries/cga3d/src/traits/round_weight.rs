use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeight
/// This is the aspect of a round object that characterizes the carrier's relationship with the horizon.
pub trait RoundWeight {
    type Output;
    fn round_weight(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct round_weight;
impl<A: RoundWeight> std::ops::Div<A> for round_weight {
    type Output = <A as RoundWeight>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.round_weight()
    }
}
include!("./impls/round_weight.rs");
