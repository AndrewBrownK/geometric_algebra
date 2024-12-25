using data::*;

/// AntiFix
/// Automatically fix the anti geometric constraint by adjusting the bulk to comply with the weight, and then weight normalize the result.
pub trait AntiFix {
    fn anti_fix(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_fix;
impl<A: AntiFix> std::ops::Div<A> for anti_fix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_fix()
    }
}
__include ./impls/anti_fix;
