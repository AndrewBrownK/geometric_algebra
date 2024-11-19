use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiConstraintViolation
/// TODO
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
include!("./impls/anti_constraint_violation.rs");
