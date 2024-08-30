use crate::data::*;
use crate::simd::*;

/// ConstraintViolation
/// TODO
pub trait ConstraintViolation {
    type Output;
    fn constraint_violation(self) -> Self::Output;
}
#[allow(non_camel_case_types)]
pub struct constraint_violation;
impl<A: ConstraintViolation> std::ops::Div<constraint_violation> for A {
    type Output = <A as ConstraintViolation>::Output;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl<A: ConstraintViolation> std::ops::Div<A> for constraint_violation {
    type Output = <A as ConstraintViolation>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.constraint_violation()
    }
}
include!("./impls/constraint_violation.rs");
