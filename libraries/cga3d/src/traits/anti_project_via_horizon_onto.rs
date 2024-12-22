use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiProjectViaHorizonOnto
/// Outward (to horizon) AntiProjection.
pub trait AntiProjectViaHorizonOnto<T> {
    type Output;
    fn anti_project_via_horizon_onto(self, other: T) -> Self::Output;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_project_via_horizon_onto;
#[allow(non_camel_case_types)]
pub struct anti_project_via_horizon_onto_partial<A>(A);
impl<A: AntiProjectViaHorizonOnto<B>, B> std::ops::Div<B> for anti_project_via_horizon_onto_partial<A> {
    type Output = <A as AntiProjectViaHorizonOnto<B>>::Output;
    fn div(self, rhs: B) -> Self::Output {
        self.0.anti_project_via_horizon_onto(rhs)
    }
}
include!("./impls/anti_project_via_horizon_onto.rs");
