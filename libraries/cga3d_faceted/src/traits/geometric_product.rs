use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricProduct
/// The geometric product is what lets us treat geometry like algebra. It is literally multiplication of geometry. It is derived by the wedge product and the algebra's metric. Generator elements (grade 1 BasisElements) will square to a particular value (as specified by the algebra and metric), typically 1, -1, or 0. This is in contrast to the wedge product which always wedge-squares elements to zero. Multiplying uniform grade geometry may result in mixed grade products, topping out at the AntiScalar. See also Sandwich.
pub trait GeometricProduct<T> {
    type Output;
    fn geometric_product(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static geometric_product: GeometricProductInfix = GeometricProductInfix;
pub struct GeometricProductInfix;
pub struct GeometricProductInfixPartial<A>(A);
impl<A: GeometricProduct<B>, B> std::ops::Div<B> for GeometricProductInfixPartial<A> {
    type Output = <A as GeometricProduct<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_product(rhs)
    }
}
include!("./impls/geometric_product.rs");
