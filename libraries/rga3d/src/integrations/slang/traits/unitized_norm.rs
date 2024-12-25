using data::*;

/// UnitizedNorm
/// Unitized FlatNorm.
pub trait UnitizedNorm {
    fn unitized_norm(self) -> float;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_norm;
impl<A: UnitizedNorm> std::ops::Div<A> for unitized_norm {
    type Output = float;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_norm()
    }
}
__include ./impls/unitized_norm;
