use crate::traits::RadiusNormSquared;
use crate::traits::SquareRoot;
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
impl std::ops::Div<radius_norm> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       16        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       11        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       11        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       16        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Line {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       33        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl std::ops::DivAssign<radius_norm> for Scalar {
    fn div_assign(&mut self, _rhs: radius_norm) {
        *self = self.radius_norm()
    }
}
impl RadiusNorm for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       17        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<radius_norm> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: radius_norm) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       17        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
