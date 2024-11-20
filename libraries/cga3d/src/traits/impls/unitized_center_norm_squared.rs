use crate::traits::CenterNormSquared;
use crate::traits::RoundWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:        12      25       1
//   Median:        22      38       1
//  Average:        30      47       1
//  Maximum:       115     148       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:        18      31       1
//   Median:        33      51       1
//  Average:        40      59       1
//  Maximum:       128     165       1
impl std::ops::Div<unitized_center_norm_squared> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       29        1
    //  no simd       27       42        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       44        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       28       48        1
    //  no simd       37       59        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       23        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       25        1
    //  no simd       18       31        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       24        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       13       26        1
    //  no simd       19       32        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       22        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       28        1
    //  no simd       26       41        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       33        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       22       38        1
    //  no simd       33       51        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      110      141        1
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      115      148        1
    //  no simd      128      165        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       45        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       29       49        1
    //  no simd       38       60        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
impl std::ops::Div<unitized_center_norm_squared> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       34        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       39        1
    //  no simd       34       52        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return ((1.0 / self.round_weight_norm_squared()[e12345]) * self.center_norm_squared()[scalar]);
    }
}
