using data::*;

/// One
/// The scalar element set to one, and all other elements set to zero.
pub trait One {
    fn one() -> Self;
}
#[allow(non_camel_case_types, dead_code)]
pub struct one;
__include ./impls/one;
