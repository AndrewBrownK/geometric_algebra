use crate::data::*;
use crate::simd::*;

/// CoCarrier
/// TODO
pub trait CoCarrier {
    type Output;
    fn co_carrier(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct co_carrier;
impl<A: CoCarrier> std::ops::Div<co_carrier> for A {
    type Output = <A as CoCarrier>::Output;
    fn div(self, _rhs: co_carrier) -> Self::Output {
        self.co_carrier()
    }
}
impl<A: CoCarrier> std::ops::Div<A> for co_carrier {
    type Output = <A as CoCarrier>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.co_carrier()
    }
}
include!("./impls/co_carrier.rs");
