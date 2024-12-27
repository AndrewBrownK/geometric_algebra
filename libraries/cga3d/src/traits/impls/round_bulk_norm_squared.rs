use crate::traits::DotProduct;
use crate::traits::RoundBulk;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         3       4       0
//  Maximum:        31      32       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       3       0
//  Average:         3       4       0
//  Maximum:        31      32       0
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
