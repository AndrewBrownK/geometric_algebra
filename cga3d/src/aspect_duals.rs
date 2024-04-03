//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::aspects::{Bulk, RoundBulk, RoundWeight, Weight};
use crate::involutions::*;
use crate::*;
use geometric_algebra::{simd::*, *};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Right Bulk Dual
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait RightBulkDual {
    type Output;
    fn right_bulk_dual(self) -> Self::Output;
}

/// Right Weight Dual
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait RightWeightDual {
    type Output;
    fn right_weight_dual(self) -> Self::Output;
}

/// Left Bulk Dual
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait LeftBulkDual {
    type Output;
    fn left_bulk_dual(self) -> Self::Output;
}

/// Left Weight Dual
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait LeftWeightDual {
    type Output;
    fn left_weight_dual(self) -> Self::Output;
}

/// Right Round Bulk Dual
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait RightRoundBulkDual {
    type Output;
    fn right_round_bulk_dual(self) -> Self::Output;
}

/// Right Round Weight Dual. Needed to implement CoCarriers.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait RightRoundWeightDual {
    type Output;
    fn right_round_weight_dual(self) -> Self::Output;
}

/// Left Round Bulk Dual
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait LeftRoundBulkDual {
    type Output;
    fn left_round_bulk_dual(self) -> Self::Output;
}

/// Left Round Weight Dual
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
/// https://projectivegeometricalgebra.org/confgeomalg.pdf
pub trait LeftRoundWeightDual {
    type Output;
    fn left_round_weight_dual(self) -> Self::Output;
}

impl LeftBulkDual for Circle {
    type Output = DipoleWeight;

