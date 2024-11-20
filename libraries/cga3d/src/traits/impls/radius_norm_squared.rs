use crate::traits::AntiScalarProduct;
use crate::traits::RightAntiDual;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3       5       0
//  Average:         6       8       0
//  Maximum:        31      33       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3       5       0
//  Average:         6       8       0
//  Maximum:        31      33       0
impl std::ops::Div<radius_norm_squared> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       16        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       11        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       11        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       16        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Line {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       33        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl std::ops::DivAssign<radius_norm_squared> for Scalar {
    fn div_assign(&mut self, _rhs: radius_norm_squared) {
        *self = self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       17        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
impl std::ops::Div<radius_norm_squared> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm_squared) -> Self::Output {
        self.radius_norm_squared()
    }
}
impl RadiusNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       17        0
    fn radius_norm_squared(self) -> Scalar {
        return self.anti_scalar_product(self).right_anti_dual();
    }
}
