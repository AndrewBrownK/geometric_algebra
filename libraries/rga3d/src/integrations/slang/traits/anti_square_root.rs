using data::*;

/// AntiSquareRoot
/// Square root of geometry with respect to the AntiProduct. Multiple different types of geometry might anti-square to one type of geometry, so this is only defined for types that are closed with themselves under the geometric anti-product.
pub trait AntiSquareRoot {
    fn anti_square_root(self) -> AntiScalar;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_square_root;
impl<A: AntiSquareRoot> std::ops::Div<A> for anti_square_root {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_square_root()
    }
}
__include ./impls/anti_square_root;
