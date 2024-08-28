use crate::data::*;
use crate::simd::*;

/// GeometricAntiQuotient
/// TODO
pub trait GeometricAntiQuotient<T> {
    fn geometric_anti_quotient(self, other: T) -> Self;
}
pub trait InfixGeometricAntiQuotient {}
#[allow(non_camel_case_types)]
pub struct geometric_anti_quotient;
#[allow(non_camel_case_types)]
pub struct geometric_anti_quotient_partial<A>(A);
impl<A: InfixGeometricAntiQuotient> std::ops::Div<geometric_anti_quotient> for A {
    type Output = geometric_anti_quotient_partial<A>;
    fn div(self, _rhs: geometric_anti_quotient) -> Self::Output {
        geometric_anti_quotient_partial(self)
    }
}
impl<A: GeometricAntiQuotient<B>, B> std::ops::Div<B> for geometric_anti_quotient_partial<A> {
    type Output = A;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_anti_quotient(rhs)
    }
}
include!("./impls/geometric_anti_quotient.rs");
