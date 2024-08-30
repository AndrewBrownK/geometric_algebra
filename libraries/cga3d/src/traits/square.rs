use crate::data::*;
use crate::simd::*;

/// Square
/// TODO
pub trait Square {
    type Output;
    fn square(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct square;
impl<A: Square> std::ops::Div<square> for A {
    type Output = <A as Square>::Output;
    fn div(self, _rhs: square) -> Self::Output {
        self.square()
    }
}
impl<A: Square> std::ops::Div<A> for square {
    type Output = <A as Square>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.square()
    }
}
include!("./impls/square.rs");
