using data::*;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
pub trait ProjectOrthogonallyOnto<T> {
    type Output;
    fn project_orthogonally_onto(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct project_orthogonally_onto;
#[allow(non_camel_case_types)]
pub struct project_orthogonally_onto_partial<A>(A);
impl<A: ProjectOrthogonallyOnto<B>, B> std::ops::Div<B> for project_orthogonally_onto_partial<A> {
    type Output = <A as ProjectOrthogonallyOnto<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.project_orthogonally_onto(rhs)
    }
}
__include ./impls/project_orthogonally_onto;
