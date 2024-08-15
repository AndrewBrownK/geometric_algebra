use crate::data::*;
use crate::simd::*;

/// AntiOne
/// The anti-scalar element set to one, and all other elements set to zero.
pub trait AntiOne {
    fn anti_one() -> Self;
}
#[allow(non_camel_case_types)]
pub struct anti_one;
include!("./impls/anti_one.rs");
