use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// One
/// The scalar element set to one, and all other elements set to zero.
pub trait One {
    fn one() -> Self;
}
include!("./impls/one.rs");
