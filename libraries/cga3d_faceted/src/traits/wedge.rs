use crate::data::*;
use crate::simd::*;

/// Wedge
/// TODO
pub trait Wedge<T> {
    type Output;
    fn wedge(self, other: T) -> Self::Output;
}
pub trait InfixWedge {}
#[allow(non_camel_case_types)]
pub struct wedge;
#[allow(non_camel_case_types)]
pub struct wedge_partial<A>(A);
impl<A: InfixWedge> std::ops::Div<wedge> for A {
    type Output = wedge_partial<A>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl<A: Wedge<B>, B> std::ops::Div<B> for wedge_partial<A> {
    type Output = <A as Wedge<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.wedge(rhs)
    }
}
include!("./impls/wedge.rs");
