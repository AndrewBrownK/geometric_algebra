use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Support
/// The support is the point enclosed by the object closest to the origin.
pub trait Support {
    type Output;
    fn support(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct support;
impl<A: Support> std::ops::Div<A> for support {
    type Output = <A as Support>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.support()
    }
}
include!("./impls/support.rs");
