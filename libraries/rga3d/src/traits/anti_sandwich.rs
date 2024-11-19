use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiSandwich
/// TODO
pub trait AntiSandwich<T> {
    type Output;
    fn anti_sandwich(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_sandwich;
#[allow(non_camel_case_types)]
pub struct anti_sandwich_partial<A>(A);
impl<A: AntiSandwich<B>, B> std::ops::Div<B> for anti_sandwich_partial<A> {
    type Output = <A as AntiSandwich<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_sandwich(rhs)
    }
}
include!("./impls/anti_sandwich.rs");
