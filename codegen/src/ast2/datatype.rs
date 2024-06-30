use std::sync::Arc;

use crate::algebra2::multivector::{BoxedMultiVec, BoxIt, mono_grade_groups, MultiVec, num_elements};

// TODO move these items to better locations

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Integer;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Float;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Vec2;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Vec3;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Vec4;
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct MultiVector(Arc<BoxedMultiVec>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionType {
    Int(Integer),
    Float(Float),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Class(MultiVector),
}



pub trait ClassesFromRegistry {
    fn include_class(&self, mvc: &BoxedMultiVec) -> bool;
}
pub struct NoParam;
impl ClassesFromRegistry for NoParam {
    fn include_class(&self, _: &BoxedMultiVec) -> bool {
        false
    }
}
pub struct AnyClasses;
impl ClassesFromRegistry for AnyClasses {
    fn include_class(&self, _: &BoxedMultiVec) -> bool {
        true
    }
}



/// Good for manual implementations
pub struct Specifically<const D: u8>(MultiVec<D>) where
    MultiVec<D>: BoxIt,
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized;
impl<const D: u8> ClassesFromRegistry for Specifically<D> where
    MultiVec<D>: BoxIt,
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {
    fn include_class(&self, mvc: &BoxedMultiVec) -> bool {
        mvc.adapt_eq(&self.0)
    }
}