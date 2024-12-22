use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiRejectViaHorizonFrom
/// Counterpart to AntiProjectViaHorizonOnto.
pub trait AntiRejectViaHorizonFrom<T> {
    type Output;
    fn anti_reject_via_horizon_from(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_reject_via_horizon_from;
#[allow(non_camel_case_types)]
pub struct anti_reject_via_horizon_from_partial<A>(A);
impl<A: AntiRejectViaHorizonFrom<B>, B> std::ops::Div<B> for anti_reject_via_horizon_from_partial<A> {
    type Output = <A as AntiRejectViaHorizonFrom<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_reject_via_horizon_from(rhs)
    }
}
include!("./impls/anti_reject_via_horizon_from.rs");
