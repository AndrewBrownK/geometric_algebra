use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricQuotient
/// Product of A with Inverse of B.
pub trait GeometricQuotient<T> {
    type Output;
    fn geometric_quotient(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static geometric_quotient: GeometricQuotientInfix = GeometricQuotientInfix;
pub struct GeometricQuotientInfix;
pub struct GeometricQuotientInfixPartial<A>(A);
impl<A: GeometricQuotient<B>, B> std::ops::Div<B> for GeometricQuotientInfixPartial<A> {
    type Output = <A as GeometricQuotient<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_quotient(rhs)
    }
}
include!("./impls/geometric_quotient.rs");
