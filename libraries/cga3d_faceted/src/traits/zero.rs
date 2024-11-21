use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Zero
/// All elements set to zero.
pub trait Zero {
    fn zero() -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct zero;
include!("./impls/zero.rs");
