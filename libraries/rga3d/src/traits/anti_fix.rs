use crate::data::*;
use crate::simd::*;

/// AntiFix
/// TODO
pub trait AntiFix {
    fn anti_fix(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct anti_fix;
impl<A: AntiFix> std::ops::Div<anti_fix> for A {
    type Output = A;
    fn div(self, _rhs: anti_fix) -> Self::Output {
        self.anti_fix()
    }
}
impl<A: AntiFix> std::ops::Div<A> for anti_fix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_fix()
    }
}
impl<A: AntiFix> std::ops::DivAssign<A> for anti_fix {
    fn div_assign(&mut self, rhs: anti_fix) {
        *self = *self.anti_fix()
    }
}
include!("./impls/anti_fix.rs");
