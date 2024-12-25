using data::*;

/// Zero
/// All elements set to zero.
pub trait Zero {
    fn zero() -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct zero;
__include ./impls/zero;
