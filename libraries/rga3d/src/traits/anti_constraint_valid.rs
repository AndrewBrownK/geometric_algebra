use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiConstraintValid
/// TODO
pub trait AntiConstraintValid {
    fn anti_constraint_valid(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_constraint_valid;
impl<A: AntiConstraintValid> std::ops::Div<A> for anti_constraint_valid {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_constraint_valid()
    }
}
include!("./impls/anti_constraint_valid.rs");
