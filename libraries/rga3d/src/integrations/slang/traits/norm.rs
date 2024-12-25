using data::*;

/// Norm
/// Norm for flat aspect.
pub trait Norm {
    fn norm(self) -> DualNum;
}
#[allow(non_camel_case_types, dead_code)]
pub struct norm;
impl<A: Norm> std::ops::Div<A> for norm {
    type Output = DualNum;
    fn div(self, rhs: A) -> Self::Output {
        rhs.norm()
    }
}
__include ./impls/norm;
