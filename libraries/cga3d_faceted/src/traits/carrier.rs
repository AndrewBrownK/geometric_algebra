use crate::data::*;
use crate::simd::*;

/// Carrier
/// TODO
pub trait Carrier {
    type Output;
    fn carrier(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct carrier;
impl<A: Carrier> std::ops::Div<carrier> for A {
    type Output = <A as Carrier>::Output;
    fn div(self, _rhs: carrier) -> Self::Output {
        self.carrier()
    }
}
impl<A: Carrier> std::ops::Div<A> for carrier {
    type Output = <A as Carrier>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.carrier()
    }
}
include!("./impls/carrier.rs");
