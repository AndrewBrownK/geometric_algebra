use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiFix
/// Automatically fix the anti geometric constraint by adjusting the bulk to comply with the weight, and then weight normalize the result.
pub trait AntiFix {
    fn anti_fix(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_fix: AntiFixPrefixOrPostfix = AntiFixPrefixOrPostfix;
pub struct AntiFixPrefixOrPostfix;
impl<A: AntiFix> std::ops::Div<A> for AntiFixPrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_fix()
    }
}
include!("./impls/anti_fix.rs");
