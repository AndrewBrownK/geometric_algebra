use crate::traits::RadiusNormSquared;
use crate::traits::RoundWeightNormSquared;
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
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       10        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       15        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       16        1
    //  no simd       12       18        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       11        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       20        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       21        1
    //  no simd       17       24        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       17        1
    //  no simd       13       20        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        2        9        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       16        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        7       11        1
    //  no simd        7       13        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       10        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       12        1
    //  no simd        8       14        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       15        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       16        1
    //  no simd       12       18        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       11        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       12        1
    //  no simd        8       14        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        7       11        1
    //  no simd        7       13        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       20        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       21        1
    //  no simd       17       24        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       17        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       16        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       17        1
    //  no simd       13       20        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        6        1
    //  no simd        2        8        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       16        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for DualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       72        1
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       62       77        1
    //  no simd       62       87        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for Sphere {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for SphereAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for SphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       18       22        1
    //  no simd       18       25        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       17        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       13        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       21        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       18       22        1
    //  no simd       18       25        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRadiusNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRadiusNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_radius_norm_squared()
    }
}
impl UnitizedRadiusNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       18        1
    //  no simd       14       21        1
    fn unitized_radius_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.radius_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
