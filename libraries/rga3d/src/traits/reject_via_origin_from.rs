use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
pub trait RejectViaOriginFrom<T> {
    type Output;
    fn reject_via_origin_from(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static reject_via_origin_from: RejectViaOriginFromInfix = RejectViaOriginFromInfix;
pub struct RejectViaOriginFromInfix;
pub struct RejectViaOriginFromInfixPartial<A>(A);
impl<A: RejectViaOriginFrom<B>, B> std::ops::Div<B> for RejectViaOriginFromInfixPartial<A> {
    type Output = <A as RejectViaOriginFrom<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.reject_via_origin_from(rhs)
    }
}
include!("./impls/reject_via_origin_from.rs");
