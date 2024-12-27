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
// Total Implementations: 49
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         2       4       0
//  Average:         2       4       0
//  Maximum:        31      44       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         2       6       0
//  Average:         2       6       0
//  Maximum:        31      54       0
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Circle {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Dipole {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       39        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       31       54        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullSphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for Sphere {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for SphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
impl std::ops::Div<RoundWeightNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormSquaredPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm_squared()
    }
}
impl RoundWeightNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm_squared(self) -> AntiScalar {
        let round_weight_carrier = self.round_weight().wedge(Infinity::from_groups(/* e5 */ 1.0));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
