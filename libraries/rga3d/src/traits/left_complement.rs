use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// LeftComplement
/// TODO
pub trait LeftComplement {
    type Output;
    fn left_complement(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct left_complement;
impl<A: LeftComplement> std::ops::Div<A> for left_complement {
    type Output = <A as LeftComplement>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.left_complement()
    }
}
include!("./impls/left_complement.rs");
