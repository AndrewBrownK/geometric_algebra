use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Unitize
/// Scale the object to have a weight norm of 1.
pub trait Unitize {
    fn unitize(self) -> Self;
}
#[allow(non_upper_case_globals, dead_code)]
pub static unitize: UnitizePrefixOrPostfix = UnitizePrefixOrPostfix;
pub struct UnitizePrefixOrPostfix;
impl<A: Unitize> std::ops::Div<A> for UnitizePrefixOrPostfix {
    type Output = A;
    fn div(self, rhs: A) -> Self::Output {
        rhs.unitize()
    }
}
include!("./impls/unitize.rs");
