//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspects::{FlatBulk, FlatWeight, RoundBulk, RoundWeight};
use crate::involutions::*;
use crate::*;
use projective_ga::{simd::*, *};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// FlatBulkDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait FlatBulkDual {
    type Output;
    fn flat_bulk_dual(self) -> Self::Output;
}

/// FlatWeightDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait FlatWeightDual {
    type Output;
    fn flat_weight_dual(self) -> Self::Output;
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

impl FlatBulkDual for AntiPlane {
    type Output = Horizon;

    fn flat_bulk_dual(self) -> Horizon {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Circle {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for CircleAligningOrigin {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for CircleAtInfinity {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for CircleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for CircleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Dipole {
    type Output = LineAtInfinity;

    fn flat_bulk_dual(self) -> LineAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for DipoleAligningOrigin {
    type Output = LineAtInfinity;

    fn flat_bulk_dual(self) -> LineAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for DipoleAtInfinity {
    type Output = LineAtInfinity;

    fn flat_bulk_dual(self) -> LineAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for DipoleAtOrigin {
    type Output = LineAtInfinity;

    fn flat_bulk_dual(self) -> LineAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for DipoleOrthogonalOrigin {
    type Output = LineAtInfinity;

    fn flat_bulk_dual(self) -> LineAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for FlatPoint {
    type Output = LineAtInfinity;

    fn flat_bulk_dual(self) -> LineAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for FlatPointAtInfinity {
    type Output = LineAtInfinity;

    fn flat_bulk_dual(self) -> LineAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Flector {
    type Output = MultiVector;

    fn flat_bulk_dual(self) -> MultiVector {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for FlectorAtInfinity {
    type Output = MultiVector;

    fn flat_bulk_dual(self) -> MultiVector {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Horizon {
    type Output = Infinity;

    fn flat_bulk_dual(self) -> Infinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Infinity {
    type Output = Horizon;

    fn flat_bulk_dual(self) -> Horizon {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Line {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for LineAtInfinity {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Motor {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for MultiVector {
    type Output = MultiVector;

    fn flat_bulk_dual(self) -> MultiVector {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Plane {
    type Output = Infinity;

    fn flat_bulk_dual(self) -> Infinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for RoundPoint {
    type Output = Horizon;

    fn flat_bulk_dual(self) -> Horizon {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for RoundPointAtOrigin {
    type Output = Horizon;

    fn flat_bulk_dual(self) -> Horizon {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Sphere {
    type Output = Infinity;

    fn flat_bulk_dual(self) -> Infinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for SphereAtOrigin {
    type Output = Infinity;

    fn flat_bulk_dual(self) -> Infinity {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Transflector {
    type Output = MultiVector;

    fn flat_bulk_dual(self) -> MultiVector {
        self.flat_bulk().dual()
    }
}

impl FlatBulkDual for Translator {
    type Output = FlatPointAtInfinity;

    fn flat_bulk_dual(self) -> FlatPointAtInfinity {
        self.flat_bulk().dual()
    }
}

impl FlatWeightDual for AntiScalar {
    type Output = Scalar;

    fn flat_weight_dual(self) -> Scalar {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Circle {
    type Output = AntiLineAtOrigin;

    fn flat_weight_dual(self) -> AntiLineAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for CircleAligningOrigin {
    type Output = AntiLineAtOrigin;

    fn flat_weight_dual(self) -> AntiLineAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for CircleAtInfinity {
    type Output = AntiLineAtOrigin;

    fn flat_weight_dual(self) -> AntiLineAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for CircleOnOrigin {
    type Output = AntiLineAtOrigin;

    fn flat_weight_dual(self) -> AntiLineAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Dipole {
    type Output = AntiFlatPointAtOrigin;

    fn flat_weight_dual(self) -> AntiFlatPointAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for DipoleAligningOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn flat_weight_dual(self) -> AntiFlatPointAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for DipoleAtInfinity {
    type Output = AntiFlatPointAtOrigin;

    fn flat_weight_dual(self) -> AntiFlatPointAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for DipoleOnOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn flat_weight_dual(self) -> AntiFlatPointAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for DualNum {
    type Output = Scalar;

    fn flat_weight_dual(self) -> Scalar {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for FlatPoint {
    type Output = AntiFlatPointAtOrigin;

    fn flat_weight_dual(self) -> AntiFlatPointAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for FlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn flat_weight_dual(self) -> AntiFlatPointAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Flector {
    type Output = MultiVector;

    fn flat_weight_dual(self) -> MultiVector {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Line {
    type Output = AntiLineAtOrigin;

    fn flat_weight_dual(self) -> AntiLineAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for LineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn flat_weight_dual(self) -> AntiLineAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Motor {
    type Output = MultiVector;

    fn flat_weight_dual(self) -> MultiVector {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for MultiVector {
    type Output = MultiVector;

    fn flat_weight_dual(self) -> MultiVector {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Plane {
    type Output = AntiPlaneAtOrigin;

    fn flat_weight_dual(self) -> AntiPlaneAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn flat_weight_dual(self) -> AntiPlaneAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Rotor {
    type Output = MultiVector;

    fn flat_weight_dual(self) -> MultiVector {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Sphere {
    type Output = AntiPlaneAtOrigin;

    fn flat_weight_dual(self) -> AntiPlaneAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for SphereOnOrigin {
    type Output = AntiPlaneAtOrigin;

    fn flat_weight_dual(self) -> AntiPlaneAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Transflector {
    type Output = AntiPlaneAtOrigin;

    fn flat_weight_dual(self) -> AntiPlaneAtOrigin {
        self.flat_weight().dual()
    }
}

impl FlatWeightDual for Translator {
    type Output = Scalar;

    fn flat_weight_dual(self) -> Scalar {
        self.flat_weight().dual()
    }
}

impl RoundBulkDual for AntiCircleOnOrigin {
    type Output = LineAtOrigin;

    fn round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for AntiDipoleOnOrigin {
    type Output = FlatPointAtOrigin;

    fn round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for AntiFlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for AntiLineAtOrigin {
    type Output = LineAtOrigin;

    fn round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for AntiPlane {
    type Output = PlaneAtOrigin;

    fn round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for AntiSphereOnOrigin {
    type Output = PlaneAtOrigin;

    fn round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for Circle {
    type Output = FlatPointAtOrigin;

    fn round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for CircleAtInfinity {
    type Output = FlatPointAtOrigin;

    fn round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for CircleOrthogonalOrigin {
    type Output = FlatPointAtOrigin;

    fn round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for Dipole {
    type Output = LineAtOrigin;

    fn round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for DipoleAtInfinity {
    type Output = LineAtOrigin;

    fn round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for DipoleOrthogonalOrigin {
    type Output = LineAtOrigin;

    fn round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for DualNum {
    type Output = AntiScalar;

    fn round_bulk_dual(self) -> AntiScalar {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for MultiVector {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for RoundPoint {
    type Output = PlaneAtOrigin;

    fn round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().dual()
    }
}

impl RoundBulkDual for Scalar {
    type Output = AntiScalar;

    fn round_bulk_dual(self) -> AntiScalar {
        self.round_bulk().dual()
    }
}

impl RoundWeightDual for AntiCircleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight_dual(self) -> NullCircleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for AntiDipoleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight_dual(self) -> NullDipoleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for AntiSphereOnOrigin {
    type Output = NullSphereAtOrigin;

    fn round_weight_dual(self) -> NullSphereAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for Circle {
    type Output = NullDipoleAtOrigin;

    fn round_weight_dual(self) -> NullDipoleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for CircleAligningOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight_dual(self) -> NullDipoleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for CircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight_dual(self) -> NullDipoleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for CircleOnOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight_dual(self) -> NullDipoleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for CircleOrthogonalOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight_dual(self) -> NullDipoleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for Dipole {
    type Output = NullCircleAtOrigin;

    fn round_weight_dual(self) -> NullCircleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for DipoleAligningOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight_dual(self) -> NullCircleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for DipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight_dual(self) -> NullCircleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for DipoleOnOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight_dual(self) -> NullCircleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for DipoleOrthogonalOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight_dual(self) -> NullCircleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for MultiVector {
    type Output = MultiVector;

    fn round_weight_dual(self) -> MultiVector {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn round_weight_dual(self) -> NullDipoleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn round_weight_dual(self) -> NullCircleAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for NullSphereAtOrigin {
    type Output = Origin;

    fn round_weight_dual(self) -> Origin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for Origin {
    type Output = NullSphereAtOrigin;

    fn round_weight_dual(self) -> NullSphereAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for RoundPoint {
    type Output = NullSphereAtOrigin;

    fn round_weight_dual(self) -> NullSphereAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for RoundPointAtOrigin {
    type Output = NullSphereAtOrigin;

    fn round_weight_dual(self) -> NullSphereAtOrigin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for Sphere {
    type Output = Origin;

    fn round_weight_dual(self) -> Origin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for SphereAtOrigin {
    type Output = Origin;

    fn round_weight_dual(self) -> Origin {
        self.round_weight().dual()
    }
}

impl RoundWeightDual for SphereOnOrigin {
    type Output = Origin;

    fn round_weight_dual(self) -> Origin {
        self.round_weight().dual()
    }
}
