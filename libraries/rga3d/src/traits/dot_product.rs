use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// DotProduct
/// TODO
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
