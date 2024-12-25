using data::*;

/// UnitizedFlatNormSquared
/// Intermediate result to UnitizedFlatNorm.
pub trait UnitizedFlatNormSquared {
    fn unitized_flat_norm_squared(self) -> float;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unitized_flat_norm_squared;
impl<A: UnitizedFlatNormSquared> std::ops::Div<A> for unitized_flat_norm_squared {
    type Output = float;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitized_flat_norm_squared()
    }
}
__include ./impls/unitized_flat_norm_squared;
