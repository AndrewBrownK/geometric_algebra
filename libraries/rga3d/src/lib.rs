pub mod data;
pub mod integrations;
pub mod simd;
pub mod traits;
#[allow(non_camel_case_types)]
pub mod elements {
    pub struct scalar;
    pub struct e1;
    pub struct e2;
    pub struct e3;
    pub struct e4;
    pub struct e12;
    pub struct e31;
    pub struct e41;
    pub struct e23;
    pub struct e42;
    pub struct e43;
    pub struct e321;
    pub struct e412;
    pub struct e431;
    pub struct e423;
    pub struct e1234;
}
#[test]
fn double_check_this_crate_compiles() {}
