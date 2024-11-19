use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Unit
/// All elements set to one.
pub trait Unit {
    fn unit() -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct unit;
include!("./impls/unit.rs");
