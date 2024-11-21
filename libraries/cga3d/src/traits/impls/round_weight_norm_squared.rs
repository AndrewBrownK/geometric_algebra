use crate::traits::AntiDotProduct;
use crate::traits::RoundWeight;
use crate::traits::Wedge;
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
//  Average:         4      12       0
//  Maximum:        32      49       0
impl std::ops::Div<round_weight_norm_squared> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       38        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       32       41        0
    //  no simd       32       49        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([1.0, 0.0])));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
