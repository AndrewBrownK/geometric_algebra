//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::aspects::{Bulk, Weight};
use crate::involutions::*;
use crate::*;
use projective_ga::{simd::*, *};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// RightBulkDual
/// Get the complement of an aspect of an object.
///
/// The metric of this algebra is degenerate. One of the side effects of this is that
/// invoking the `Dual` operation erases the Weight (prior to invoking `RightComplement`), and
/// invoking the `AntiDual` operation erases the Bulk (prior to invoking `RightComplement`). It is for
/// this reason that (in this algebra) `RightBulkDual` = `Dual` and `RightWeightDual` = `AntiDual`.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait RightBulkDual {
    type Output;
    fn right_bulk_dual(self) -> Self::Output;
}

/// RightWeightDual
/// Get the complement of an aspect of an object.
///
/// The metric of this algebra is degenerate. One of the side effects of this is that
/// invoking the `Dual` operation erases the Weight (prior to invoking `RightComplement`), and
/// invoking the `AntiDual` operation erases the Bulk (prior to invoking `RightComplement`). It is for
/// this reason that (in this algebra) `RightBulkDual` = `Dual` and `RightWeightDual` = `AntiDual`.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait RightWeightDual {
    type Output;
    fn right_weight_dual(self) -> Self::Output;
}

/// LeftBulkDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait LeftBulkDual {
    type Output;
    fn left_bulk_dual(self) -> Self::Output;
}

/// LeftWeightDual
/// Get the complement of an aspect of an object.
///
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait LeftWeightDual {
    type Output;
    fn left_weight_dual(self) -> Self::Output;
}

impl LeftBulkDual for Flector {
    type Output = Flector;

    fn left_bulk_dual(self) -> Flector {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for FlectorAtInfinity {
    type Output = Flector;

    fn left_bulk_dual(self) -> Flector {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Horizon {
    type Output = Origin;

    fn left_bulk_dual(self) -> Origin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Line {
    type Output = LineAtOrigin;

    fn left_bulk_dual(self) -> LineAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for LineAtInfinity {
    type Output = LineAtOrigin;

    fn left_bulk_dual(self) -> LineAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Magnitude {
    type Output = AntiScalar;

    fn left_bulk_dual(self) -> AntiScalar {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Motor {
    type Output = LineAtOrigin;

    fn left_bulk_dual(self) -> LineAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn left_bulk_dual(self) -> MultiVectorAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn left_bulk_dual(self) -> MultiVectorAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Plane {
    type Output = Origin;

    fn left_bulk_dual(self) -> Origin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Point {
    type Output = PlaneAtOrigin;

    fn left_bulk_dual(self) -> PlaneAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn left_bulk_dual(self) -> PlaneAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Scalar {
    type Output = AntiScalar;

    fn left_bulk_dual(self) -> AntiScalar {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Transflector {
    type Output = Flector;

    fn left_bulk_dual(self) -> Flector {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Translator {
    type Output = LineAtOrigin;

    fn left_bulk_dual(self) -> LineAtOrigin {
        self.bulk().left_complement()
    }
}

impl LeftWeightDual for AntiScalar {
    type Output = Scalar;

    fn left_weight_dual(self) -> Scalar {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Flector {
    type Output = Flector;

    fn left_weight_dual(self) -> Flector {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Line {
    type Output = LineAtInfinity;

    fn left_weight_dual(self) -> LineAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for LineAtOrigin {
    type Output = LineAtInfinity;

    fn left_weight_dual(self) -> LineAtInfinity {
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
    type Output = MultiVectorAtInfinity;

    fn left_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn left_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn left_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Origin {
    type Output = Horizon;

    fn left_weight_dual(self) -> Horizon {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Plane {
    type Output = PointAtInfinity;

    fn left_weight_dual(self) -> PointAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn left_weight_dual(self) -> PointAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Point {
    type Output = Horizon;

    fn left_weight_dual(self) -> Horizon {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Rotor {
    type Output = MultiVectorAtInfinity;

    fn left_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Transflector {
    type Output = PointAtInfinity;

    fn left_weight_dual(self) -> PointAtInfinity {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Translator {
    type Output = Scalar;

    fn left_weight_dual(self) -> Scalar {
        self.weight().left_complement()
    }
}

impl RightBulkDual for Flector {
    type Output = Flector;

    fn right_bulk_dual(self) -> Flector {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for FlectorAtInfinity {
    type Output = Flector;

    fn right_bulk_dual(self) -> Flector {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Horizon {
    type Output = Origin;

    fn right_bulk_dual(self) -> Origin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Line {
    type Output = LineAtOrigin;

    fn right_bulk_dual(self) -> LineAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for LineAtInfinity {
    type Output = LineAtOrigin;

    fn right_bulk_dual(self) -> LineAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Magnitude {
    type Output = AntiScalar;

    fn right_bulk_dual(self) -> AntiScalar {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Motor {
    type Output = LineAtOrigin;

    fn right_bulk_dual(self) -> LineAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn right_bulk_dual(self) -> MultiVectorAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn right_bulk_dual(self) -> MultiVectorAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Plane {
    type Output = Origin;

    fn right_bulk_dual(self) -> Origin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Point {
    type Output = PlaneAtOrigin;

    fn right_bulk_dual(self) -> PlaneAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn right_bulk_dual(self) -> PlaneAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Scalar {
    type Output = AntiScalar;

    fn right_bulk_dual(self) -> AntiScalar {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Transflector {
    type Output = Flector;

    fn right_bulk_dual(self) -> Flector {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Translator {
    type Output = LineAtOrigin;

    fn right_bulk_dual(self) -> LineAtOrigin {
        self.bulk().right_complement()
    }
}

impl RightWeightDual for AntiScalar {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Flector {
    type Output = Flector;

    fn right_weight_dual(self) -> Flector {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Line {
    type Output = LineAtInfinity;

    fn right_weight_dual(self) -> LineAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for LineAtOrigin {
    type Output = LineAtInfinity;

    fn right_weight_dual(self) -> LineAtInfinity {
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
    type Output = MultiVectorAtInfinity;

    fn right_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn right_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn right_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Origin {
    type Output = Horizon;

    fn right_weight_dual(self) -> Horizon {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Plane {
    type Output = PointAtInfinity;

    fn right_weight_dual(self) -> PointAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn right_weight_dual(self) -> PointAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Point {
    type Output = Horizon;

    fn right_weight_dual(self) -> Horizon {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Rotor {
    type Output = MultiVectorAtInfinity;

    fn right_weight_dual(self) -> MultiVectorAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Transflector {
    type Output = PointAtInfinity;

    fn right_weight_dual(self) -> PointAtInfinity {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Translator {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
        self.weight().right_complement()
    }
}
