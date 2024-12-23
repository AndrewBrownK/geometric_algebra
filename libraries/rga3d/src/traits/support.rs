use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Support
/// The support is the point enclosed by the object closest to the origin.
pub trait Support {
    fn support(self) -> Line;
}
#[allow(non_camel_case_types, dead_code)]
pub struct support;
impl<A: Support> std::ops::Div<A> for support {
    type Output = Line;
    fn div(self, rhs: A) -> Self::Output {
        rhs.support()
    }
}
include!("./impls/support.rs");
