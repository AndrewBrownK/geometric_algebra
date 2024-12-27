use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiDotProduct
/// This is the dual to the dot product, and always returns an AntiScalar.
pub trait AntiDotProduct<T> {
    fn anti_dot_product(self, other: T) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_dot_product: AntiDotProductInfix = AntiDotProductInfix;
pub struct AntiDotProductInfix;
pub struct AntiDotProductInfixPartial<A>(A);
impl<A: AntiDotProduct<B>, B> std::ops::Div<B> for AntiDotProductInfixPartial<A> {
    type Output = AntiScalar;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_dot_product(rhs)
    }
}
include!("./impls/anti_dot_product.rs");
