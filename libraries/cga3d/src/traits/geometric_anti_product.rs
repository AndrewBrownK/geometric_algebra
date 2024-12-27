use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricAntiProduct
/// The GeometricAntiProduct or sometimes called AntiProduct is the dual to the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade geometry may result in mixed grade anti-products, bottoming out at the Scalar. See also AntiSandwich.
pub trait GeometricAntiProduct<T> {
    type Output;
    fn geometric_anti_product(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static geometric_anti_product: GeometricAntiProductInfix = GeometricAntiProductInfix;
pub struct GeometricAntiProductInfix;
pub struct GeometricAntiProductInfixPartial<A>(A);
impl<A: GeometricAntiProduct<B>, B> std::ops::Div<B> for GeometricAntiProductInfixPartial<A> {
    type Output = <A as GeometricAntiProduct<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_anti_product(rhs)
    }
}
include!("./impls/geometric_anti_product.rs");
