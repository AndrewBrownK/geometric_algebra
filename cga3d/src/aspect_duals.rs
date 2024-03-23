//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/geometric_algebra/
//

use crate::aspects::{Bulk, Weight};
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

impl LeftBulkDual for Circle {
    type Output = Dipole;

    fn left_bulk_dual(self) -> Dipole {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Dipole {
    type Output = Circle;

    fn left_bulk_dual(self) -> Circle {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Horizon {
    type Output = RoundPoint;

    fn left_bulk_dual(self) -> RoundPoint {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Infinity {
    type Output = Sphere;

    fn left_bulk_dual(self) -> Sphere {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Line {
    type Output = Dipole;

    fn left_bulk_dual(self) -> Dipole {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for LineAtInfinity {
    type Output = Dipole;

    fn left_bulk_dual(self) -> Dipole {
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
    type Output = RoundPoint;

    fn left_bulk_dual(self) -> RoundPoint {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Point {
    type Output = Circle;

    fn left_bulk_dual(self) -> Circle {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for PointAtInfinity {
    type Output = Circle;

    fn left_bulk_dual(self) -> Circle {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for RoundPoint {
    type Output = Sphere;

    fn left_bulk_dual(self) -> Sphere {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Sphere {
    type Output = RoundPoint;

    fn left_bulk_dual(self) -> RoundPoint {
        self.bulk().left_complement()
    }
}

impl LeftWeightDual for AntiScalar {
    type Output = Scalar;

    fn left_weight_dual(self) -> Scalar {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Circle {
    type Output = Dipole;

    fn left_weight_dual(self) -> Dipole {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Dipole {
    type Output = Circle;

    fn left_weight_dual(self) -> Circle {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Line {
    type Output = Dipole;

    fn left_weight_dual(self) -> Dipole {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for LineAtOrigin {
    type Output = Dipole;

    fn left_weight_dual(self) -> Dipole {
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
    type Output = Circle;

    fn left_weight_dual(self) -> Circle {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Plane {
    type Output = RoundPoint;

    fn left_weight_dual(self) -> RoundPoint {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for PlaneAtOrigin {
    type Output = RoundPoint;

    fn left_weight_dual(self) -> RoundPoint {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Point {
    type Output = Circle;

    fn left_weight_dual(self) -> Circle {
        self.weight().left_complement()
    }
}

impl LeftWeightDual for Sphere {
    type Output = RoundPoint;

    fn left_weight_dual(self) -> RoundPoint {
        self.weight().left_complement()
    }
}

impl RightBulkDual for Circle {
    type Output = Dipole;

    fn right_bulk_dual(self) -> Dipole {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Dipole {
    type Output = Circle;

    fn right_bulk_dual(self) -> Circle {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Horizon {
    type Output = RoundPoint;

    fn right_bulk_dual(self) -> RoundPoint {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Infinity {
    type Output = Sphere;

    fn right_bulk_dual(self) -> Sphere {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Line {
    type Output = Dipole;

    fn right_bulk_dual(self) -> Dipole {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for LineAtInfinity {
    type Output = Dipole;

    fn right_bulk_dual(self) -> Dipole {
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
    type Output = RoundPoint;

    fn right_bulk_dual(self) -> RoundPoint {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Point {
    type Output = Circle;

    fn right_bulk_dual(self) -> Circle {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for PointAtInfinity {
    type Output = Circle;

    fn right_bulk_dual(self) -> Circle {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for RoundPoint {
    type Output = Sphere;

    fn right_bulk_dual(self) -> Sphere {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Sphere {
    type Output = RoundPoint;

    fn right_bulk_dual(self) -> RoundPoint {
        self.bulk().right_complement()
    }
}

impl RightWeightDual for AntiScalar {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Circle {
    type Output = Dipole;

    fn right_weight_dual(self) -> Dipole {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Dipole {
    type Output = Circle;

    fn right_weight_dual(self) -> Circle {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Line {
    type Output = Dipole;

    fn right_weight_dual(self) -> Dipole {
        self.weight().right_complement()
    }
}

impl RightWeightDual for LineAtOrigin {
    type Output = Dipole;

    fn right_weight_dual(self) -> Dipole {
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
    type Output = Circle;

    fn right_weight_dual(self) -> Circle {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Plane {
    type Output = RoundPoint;

    fn right_weight_dual(self) -> RoundPoint {
        self.weight().right_complement()
    }
}

impl RightWeightDual for PlaneAtOrigin {
    type Output = RoundPoint;

    fn right_weight_dual(self) -> RoundPoint {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Point {
    type Output = Circle;

    fn right_weight_dual(self) -> Circle {
        self.weight().right_complement()
    }
}

impl RightWeightDual for Sphere {
    type Output = RoundPoint;

    fn right_weight_dual(self) -> RoundPoint {
        self.weight().right_complement()
    }
}
