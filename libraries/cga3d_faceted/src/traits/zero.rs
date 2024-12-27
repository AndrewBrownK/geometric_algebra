use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Zero
/// All elements set to zero.
pub trait Zero {
    fn zero() -> Self;
}
include!("./impls/zero.rs");
