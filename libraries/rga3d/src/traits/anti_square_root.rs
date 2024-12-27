use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiSquareRoot
/// Square root of geometry with respect to the AntiProduct. Multiple different types of geometry might anti-square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric anti-product.
pub trait AntiSquareRoot {
    fn anti_square_root(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static anti_square_root: AntiSquareRootPrefixOrPostfix = AntiSquareRootPrefixOrPostfix;
pub struct AntiSquareRootPrefixOrPostfix;
impl<A: AntiSquareRoot> std::ops::Div<A> for AntiSquareRootPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_square_root()
    }
}
include!("./impls/anti_square_root.rs");
