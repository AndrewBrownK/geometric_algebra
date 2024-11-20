use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ScalarProduct
/// TODO
pub trait ScalarProduct<T> {
    fn scalar_product(self, other: T) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct scalar_product;
#[allow(non_camel_case_types)]
pub struct scalar_product_partial<A>(A);
impl<A: ScalarProduct<B>, B> std::ops::Div<B> for scalar_product_partial<A> {
    type Output = Scalar;
    fn div(self, rhs: B) -> Self::Output {
        self.0.scalar_product(rhs)
    }
}
include!("./impls/scalar_product.rs");
