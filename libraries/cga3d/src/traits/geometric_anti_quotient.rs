use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
pub trait GeometricAntiQuotient<T> {
    type Output;
    fn geometric_anti_quotient(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct geometric_anti_quotient;
#[allow(non_camel_case_types)]
pub struct geometric_anti_quotient_partial<A>(A);
impl<A: GeometricAntiQuotient<B>, B> std::ops::Div<B> for geometric_anti_quotient_partial<A> {
    type Output = <A as GeometricAntiQuotient<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_anti_quotient(rhs)
    }
}
include!("./impls/geometric_anti_quotient.rs");
