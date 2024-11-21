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
//  Minimum:        11      15       1
//   Median:        24      36       1
//  Average:        27      40       1
//  Maximum:        91     121       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:        11      17       1
//   Median:        26      38       1
//  Average:        30      43       1
//  Maximum:        96     128       1
impl std::ops::Div<unitized_round_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_round_norm) -> Self::Output {
        self.unitized_round_norm()
    }
}
impl UnitizedRoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       35        1
    //    simd3        1        1        0
    // Totals...
    // yes simd       24       36        1
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
    //      f32       30       44        1
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       31       46        1
    //  no simd       34       51        1
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
    //      f32       11       19        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       20        1
    //  no simd       15       23        1
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
    //      f32       11       19        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       20        1
    //  no simd       15       23        1
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
    //      f32       22       34        1
    //    simd3        1        1        0
    // Totals...
    // yes simd       23       35        1
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
    //      f32       25       38        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       39        1
    //  no simd       29       42        1
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
    //      f32       89      118        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      121        1
    //  no simd       96      128        1
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
    //      f32       30       44        1
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       31       46        1
    //  no simd       34       51        1
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
    //      f32       26       39        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       27       40        1
    //  no simd       30       43        1
    fn unitized_round_norm(self) -> f32 {
        return f32::powf(self.unitized_round_norm_squared(), 0.5);
    }
}
