use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiRejectOrthogonallyFrom
/// Counterpart to AntiProjectOrthogonallyOnto.
pub trait AntiRejectOrthogonallyFrom<T> {
    type Output;
    fn anti_reject_orthogonally_from(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_reject_orthogonally_from: AntiRejectOrthogonallyFromInfix = AntiRejectOrthogonallyFromInfix;
pub struct AntiRejectOrthogonallyFromInfix;
pub struct AntiRejectOrthogonallyFromInfixPartial<A>(A);
impl<A: AntiRejectOrthogonallyFrom<B>, B> std::ops::Div<B> for AntiRejectOrthogonallyFromInfixPartial<A> {
    type Output = <A as AntiRejectOrthogonallyFrom<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_reject_orthogonally_from(rhs)
    }
}
include!("./impls/anti_reject_orthogonally_from.rs");
