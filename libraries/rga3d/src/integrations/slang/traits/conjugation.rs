using data::*;

/// Conjugation
/// This composes the reverse and grade involution (automorphism).
pub trait Conjugation {
    fn conjugation(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct conjugation;
impl<A: Conjugation> std::ops::Div<A> for conjugation {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.conjugation()
    }
}
__include ./impls/conjugation;
