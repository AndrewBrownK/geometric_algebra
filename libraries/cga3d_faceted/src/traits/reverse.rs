use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Reverse
/// TODO
pub trait Reverse {
    fn reverse(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct reverse;
impl<A: Reverse> std::ops::Div<A> for reverse {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.reverse()
    }
}
include!("./impls/reverse.rs");
