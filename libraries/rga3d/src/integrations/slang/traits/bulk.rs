using data::*;

/// Bulk
/// This characterizes the flat aspect's relationship with the origin.
pub trait Bulk {
    type Output;
    fn bulk(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct bulk;
impl<A: Bulk> std::ops::Div<A> for bulk {
    type Output = <A as Bulk>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.bulk()
    }
}
__include ./impls/bulk;
