use crate::data::*;
use crate::simd::*;

/// Unit
/// All elements set to one.
pub trait Unit {
    fn unit() -> Self;
}
#[allow(non_camel_case_types)]
pub struct unit;
include!("./impls/unit.rs");
