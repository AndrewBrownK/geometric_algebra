use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiProjectOrthogonallyOnto
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
pub trait AntiProjectOrthogonallyOnto<T> {
    type Output;
    fn anti_project_orthogonally_onto(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_project_orthogonally_onto: AntiProjectOrthogonallyOntoInfix = AntiProjectOrthogonallyOntoInfix;
pub struct AntiProjectOrthogonallyOntoInfix;
pub struct AntiProjectOrthogonallyOntoInfixPartial<A>(A);
impl<A: AntiProjectOrthogonallyOnto<B>, B> std::ops::Div<B> for AntiProjectOrthogonallyOntoInfixPartial<A> {
    type Output = <A as AntiProjectOrthogonallyOnto<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_project_orthogonally_onto(rhs)
    }
}
include!("./impls/anti_project_orthogonally_onto.rs");
