using data::*;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
pub trait AntiInverse {
    fn anti_inverse(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_inverse;
impl<A: AntiInverse> std::ops::Div<A> for anti_inverse {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_inverse()
    }
}
__include ./impls/anti_inverse;
