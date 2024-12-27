use crate::traits::UnitizedFlatNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       5       1
//   Median:         5       9       1
//  Average:         6      10       1
//  Maximum:        62      73       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       6       1
//   Median:         5      12       1
//  Average:         6      14       1
//  Maximum:        62      90       1
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       15        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       15        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       15        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for FlatPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Flector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Line {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Motor {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       66        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       62       73        1
    //  no simd       62       90        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Plane {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedFlatNormPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = f32;
    fn div(self, _rhs: UnitizedFlatNormPrefixOrPostfix) -> Self::Output {
        self.unitized_flat_norm()
    }
}
impl UnitizedFlatNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_flat_norm(self) -> f32 {
        return f32::powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
