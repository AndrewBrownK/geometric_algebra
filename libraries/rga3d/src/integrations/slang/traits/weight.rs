using data::*;

/// Weight
/// This characterizes the flat aspect's relationship with the horizon.
pub trait Weight {
    type Output;
    fn weight(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct weight;
impl<A: Weight> std::ops::Div<A> for weight {
    type Output = <A as Weight>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.weight()
    }
}
__include ./impls/weight;
