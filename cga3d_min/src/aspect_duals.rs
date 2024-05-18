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

impl FlatBulkDual for Circle {
    type Output = Dipole;

    fn flat_bulk_dual(self) -> Dipole {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for Dipole {
    type Output = Circle;

    fn flat_bulk_dual(self) -> Circle {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for FlatPoint {
    type Output = Circle;

    fn flat_bulk_dual(self) -> Circle {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for Flector {
    type Output = MultiVector;

    fn flat_bulk_dual(self) -> MultiVector {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for Line {
    type Output = Dipole;

    fn flat_bulk_dual(self) -> Dipole {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for Motor {
    type Output = MultiVector;

    fn flat_bulk_dual(self) -> MultiVector {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for MultiVector {
    type Output = MultiVector;

    fn flat_bulk_dual(self) -> MultiVector {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for Plane {
    type Output = RoundPoint;

    fn flat_bulk_dual(self) -> RoundPoint {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for RoundPoint {
    type Output = Sphere;

    fn flat_bulk_dual(self) -> Sphere {
        self.flat_bulk().complement()
    }
}

impl FlatBulkDual for Sphere {
    type Output = RoundPoint;

    fn flat_bulk_dual(self) -> RoundPoint {
        self.flat_bulk().complement()
    }
}

impl FlatWeightDual for AntiScalar {
    type Output = Scalar;

    fn flat_weight_dual(self) -> Scalar {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for Circle {
    type Output = Dipole;

    fn flat_weight_dual(self) -> Dipole {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for Dipole {
    type Output = Circle;

    fn flat_weight_dual(self) -> Circle {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for DualNum {
    type Output = DualNum;

    fn flat_weight_dual(self) -> DualNum {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for FlatPoint {
    type Output = Circle;

    fn flat_weight_dual(self) -> Circle {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for Flector {
    type Output = MultiVector;

    fn flat_weight_dual(self) -> MultiVector {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for Line {
    type Output = Dipole;

    fn flat_weight_dual(self) -> Dipole {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for Motor {
    type Output = MultiVector;

    fn flat_weight_dual(self) -> MultiVector {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for MultiVector {
    type Output = MultiVector;

    fn flat_weight_dual(self) -> MultiVector {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for Plane {
    type Output = RoundPoint;

    fn flat_weight_dual(self) -> RoundPoint {
        self.flat_weight().complement()
    }
}

impl FlatWeightDual for Sphere {
    type Output = RoundPoint;

    fn flat_weight_dual(self) -> RoundPoint {
        self.flat_weight().complement()
    }
}

impl RoundBulkDual for Circle {
    type Output = Dipole;

    fn round_bulk_dual(self) -> Dipole {
        self.round_bulk().complement()
    }
}

impl RoundBulkDual for Dipole {
    type Output = Circle;

    fn round_bulk_dual(self) -> Circle {
        self.round_bulk().complement()
    }
}

impl RoundBulkDual for DualNum {
    type Output = DualNum;

    fn round_bulk_dual(self) -> DualNum {
        self.round_bulk().complement()
    }
}

impl RoundBulkDual for MultiVector {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
        self.round_bulk().complement()
    }
}

impl RoundBulkDual for RoundPoint {
    type Output = Sphere;

    fn round_bulk_dual(self) -> Sphere {
        self.round_bulk().complement()
    }
}

impl RoundBulkDual for Scalar {
    type Output = AntiScalar;

    fn round_bulk_dual(self) -> AntiScalar {
        self.round_bulk().complement()
    }
}

impl RoundWeightDual for Circle {
    type Output = Dipole;

    fn round_weight_dual(self) -> Dipole {
        self.round_weight().complement()
    }
}

impl RoundWeightDual for Dipole {
    type Output = Circle;

    fn round_weight_dual(self) -> Circle {
        self.round_weight().complement()
    }
}

impl RoundWeightDual for MultiVector {
    type Output = MultiVector;

    fn round_weight_dual(self) -> MultiVector {
        self.round_weight().complement()
    }
}

impl RoundWeightDual for RoundPoint {
    type Output = Sphere;

    fn round_weight_dual(self) -> Sphere {
        self.round_weight().complement()
    }
}

impl RoundWeightDual for Sphere {
    type Output = RoundPoint;

    fn round_weight_dual(self) -> RoundPoint {
        self.round_weight().complement()
    }
}
