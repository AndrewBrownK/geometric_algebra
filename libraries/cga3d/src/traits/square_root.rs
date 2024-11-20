use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// SquareRoot
/// TODO
pub trait SquareRoot {
    fn square_root(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct square_root;
impl<A: SquareRoot> std::ops::Div<A> for square_root {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.square_root()
    }
}
include!("./impls/square_root.rs");
