use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Sandwich
/// The so-called "sandwich product" squeezes some factor A between another factor B and the reversal of B. This is frequently used to represent geometric transformations, for example reflecting across a plane or rotating around a line.
pub trait Sandwich<T> {
    type Output;
    fn sandwich(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static sandwich: SandwichInfix = SandwichInfix;
pub struct SandwichInfix;
pub struct SandwichInfixPartial<A>(A);
impl<A: Sandwich<B>, B> std::ops::Div<B> for SandwichInfixPartial<A> {
    type Output = <A as Sandwich<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.sandwich(rhs)
    }
}
include!("./impls/sandwich.rs");
