use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiWedge
/// TODO
pub trait AntiWedge<T> {
    type Output;
    fn anti_wedge(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_wedge;
#[allow(non_camel_case_types)]
pub struct anti_wedge_partial<A>(A);
impl<A: AntiWedge<B>, B> std::ops::Div<B> for anti_wedge_partial<A> {
    type Output = <A as AntiWedge<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_wedge(rhs)
    }
}
include!("./impls/anti_wedge.rs");
