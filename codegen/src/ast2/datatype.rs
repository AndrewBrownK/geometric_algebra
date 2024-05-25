use std::sync::Arc;

use crate::algebra::MultiVectorClass;

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
    fn include_class(mvc: &MultiVectorClass) -> bool;
}
pub struct NoParam;
impl ClassesFromRegistry for NoParam {
    fn include_class(_: &MultiVectorClass) -> bool {
        false
    }
}
pub struct AnyClasses;
impl ClassesFromRegistry for AnyClasses {
    fn include_class(_: &MultiVectorClass) -> bool {
        true
    }
}