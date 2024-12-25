using data::*;

/// ProjectViaOriginOnto
/// Central (to origin) Projection.
pub trait ProjectViaOriginOnto<T> {
    type Output;
    fn project_via_origin_onto(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct project_via_origin_onto;
#[allow(non_camel_case_types)]
pub struct project_via_origin_onto_partial<A>(A);
impl<A: ProjectViaOriginOnto<B>, B> std::ops::Div<B> for project_via_origin_onto_partial<A> {
    type Output = <A as ProjectViaOriginOnto<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.project_via_origin_onto(rhs)
    }
}
__include ./impls/project_via_origin_onto;
