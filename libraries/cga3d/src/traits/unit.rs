use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Unit
/// All elements set to one.
pub trait Unit {
    fn unit() -> Self;
}
include!("./impls/unit.rs");
