use crate::traits::AntiSquareRoot;
use crate::traits::RoundWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 11
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         2       5       0
//  Average:         4       8       0
//  Maximum:        32      41       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         2       9       0
//  Average:         4      14       0
//  Maximum:        32      58       0
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       20        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       35        0
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       32       41        0
    //  no simd       32       58        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       20        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
