use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
pub trait GeometricAntiQuotient<T> {
    type Output;
    fn geometric_anti_quotient(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static geometric_anti_quotient: GeometricAntiQuotientInfix = GeometricAntiQuotientInfix;
pub struct GeometricAntiQuotientInfix;
pub struct GeometricAntiQuotientInfixPartial<A>(A);
impl<A: GeometricAntiQuotient<B>, B> std::ops::Div<B> for GeometricAntiQuotientInfixPartial<A> {
    type Output = <A as GeometricAntiQuotient<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_anti_quotient(rhs)
    }
}
include!("./impls/geometric_anti_quotient.rs");
