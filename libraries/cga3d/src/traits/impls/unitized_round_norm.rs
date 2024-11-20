use crate::traits::UnitizedRoundNormSquared;
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
//  Minimum:         9      15       1
//   Median:        17      25       1
//  Average:        22      35       1
//  Maximum:        83     114       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:        11      17       1
//   Median:        26      38       1
//  Average:        30      45       1
//  Maximum:        96     131       1
impl std::ops::Div<unitized_round_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       19        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        1
    //  no simd       26       38        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       43        1
    //  no simd       34       54        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for AntiDualNum {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       22        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        1
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        1
    //  no simd       15       26        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        1
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        1
    //  no simd       15       26        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       16       24        1
    //  no simd       25       37        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       27        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       18       32        1
    //  no simd       29       45        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78      107        1
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       83      114        1
    //  no simd       96      131        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for RoundPoint {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        1
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        1
    //  no simd       11       17        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       43        1
    //  no simd       34       54        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_round_norm> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       33        1
    //  no simd       30       46        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
