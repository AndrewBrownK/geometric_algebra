use std::collections::BTreeMap;
use std::sync::Arc;

use crate::algebra::MultiVectorClass;
use crate::ast2::basis::BasisSignature;
use crate::ast2::datatype::{DataType, Float, FloatVec, Integer};
use crate::ast2::RawVariableInvocation;

pub trait ExpressionOf<'vars, DT>: PartialEq + Into<AnyExpression>  {
    fn get_datatype(&self) -> DataType;
    fn ty(&self) -> DT;
}












#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub struct ConstInteger(u32);

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum ConstFloat {
    One, Zero, NegOne
}
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub struct ConstVec2(ConstFloat, ConstFloat);
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub struct ConstVec3(ConstFloat, ConstFloat, ConstFloat);
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub struct ConstVec4(ConstFloat, ConstFloat, ConstFloat, ConstFloat);
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct ConstVecN(Vec<ConstFloat>);

#[derive(PartialEq, Eq, Clone, Debug)]
struct AccessBasis<Expr: ExpressionOf<Arc<MultiVectorClass>>>(Expr, BasisSignature, );
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
struct RawAccessBasis(AnyExpression, BasisSignature);
#[derive(PartialEq, Eq, Clone, Debug)]
struct AccessBasisFlat<Expr: ExpressionOf<Arc<MultiVectorClass>>>(Expr, u8);
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
struct RawAccessBasisFlat(AnyExpression, u8);
#[derive(PartialEq, Eq, Clone, Debug)]
struct AccessBasisGroup<Expr: ExpressionOf<Arc<MultiVectorClass>>>(Expr, u8);
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
struct RawAccessBasisGroup(AnyExpression, u8);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SumOfProductsFloat<Expr1: ExpressionOf<Float>, Expr2: ExpressionOf<Float>, > {
    values: BTreeMap<(Expr1, Expr2), ConstFloat>
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RawSumOfProductsFloat {
    values: BTreeMap<(AnyExpression, AnyExpression), ConstFloat>
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
struct ConstructRaw {
    multi_vector_class: Arc<MultiVectorClass>,
    values: BTreeMap<BasisSignature, AnyExpression>
}
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
struct ConstructGrouped {
    multi_vector_class: Arc<MultiVectorClass>,
    values: Vec<AnyExpression>
}
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum SomeMVC {
    ConstructedRaw(ConstructRaw),
    ConstructedGrouped(ConstructGrouped),
    Var(RawVariableInvocation),
}


#[derive(Clone, Debug, PartialEq, Eq)]
struct SumOfProductsSimd<Expr1: ExpressionOf<FloatVec>, Expr2: ExpressionOf<FloatVec>, > {
    values: BTreeMap<(Expr1, Expr2), ConstVecN>
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RawSumOfProductsSimd {
    values: BTreeMap<(AnyExpression, AnyExpression), ConstVecN>
}
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum TraitInv {
    TraitInv10(String, Arc<MultiVectorClass>, DataType),
    TraitInv11(String, SomeMVC, DataType),
    TraitInv21(String, SomeMVC, Arc<MultiVectorClass>, DataType),
    TraitInv22(String, SomeMVC, SomeMVC, DataType),
}

// TODO new strategy...
//  You might notice the pattern that there is basically one variant per data type
//  but that is not about expressions, that is about data types
//  So maybe I should organize this differently, and ExpressionOf should be slightly different too
//  Maybe I should have a "variety" of structs (that consolidate into an enum) that are of the
//  nature "expression of type", but then inside each of those is an associated "expression by means"
//  which is all of the different ways you can get an expression of that type.
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum AnyExpression {
    CI(ConstInteger),
    CF(ConstFloat),
    CV2(ConstVec2),
    CV3(ConstVec3),
    CV4(ConstVec4),
    CVN(ConstVecN),
    MVC(SomeMVC),
    TI(TraitInv),
    AB(RawAccessBasis),
    ABF(RawAccessBasisFlat),
    ABG(RawAccessBasisGroup),
    SOPF(RawSumOfProductsFloat),
    SOPG(RawSumOfProductsSimd),
}











impl<'vars> ExpressionOf<'vars, Integer> for ConstInteger {
    fn get_datatype(&self) -> DataType {
        DataType::Integer
    }

    fn ty(&self) -> Integer {
        Integer
    }
}
impl<'vars> ExpressionOf<'vars, Float> for ConstFloat {
    fn get_datatype(&self) -> DataType {
        DataType::Float
    }

    fn ty(&self) -> Float {
        Float
    }
}
impl<'vars> ExpressionOf<'vars, FloatVec> for ConstVec2 {
    fn get_datatype(&self) -> DataType {
        DataType::Simd(FloatVec::Vec2)
    }

    fn ty(&self) -> FloatVec {
        FloatVec::Vec2
    }
}
impl<'vars> ExpressionOf<'vars, FloatVec> for ConstVec3 {
    fn get_datatype(&self) -> DataType {
        DataType::Simd(FloatVec::Vec3)
    }

    fn ty(&self) -> FloatVec {
        FloatVec::Vec3
    }
}
impl<'vars> ExpressionOf<'vars, FloatVec> for ConstVec4 {
    fn get_datatype(&self) -> DataType {
        DataType::Simd(FloatVec::Vec4)
    }

    fn ty(&self) -> FloatVec {
        FloatVec::Vec4
    }
}

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

impl<Expr: ExpressionOf<Arc<MultiVectorClass>>> From<AccessBasis<Expr>> for RawAccessBasis {
    fn from(value: AccessBasis<Expr>) -> Self {
        RawAccessBasis(
            value.0.into(),
            value.1
        )
    }
}
impl<'vars, Expr: ExpressionOf<Arc<MultiVectorClass>>> ExpressionOf<Float> for AccessBasis<Expr> {
    fn get_datatype(&self) -> DataType {
        DataType::Float
    }

    fn ty(&self) -> Float {
        Float
    }
}




impl<Expr: ExpressionOf<Arc<MultiVectorClass>>> From<AccessBasisFlat<Expr>> for RawAccessBasisFlat {
    fn from(value: AccessBasisFlat<Expr>) -> Self {
        RawAccessBasisFlat(
            value.0.into(),
            value.1
        )
    }
}




impl<Expr: ExpressionOf<Arc<MultiVectorClass>>> From<AccessBasisGroup<Expr>> for RawAccessBasisGroup {
    fn from(value: AccessBasisGroup<Expr>) -> Self {
        RawAccessBasisGroup(
            value.0.into(),
            value.1
        )
    }
}





impl<'vars, Expr1, Expr2> ExpressionOf<'vars, Float> for SumOfProductsFloat<Expr1, Expr2> where
    Expr1: PartialEq,
    Expr2: PartialEq {
    fn get_datatype(&self) -> DataType {
        DataType::Float
    }

    fn ty(&self) -> Float {
        Float
    }
}
impl<Expr1, Expr2> From<SumOfProductsFloat<Expr1, Expr2>> for RawSumOfProductsFloat {
    fn from(value: SumOfProductsFloat<Expr1, Expr2>) -> Self {
        let mut values = BTreeMap::new();
        for ((e1, e2), c) in value.values {
            values.insert((e1.into(), e2.into()), c);
        }
        RawSumOfProductsFloat { values }
    }
}







impl<'vars,
    Expr1: ExpressionOf<FloatVec>,
    Expr2: ExpressionOf<FloatVec>,
> ExpressionOf<'vars, FloatVec> for SumOfProductsSimd<Expr1, Expr2> {
    fn get_datatype(&self) -> DataType {
        todo!();
    }

    fn ty(&self) -> FloatVec {
        todo!()
    }
}
impl<Expr1, Expr2> From<SumOfProductsSimd<Expr1, Expr2>> for RawSumOfProductsSimd {
    fn from(value: SumOfProductsSimd<Expr1, Expr2>) -> Self {
        let mut values = BTreeMap::new();
        for ((e1, e2), c) in value.values {
            values.insert((e1.into(), e2.into()), c);
        }
        RawSumOfProductsSimd { values }
    }
}










impl<'vars> ExpressionOf<'vars, Arc<MultiVectorClass>> for ConstructRaw {
    fn get_datatype(&self) -> DataType {
        DataType::MultiVector(self.multi_vector_class.clone())
    }

