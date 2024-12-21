use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// LeftComplement
/// The LeftComplement of a BasisElement is the missing BasisElement that when wedged together will create the AntiScalar. For example, with an AntiScalar of e1234, the left_complement(e1) = -e234, because wedge(e234, e1) = e1234. In this example, the left_complement(e234) = e1, because wedge(e1, e234) = e1234. See also RightComplement and DoubleComplement. The RightComplement can be used to undo a LeftComplement.
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
