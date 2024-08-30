use crate::data::*;
use crate::simd::*;

/// AntiSquare
/// TODO
pub trait AntiSquare {
    type Output;
    fn anti_square(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct anti_square;
impl<A: AntiSquare> std::ops::Div<anti_square> for A {
    type Output = <A as AntiSquare>::Output;
    fn div(self, _rhs: anti_square) -> Self::Output {
        self.anti_square()
    }
}
impl<A: AntiSquare> std::ops::Div<A> for anti_square {
    type Output = <A as AntiSquare>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_square()
    }
}
include!("./impls/anti_square.rs");
