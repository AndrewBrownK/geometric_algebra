use crate::traits::UnitizedRadiusNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 43
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       4       1
//   Median:         8      12       1
//  Average:         9      13       1
//  Maximum:        62      77       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       4       1
//   Median:         8      14       1
//  Average:         9      16       1
//  Maximum:        62      87       1
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       10        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       15        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       16        1
    //  no simd       12       18        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       11        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       20        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       21        1
    //  no simd       17       24        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       17        1
    //  no simd       13       20        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        2        9        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       16        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        7       11        1
    //  no simd        7       13        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       10        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       12        1
    //  no simd        8       14        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       15        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       16        1
    //  no simd       12       18        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       11        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       12        1
    //  no simd        8       14        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        7       11        1
    //  no simd        7       13        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       20        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       21        1
    //  no simd       17       24        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       17        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       17        1
    //  no simd       13       20        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        6        1
    //  no simd        2        8        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       16        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       72        1
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       62       77        1
    //  no simd       62       87        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for SphereAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for SphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       18       22        1
    //  no simd       18       25        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       17        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       18       22        1
    //  no simd       18       25        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedRadiusNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm()
    }
}
impl UnitizedRadiusNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm(self) -> f32 {
        return f32::powf(self.unitized_radius_norm_squared(), 0.5);
    }
}
