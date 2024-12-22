use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
pub trait RejectOrthogonallyFrom<T> {
    type Output;
    fn reject_orthogonally_from(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct reject_orthogonally_from;
#[allow(non_camel_case_types)]
pub struct reject_orthogonally_from_partial<A>(A);
impl<A: RejectOrthogonallyFrom<B>, B> std::ops::Div<B> for reject_orthogonally_from_partial<A> {
    type Output = <A as RejectOrthogonallyFrom<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.reject_orthogonally_from(rhs)
    }
}
include!("./impls/reject_orthogonally_from.rs");
