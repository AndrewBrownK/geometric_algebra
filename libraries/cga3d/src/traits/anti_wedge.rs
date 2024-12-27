use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiWedge
/// The AntiWedge product is the dual operation to the Wedge product, that depends on a specified AntiScalar. It combines BasisElements by which parts are missing, instead of which parts are present. For example, with an AntiScalar of e1234, anti_wedge(e423, e321) = e23. This behaves something like an intersection of the subscripts in the BasisElements.
pub trait AntiWedge<T> {
    type Output;
    fn anti_wedge(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_wedge: AntiWedgeInfix = AntiWedgeInfix;
pub struct AntiWedgeInfix;
pub struct AntiWedgeInfixPartial<A>(A);
impl<A: AntiWedge<B>, B> std::ops::Div<B> for AntiWedgeInfixPartial<A> {
    type Output = <A as AntiWedge<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_wedge(rhs)
    }
}
include!("./impls/anti_wedge.rs");
