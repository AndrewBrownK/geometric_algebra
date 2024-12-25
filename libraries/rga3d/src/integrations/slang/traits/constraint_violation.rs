using data::*;

/// ConstraintViolation
/// Not every combinations of floats is valid geometry. Some types of objects are required to fulfill a constraint in order to be valid geometry. We call this the geometric constraint. If a type of object may possibly violate this constraint, then it will implement this trait. The constraint is violated if a non-zero value is returned. See also ConstraintValid and Fix.
pub trait ConstraintViolation {
    type Output;
    fn constraint_violation(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct constraint_violation;
impl<A: ConstraintViolation> std::ops::Div<A> for constraint_violation {
    type Output = <A as ConstraintViolation>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.constraint_violation()
    }
}
__include ./impls/constraint_violation;
