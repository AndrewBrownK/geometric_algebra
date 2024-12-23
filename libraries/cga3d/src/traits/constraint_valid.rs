use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ConstraintValid
/// Implementors of this trait cannot violate the geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with ConstraintViolation. See also ConstraintViolation and Fix.
pub trait ConstraintValid {
    fn constraint_valid(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct constraint_valid;
impl<A: ConstraintValid> std::ops::Div<A> for constraint_valid {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.constraint_valid()
    }
}
include!("./impls/constraint_valid.rs");
