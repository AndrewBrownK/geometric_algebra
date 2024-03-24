//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::characteristics::Sqrt;
use crate::products::dot::{AntiDot, Dot};
use crate::*;

/// BulkNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNorm {
    type Output;
    fn bulk_norm(self) -> Self::Output;
}

/// BulkNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNormSquared {
    type Output;
    fn bulk_norm_squared(self) -> Self::Output;
}

/// GeometricNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait GeometricNorm {
    type Output;
    fn geometric_norm(self) -> Self::Output;
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
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNorm {
    type Output;
    fn weight_norm(self) -> Self::Output;
}

/// WeightNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNormSquared {
    type Output;
    fn weight_norm_squared(self) -> Self::Output;
}

impl BulkNorm for Flector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Horizon {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Line {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for LineAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Magnitude {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Motor {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for MultiVector {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Plane {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Point {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for PointAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Scalar {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Translator {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNormSquared for Flector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Horizon {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Line {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for LineAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Magnitude {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Motor {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for MultiVector {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Plane {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Point {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for PointAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Scalar {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Translator {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl WeightNorm for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Flector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Line {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Magnitude {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Motor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for MultiVector {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Origin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Plane {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Point {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Rotor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Translator {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNormSquared for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Flector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Line {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Magnitude {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Motor {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for MultiVector {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Origin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Plane {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Point {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Rotor {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Translator {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl GeometricNorm for Flector {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Line {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Magnitude {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Motor {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for MultiVector {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Plane {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Point {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Translator {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
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

impl UnitizedNormSquared for Magnitude {
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

impl UnitizedNormSquared for Translator {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
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

impl UnitizedNorm for Magnitude {
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

impl UnitizedNorm for Translator {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}
