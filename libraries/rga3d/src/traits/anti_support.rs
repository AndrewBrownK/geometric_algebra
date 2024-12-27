use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiSupport
/// The anti-support is the anti-vector furthest from the origin that encloses the object.
pub trait AntiSupport {
    type Output;
    fn anti_support(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_support: AntiSupportPrefixOrPostfix = AntiSupportPrefixOrPostfix;
pub struct AntiSupportPrefixOrPostfix;
impl<A: AntiSupport> std::ops::Div<A> for AntiSupportPrefixOrPostfix {
    type Output = <A as AntiSupport>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_support()
    }
}
include!("./impls/anti_support.rs");
