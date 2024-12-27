use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// WeightExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static weight_expansion: WeightExpansionInfix = WeightExpansionInfix;
pub struct WeightExpansionInfix;
pub struct WeightExpansionInfixPartial<A>(A);
impl<A: WeightExpansion<B>, B> std::ops::Div<B> for WeightExpansionInfixPartial<A> {
    type Output = <A as WeightExpansion<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.weight_expansion(rhs)
    }
}
include!("./impls/weight_expansion.rs");
