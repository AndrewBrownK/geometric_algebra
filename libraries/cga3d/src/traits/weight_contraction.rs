use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// WeightContraction
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
pub trait WeightContraction<T> {
    type Output;
    fn weight_contraction(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct weight_contraction;
#[allow(non_camel_case_types)]
pub struct weight_contraction_partial<A>(A);
impl<A: WeightContraction<B>, B> std::ops::Div<B> for weight_contraction_partial<A> {
    type Output = <A as WeightContraction<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.weight_contraction(rhs)
    }
}
include!("./impls/weight_contraction.rs");
