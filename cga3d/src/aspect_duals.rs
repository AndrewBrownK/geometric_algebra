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

impl LeftBulkDual for MultiVector {
    type Output = MultiVector;

    fn left_bulk_dual(self) -> MultiVector {
        self.bulk().left_complement()
    }
}

impl LeftBulkDual for Radial {
    type Output = Sphere;

    fn left_bulk_dual(self) -> Sphere {
        self.bulk().left_complement()
    }
}

impl LeftWeightDual for AntiScalar {
    type Output = Scalar;

    fn left_weight_dual(self) -> Scalar {
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

impl RightBulkDual for MultiVector {
    type Output = MultiVector;

    fn right_bulk_dual(self) -> MultiVector {
        self.bulk().right_complement()
    }
}

impl RightBulkDual for Radial {
    type Output = Sphere;

    fn right_bulk_dual(self) -> Sphere {
        self.bulk().right_complement()
    }
}

impl RightWeightDual for AntiScalar {
    type Output = Scalar;

    fn right_weight_dual(self) -> Scalar {
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
