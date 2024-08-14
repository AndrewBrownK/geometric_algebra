use crate::data::*;
use crate::simd::*;

/// Sandwich
/// TODO
pub trait Sandwich<T> {
    type Output;
    fn sandwich(self, other: T) -> Self::Output;
}
pub trait InfixSandwich {}
#[allow(non_camel_case_types)]
pub struct sandwich;
#[allow(non_camel_case_types)]
pub struct sandwich_partial<A>(A);
impl<A: InfixSandwich> std::ops::Div<sandwich> for A {
    type Output = sandwich_partial<A>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl<A: Sandwich<B>, B> std::ops::Div<B> for sandwich_partial<A> {
    type Output = <A as Sandwich<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.sandwich(rhs)
    }
}
include!("./impls/sandwich.rs");
