use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RightAntiDual
/// This dual is the "AntiMetric Right Dual". To take this dual of an object, the object is multiplied by the anti-metric, and then we take the right complement. This will turn a Scalar into an AntiScalar, a Vector into an AntiVector, so on, and vice versa. The use of the anti-metric may give distinct results from a simple right complement, typically by changing the coefficient on some terms (1, -1, or 0)
pub trait RightAntiDual {
    type Output;
    fn right_anti_dual(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static right_anti_dual: RightAntiDualPrefixOrPostfix = RightAntiDualPrefixOrPostfix;
pub struct RightAntiDualPrefixOrPostfix;
impl<A: RightAntiDual> std::ops::Div<A> for RightAntiDualPrefixOrPostfix {
    type Output = <A as RightAntiDual>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.right_anti_dual()
    }
}
include!("./impls/right_anti_dual.rs");
