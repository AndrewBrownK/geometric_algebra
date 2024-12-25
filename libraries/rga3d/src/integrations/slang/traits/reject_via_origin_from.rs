using data::*;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
pub trait RejectViaOriginFrom<T> {
    type Output;
    fn reject_via_origin_from(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct reject_via_origin_from;
#[allow(non_camel_case_types)]
pub struct reject_via_origin_from_partial<A>(A);
impl<A: RejectViaOriginFrom<B>, B> std::ops::Div<B> for reject_via_origin_from_partial<A> {
    type Output = <A as RejectViaOriginFrom<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.reject_via_origin_from(rhs)
    }
}
__include ./impls/reject_via_origin_from;
