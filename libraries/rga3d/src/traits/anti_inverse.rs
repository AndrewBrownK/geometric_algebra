use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiInverse
/// TODO
pub trait AntiInverse {
    fn anti_inverse(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_inverse;
impl<A: AntiInverse> std::ops::Div<A> for anti_inverse {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_inverse()
    }
}
include!("./impls/anti_inverse.rs");
