//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspects::*;
use crate::characteristics::*;
use crate::products::dot::*;
use crate::*;

/// BulkNorm
/// Note that this does not measure unitized distance unless you combine
/// it with the corresponding weight norm. You can do this by unitizing the object
/// before taking this BulkNorm, or adding the corresponding weight norm and
/// unitizing the resulting DualNum.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNorm {
    type Output;
    fn bulk_norm(self) -> Self::Output;
}

/// BulkNormSquared
/// Note that this does not measure unitized distance squared unless you combine
/// it with the corresponding weight norm. You can do this by unitizing the object
/// before taking this BulkNormSquared, or adding the corresponding weight norm and
/// unitizing the resulting DualNum.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNormSquared {
    type Output;
    fn bulk_norm_squared(self) -> Self::Output;
}

/// Norm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}

/// NormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait NormSquared {
    type Output;
    fn norm_squared(self) -> Self::Output;
}

/// UnitizedNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedNorm {
    type Output;
    fn unitized_norm(self) -> Self::Output;
}

/// UnitizedNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait UnitizedNormSquared {
    type Output;
    fn unitized_norm_squared(self) -> Self::Output;
}

/// WeightNorm
/// Note that this does not provide a unitized orientation unless your object
/// is unitized first. Sometimes you want the weight norm before unitization
/// so you can perform unitization later.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNorm {
    type Output;
    fn weight_norm(self) -> Self::Output;
}

/// WeightNormSquared
/// Note that this does not provide a unitized orientation unless your object
/// is unitized first. Sometimes you want the weight norm before unitization
/// so you can perform unitization later.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNormSquared {
    type Output;
    fn weight_norm_squared(self) -> Self::Output;
}

impl BulkNormSquared for DualNum {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Scalar = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Flector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: FlectorAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for FlectorAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: FlectorAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Horizon {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Horizon = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Line {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: LineAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for LineAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: LineAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Motor {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: LineAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for MultiVector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: MultiVectorAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for MultiVectorAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: MultiVectorAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Plane {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Horizon = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Point {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: PointAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for PointAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: PointAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Scalar {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: Scalar = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Transflector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: FlectorAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNormSquared for Translator {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        let mut flat_bulk_thing: LineAtInfinity = self.bulk();
        flat_bulk_thing.dot(flat_bulk_thing)
    }
}

impl BulkNorm for DualNum {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Flector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for FlectorAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Horizon {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Line {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for LineAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Motor {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for MultiVector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for MultiVectorAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Plane {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Point {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for PointAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Scalar {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Transflector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl BulkNorm for Translator {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.bulk_norm_squared().sqrt()
    }
}

impl WeightNormSquared for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: AntiScalar = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for DualNum {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: AntiScalar = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Flector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Flector = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Line {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: LineAtOrigin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: LineAtOrigin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Motor {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Rotor = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for MultiVector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: MultiVectorAtOrigin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for MultiVectorAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: MultiVectorAtOrigin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Origin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Origin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Plane {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: PlaneAtOrigin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: PlaneAtOrigin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Point {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Origin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Rotor {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: Rotor = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Transflector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: PlaneAtOrigin = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNormSquared for Translator {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        let mut flat_weight: AntiScalar = self.weight();
        flat_weight.anti_dot(flat_weight)
    }
}

impl WeightNorm for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for DualNum {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Flector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Line {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Motor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for MultiVector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for MultiVectorAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Origin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Plane {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Point {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Rotor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Transflector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl WeightNorm for Translator {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.weight_norm_squared().anti_sqrt()
    }
}

impl NormSquared for DualNum {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for Flector {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for Line {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for Motor {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for MultiVector {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for Plane {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for Point {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for Transflector {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl NormSquared for Translator {
    type Output = DualNum;

    fn norm_squared(self) -> DualNum {
        self.bulk_norm_squared().add(self.weight_norm_squared())
    }
}

impl Norm for DualNum {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for Flector {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for Line {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for Motor {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for MultiVector {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for Plane {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for Point {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for Transflector {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl Norm for Translator {
    type Output = DualNum;

    fn norm(self) -> DualNum {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl UnitizedNormSquared for DualNum {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Flector {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Line {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Motor {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for MultiVector {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Plane {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Point {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Transflector {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Translator {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNorm for DualNum {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Flector {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Line {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Motor {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for MultiVector {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Plane {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Point {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Transflector {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Translator {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}
