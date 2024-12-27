use crate::traits::AntiSquareRoot;
use crate::traits::RoundWeightNormSquared;
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
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
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
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiDualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for CircleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for CircleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
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
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
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
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
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
    //      f32       31       39        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       31       54        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for NullCircleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for NullDipoleAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for NullDipoleInversionAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for NullDipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for NullSphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for NullVersorEvenAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
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
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
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
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for SphereAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for SphereOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for SphereOnOrigin {
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
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
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
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
impl std::ops::Div<RoundWeightNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: RoundWeightNormPrefixOrPostfix) -> Self::Output {
        self.round_weight_norm()
    }
}
impl RoundWeightNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn round_weight_norm(self) -> AntiScalar {
        return self.round_weight_norm_squared().anti_square_root();
    }
}
