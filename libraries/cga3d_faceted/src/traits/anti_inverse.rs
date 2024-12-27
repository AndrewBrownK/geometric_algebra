use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
pub trait AntiInverse {
    fn anti_inverse(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_inverse: AntiInversePrefixOrPostfix = AntiInversePrefixOrPostfix;
pub struct AntiInversePrefixOrPostfix;
impl<A: AntiInverse> std::ops::Div<A> for AntiInversePrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_inverse()
    }
}
include!("./impls/anti_inverse.rs");
