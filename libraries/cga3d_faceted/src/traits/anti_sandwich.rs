use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiSandwich
/// The anti-sandwich is the dual to the sandwich, using the AntiProduct instead of the Product, and the AntiReverse instead of the Reverse. This is also used to represent geometric transformations, for example reflecting across a plane or rotating around a line. The Sandwich and AntiSandwich are not identical for the purposes of transforming geometry, you simply choose which one to use depending on your geometric interpretation and the algebra. For example, in G(3,0,1) you may interpret grade 1 vectors as points or planes, since they are dual to one another. The sandwich product gives euclidean transformations in the grade 1 = planes interpretation, and the AntiSandwich gives euclidean transformations in the grade 1 = points interpretation.
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
