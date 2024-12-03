use crate::traits::RoundBulkNormSquared;
use crate::traits::RoundWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 10
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       6       1
//   Median:         5      10       1
//  Average:        10      15       1
//  Maximum:        63      73       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       9       1
//   Median:         5      16       1
//  Average:        10      22       1
//  Maximum:        63      90       1
impl std::ops::Div<unitized_round_norm_squared> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm_squared) -> Self::Output {
        self.unitized_round_norm_squared()
    }
}
impl UnitizedRoundNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       13        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        2        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        1
    //  no simd        2        9        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        2        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        1
    //  no simd        2        9        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        4        6        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        1
    //  no simd        4       12        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        5        7        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       11        1
    //  no simd        5       23        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32       63       67        1
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       63       73        1
    //  no simd       63       90        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        2        5        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        1
    //  no simd        2        9        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        6        8        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       16        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
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
    //      f32        6        8        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       12        1
    //  no simd        6       24        1
    fn unitized_round_norm_squared(self) -> f32 {
        use crate::elements::*;
        return (self.round_bulk_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]));
    }
}