    fn ty(&self) -> Arc<MultiVectorClass> {
        self.multi_vector_class.clone()
    }
}

impl<'vars> ExpressionOf<'vars, Arc<MultiVectorClass>> for ConstructGrouped {
    fn get_datatype(&self) -> DataType {
        DataType::MultiVector(self.multi_vector_class.clone())
    }

    fn ty(&self) -> Arc<MultiVectorClass> {
        self.multi_vector_class.clone()
    }
}


impl<'vars> ExpressionOf<'vars, DataType> for SomeMVC {
    fn get_datatype(&self) -> DataType {
        todo!()
    }

    fn ty(&self) -> DataType {
        todo!()
    }
}


impl<'vars> ExpressionOf<'vars, DataType> for TraitInv {
    fn get_datatype(&self) -> DataType {
        match self {
            TraitInv::TraitInv10(_, _, dt) => { dt.clone() }
            TraitInv::TraitInv11(_, _, dt) => { dt.clone() }
            TraitInv::TraitInv21(_, _, _, dt) => { dt.clone() }
            TraitInv::TraitInv22(_, _, _, dt) => { dt.clone() }
        }
    }

    fn ty(&self) -> DataType {
        self.get_datatype()
    }
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
            AnyExpression::MVC(m) => { m.get_datatype() }
            AnyExpression::TI(ti) => { ti.get_datatype() }
            AnyExpression::AB(ab) => { DataType::Float }
            AnyExpression::ABF(abf) => { DataType::Float }
            // TODO I might need to split this up per simd size
            AnyExpression::ABG(abg) => { DataType::Simd(FloatVec::Just1Float) }
            AnyExpression::SOPF(sopf) => { DataType::Float }
            // TODO I might need to split this up per simd size
            AnyExpression::SOPG(sopg) => { DataType::Simd(FloatVec::Just1Float)}
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
impl From<SomeMVC> for AnyExpression {
    fn from(value: SomeMVC) -> Self {
        return AnyExpression::MVC(value)
    }
}
impl From<TraitInv> for AnyExpression {
    fn from(value: TraitInv) -> Self {
        return AnyExpression::TI(value)
    }
}

impl From<ConstructRaw> for SomeMVC {
    fn from(value: ConstructRaw) -> Self {
        SomeMVC::ConstructedRaw(value)
    }
}
impl From<ConstructGrouped> for SomeMVC {
    fn from(value: ConstructGrouped) -> Self {
        SomeMVC::ConstructedGrouped(value)
    }
}
impl From<RawVariableInvocation> for SomeMVC {
    fn from(value: RawVariableInvocation) -> Self {
        SomeMVC::Var(value)
    }
}

impl From<ConstructRaw> for AnyExpression {
    fn from(value: ConstructRaw) -> Self {
        let value: SomeMVC = value.into();
        return AnyExpression::MVC(value.into())
    }
}
impl From<ConstructGrouped> for AnyExpression {
    fn from(value: ConstructGrouped) -> Self {
        let value: SomeMVC = value.into();
        return AnyExpression::MVC(value.into())
    }
}
impl From<ConstructGrouped> for AnyExpression {
    fn from(value: ConstructGrouped) -> Self {
        let value: SomeMVC = value.into();
        return AnyExpression::MVC(value.into())
    }
}

impl From<RawAccessBasis> for AnyExpression {
    fn from(value: RawAccessBasis) -> Self {
        AnyExpression::AB(value)
    }
}
impl From<RawAccessBasisFlat> for AnyExpression {
    fn from(value: RawAccessBasisFlat) -> Self {
        AnyExpression::ABF(value)
    }
}
impl From<RawAccessBasisGroup> for AnyExpression {
    fn from(value: RawAccessBasisGroup) -> Self {
        AnyExpression::ABG(value)
    }
}
impl<Expr: ExpressionOf<Arc<MultiVectorClass>>> From<AccessBasis<Expr>> for AnyExpression {
    fn from(value: AccessBasis<Expr>) -> Self {
        let value = value.into();
        AnyExpression::AB(value)
    }
}
impl<Expr: ExpressionOf<Arc<MultiVectorClass>>> From<AccessBasisFlat<Expr>> for AnyExpression {
    fn from(value: AccessBasisFlat<Expr>) -> Self {
        let value = value.into();
        AnyExpression::ABF(value)
    }
}
impl<Expr: ExpressionOf<Arc<MultiVectorClass>>> From<AccessBasisGroup<Expr>> for AnyExpression {
    fn from(value: AccessBasisGroup<Expr>) -> Self {
        let value = value.into();
        AnyExpression::ABG(value)
    }
}



impl From<RawSumOfProductsFloat> for AnyExpression {
    fn from(value: RawSumOfProductsFloat) -> Self {
        AnyExpression::SOPF(value)
    }
}

impl<Expr1, Expr2> From<SumOfProductsFloat<Expr1, Expr2>> for AnyExpression {
    fn from(value: SumOfProductsFloat<Expr1, Expr2>) -> Self {
        let value = value.into();
        AnyExpression::SOPF(value)
    }
}

impl From<RawSumOfProductsSimd> for AnyExpression {
    fn from(value: RawSumOfProductsSimd) -> Self {
        AnyExpression::SOPG(value)
    }
}

impl<Expr1, Expr2> From<SumOfProductsSimd<Expr1, Expr2>> for AnyExpression {
    fn from(value: SumOfProductsSimd<Expr1, Expr2>) -> Self {
        let value = value.into();
        AnyExpression::SOPG(value)
    }
}

