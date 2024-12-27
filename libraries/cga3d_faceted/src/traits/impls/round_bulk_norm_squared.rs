use crate::traits::DotProduct;
use crate::traits::RoundBulk;
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
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiFlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiFlector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiFlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiLine {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiLineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiMotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiMotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for Circle {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for Dipole {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for MysteryCircle {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for MysteryCircleRotor {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for MysteryDipole {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for MysteryVersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for MysteryVersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl std::ops::DivAssign<RoundBulkNormSquaredPrefixOrPostfix> for Scalar {
    fn div_assign(&mut self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) {
        *self = self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
impl std::ops::Div<RoundBulkNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = Scalar;
    fn div(self, _rhs: RoundBulkNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_bulk_norm_squared()
    }
}
impl RoundBulkNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn round_bulk_norm_squared(self) -> Scalar {
        let round_bulk = self.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
