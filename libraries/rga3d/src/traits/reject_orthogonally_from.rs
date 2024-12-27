use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
pub trait RejectOrthogonallyFrom<T> {
    type Output;
    fn reject_orthogonally_from(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static reject_orthogonally_from: RejectOrthogonallyFromInfix = RejectOrthogonallyFromInfix;
pub struct RejectOrthogonallyFromInfix;
pub struct RejectOrthogonallyFromInfixPartial<A>(A);
impl<A: RejectOrthogonallyFrom<B>, B> std::ops::Div<B> for RejectOrthogonallyFromInfixPartial<A> {
    type Output = <A as RejectOrthogonallyFrom<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.reject_orthogonally_from(rhs)
    }
}
include!("./impls/reject_orthogonally_from.rs");
