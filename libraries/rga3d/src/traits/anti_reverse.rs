use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiReverse
/// The AntiReversal is like Reversal, but with respect to the AntiWedge product instead of the Wedge product. This means we can only find the AntiReverse of an element if we specify an AntiScalar first. By example, if our AntiScalar is e123 in vanilla geometric algebra, then e2 = anti_wedge(e12, e32), and anti_reverse(e2) = anti_wedge(e32, e12) = -e2. Notably, the Reverse of grade 1 vectors does not change sign, but the AntiReverse of grade 1 vectors may change sign (depending on the AntiScalar). When it comes to the AntiReverse, it is AntiGrade 1 Vectors (AntiVectors) that will not change sign.
pub trait AntiReverse {
    fn anti_reverse(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_reverse: AntiReversePrefixOrPostfix = AntiReversePrefixOrPostfix;
pub struct AntiReversePrefixOrPostfix;
impl<A: AntiReverse> std::ops::Div<A> for AntiReversePrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_reverse()
    }
}
include!("./impls/anti_reverse.rs");
