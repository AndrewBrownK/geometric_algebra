use crate::data::*;
use crate::simd::*;

/// ConstraintValid
/// TODO
pub trait ConstraintValid {
    fn constraint_valid(self) -> Self;
}
#[allow(non_camel_case_types)]
pub struct constraint_valid;
impl<A: ConstraintValid> std::ops::Div<constraint_valid> for A {
    type Output = A;
    fn div(self, _rhs: constraint_valid) -> Self::Output {
        self.constraint_valid()
    }
}
impl<A: ConstraintValid> std::ops::Div<A> for constraint_valid {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.constraint_valid()
    }
}
impl<A: ConstraintValid> std::ops::DivAssign<A> for constraint_valid {
    fn div_assign(&mut self, rhs: constraint_valid) {
        *self = *self.constraint_valid()
    }
}
include!("./impls/constraint_valid.rs");
