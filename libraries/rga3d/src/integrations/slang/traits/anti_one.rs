using data::*;

/// AntiOne
/// The anti-scalar element set to one, and all other elements set to zero.
pub trait AntiOne {
    fn anti_one() -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_one;
__include ./impls/anti_one;
