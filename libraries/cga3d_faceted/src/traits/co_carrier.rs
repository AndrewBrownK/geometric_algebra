use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CoCarrier
/// TODO
pub trait CoCarrier {
    type Output;
    fn co_carrier(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct co_carrier;
impl<A: CoCarrier> std::ops::Div<A> for co_carrier {
    type Output = <A as CoCarrier>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.co_carrier()
    }
}
include!("./impls/co_carrier.rs");
