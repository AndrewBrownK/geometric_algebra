use crate::data::*;
use crate::simd::*;

/// Fix
/// TODO
pub trait Fix {
    fn fix(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct fix;
impl<A: Fix> std::ops::Div<fix> for A {
    type Output = A;
    fn div(self, _rhs: fix) -> Self::Output {
        self.fix()
    }
}
impl<A: Fix> std::ops::Div<A> for fix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.fix()
    }
}
impl<A: Fix> std::ops::DivAssign<A> for fix {
    fn div_assign(&mut self, rhs: fix) {
        *self = *self.fix()
    }
}
include!("./impls/fix.rs");
