use crate::traits::RoundBulkNormSquared;
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
//  Minimum:         9      16       1
//   Median:        17      26       1
//  Average:        22      36       1
//  Maximum:        83     115       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:        11      18       1
//   Median:        26      39       1
//  Average:        30      46       1
//  Maximum:        96     132       1
impl std::ops::Div<unitized_round_norm_squared> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       20        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       26        1
    //  no simd       26       39        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       40        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       44        1
    //  no simd       34       55        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       23        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       19        1
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       21        1
    //  no simd       15       27        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       19        1
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       21        1
    //  no simd       15       27        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       19        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       16       25        1
    //  no simd       25       38        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       18       33        1
    //  no simd       29       46        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78      108        1
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       83      115        1
    //  no simd       96      132        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       15        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       16        1
    //  no simd       11       18        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       40        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       44        1
    //  no simd       34       55        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_round_norm_squared> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       29        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       34        1
    //  no simd       30       47        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.round_bulk_norm_squared()[scalar]);
    }
}
