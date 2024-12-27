use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Inverse
/// The inverse with respect to geometric product. Inverse(x) = x^-1.
pub trait Inverse {
    fn inverse(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static inverse: InversePrefixOrPostfix = InversePrefixOrPostfix;
pub struct InversePrefixOrPostfix;
impl<A: Inverse> std::ops::Div<A> for InversePrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.inverse()
    }
}
include!("./impls/inverse.rs");
