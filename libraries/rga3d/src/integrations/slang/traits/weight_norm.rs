using data::*;

/// WeightNorm
/// Weight Norm for flat aspect.
pub trait WeightNorm {
    fn weight_norm(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct weight_norm;
impl<A: WeightNorm> std::ops::Div<A> for weight_norm {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.weight_norm()
    }
}
__include ./impls/weight_norm;
