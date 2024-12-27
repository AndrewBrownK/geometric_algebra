use crate::traits::RoundBulkNormSquared;
use crate::traits::RoundWeightNormSquared;
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
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        1
    //  no simd        4        9        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       10        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       10        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       10        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        5        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        1
    //  no simd        2        7        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        1
    //  no simd        4        9        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       11        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        1
    //  no simd        5       11        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        1
    //  no simd        4        9        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       71        1
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       62       76        1
    //  no simd       62       86        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        5        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
impl std::ops::Div<UnitizedRoundNormSquaredPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = f32;
    fn div(self, _rhs: UnitizedRoundNormSquaredPrefixOrPostfix) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        1
    //  no simd        6       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
