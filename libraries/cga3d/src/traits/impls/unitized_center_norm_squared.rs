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
//  Minimum:         5      10       1
//   Median:         9      14       1
//  Average:        17      23       1
//  Maximum:        95     106       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         5      13       1
//   Median:         9      20       1
//  Average:        17      31       1
//  Maximum:        95     123       1
impl std::ops::Div<unitized_center_norm_squared> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm_squared) -> Self::Output {
        self.unitized_center_norm_squared()
    }
}
impl UnitizedCenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        1
    //  no simd        6       16        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32        9       12        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       14        1
    //  no simd        9       20        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32        5        9        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       10        1
    //  no simd        5       13        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32        6       10        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        1
    //  no simd        6       14        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32        5        9        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        5       11        1
    //  no simd        5       15        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32        9       12        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        9       16        1
    //  no simd        9       28        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       95      100        1
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       95      106        1
    //  no simd       95      123        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       10       13        1
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       15        1
    //  no simd       10       21        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
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
    //      f32       10       13        1
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       17        1
    //  no simd       10       29        1
    fn unitized_center_norm_squared(self) -> f32 {
        use crate::elements::*;
        return self.center_norm_squared()[scalar] / (self.round_weight_norm_squared()[e12345]);
    }
}
