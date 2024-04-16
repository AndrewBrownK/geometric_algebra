//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspects::*;
use crate::characteristics::*;
use crate::products::dot::{AntiDot, Dot};
use crate::*;

///
/// BulkNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait BulkNorm {
    type Output;
    fn bulk_norm(self) -> Self::Output;
}

///
/// BulkNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait BulkNormSquared {
    type Output;
    fn bulk_norm_squared(self) -> Self::Output;
}

///
/// CenterBulkNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait CenterBulkNorm {
    type Output;
    fn center_bulk_norm(self) -> Self::Output;
}

///
/// CenterBulkNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait CenterBulkNormSquared {
    type Output;
    fn center_bulk_norm_squared(self) -> Self::Output;
}

///
/// GeometricNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait GeometricNorm {
    type Output;
    fn geometric_norm(self) -> Self::Output;
}

///
/// UnitizedNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait UnitizedNorm {
    type Output;
    fn unitized_norm(self) -> Self::Output;
}

///
/// UnitizedNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait UnitizedNormSquared {
    type Output;
    fn unitized_norm_squared(self) -> Self::Output;
}

///
/// WeightNorm
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait WeightNorm {
    type Output;
    fn weight_norm(self) -> Self::Output;
}

///
/// WeightNormSquared
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
///
pub trait WeightNormSquared {
    type Output;
    fn weight_norm_squared(self) -> Self::Output;
}

impl BulkNormSquared for AntiScalar {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Circle {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for CircleBulkAspect {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Dipole {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DipoleBulkAspect {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for FlatPoint {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Flector {
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

impl BulkNormSquared for LineAtOrigin {
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

impl BulkNormSquared for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Rotor {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for RoundPoint {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for RoundPointBulkAspect {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for RoundPointCarrierAspect {
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

impl BulkNormSquared for Sphere {
    type Output = Scalar;

    fn bulk_norm_squared(self) -> Scalar {
        self.dot(self)
    }
}

impl BulkNormSquared for Transflector {
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

impl BulkNorm for AntiScalar {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Circle {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for CircleBulkAspect {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for CircleCarrierAspect {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Dipole {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DipoleBulkAspect {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for DipoleCarrierAspect {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for FlatPoint {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for FlatPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Flector {
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

impl BulkNorm for LineAtOrigin {
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

impl BulkNorm for PlaneAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Rotor {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for RoundPoint {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for RoundPointAtInfinity {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for RoundPointAtOrigin {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for RoundPointBulkAspect {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for RoundPointCarrierAspect {
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

impl BulkNorm for Sphere {
    type Output = Scalar;

    fn bulk_norm(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl BulkNorm for Transflector {
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

impl WeightNormSquared for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Circle {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for CircleBulkAspect {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for CircleCarrierAspect {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Dipole {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DipoleBulkAspect {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for FlatPoint {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for FlatPointAtOrigin {
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

impl WeightNormSquared for Rotor {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for RoundPoint {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Scalar {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Sphere {
    type Output = AntiScalar;

    fn weight_norm_squared(self) -> AntiScalar {
        self.anti_dot(self)
    }
}

impl WeightNormSquared for Transflector {
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

impl WeightNorm for AntiScalar {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Circle {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for CircleBulkAspect {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for CircleCarrierAspect {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Dipole {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for DipoleBulkAspect {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for DipoleCarrierAspect {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for FlatPoint {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for FlatPointAtOrigin {
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

impl WeightNorm for Rotor {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for RoundPoint {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for RoundPointAtInfinity {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for RoundPointAtOrigin {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for RoundPointBulkAspect {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for RoundPointCarrierAspect {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Scalar {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Sphere {
    type Output = AntiScalar;

    fn weight_norm(self) -> AntiScalar {
        self.anti_dot(self).sqrt()
    }
}

impl WeightNorm for Transflector {
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

impl GeometricNorm for AntiScalar {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Circle {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for CircleBulkAspect {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for CircleCarrierAspect {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Dipole {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DipoleBulkAspect {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for DipoleCarrierAspect {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for FlatPoint {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for FlatPointAtOrigin {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
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

impl GeometricNorm for LineAtOrigin {
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

impl GeometricNorm for PlaneAtOrigin {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Rotor {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for RoundPoint {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for RoundPointAtInfinity {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for RoundPointAtOrigin {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for RoundPointBulkAspect {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for RoundPointCarrierAspect {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Scalar {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Sphere {
    type Output = Magnitude;

    fn geometric_norm(self) -> Magnitude {
        self.bulk_norm().add(self.weight_norm())
    }
}

impl GeometricNorm for Transflector {
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

impl UnitizedNormSquared for AntiScalar {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Circle {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for CircleBulkAspect {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for CircleCarrierAspect {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Dipole {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DipoleBulkAspect {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for DipoleCarrierAspect {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for FlatPoint {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for FlatPointAtOrigin {
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

impl UnitizedNormSquared for LineAtOrigin {
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

impl UnitizedNormSquared for PlaneAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Rotor {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for RoundPoint {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for RoundPointAtInfinity {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for RoundPointAtOrigin {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for RoundPointBulkAspect {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for RoundPointCarrierAspect {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Scalar {
    type Output = f32;

    fn unitized_norm_squared(self) -> f32 {
        self.bulk_norm_squared().group0() / self.weight_norm_squared().group0()
    }
}

impl UnitizedNormSquared for Sphere {
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

impl UnitizedNorm for AntiScalar {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Circle {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for CircleBulkAspect {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for CircleCarrierAspect {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Dipole {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DipoleBulkAspect {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for DipoleCarrierAspect {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for FlatPoint {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for FlatPointAtOrigin {
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

impl UnitizedNorm for LineAtOrigin {
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

impl UnitizedNorm for PlaneAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Rotor {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for RoundPoint {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for RoundPointAtInfinity {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for RoundPointAtOrigin {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for RoundPointBulkAspect {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for RoundPointCarrierAspect {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Scalar {
    type Output = f32;

    fn unitized_norm(self) -> f32 {
        self.unitized_norm_squared().sqrt()
    }
}

impl UnitizedNorm for Sphere {
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

impl CenterBulkNormSquared for Circle {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: RoundPointBulkAspect = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for Dipole {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: RoundPointBulkAspect = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for RoundPoint {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: RoundPointBulkAspect = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNormSquared for Sphere {
    type Output = Scalar;

    fn center_bulk_norm_squared(self) -> Scalar {
        let mut round_bulk: RoundPointBulkAspect = self.center().round_bulk();
        round_bulk.dot(round_bulk)
    }
}

impl CenterBulkNorm for Circle {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for Dipole {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for RoundPoint {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}

impl CenterBulkNorm for Sphere {
    type Output = Scalar;

    fn center_bulk_norm(self) -> Scalar {
        self.center_bulk_norm_squared().sqrt()
    }
}
