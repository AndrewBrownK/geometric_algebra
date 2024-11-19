use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiOne
/// The anti-scalar element set to one, and all other elements set to zero.
pub trait AntiOne {
    fn anti_one() -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_one;
include!("./impls/anti_one.rs");
