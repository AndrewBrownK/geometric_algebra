//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspects::{Bulk, RoundBulk, RoundWeight, Weight};
use crate::involutions::*;
use crate::*;
use projective_ga::{simd::*, *};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// BulkDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait BulkDual {
    type Output;
    fn bulk_dual(self) -> Self::Output;
}

/// WeightDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait WeightDual {
    type Output;
    fn weight_dual(self) -> Self::Output;
}

/// RoundBulkDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait RoundBulkDual {
    type Output;
    fn round_bulk_dual(self) -> Self::Output;
}

/// RoundWeightDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait RoundWeightDual {
    type Output;
    fn round_weight_dual(self) -> Self::Output;
}

impl BulkDual for Circle {
    type Output = DipoleWeightAspect;

    fn bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for Dipole {
    type Output = CircleWeightAspect;

    fn bulk_dual(self) -> CircleWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for FlatPoint {
    type Output = CircleWeightAspect;

    fn bulk_dual(self) -> CircleWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for FlatPointAtInfinity {
    type Output = CircleWeightAspect;

    fn bulk_dual(self) -> CircleWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for Flector {
    type Output = MultiVector;

    fn bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl BulkDual for FlectorAtInfinity {
    type Output = MultiVector;

    fn bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl BulkDual for Horizon {
    type Output = Origin;

    fn bulk_dual(self) -> Origin {
        self.bulk().complement()
    }
}

impl BulkDual for Infinity {
    type Output = SphereWeightAspect;

    fn bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for Line {
    type Output = DipoleWeightAspect;

    fn bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for LineAtInfinity {
    type Output = DipoleWeightAspect;

    fn bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for Motor {
    type Output = DipoleWeightAspect;

    fn bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for MultiVector {
    type Output = MultiVector;

    fn bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl BulkDual for Plane {
    type Output = Origin;

    fn bulk_dual(self) -> Origin {
        self.bulk().complement()
    }
}

impl BulkDual for RoundPoint {
    type Output = SphereWeightAspect;

    fn bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for RoundPointAtInfinity {
    type Output = SphereWeightAspect;

    fn bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for RoundPointAtOrigin {
    type Output = SphereWeightAspect;

    fn bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl BulkDual for Sphere {
    type Output = Origin;

    fn bulk_dual(self) -> Origin {
        self.bulk().complement()
    }
}

impl BulkDual for Transflector {
    type Output = MultiVector;

    fn bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl BulkDual for Translator {
    type Output = DipoleWeightAspect;

    fn bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Circle {
    type Output = DipoleWeightAspect;

    fn round_bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Dipole {
    type Output = CircleWeightAspect;

    fn round_bulk_dual(self) -> CircleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for FlatPoint {
    type Output = CircleWeightAspect;

    fn round_bulk_dual(self) -> CircleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for FlatPointAtInfinity {
    type Output = CircleWeightAspect;

    fn round_bulk_dual(self) -> CircleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Flector {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl RoundBulkDual for FlectorAtInfinity {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Horizon {
    type Output = Origin;

    fn round_bulk_dual(self) -> Origin {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Infinity {
    type Output = SphereWeightAspect;

    fn round_bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Line {
    type Output = DipoleWeightAspect;

    fn round_bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for LineAtInfinity {
    type Output = DipoleWeightAspect;

    fn round_bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Motor {
    type Output = DipoleWeightAspect;

    fn round_bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for MultiVector {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Plane {
    type Output = Origin;

    fn round_bulk_dual(self) -> Origin {
        self.bulk().complement()
    }
}

impl RoundBulkDual for RoundPoint {
    type Output = SphereWeightAspect;

    fn round_bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for RoundPointAtInfinity {
    type Output = SphereWeightAspect;

    fn round_bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for RoundPointAtOrigin {
    type Output = SphereWeightAspect;

    fn round_bulk_dual(self) -> SphereWeightAspect {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Sphere {
    type Output = Origin;

    fn round_bulk_dual(self) -> Origin {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Transflector {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Translator {
    type Output = DipoleWeightAspect;

    fn round_bulk_dual(self) -> DipoleWeightAspect {
        self.bulk().complement()
    }
}

impl RoundWeightDual for AntiScalar {
    type Output = Scalar;

    fn round_weight_dual(self) -> Scalar {
        self.weight().complement()
    }
}

impl RoundWeightDual for Circle {
    type Output = DipoleBulkAspect;

    fn round_weight_dual(self) -> DipoleBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for Dipole {
    type Output = CircleBulkAspect;

    fn round_weight_dual(self) -> CircleBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for FlatPoint {
    type Output = CircleBulkAspect;

    fn round_weight_dual(self) -> CircleBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for FlatPointAtOrigin {
    type Output = CircleBulkAspect;

    fn round_weight_dual(self) -> CircleBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for Flector {
    type Output = MultiVector;

    fn round_weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl RoundWeightDual for Line {
    type Output = DipoleBulkAspect;

    fn round_weight_dual(self) -> DipoleBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for LineAtOrigin {
    type Output = DipoleBulkAspect;

    fn round_weight_dual(self) -> DipoleBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for Magnitude {
    type Output = Scalar;

    fn round_weight_dual(self) -> Scalar {
        self.weight().complement()
    }
}

impl RoundWeightDual for Motor {
    type Output = MultiVector;

    fn round_weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl RoundWeightDual for MultiVector {
    type Output = MultiVector;

    fn round_weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl RoundWeightDual for Plane {
    type Output = RoundPointBulkAspect;

    fn round_weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for PlaneAtOrigin {
    type Output = RoundPointBulkAspect;

    fn round_weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for Rotor {
    type Output = MultiVector;

    fn round_weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl RoundWeightDual for Sphere {
    type Output = RoundPointBulkAspect;

    fn round_weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for Transflector {
    type Output = RoundPointBulkAspect;

    fn round_weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl RoundWeightDual for Translator {
    type Output = Scalar;

    fn round_weight_dual(self) -> Scalar {
        self.weight().complement()
    }
}

impl WeightDual for AntiScalar {
    type Output = Scalar;

    fn weight_dual(self) -> Scalar {
        self.weight().complement()
    }
}

impl WeightDual for Circle {
    type Output = DipoleBulkAspect;

    fn weight_dual(self) -> DipoleBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for Dipole {
    type Output = CircleBulkAspect;

    fn weight_dual(self) -> CircleBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for FlatPoint {
    type Output = CircleBulkAspect;

    fn weight_dual(self) -> CircleBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for FlatPointAtOrigin {
    type Output = CircleBulkAspect;

    fn weight_dual(self) -> CircleBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for Flector {
    type Output = MultiVector;

    fn weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl WeightDual for Line {
    type Output = DipoleBulkAspect;

    fn weight_dual(self) -> DipoleBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for LineAtOrigin {
    type Output = DipoleBulkAspect;

    fn weight_dual(self) -> DipoleBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for Magnitude {
    type Output = Scalar;

    fn weight_dual(self) -> Scalar {
        self.weight().complement()
    }
}

impl WeightDual for Motor {
    type Output = MultiVector;

    fn weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl WeightDual for MultiVector {
    type Output = MultiVector;

    fn weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl WeightDual for Plane {
    type Output = RoundPointBulkAspect;

    fn weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for PlaneAtOrigin {
    type Output = RoundPointBulkAspect;

    fn weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for Rotor {
    type Output = MultiVector;

    fn weight_dual(self) -> MultiVector {
        self.weight().complement()
    }
}

impl WeightDual for Sphere {
    type Output = RoundPointBulkAspect;

    fn weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for Transflector {
    type Output = RoundPointBulkAspect;

    fn weight_dual(self) -> RoundPointBulkAspect {
        self.weight().complement()
    }
}

impl WeightDual for Translator {
    type Output = Scalar;

    fn weight_dual(self) -> Scalar {
        self.weight().complement()
    }
}
