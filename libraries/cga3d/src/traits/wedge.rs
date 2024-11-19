use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Wedge
/// TODO
pub trait Wedge<T> {
    type Output;
    fn wedge(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct wedge;
#[allow(non_camel_case_types)]
pub struct wedge_partial<A>(A);
impl<A: Wedge<B>, B> std::ops::Div<B> for wedge_partial<A> {
    type Output = <A as Wedge<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.wedge(rhs)
    }
}
include!("./impls/wedge.rs");
