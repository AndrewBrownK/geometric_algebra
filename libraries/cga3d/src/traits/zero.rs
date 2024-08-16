use crate::data::*;
use crate::simd::*;

/// Zero
/// All elements set to zero.
pub trait Zero {
    fn zero() -> Self;
}
#[allow(non_camel_case_types)]
pub struct zero;
include!("./impls/zero.rs");
