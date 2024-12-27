use crate::traits::RoundBulkNormSquared;
use crate::traits::SquareRoot;
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
