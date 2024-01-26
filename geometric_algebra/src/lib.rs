#![cfg_attr(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"), feature(stdsimd))]

// pub mod cga3d;
pub mod rga3d;
pub mod simd;

#[cfg(test)]
mod tests;


//
//
// impl Exp for ppga3d::IdealPoint {
//     type Output = ppga3d::Translator;
//
//     fn exp(self) -> ppga3d::Translator {
//         ppga3d::Translator::new(1.0, self[0], self[1], self[2])
//     }
// }
//
// impl Ln for ppga3d::Translator {
//     type Output = ppga3d::IdealPoint;
//
//     fn ln(self) -> ppga3d::IdealPoint {
//         let result: ppga3d::IdealPoint = self.into();
//         result.scale(1.0 / self[0])
//     }
// }
//
// impl Powf for ppga3d::Translator {
//     type Output = Self;
//
//     fn powf(self, exponent: f32) -> Self {
//         self.ln().scale(exponent).exp()
//     }
// }
//
// impl Exp for ppga3d::Line {
//     type Output = ppga3d::Motor;
//
//     fn exp(self) -> ppga3d::Motor {
//         let det = self[3] * self[3] + self[4] * self[4] + self[5] * self[5];
//         if det <= 0.0 {
//             return ppga3d::Motor::new(1.0, 0.0, 0.0, 0.0, 0.0, self[0], self[1], self[2]);
//         }
//         let a = det.sqrt();
//         let c = a.cos();
//         let s = a.sin() / a;
//         let m = self[0] * self[3] + self[1] * self[4] + self[2] * self[5];
//         let t = m / det * (c - s);
//         let g0 = simd::Simd32x3::from(s) * self.group1();
//         let g1 = simd::Simd32x3::from(s) * self.group0() + simd::Simd32x3::from(t) * self.group1();
//         ppga3d::Motor::new(c, g0[0], g0[1], g0[2], s * m, g1[0], g1[1], g1[2])
//     }
// }
//
// impl Ln for ppga3d::Motor {
//     type Output = ppga3d::Line;
//
//     fn ln(self) -> ppga3d::Line {
//         let det = 1.0 - self[0] * self[0];
//         if det <= 0.0 {
//             return ppga3d::Line::new(self[5], self[6], self[7], 0.0, 0.0, 0.0);
//         }
//         let a = 1.0 / det;
//         let b = self[0].acos() * a.sqrt();
//         let c = a * self[4] * (1.0 - self[0] * b);
//         let g0 = simd::Simd32x4::from(b) * self.group1() + simd::Simd32x4::from(c) * self.group0();
//         let g1 = simd::Simd32x4::from(b) * self.group0();
//         ppga3d::Line::new(g0[1], g0[2], g0[3], g1[1], g1[2], g1[3])
//     }
// }
//
// impl Powf for ppga3d::Motor {
//     type Output = Self;
//
//     fn powf(self, exponent: f32) -> Self {
//         self.ln().scale(exponent).exp()
//     }
// }

/// All elements set to `0.0`
pub trait Zero {
    fn zero() -> Self;
}

/// All elements set to `0.0`, except for the scalar, which is set to `1.0`
pub trait One {
    fn one() -> Self;
}

/// Element order reversed
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}

/// Negates elements with `grade % 2 == 1`
///
/// Also called main involution
pub trait Automorphism {
    type Output;
    fn automorphism(self) -> Self::Output;
}

/// Negates elements with `grade % 4 >= 2`
///
/// Also called transpose
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Reverses
pub trait Reversal {
    type Output;
    fn reversal(self) -> Self::Output;
}

// (Implementation done!)
/// Negates elements with `grade % 4 >= 2`
///
/// Also called transpose
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Reverses
pub trait AntiReversal {
    type Output;
    fn anti_reversal(self) -> Self::Output;
}

/// Negates elements with `(grade + 3) % 4 < 2`
pub trait Conjugation {
    type Output;
    fn conjugation(self) -> Self::Output;
}

/// General multi vector multiplication
pub trait GeometricProduct<T> {
    type Output;
    fn geometric_product(self, other: T) -> Self::Output;
}

/// General multi vector multiplication
pub trait GeometricAntiProduct<T> {
    type Output;
    fn geometric_anti_product(self, other: T) -> Self::Output;
}

/// General multi vector division
pub trait GeometricQuotient<T> {
    type Output;
    fn geometric_quotient(self, other: T) -> Self::Output;
}

/// Dual of the geometric product grade filtered by `t == r + s`
///
/// Also called meet, anti-wedge, or exterior anti-product
pub trait RegressiveProduct<T> {
    type Output;
    fn regressive_product(self, other: T) -> Self::Output;
}

