use crate::data::*;
use crate::simd::*;

/// AntiConstraintValid
/// TODO
pub trait AntiConstraintValid {
    fn anti_constraint_valid(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct anti_constraint_valid;
impl<A: AntiConstraintValid> std::ops::Div<anti_constraint_valid> for A {
    type Output = A;
    fn div(self, _rhs: anti_constraint_valid) -> Self::Output {
        self.anti_constraint_valid()
    }
}
impl<A: AntiConstraintValid> std::ops::Div<A> for anti_constraint_valid {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_constraint_valid()
    }
}
impl<A: AntiConstraintValid> std::ops::DivAssign<A> for anti_constraint_valid {
    fn div_assign(&mut self, rhs: anti_constraint_valid) {
        *self = *self.anti_constraint_valid()
    }
}
include!("./impls/anti_constraint_valid.rs");
