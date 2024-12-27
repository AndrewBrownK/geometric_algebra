use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Wedge
/// The Wedge product (also known as "Exterior Product" or Grassmann's "Progressive Combinatorial Product") combines BasisElements into higher grade BasisElements. For example, wedge(e1, e2) = e12, and wedge(e1, e23) = e123. The Wedge product is anti-commutative, so wedge(a, b) = -wedge(b, a). A non-scalar element wedged with itself is zero. This behaves something like a union of the subscripts in the BasisElements.
pub trait Wedge<T> {
    type Output;
    fn wedge(self, other: T) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static wedge: WedgeInfix = WedgeInfix;
pub struct WedgeInfix;
pub struct WedgeInfixPartial<A>(A);
impl<A: Wedge<B>, B> std::ops::Div<B> for WedgeInfixPartial<A> {
    type Output = <A as Wedge<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.wedge(rhs)
    }
}
include!("./impls/wedge.rs");
