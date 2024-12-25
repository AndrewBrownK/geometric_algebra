using data::*;

/// AntiProjectOrthogonallyOnto
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
pub trait AntiProjectOrthogonallyOnto<T> {
    type Output;
    fn anti_project_orthogonally_onto(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_project_orthogonally_onto;
#[allow(non_camel_case_types)]
pub struct anti_project_orthogonally_onto_partial<A>(A);
impl<A: AntiProjectOrthogonallyOnto<B>, B> std::ops::Div<B> for anti_project_orthogonally_onto_partial<A> {
    type Output = <A as AntiProjectOrthogonallyOnto<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_project_orthogonally_onto(rhs)
    }
}
__include ./impls/anti_project_orthogonally_onto;
