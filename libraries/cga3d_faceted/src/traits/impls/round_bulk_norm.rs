use crate::traits::RoundBulkNormSquared;
use crate::traits::SquareRoot;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 51
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         2       3       0
//  Maximum:        31      32       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         2       3       0
//  Maximum:        31      32       0
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryCircle {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryDipole {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryVersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for MysteryVersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl std::ops::DivAssign<RoundBulkNormPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RoundBulkNormPrefixOrPostfix) {
        *self = self.round_bulk_norm()
    }
}
impl RoundBulkNorm for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
impl std::ops::Div<RoundBulkNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm()
    }
}
impl RoundBulkNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm(self) -> Scalar {
        return self.round_bulk_norm_squared().square_root();
    }
}
