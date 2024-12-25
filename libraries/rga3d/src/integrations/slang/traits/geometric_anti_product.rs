using data::*;

/// GeometricAntiProduct
/// The GeometricAntiProduct or sometimes called AntiProduct is the dual to the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade geometry may result in mixed grade anti-products, bottoming out at the Scalar. See also AntiSandwich.
pub trait GeometricAntiProduct<T> {
    type Output;
    fn geometric_anti_product(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct geometric_anti_product;
#[allow(non_camel_case_types)]
pub struct geometric_anti_product_partial<A>(A);
impl<A: GeometricAntiProduct<B>, B> std::ops::Div<B> for geometric_anti_product_partial<A> {
    type Output = <A as GeometricAntiProduct<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_anti_product(rhs)
    }
}
__include ./impls/geometric_anti_product;