pub trait AntiWedge<T> {
    type Output;
    fn anti_wedge(self, other: T) -> Self::Output;
}
pub trait Meet<T> {
    type Output;
    fn meet(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == r + s`
///
/// Also called join, wedge, or exterior product
pub trait OuterProduct<T> {
    type Output;
    fn outer_product(self, other: T) -> Self::Output;
}
pub trait Wedge<T> {
    type Output;
    fn wedge(self, other: T) -> Self::Output;
}
pub trait Join<T> {
    type Output;
    fn join(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == (r - s).abs()`
///
/// Also called fat dot product
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
pub trait InnerProduct<T> {
    type Output;
    fn inner_product(self, other: T) -> Self::Output;
}
pub trait Dot<T> {
    type Output;
    fn dot(self, other: T) -> Self::Output;
}


/// Geometric product grade filtered by `t == (r - s).abs()`
///
/// Also called fat anti-dot product
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
pub trait InnerAntiProduct<T> {
    type Output;
    fn inner_anti_product(self, other: T) -> Self::Output;
}

pub trait AntiDot<T> {
    type Output;
    fn anti_dot(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == s - r`
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait LeftContraction<T> {
    type Output;
    fn left_contraction(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == r - s`
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait RightContraction<T> {
    type Output;
    fn right_contraction(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == s - r`
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait LeftAntiContraction<T> {
    type Output;
    fn left_anti_contraction(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == r - s`
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait RightAntiContraction<T> {
    type Output;
    fn right_anti_contraction(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == 0`
pub trait ScalarProduct<T> {
    type Output;
    fn scalar_product(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == 0`
pub trait AntiScalarProduct<T> {
    type Output;
    fn anti_scalar_product(self, other: T) -> Self::Output;
}

/// `self * other * self.reversion()`
///
/// Also called sandwich product
pub trait Transformation<T> {
    type Output;
    fn transformation(self, other: T) -> Self::Output;
}

/// (self geometric_anti_product other) geometric_anti_product (anti_reversal self)
///
/// Also called sandwich product
/// See article "Projective Geometric Algebra Done Right"
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projective_Geometric_Algebra_Done_Right
pub trait Sandwich<T> {
    type Output;
    fn sandwich(self, other: T) -> Self::Output;
}

/// Geometric product with a scalar
pub trait Scale {
    type Output;
    fn scale(self, other: f32) -> Self::Output;
}

/// Square of the magnitude
pub trait SquaredMagnitude {
    type Output;
    fn squared_magnitude(self) -> Self::Output;
}

/// Square of the anti-magnitude
pub trait SquaredAntiMagnitude {
    type Output;
    fn squared_anti_magnitude(self) -> Self::Output;
}

// /// Length as scalar
// ///
// /// Also called amplitude, absolute value or norm
// pub trait Magnitude {
//     type Output;
//     fn magnitude(self) -> Self::Output;
// }

/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNormSquared {
    type Output;
    fn bulk_norm_squared(self) -> Self::Output;
}

/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNormSquared {
    type Output;
    fn weight_norm_squared(self) -> Self::Output;
}
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait BulkNorm {
    type Output;
    fn bulk_norm(self) -> Self::Output;
}

/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait WeightNorm {
    type Output;
    fn weight_norm(self) -> Self::Output;
}

/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_norm
pub trait GeometricNorm {
    type Output;
    fn geometric_norm(self) -> Self::Output;
}


/// Direction without magnitude (set to scalar `-1.0` or `1.0`)
///
/// Also called sign or normalize
pub trait Signum {
    type Output;
    fn signum(self) -> Self::Output;
}

/// Raises a number to the scalar power of `-1.0`
pub trait Inverse {
    type Output;
    fn inverse(self) -> Self::Output;
}

/// The natural logarithm
pub trait Ln {
    type Output;
    fn ln(self) -> Self::Output;
}

/// The exponential function
pub trait Exp {
    type Output;
    fn exp(self) -> Self::Output;
}

/// Raises a number to an integer scalar power
pub trait Powi {
    type Output;
    fn powi(self, exponent: isize) -> Self::Output;
}

/// Raises a number to an floating point scalar power
pub trait Powf {
    type Output;
    fn powf(self, exponent: f32) -> Self::Output;
}

/// Square root
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

/// Grade
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
pub trait Grade {
    type Output;
    fn grade(self) -> Self::Output;
}

/// Anti-Grade
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Grade_and_antigrade
pub trait AntiGrade {
    type Output;
    fn anti_grade(self) -> Self::Output;
}

/// Right Complement
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait RightComplement {
    type Output;
    fn right_complement(self) -> Self::Output;
}

/// Left Complement
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait LeftComplement {
    type Output;
    fn left_complement(self) -> Self::Output;
}

/// Double Complement
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait DoubleComplement {
    type Output;
    fn double_complement(self) -> Self::Output;
}


/// Attitude
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Attitude
pub trait Attitude {
    type Output;
    fn attitude(self) -> Self::Output;
}

/// The Bulk of an object usually describes the object's relationship with the origin.
/// An object with a Bulk of zero contains the origin.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait Bulk {
    type Output;
    fn bulk(self) -> Self::Output;
}

/// The Weight of an object usually describes the object's attitude and orientation.
/// An object with zero weight is contained by the horizon.
///
/// Also known as the attitude operator.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait Weight {
    type Output;
    fn weight(self) -> Self::Output;
}

// TODO generate implementations
/// Round Bulk is a special type of bulk in CGA
/// https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
pub trait RoundBulk {
    type Output;
    fn round_bulk(self) -> Self::Output;
}

// TODO generate implementations
/// Round Weight is a special type of weight in CGA
/// https://conformalgeometricalgebra.com/wiki/index.php?title=Main_Page
pub trait RoundWeight {
    type Output;
    fn round_weight(self) -> Self::Output;
}

/// Euclidean distance between objects
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance
/// distance(a,b) = bulk_norm(attitude(a wedge b)) + weight_norm(a wedge attitude(b))
/// where attitude(c) = c anti_wedge complement(e4) where e4 is the projective dimension
pub trait Distance<T> {
    type Output;
    fn distance(self, other: T) -> Self::Output;
}

/// Unitization
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Unitization
pub trait Unitize {
    type Output;
    fn unitize(self) -> Self::Output;
}


/// Invert (Inversion)
/// An improper isometry that performs an inversion through a point.
/// Be careful not to confuse with `Inverse`, which raises a number to the power of `-1.0`.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Inversion
pub trait Invert<T> {
    type Output;
    fn invert(self, other: T) -> Self::Output;
}


/// Reflection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Reflection
pub trait Reflect<T> {
    type Output;
    fn reflect(self, other: T) -> Self::Output;
}

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

/// Bulk Contraction
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait BulkContraction<T> {
    type Output;
    fn bulk_contraction(self, other: T) -> Self::Output;
}

/// Weight Contraction
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait WeightContraction<T> {
    type Output;
    fn weight_contraction(self, other: T) -> Self::Output;
}

/// Bulk Expansion
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}

/// Weight Expansion
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
}

/// Orthogonal Projection
/// Typically involves bringing a lower dimensional object to a higher dimensional object
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectOrthogonallyOnto<T> {
    type Output;
    fn project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Orthogonal AntiProjection
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait AntiProjectOrthogonallyOnto<T> {
    type Output;
    fn anti_project_orthogonally_onto(self, other: T) -> Self::Output;
}

/// Central (to origin) Projection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait ProjectViaOriginOnto<T> {
    type Output;
    fn project_via_origin_onto(self, other: T) -> Self::Output;
}

/// Outward (to horizon) AntiProjection
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Projections
pub trait AntiProjectViaHorizonOnto<T> {
    type Output;
    fn anti_project_via_horizon_onto(self, other: T) -> Self::Output;
}

/// The cosine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait CosineAngle<T> {
    type Output;
    fn cosine_angle(self, other: T) -> Self::Output;
}

/// The sine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait SineAngle<T> {
    type Output;
    fn sine_angle(self, other: T) -> Self::Output;
}

/// Synonym for geometric product
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
pub trait WedgeDot<T> {
    type Output;
    fn wedge_dot(self, other: T) -> Self::Output;
}

/// Synonym for geometric anti product
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
pub trait AntiWedgeDot<T> {
    type Output;
    fn anti_wedge_dot(self, other: T) -> Self::Output;
}



// TODO generate implementations
// /// Dilation is a conformal transformation
// /// https://conformalgeometricalgebra.com/wiki/index.php?title=Dilation
// pub trait Dilate {
//     fn dilate(factor: f32) -> Self;
// }


// pub trait FlatPointExt {
//     fn from_xyz(x: f32, y: f32, z: f32) -> Self;
//     fn from_xyz_at_infinity(x: f32, y: f32, z: f32) -> Self;
// }
// impl FlatPointExt for cga3d::FlatPoint {
//     fn from_xyz(x: f32, y: f32, z: f32) -> Self {
//         cga3d::FlatPoint::new(x, y, z, 1.0)
//     }
//     fn from_xyz_at_infinity(x: f32, y: f32, z: f32) -> Self {
//         cga3d::FlatPoint::new(x, y, z, 0.0)
//     }
// }