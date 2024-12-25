using data::*;

/// Unitize
/// Scale the object to have a weight norm of 1.
pub trait Unitize {
    fn unitize(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitize;
impl<A: Unitize> std::ops::Div<A> for unitize {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitize()
    }
}
__include ./impls/unitize;
