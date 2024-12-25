using data::*;

/// AntiSupport
/// The anti-support is the anti-vector furthest from the origin that encloses the object.
pub trait AntiSupport {
    type Output;
    fn anti_support(self) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_support;
impl<A: AntiSupport> std::ops::Div<A> for anti_support {
    type Output = <A as AntiSupport>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.anti_support()
    }
}
__include ./impls/anti_support;
