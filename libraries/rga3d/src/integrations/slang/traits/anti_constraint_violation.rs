using data::*;

/// AntiConstraintViolation
/// Not every combinations of floats is valid geometry. Some types of objects are required to fulfill a constraint in order to be valid geometry. We call this the (anti) geometric constraint. If a type of object may possibly violate this constraint, then it will implement this trait. The constraint is violated if a non-zero value is returned. See also AntiConstraintValid and AntiFix.
pub trait AntiConstraintViolation {
    type Output;
    fn anti_constraint_violation(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_constraint_violation;
impl<A: AntiConstraintViolation> std::ops::Div<A> for anti_constraint_violation {
    type Output = <A as AntiConstraintViolation>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_constraint_violation()
    }
}
__include ./impls/anti_constraint_violation;
