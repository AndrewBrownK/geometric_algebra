use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::algebra::MultiVectorClass;
use crate::ast2::{RawVariableInvocation, VariableInvocation};
use crate::ast2::basis::BasisSignature;
use crate::ast2::datatype::{DataType, Float, FloatVec, Integer};

pub trait ExpressionOf<'vars, DT>: PartialEq + Into<AnyExpression>  {
    fn get_datatype(&self) -> DataType;
    fn ty(&self) -> DT;
}


#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ConstInteger(u32);
impl<'vars> ExpressionOf<'vars, Integer> for ConstInteger {
    fn get_datatype(&self) -> DataType {
        DataType::Integer
    }

    fn ty(&self) -> Integer {
        Integer
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ConstFloat {
    One, Zero, NegOne
}
impl<'vars> ExpressionOf<'vars, Float> for ConstFloat {
    fn get_datatype(&self) -> DataType {
        DataType::Float
    }

    fn ty(&self) -> Float {
        Float
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ConstVec2(ConstFloat, ConstFloat);
impl<'vars> ExpressionOf<'vars, FloatVec> for ConstVec2 {
    fn get_datatype(&self) -> DataType {
        DataType::Simd(FloatVec::Vec2)
    }

    fn ty(&self) -> FloatVec {
        FloatVec::Vec2
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ConstVec3(ConstFloat, ConstFloat, ConstFloat);
impl<'vars> ExpressionOf<'vars, FloatVec> for ConstVec3 {
    fn get_datatype(&self) -> DataType {
        DataType::Simd(FloatVec::Vec3)
    }

    fn ty(&self) -> FloatVec {
        FloatVec::Vec3
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ConstVec4(ConstFloat, ConstFloat, ConstFloat, ConstFloat);
impl<'vars> ExpressionOf<'vars, FloatVec> for ConstVec4 {
    fn get_datatype(&self) -> DataType {
        DataType::Simd(FloatVec::Vec4)
    }

    fn ty(&self) -> FloatVec {
        FloatVec::Vec4
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ConstVecN(Vec<ConstFloat>);
impl<'vars> ExpressionOf<'vars, FloatVec> for ConstVecN {
    fn get_datatype(&self) -> DataType {
        DataType::Simd(self.ty())
    }

    fn ty(&self) -> FloatVec {
        match self.0.len() {
            1 => FloatVec::Just1Float,
            2 => FloatVec::Vec2,
            3 => FloatVec::Vec3,
            4 => FloatVec::Vec4,
            _ => panic!("Please only use ConstVecN sized 1-4: {self:?}")
        }
    }
}
impl From<ConstVec4> for ConstVecN {
    fn from(value: ConstVec4) -> Self {
        ConstVecN(vec![value.0, value.1, value.2, value.3])
    }
}
impl From<ConstVec3> for ConstVecN {
    fn from(value: ConstVec3) -> Self {
        ConstVecN(vec![value.0, value.1, value.2])
    }
}
impl From<ConstVec2> for ConstVecN {
    fn from(value: ConstVec2) -> Self {
        ConstVecN(vec![value.0, value.1])
    }
}
impl From<ConstFloat> for ConstVecN {
    fn from(value: ConstFloat) -> Self {
        ConstVecN(vec![value])
    }
}



#[derive(Clone, Debug, PartialEq, Eq)]
struct SumOfProductsRaw<'vars> {
    phantom: PhantomData<&'vars ()>,
    values: BTreeMap<(
        // dyn ExpressionOf<'vars, Float>,
        // dyn ExpressionOf<'vars, Float>,
        (VariableInvocation<'vars, Arc<MultiVectorClass>>, BasisSignature),
        (VariableInvocation<'vars, Arc<MultiVectorClass>>, BasisSignature),
    ), ConstFloat>
}
impl<'vars> ExpressionOf<'vars, Float> for SumOfProductsRaw<'vars> {
    fn get_datatype(&self) -> DataType {
        DataType::Float
    }

    fn ty(&self) -> Float {
        Float
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct SumOfProductsGrouped {
    values: BTreeMap<(
        AnyExpression,
        AnyExpression
    ), ConstVecN>
}
impl<'vars> ExpressionOf<'vars, FloatVec> for SumOfProductsGrouped {
    fn get_datatype(&self) -> DataType {
        todo!();
    }

    fn ty(&self) -> FloatVec {
        todo!()
    }
}



#[derive(PartialEq, Eq, Clone, Debug)]
struct ConstructRaw {
    multi_vector_class: Arc<MultiVectorClass>,
    values: BTreeMap<BasisSignature, AnyExpression>
}
impl<'vars> ExpressionOf<'vars, Arc<MultiVectorClass>> for ConstructRaw {
    fn get_datatype(&self) -> DataType {
        DataType::MultiVector(self.multi_vector_class.clone())
    }

    fn ty(&self) -> Arc<MultiVectorClass> {
        self.multi_vector_class.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ConstructGrouped {
    multi_vector_class: Arc<MultiVectorClass>,
    values: Vec<AnyExpression>
}
impl<'vars> ExpressionOf<'vars, Arc<MultiVectorClass>> for ConstructGrouped {
    fn get_datatype(&self) -> DataType {
        DataType::MultiVector(self.multi_vector_class.clone())
    }

    fn ty(&self) -> Arc<MultiVectorClass> {
        self.multi_vector_class.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AnyExpression {
    CI(ConstInteger),
    CF(ConstFloat),
    CV2(ConstVec2),
    CV3(ConstVec3),
    CV4(ConstVec4),
    CVN(ConstVecN),
    ClassRaw(ConstructRaw),
    ClassGrouped(ConstructGrouped),
    SOPR(RawSumOfProductsRaw),
    SOPG(SumOfProductsGrouped),
    Var(RawVariableInvocation)
}
impl<'vars> ExpressionOf<'vars, DataType> for AnyExpression {
    fn get_datatype(&self) -> DataType {
        match self {
            AnyExpression::CI(ci) => { ci.get_datatype() }
            AnyExpression::CF(cf) => { cf.get_datatype() }
            AnyExpression::CV2(cv2) => { cv2.get_datatype() }
            AnyExpression::CV3(cv3) => { cv3.get_datatype() }
            AnyExpression::CV4(cv4) => { cv4.get_datatype() }
            AnyExpression::CVN(cvn) => { cvn.get_datatype() }
            AnyExpression::ClassRaw(cr) => { cr.get_datatype() }
            AnyExpression::ClassGrouped(cg) => { cg.get_datatype() }
            AnyExpression::SOPR(sopr) => { sopr.get_datatype() }
            AnyExpression::SOPG(sopg) => { sopg.get_datatype() }
            AnyExpression::Var(var) => { var.ty.clone() }
        }
    }

    fn ty(&self) -> DataType {
        self.get_datatype()
    }
}
impl From<ConstInteger> for AnyExpression {
    fn from(value: ConstInteger) -> Self {
        return AnyExpression::CI(value)
    }
}
impl From<ConstFloat> for AnyExpression {
    fn from(value: ConstFloat) -> Self {
        return AnyExpression::CF(value)
    }
}
impl From<ConstVec2> for AnyExpression {
    fn from(value: ConstVec2) -> Self {
        return AnyExpression::CV2(value)
    }
}
impl From<ConstVec3> for AnyExpression {
    fn from(value: ConstVec3) -> Self {
        return AnyExpression::CV3(value)
    }
}
impl From<ConstVec4> for AnyExpression {
    fn from(value: ConstVec4) -> Self {
        return AnyExpression::CV4(value)
    }
}
impl From<ConstVecN> for AnyExpression {
    fn from(value: ConstVecN) -> Self {
        return AnyExpression::CVN(value)
    }
}
impl From<ConstructRaw> for AnyExpression {
    fn from(value: ConstructRaw) -> Self {
        return AnyExpression::ClassRaw(value)
    }
}
impl From<ConstructGrouped> for AnyExpression {
    fn from(value: ConstructGrouped) -> Self {
        return AnyExpression::ClassGrouped(value)
    }
}
impl From<RawSumOfProductsRaw> for AnyExpression {
    fn from(value: RawSumOfProductsRaw) -> Self {
        return AnyExpression::SOPR(value)
    }
}
impl<'var> From<SumOfProductsRaw<'var>> for AnyExpression {
    fn from(value: SumOfProductsRaw) -> Self {
        return AnyExpression::SOPR(value.into())
    }
}
impl From<SumOfProductsGrouped> for AnyExpression {
    fn from(value: SumOfProductsGrouped) -> Self {
        return AnyExpression::SOPG(value)
    }
}





impl<'var, DT: Clone + Into<DataType>> From<VariableInvocation<'var, DT>> for AnyExpression {
    fn from(value: VariableInvocation<'var, DT>) -> Self {
        let value = value.into();
        AnyExpression::Var(value)
    }
}




//
// #[derive(PartialEq, Eq, Clone, Debug)]
// struct RawConstructRaw {
//     multi_vector_class: Arc<MultiVectorClass>,
//     values: BTreeMap<BasisSignature, RawAnyExpression>
// }
// #[derive(PartialEq, Eq, Clone, Debug)]
// struct RawConstructGrouped {
//     multi_vector_class: Arc<MultiVectorClass>,
//     values: Vec<RawAnyExpression>
// }
//
// #[derive(Clone, Debug, PartialEq, Eq)]
// struct RawSumOfProductsGrouped {
//     values: BTreeMap<(
//         RawAnyExpression,
//         RawAnyExpression,
//     ), ConstVecN>
// }
#[derive(Clone, Debug, PartialEq, Eq)]
struct RawSumOfProductsRaw {
    values: BTreeMap<(
        (RawVariableInvocation, BasisSignature),
        (RawVariableInvocation, BasisSignature),
    ), ConstFloat>
}
impl<'vars> ExpressionOf<'vars, Float> for RawSumOfProductsRaw {
    fn get_datatype(&self) -> DataType {
        DataType::Float
    }

    fn ty(&self) -> Float {
        Float
    }
}
impl<'vars> From<SumOfProductsRaw<'vars>> for RawSumOfProductsRaw {
    fn from(value: SumOfProductsRaw<'vars>) -> Self {
        let mut values = BTreeMap::new();
        for (((var_1, sig_1), (var_2, sig_2)), c) in value.values {
            values.insert(((var_1.into(), sig_1), (var_2.into(), sig_2)), c);
        }

        return RawSumOfProductsRaw {
            values
        }
    }
}

// AnyExpression but without lifetimes
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum RawAnyExpression {
//     CI(ConstInteger),
//     CF(ConstFloat),
//     CV2(ConstVec2),
//     CV3(ConstVec3),
//     CV4(ConstVec4),
//     CVN(ConstVecN),
//     ClassRaw(RawConstructRaw),
//     ClassGrouped(RawConstructGrouped),
//     SOPR(RawSumOfProductsRaw),
//     SOPG(RawSumOfProductsGrouped),
// }