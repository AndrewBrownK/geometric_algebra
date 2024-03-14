
#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *, cga3d::*};
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::cga3d::aspects::{Bulk, Weight};
use crate::cga3d::involutions::*;


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