    fn left_bulk_dual(self) -> DipoleWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Dipole {
    type Output = CircleWeight;

    fn left_bulk_dual(self) -> CircleWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for FlatPoint {
    type Output = CircleWeight;

    fn left_bulk_dual(self) -> CircleWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for FlatPointAtInfinity {
    type Output = CircleWeight;

    fn left_bulk_dual(self) -> CircleWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Flector {
    type Output = MultiVector;

    fn left_bulk_dual(self) -> MultiVector {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Horizon {
    type Output = Origin;

    fn left_bulk_dual(self) -> Origin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Infinity {
    type Output = SphereWeight;

    fn left_bulk_dual(self) -> SphereWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Line {
    type Output = DipoleWeight;

    fn left_bulk_dual(self) -> DipoleWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for LineAtInfinity {
    type Output = DipoleWeight;

    fn left_bulk_dual(self) -> DipoleWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Motor {
    type Output = DipoleWeight;

    fn left_bulk_dual(self) -> DipoleWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for MultiVector {
    type Output = MultiVector;

    fn left_bulk_dual(self) -> MultiVector {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Plane {
    type Output = Origin;

    fn left_bulk_dual(self) -> Origin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for RoundPoint {
    type Output = SphereWeight;

    fn left_bulk_dual(self) -> SphereWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for RoundPointAtInfinity {
    type Output = SphereWeight;

    fn left_bulk_dual(self) -> SphereWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for RoundPointAtOrigin {
    type Output = SphereWeight;

    fn left_bulk_dual(self) -> SphereWeight {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Sphere {
    type Output = Origin;

    fn left_bulk_dual(self) -> Origin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Translator {
    type Output = DipoleWeight;

    fn left_bulk_dual(self) -> DipoleWeight {
        self.bulk().left_complement()
    }
}

impl LeftRoundBulkDual for Circle {
    type Output = FlatPointAtOrigin;

    fn left_round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for CircleBulk {
    type Output = FlatPointAtOrigin;

    fn left_round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for CircleCarrierAspect {
    type Output = FlatPointAtOrigin;

    fn left_round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for Dipole {
    type Output = LineAtOrigin;

    fn left_round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for DipoleBulk {
    type Output = LineAtOrigin;

    fn left_round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for DipoleCarrierAspect {
    type Output = LineAtOrigin;

    fn left_round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for Magnitude {
    type Output = AntiScalar;

    fn left_round_bulk_dual(self) -> AntiScalar {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for MultiVector {
    type Output = MultiVector;

    fn left_round_bulk_dual(self) -> MultiVector {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for RoundPoint {
    type Output = PlaneAtOrigin;

    fn left_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for RoundPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn left_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn left_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for RoundPointCarrierAspect {
    type Output = PlaneAtOrigin;

    fn left_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundBulkDual for Scalar {
    type Output = AntiScalar;

    fn left_round_bulk_dual(self) -> AntiScalar {
        self.round_bulk().left_complement()
    }
}

impl LeftRoundWeightDual for Circle {
    type Output = FlatPointAtInfinity;

    fn left_round_weight_dual(self) -> FlatPointAtInfinity {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for CircleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn left_round_weight_dual(self) -> FlatPointAtInfinity {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for CircleWeight {
    type Output = FlatPointAtInfinity;

    fn left_round_weight_dual(self) -> FlatPointAtInfinity {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for Dipole {
    type Output = LineAtInfinity;

    fn left_round_weight_dual(self) -> LineAtInfinity {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for DipoleCarrierAspect {
    type Output = LineAtInfinity;

    fn left_round_weight_dual(self) -> LineAtInfinity {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for DipoleWeight {
    type Output = LineAtInfinity;

    fn left_round_weight_dual(self) -> LineAtInfinity {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for MultiVector {
    type Output = MultiVector;

    fn left_round_weight_dual(self) -> MultiVector {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for Origin {
    type Output = Horizon;

    fn left_round_weight_dual(self) -> Horizon {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for RoundPoint {
    type Output = Horizon;

    fn left_round_weight_dual(self) -> Horizon {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for RoundPointAtOrigin {
    type Output = Horizon;

    fn left_round_weight_dual(self) -> Horizon {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for RoundPointCarrierAspect {
    type Output = Horizon;

    fn left_round_weight_dual(self) -> Horizon {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for Sphere {
    type Output = Infinity;

    fn left_round_weight_dual(self) -> Infinity {
        self.round_weight().left_complement()
    }
}

impl LeftRoundWeightDual for SphereWeight {
    type Output = Infinity;

    fn left_round_weight_dual(self) -> Infinity {
        self.round_weight().left_complement()
    }
}

impl LeftWeightDual for AntiScalar {
    type Output = Scalar;

    fn left_weight_dual(self) -> Scalar {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Circle {
    type Output = DipoleBulk;

    fn left_weight_dual(self) -> DipoleBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Dipole {
    type Output = CircleBulk;

    fn left_weight_dual(self) -> CircleBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for FlatPoint {
    type Output = CircleBulk;

    fn left_weight_dual(self) -> CircleBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for FlatPointAtOrigin {
    type Output = CircleBulk;

    fn left_weight_dual(self) -> CircleBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Flector {
    type Output = MultiVector;

    fn left_weight_dual(self) -> MultiVector {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Line {
    type Output = DipoleBulk;

    fn left_weight_dual(self) -> DipoleBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for LineAtOrigin {
    type Output = DipoleBulk;

    fn left_weight_dual(self) -> DipoleBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Magnitude {
    type Output = Scalar;

    fn left_weight_dual(self) -> Scalar {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Motor {
    type Output = MultiVector;

    fn left_weight_dual(self) -> MultiVector {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for MultiVector {
    type Output = MultiVector;

    fn left_weight_dual(self) -> MultiVector {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Plane {
    type Output = RoundPointBulk;

    fn left_weight_dual(self) -> RoundPointBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn left_weight_dual(self) -> RoundPointBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Rotor {
    type Output = MultiVector;

    fn left_weight_dual(self) -> MultiVector {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Sphere {
    type Output = RoundPointBulk;

    fn left_weight_dual(self) -> RoundPointBulk {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Translator {
    type Output = Scalar;

    fn left_weight_dual(self) -> Scalar {
        self.weight().left_complement()
    }
}

impl RightBulkDual for Circle {
    type Output = DipoleWeight;

    fn right_bulk_dual(self) -> DipoleWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Dipole {
    type Output = CircleWeight;

    fn right_bulk_dual(self) -> CircleWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for FlatPoint {
    type Output = CircleWeight;

    fn right_bulk_dual(self) -> CircleWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for FlatPointAtInfinity {
    type Output = CircleWeight;

    fn right_bulk_dual(self) -> CircleWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Flector {
    type Output = MultiVector;

    fn right_bulk_dual(self) -> MultiVector {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Horizon {
    type Output = Origin;

    fn right_bulk_dual(self) -> Origin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Infinity {
    type Output = SphereWeight;

    fn right_bulk_dual(self) -> SphereWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Line {
    type Output = DipoleWeight;

    fn right_bulk_dual(self) -> DipoleWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for LineAtInfinity {
    type Output = DipoleWeight;

    fn right_bulk_dual(self) -> DipoleWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Motor {
    type Output = DipoleWeight;

    fn right_bulk_dual(self) -> DipoleWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for MultiVector {
    type Output = MultiVector;

    fn right_bulk_dual(self) -> MultiVector {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Plane {
    type Output = Origin;

    fn right_bulk_dual(self) -> Origin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for RoundPoint {
    type Output = SphereWeight;

    fn right_bulk_dual(self) -> SphereWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for RoundPointAtInfinity {
    type Output = SphereWeight;

    fn right_bulk_dual(self) -> SphereWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for RoundPointAtOrigin {
    type Output = SphereWeight;

    fn right_bulk_dual(self) -> SphereWeight {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Sphere {
    type Output = Origin;

    fn right_bulk_dual(self) -> Origin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Translator {
    type Output = DipoleWeight;

    fn right_bulk_dual(self) -> DipoleWeight {
        self.bulk().right_complement()
    }
}

impl RightRoundBulkDual for Circle {
    type Output = FlatPointAtOrigin;

    fn right_round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for CircleBulk {
    type Output = FlatPointAtOrigin;

    fn right_round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for CircleCarrierAspect {
    type Output = FlatPointAtOrigin;

    fn right_round_bulk_dual(self) -> FlatPointAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for Dipole {
    type Output = LineAtOrigin;

    fn right_round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for DipoleBulk {
    type Output = LineAtOrigin;

    fn right_round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for DipoleCarrierAspect {
    type Output = LineAtOrigin;

    fn right_round_bulk_dual(self) -> LineAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for Magnitude {
    type Output = AntiScalar;

    fn right_round_bulk_dual(self) -> AntiScalar {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for MultiVector {
    type Output = MultiVector;

    fn right_round_bulk_dual(self) -> MultiVector {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for RoundPoint {
    type Output = PlaneAtOrigin;

    fn right_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for RoundPointAtInfinity {
    type Output = PlaneAtOrigin;

    fn right_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for RoundPointBulk {
    type Output = PlaneAtOrigin;

    fn right_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for RoundPointCarrierAspect {
    type Output = PlaneAtOrigin;

    fn right_round_bulk_dual(self) -> PlaneAtOrigin {
        self.round_bulk().right_complement()
    }
}

impl RightRoundBulkDual for Scalar {
    type Output = AntiScalar;

    fn right_round_bulk_dual(self) -> AntiScalar {
        self.round_bulk().right_complement()
    }
}

impl RightRoundWeightDual for Circle {
    type Output = FlatPointAtInfinity;

    fn right_round_weight_dual(self) -> FlatPointAtInfinity {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for CircleCarrierAspect {
    type Output = FlatPointAtInfinity;

    fn right_round_weight_dual(self) -> FlatPointAtInfinity {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for CircleWeight {
    type Output = FlatPointAtInfinity;

    fn right_round_weight_dual(self) -> FlatPointAtInfinity {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for Dipole {
    type Output = LineAtInfinity;

    fn right_round_weight_dual(self) -> LineAtInfinity {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for DipoleCarrierAspect {
    type Output = LineAtInfinity;

    fn right_round_weight_dual(self) -> LineAtInfinity {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for DipoleWeight {
    type Output = LineAtInfinity;

    fn right_round_weight_dual(self) -> LineAtInfinity {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for MultiVector {
    type Output = MultiVector;

    fn right_round_weight_dual(self) -> MultiVector {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for Origin {
    type Output = Horizon;

    fn right_round_weight_dual(self) -> Horizon {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for RoundPoint {
    type Output = Horizon;

    fn right_round_weight_dual(self) -> Horizon {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for RoundPointAtOrigin {
    type Output = Horizon;

    fn right_round_weight_dual(self) -> Horizon {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for RoundPointCarrierAspect {
    type Output = Horizon;

    fn right_round_weight_dual(self) -> Horizon {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for Sphere {
    type Output = Infinity;

    fn right_round_weight_dual(self) -> Infinity {
        self.round_weight().right_complement()
    }
}

impl RightRoundWeightDual for SphereWeight {
    type Output = Infinity;

    fn right_round_weight_dual(self) -> Infinity {
        self.round_weight().right_complement()
    }
}

impl RightWeightDual for AntiScalar {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Circle {
    type Output = DipoleBulk;

    fn right_weight_dual(self) -> DipoleBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Dipole {
    type Output = CircleBulk;

    fn right_weight_dual(self) -> CircleBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for FlatPoint {
    type Output = CircleBulk;

    fn right_weight_dual(self) -> CircleBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for FlatPointAtOrigin {
    type Output = CircleBulk;

    fn right_weight_dual(self) -> CircleBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Flector {
    type Output = MultiVector;

    fn right_weight_dual(self) -> MultiVector {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Line {
    type Output = DipoleBulk;

    fn right_weight_dual(self) -> DipoleBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for LineAtOrigin {
    type Output = DipoleBulk;

    fn right_weight_dual(self) -> DipoleBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Magnitude {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Motor {
    type Output = MultiVector;

    fn right_weight_dual(self) -> MultiVector {
        self.weight().right_complement()
    }
}

impl RightWeightDual for MultiVector {
    type Output = MultiVector;

    fn right_weight_dual(self) -> MultiVector {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Plane {
    type Output = RoundPointBulk;

    fn right_weight_dual(self) -> RoundPointBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for PlaneAtOrigin {
    type Output = RoundPointBulk;

    fn right_weight_dual(self) -> RoundPointBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Rotor {
    type Output = MultiVector;

    fn right_weight_dual(self) -> MultiVector {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Sphere {
    type Output = RoundPointBulk;

    fn right_weight_dual(self) -> RoundPointBulk {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Translator {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
        self.weight().right_complement()
    }
}
