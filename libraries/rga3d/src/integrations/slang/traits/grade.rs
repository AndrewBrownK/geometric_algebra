using data::*;

/// Grade
/// A multivector may have uniform grade, or mixed grade, depending on the grades of its elements. This trait only characterizes uniform grade multivectors.
pub trait Grade {
    fn grade() -> uint;
}
#[allow(non_camel_case_types, dead_code)]
pub struct grade;
__include ./impls/grade;
