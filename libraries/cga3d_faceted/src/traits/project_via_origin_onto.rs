use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ProjectViaOriginOnto
/// Central (to origin) Projection.
pub trait ProjectViaOriginOnto<T> {
    type Output;
    fn project_via_origin_onto(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static project_via_origin_onto: ProjectViaOriginOntoInfix = ProjectViaOriginOntoInfix;
pub struct ProjectViaOriginOntoInfix;
pub struct ProjectViaOriginOntoInfixPartial<A>(A);
impl<A: ProjectViaOriginOnto<B>, B> std::ops::Div<B> for ProjectViaOriginOntoInfixPartial<A> {
    type Output = <A as ProjectViaOriginOnto<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.project_via_origin_onto(rhs)
    }
}
include!("./impls/project_via_origin_onto.rs");
