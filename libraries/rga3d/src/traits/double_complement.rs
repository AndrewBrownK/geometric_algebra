use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// DoubleComplement
/// TODO
pub trait DoubleComplement {
    fn double_complement(self) -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct double_complement;
impl<A: DoubleComplement> std::ops::Div<A> for double_complement {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.double_complement()
    }
}
include!("./impls/double_complement.rs");
