use crate::data::*;
use crate::simd::*;

/// AntiSandwich
/// TODO
pub trait AntiSandwich<T> {
    type Output;
    fn anti_sandwich(self, other: T) -> Self::Output;
}
pub trait InfixAntiSandwich {}
#[allow(non_camel_case_types)]
pub struct anti_sandwich;
#[allow(non_camel_case_types)]
pub struct anti_sandwich_partial<A>(A);
impl<A: InfixAntiSandwich> std::ops::Div<anti_sandwich> for A {
    type Output = anti_sandwich_partial<A>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl<A: AntiSandwich<B>, B> std::ops::Div<B> for anti_sandwich_partial<A> {
    type Output = <A as AntiSandwich<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_sandwich(rhs)
    }
}
include!("./impls/anti_sandwich.rs");
