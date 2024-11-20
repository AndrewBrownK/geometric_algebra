use crate::traits::AntiSquareRoot;
use crate::traits::RoundWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 13
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         9      12       0
//   Median:        15      21       0
//  Average:        17      27       0
//  Maximum:        52      82       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         9      14       0
//   Median:        23      34       0
//  Average:        23      36       0
//  Maximum:        65      99       0
impl std::ops::Div<round_weight_norm> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       34        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       35        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       31       50        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for AntiDualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       21        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       24        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       24        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       34        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       27       42        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       11        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       14        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       75        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       52       82        0
    //  no simd       65       99        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       11        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       14        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       21        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       35        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       31       50        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<round_weight_norm> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       27       42        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
