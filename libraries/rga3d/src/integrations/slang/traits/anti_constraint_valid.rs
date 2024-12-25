using data::*;

/// AntiConstraintValid
/// Implementors of this trait cannot violate the anti geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with AntiConstraintViolation. See also AntiConstraintViolation and AntiFix.
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
__include ./impls/anti_constraint_valid;
