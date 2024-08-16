use crate::data::*;
use crate::simd::*;

/// GeometricAntiProduct
/// TODO
pub trait GeometricAntiProduct<T> {
    type Output;
    fn geometric_anti_product(self, other: T) -> Self::Output;
}
pub trait InfixGeometricAntiProduct {}
#[allow(non_camel_case_types)]
pub struct geometric_anti_product;
#[allow(non_camel_case_types)]
pub struct geometric_anti_product_partial<A>(A);
impl<A: InfixGeometricAntiProduct> std::ops::Div<geometric_anti_product> for A {
    type Output = geometric_anti_product_partial<A>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl<A: GeometricAntiProduct<B>, B> std::ops::Div<B> for geometric_anti_product_partial<A> {
    type Output = <A as GeometricAntiProduct<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_anti_product(rhs)
    }
}
include!("./impls/geometric_anti_product.rs");
