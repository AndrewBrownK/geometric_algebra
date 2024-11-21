use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiDotProduct
/// TODO
pub trait AntiDotProduct<T> {
    fn anti_dot_product(self, other: T) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_dot_product;
#[allow(non_camel_case_types)]
pub struct anti_dot_product_partial<A>(A);
impl<A: AntiDotProduct<B>, B> std::ops::Div<B> for anti_dot_product_partial<A> {
    type Output = AntiScalar;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_dot_product(rhs)
    }
}
include!("./impls/anti_dot_product.rs");
