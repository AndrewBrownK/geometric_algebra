use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Weight
/// This characterizes the flat aspect's relationship with the horizon.
pub trait Weight {
    type Output;
    fn weight(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static weight: WeightPrefixOrPostfix = WeightPrefixOrPostfix;
pub struct WeightPrefixOrPostfix;
impl<A: Weight> std::ops::Div<A> for WeightPrefixOrPostfix {
    type Output = <A as Weight>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.weight()
    }
}
include!("./impls/weight.rs");
