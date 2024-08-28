use crate::data::*;
use crate::simd::*;

/// GeometricQuotient
/// TODO
pub trait GeometricQuotient<T> {
    fn geometric_quotient(self, other: T) -> Self;
}
pub trait InfixGeometricQuotient {}
#[allow(non_camel_case_types)]
pub struct geometric_quotient;
#[allow(non_camel_case_types)]
pub struct geometric_quotient_partial<A>(A);
impl<A: InfixGeometricQuotient> std::ops::Div<geometric_quotient> for A {
    type Output = geometric_quotient_partial<A>;
    fn div(self, _rhs: geometric_quotient) -> Self::Output {
        geometric_quotient_partial(self)
    }
}
impl<A: GeometricQuotient<B>, B> std::ops::Div<B> for geometric_quotient_partial<A> {
    type Output = A;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_quotient(rhs)
    }
}
include!("./impls/geometric_quotient.rs");
