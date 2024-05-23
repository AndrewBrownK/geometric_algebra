use std::sync::Arc;

use crate::algebra::MultiVectorClass;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Integer;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Float;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum FloatVec {
    Just1Float, Vec2, Vec3, Vec4
}
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum DataType {
    Integer,
    Float,
    Simd(FloatVec),
    MultiVector(Arc<MultiVectorClass>)
}

impl From<Integer> for DataType {
    fn from(_: Integer) -> Self {
        return DataType::Integer
    }
}
impl From<Float> for DataType {
    fn from(_: Float) -> Self {
        return DataType::Float
    }
}
impl From<FloatVec> for DataType {
    fn from(value: FloatVec) -> Self {
        return DataType::Simd(value)
    }
}
impl From<Arc<MultiVectorClass>> for DataType {
    fn from(value: Arc<MultiVectorClass>) -> Self {
        return DataType::MultiVector(value)
    }
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