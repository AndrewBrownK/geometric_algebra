use crate::data::*;
use crate::simd::*;

/// GeometricProduct
/// TODO
pub trait GeometricProduct<T> {
    type Output;
    fn geometric_product(self, other: T) -> Self::Output;
}
pub trait InfixGeometricProduct {}
#[allow(non_camel_case_types)]
pub struct geometric_product;
#[allow(non_camel_case_types)]
pub struct geometric_product_partial<A>(A);
impl<A: InfixGeometricProduct> std::ops::Div<geometric_product> for A {
    type Output = geometric_product_partial<A>;
    fn div(self, _rhs: geometric_product) -> Self::Output {
        geometric_product_partial(self)
    }
}
impl<A: GeometricProduct<B>, B> std::ops::Div<B> for geometric_product_partial<A> {
    type Output = <A as GeometricProduct<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_product(rhs)
    }
}
include!("./impls/geometric_product.rs");
