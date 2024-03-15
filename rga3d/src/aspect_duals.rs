//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

#![allow(clippy::assign_op_pattern)]
use crate::rga3d::aspects::{Bulk, Weight};
use crate::rga3d::involutions::*;
use geometric_algebra::{rga3d::*, simd::*, *};
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

impl LeftBulkDual for Flector {
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

impl LeftWeightDual for MultiVector {
    type Output = MultiVector;

    fn left_weight_dual(self) -> MultiVector {
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

impl RightWeightDual for MultiVector {
    type Output = MultiVector;

    fn right_weight_dual(self) -> MultiVector {
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

impl RightWeightDual for Translator {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
        self.weight().right_complement()
    }
}
