use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricQuotient
/// Product of A with Inverse of B.
pub trait GeometricQuotient<T> {
    fn geometric_quotient(self, other: T) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct geometric_quotient;
#[allow(non_camel_case_types)]
pub struct geometric_quotient_partial<A>(A);
impl<A: GeometricQuotient<B>, B> std::ops::Div<B> for geometric_quotient_partial<A> {
    type Output = A;
    fn div(self, rhs: B) -> Self::Output {
        self.0.geometric_quotient(rhs)
    }
}
include!("./impls/geometric_quotient.rs");
