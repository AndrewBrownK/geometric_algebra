use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
pub trait ProjectOrthogonallyOnto<T> {
    type Output;
    fn project_orthogonally_onto(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static project_orthogonally_onto: ProjectOrthogonallyOntoInfix = ProjectOrthogonallyOntoInfix;
pub struct ProjectOrthogonallyOntoInfix;
pub struct ProjectOrthogonallyOntoInfixPartial<A>(A);
impl<A: ProjectOrthogonallyOnto<B>, B> std::ops::Div<B> for ProjectOrthogonallyOntoInfixPartial<A> {
    type Output = <A as ProjectOrthogonallyOnto<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.project_orthogonally_onto(rhs)
    }
}
include!("./impls/project_orthogonally_onto.rs");
