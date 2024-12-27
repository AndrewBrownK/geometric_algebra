use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Fix
/// Automatically fix the geometric constraint by adjusting the weight to comply with the bulk, and then bulk normalize the result.
pub trait Fix {
    fn fix(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static fix: FixPrefixOrPostfix = FixPrefixOrPostfix;
pub struct FixPrefixOrPostfix;
impl<A: Fix> std::ops::Div<A> for FixPrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.fix()
    }
}
include!("./impls/fix.rs");
