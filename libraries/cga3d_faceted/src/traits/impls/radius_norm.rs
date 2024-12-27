use crate::traits::RadiusNormSquared;
use crate::traits::SquareRoot;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 83
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3       5       0
//  Average:         5       7       0
//  Maximum:        31      33       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3       5       0
//  Average:         5       7       0
//  Maximum:        31      33       0
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       11        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiFlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       10        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        7        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       11        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        7        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       13        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       12        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       10        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for FlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Flector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for FlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Line {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for LineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Motor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MysteryCircle {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MysteryDipole {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        8        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MysteryVersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for MysteryVersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for PlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl std::ops::DivAssign<RadiusNormPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RadiusNormPrefixOrPostfix) {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for SphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for SphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        4        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       13        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       13        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
impl std::ops::Div<RadiusNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RadiusNormPrefixOrPostfix) -> Self::Output {
        self.radius_norm()
    }
}
impl RadiusNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       13        0
    fn radius_norm(self) -> Scalar {
        return self.radius_norm_squared().square_root();
    }
}
