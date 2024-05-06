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
    type Output = Dipole;

    fn bulk_dual(self) -> Dipole {
        self.bulk().complement()
    }
}

impl BulkDual for Dipole {
    type Output = Circle;

    fn bulk_dual(self) -> Circle {
        self.bulk().complement()
    }
}

impl BulkDual for FlatPoint {
    type Output = Circle;

    fn bulk_dual(self) -> Circle {
        self.bulk().complement()
    }
}

impl BulkDual for Flector {
    type Output = MultiVector;

    fn bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl BulkDual for Line {
    type Output = Dipole;

    fn bulk_dual(self) -> Dipole {
        self.bulk().complement()
    }
}

impl BulkDual for Motor {
    type Output = MultiVector;

    fn bulk_dual(self) -> MultiVector {
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
    type Output = RoundPoint;

    fn bulk_dual(self) -> RoundPoint {
        self.bulk().complement()
    }
}

impl BulkDual for RoundPoint {
    type Output = Sphere;

    fn bulk_dual(self) -> Sphere {
        self.bulk().complement()
    }
}

impl BulkDual for Sphere {
    type Output = RoundPoint;

    fn bulk_dual(self) -> RoundPoint {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Circle {
    type Output = Dipole;

    fn round_bulk_dual(self) -> Dipole {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Dipole {
    type Output = Circle;

    fn round_bulk_dual(self) -> Circle {
        self.bulk().complement()
    }
}

impl RoundBulkDual for FlatPoint {
    type Output = Circle;

    fn round_bulk_dual(self) -> Circle {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Flector {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Line {
    type Output = Dipole;

    fn round_bulk_dual(self) -> Dipole {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Motor {
    type Output = MultiVector;

    fn round_bulk_dual(self) -> MultiVector {
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
    type Output = RoundPoint;

    fn round_bulk_dual(self) -> RoundPoint {
        self.bulk().complement()
    }
}

impl RoundBulkDual for RoundPoint {
    type Output = Sphere;

    fn round_bulk_dual(self) -> Sphere {
        self.bulk().complement()
    }
}

impl RoundBulkDual for Sphere {
    type Output = RoundPoint;

    fn round_bulk_dual(self) -> RoundPoint {
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
    type Output = Dipole;

    fn round_weight_dual(self) -> Dipole {
        self.weight().complement()
    }
}

impl RoundWeightDual for Dipole {
    type Output = Circle;

    fn round_weight_dual(self) -> Circle {
        self.weight().complement()
    }
}

impl RoundWeightDual for DualNum {
    type Output = DualNum;

    fn round_weight_dual(self) -> DualNum {
        self.weight().complement()
    }
}

impl RoundWeightDual for FlatPoint {
    type Output = Circle;

    fn round_weight_dual(self) -> Circle {
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
    type Output = Dipole;

    fn round_weight_dual(self) -> Dipole {
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
    type Output = RoundPoint;

    fn round_weight_dual(self) -> RoundPoint {
        self.weight().complement()
    }
}

impl RoundWeightDual for Sphere {
    type Output = RoundPoint;

    fn round_weight_dual(self) -> RoundPoint {
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
    type Output = Dipole;

    fn weight_dual(self) -> Dipole {
        self.weight().complement()
    }
}

impl WeightDual for Dipole {
    type Output = Circle;

    fn weight_dual(self) -> Circle {
        self.weight().complement()
    }
}

impl WeightDual for DualNum {
    type Output = DualNum;

    fn weight_dual(self) -> DualNum {
        self.weight().complement()
    }
}

impl WeightDual for FlatPoint {
    type Output = Circle;

    fn weight_dual(self) -> Circle {
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
    type Output = Dipole;

    fn weight_dual(self) -> Dipole {
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
    type Output = RoundPoint;

    fn weight_dual(self) -> RoundPoint {
        self.weight().complement()
    }
}

impl WeightDual for Sphere {
    type Output = RoundPoint;

    fn weight_dual(self) -> RoundPoint {
        self.weight().complement()
    }
}
