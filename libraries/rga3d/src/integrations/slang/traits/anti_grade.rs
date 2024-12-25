using data::*;

/// AntiGrade
/// The AntiGrade can be described as the missing Grade with respect to an AntiScalar. This trait only characterizes uniform anti-grade multivectors.
pub trait AntiGrade {
    fn anti_grade() -> uint;
}
#[allow(non_camel_case_types, dead_code)]
pub struct anti_grade;
__include ./impls/anti_grade;
