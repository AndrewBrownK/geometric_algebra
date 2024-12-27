use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RightComplement
/// The RightComplement of a BasisElement is the missing BasisElement that when wedged together will create the AntiScalar. For example, with an AntiScalar of e1234, the right_complement(e1) = e234, because wedge(e1, e234) = e1234. In this example, the right_complement(e234) = -e1, because wedge(e234, -e1) = e1234. See also LeftComplement and DoubleComplement. The LeftComplement can be used to undo a RightComplement.
pub trait RightComplement {
    type Output;
    fn right_complement(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static right_complement: RightComplementPrefixOrPostfix = RightComplementPrefixOrPostfix;
pub struct RightComplementPrefixOrPostfix;
impl<A: RightComplement> std::ops::Div<A> for RightComplementPrefixOrPostfix {
    type Output = <A as RightComplement>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.right_complement()
    }
}
include!("./impls/right_complement.rs");
