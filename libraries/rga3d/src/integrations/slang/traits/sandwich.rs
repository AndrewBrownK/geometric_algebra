using data::*;

/// Sandwich
/// The so-called "sandwich product" squeezes some factor A between another factor B and the reversal of B. This is frequently used to represent geometric transformations, for example reflecting across a plane or rotating around a line.
pub trait Sandwich<T> {
    type Output;
    fn sandwich(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct sandwich;
#[allow(non_camel_case_types)]
pub struct sandwich_partial<A>(A);
impl<A: Sandwich<B>, B> std::ops::Div<B> for sandwich_partial<A> {
    type Output = <A as Sandwich<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.sandwich(rhs)
    }
}
__include ./impls/sandwich;
