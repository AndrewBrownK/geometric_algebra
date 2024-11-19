use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiSquareRoot
/// TODO
pub trait AntiSquareRoot {
    fn anti_square_root(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_square_root;
impl<A: AntiSquareRoot> std::ops::Div<A> for anti_square_root {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_square_root()
    }
}
include!("./impls/anti_square_root.rs");
