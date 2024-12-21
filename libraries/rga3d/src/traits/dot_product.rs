use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// DotProduct
/// This dot product is almost exactly what you would expect from regular vector algebra. It always returns a scalar result. It is determined by the metric of the algebra, so intermediate terms might come out to zero or negative, depending on the generator squares. The "dot product" is overloaded to several different meanings depending on which community you discuss with, so if someone or something refers to the "dot product" then use care and double check the definition that is intended.
pub trait DotProduct<T> {
    fn dot_product(self, other: T) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct dot_product;
#[allow(non_camel_case_types)]
pub struct dot_product_partial<A>(A);
impl<A: DotProduct<B>, B> std::ops::Div<B> for dot_product_partial<A> {
    type Output = Scalar;
    fn div(self, rhs: B) -> Self::Output {
        self.0.dot_product(rhs)
    }
}
include!("./impls/dot_product.rs");
