using data::*;

/// AntiRejectOrthogonallyFrom
/// Counterpart to AntiProjectOrthogonallyOnto.
pub trait AntiRejectOrthogonallyFrom<T> {
    type Output;
    fn anti_reject_orthogonally_from(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_reject_orthogonally_from;
#[allow(non_camel_case_types)]
pub struct anti_reject_orthogonally_from_partial<A>(A);
impl<A: AntiRejectOrthogonallyFrom<B>, B> std::ops::Div<B> for anti_reject_orthogonally_from_partial<A> {
    type Output = <A as AntiRejectOrthogonallyFrom<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_reject_orthogonally_from(rhs)
    }
}
__include ./impls/anti_reject_orthogonally_from;
