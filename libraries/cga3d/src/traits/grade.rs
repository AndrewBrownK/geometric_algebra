use crate::data::*;
use crate::simd::*;

/// Grade
/// A multivector class may have uniform grade, or mixed grade, depending on the grades of its elements. This trait only characterizes uniform grade multivectors.
pub trait Grade {
    fn grade() -> usize;
}
#[allow(non_camel_case_types)]
pub struct grade;
include!("./impls/grade.rs");
