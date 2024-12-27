use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CenterNorm
/// Distance between origin and center (not yet unitized, still requires division by round weight).
pub trait CenterNorm {
    fn center_norm(self) -> Scalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static center_norm: CenterNormPrefixOrPostfix = CenterNormPrefixOrPostfix;
pub struct CenterNormPrefixOrPostfix;
impl<A: CenterNorm> std::ops::Div<A> for CenterNormPrefixOrPostfix {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.center_norm()
    }
}
include!("./impls/center_norm.rs");
