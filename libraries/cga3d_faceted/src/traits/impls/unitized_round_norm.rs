use crate::traits::UnitizedRoundNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       3       1
//   Median:         5       8       1
//  Average:         6      10       1
//  Maximum:        62      76       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       3       1
//   Median:         5      10       1
//  Average:         6      12       1
//  Maximum:        62      86       1
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        1
    //  no simd        4        9        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       10        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       10        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       10        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        5        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        1
    //  no simd        4        9        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       11        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       11        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        1
    //  no simd        4        9        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       71        1
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       62       76        1
    //  no simd       62       86        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        5        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRoundNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
