use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Complement
/// The Complement of a BasisElement is the missing BasisElement that when wedged together will create the AntiScalar. In geometric algebras with an odd number of dimensions, the LeftComplement and RightComplement are the same, so we just call it the Complement. For example, with an AntiScalar of e123, the complement of e1 can be found equivalently by solving e123 = wedge(e1, x) or e123 = wedge(x, e1). See also DoubleComplement.
pub trait Complement {
    type Output;
    fn complement(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct complement;
impl<A: Complement> std::ops::Div<A> for complement {
    type Output = <A as Complement>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.complement()
    }
}
include!("./impls/complement.rs");
