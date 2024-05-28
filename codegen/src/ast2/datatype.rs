use std::sync::Arc;

use crate::algebra::MultiVectorClass;

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
pub struct MultiVector(Arc<MultiVectorClass>);

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
    fn include_class(&self, mvc: &MultiVectorClass) -> bool;
}
pub struct NoParam;
impl ClassesFromRegistry for NoParam {
    fn include_class(&self, _: &MultiVectorClass) -> bool {
        false
    }
}
pub struct AnyClasses;
impl ClassesFromRegistry for AnyClasses {
    fn include_class(&self, _: &MultiVectorClass) -> bool {
        true
    }
}



/// Good for manual implementations
pub struct Specifically(MultiVector);
impl ClassesFromRegistry for Specifically {
    fn include_class(&self, mvc: &MultiVectorClass) -> bool {
        mvc == self.0.0.as_ref()
    }
}