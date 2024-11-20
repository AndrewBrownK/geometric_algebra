use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiScalarProduct
/// TODO
pub trait AntiScalarProduct<T> {
    fn anti_scalar_product(self, other: T) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_scalar_product;
#[allow(non_camel_case_types)]
pub struct anti_scalar_product_partial<A>(A);
impl<A: AntiScalarProduct<B>, B> std::ops::Div<B> for anti_scalar_product_partial<A> {
    type Output = AntiScalar;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_scalar_product(rhs)
    }
}
include!("./impls/anti_scalar_product.rs");
