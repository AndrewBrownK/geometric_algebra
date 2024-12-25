using data::*;

/// FlatWeight
/// This characterizes the flat aspect's relationship with the horizon.
pub trait FlatWeight {
    type Output;
    fn flat_weight(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct flat_weight;
impl<A: FlatWeight> std::ops::Div<A> for flat_weight {
    type Output = <A as FlatWeight>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.flat_weight()
    }
}
__include ./impls/flat_weight;
