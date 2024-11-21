use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RightComplement
/// TODO
pub trait RightComplement {
    type Output;
    fn right_complement(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct right_complement;
impl<A: RightComplement> std::ops::Div<A> for right_complement {
    type Output = <A as RightComplement>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.right_complement()
    }
}
include!("./impls/right_complement.rs");
