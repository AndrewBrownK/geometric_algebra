use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RadiusNorm
/// Distance radius of a round object (not yet unitized, still requires division by round weight).
pub trait RadiusNorm {
    fn radius_norm(self) -> Scalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct radius_norm;
impl<A: RadiusNorm> std::ops::Div<A> for radius_norm {
    type Output = Scalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.radius_norm()
    }
}
include!("./impls/radius_norm.rs");
