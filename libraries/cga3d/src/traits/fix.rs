use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Fix
/// TODO
pub trait Fix {
    fn fix(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct fix;
impl<A: Fix> std::ops::Div<A> for fix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.fix()
    }
}
include!("./impls/fix.rs");
