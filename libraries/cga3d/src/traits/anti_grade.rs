use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiGrade
/// The AntiGrade can be described as the missing Grade with respect to an AntiScalar. This trait only characterizes uniform anti-grade multivectors.
pub trait AntiGrade {
    fn anti_grade() -> usize;
}
include!("./impls/anti_grade.rs");
