use crate::traits::AntiScalarProduct;
use crate::traits::RoundWeight;
use crate::traits::Wedge;
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
impl std::ops::Div<round_weight_norm_squared> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       34        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32       19       35        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       31       50        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for AntiDualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       21        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32        7       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       24        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32        7       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       24        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32       10       15        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       34        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32       12       24        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       27       42        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
    }
}
impl std::ops::Div<round_weight_norm_squared> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: round_weight_norm_squared) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       11        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       14        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32       47       75        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       52       82        0
    //  no simd       65       99        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32        9       11        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       14        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    // f32       15       21        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32       19       35        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       31       50        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
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
    //      f32       12       24        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       27       42        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0])));
        return round_weight_carrier.anti_scalar_product(round_weight_carrier);
    }
}
