use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiReverse
/// TODO
pub trait AntiReverse {
    fn anti_reverse(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_reverse;
impl<A: AntiReverse> std::ops::Div<A> for anti_reverse {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_reverse()
    }
}
include!("./impls/anti_reverse.rs");
