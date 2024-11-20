use crate::traits::UnitizedCenterNormSquared;
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
//  Minimum:        12      24       1
//   Median:        22      37       1
//  Average:        30      46       1
//  Maximum:       115     147       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:        18      30       1
//   Median:        33      50       1
//  Average:        40      58       1
//  Maximum:       128     164       1
impl std::ops::Div<unitized_center_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       28        1
    //  no simd       27       41        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       43        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       28       47        1
    //  no simd       37       58        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for Circle {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        1
    //  no simd       18       30        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        1
    //    simd4        2        2        0
    // Totals...
    // yes simd       13       25        1
    //  no simd       19       31        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for Dipole {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        1
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       27        1
    //  no simd       26       40        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       22       37        1
    //  no simd       33       50        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      110      140        1
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      115      147        1
    //  no simd      128      164        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       44        1
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       29       48        1
    //  no simd       38       59        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<unitized_center_norm> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        1
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       38        1
    //  no simd       34       51        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
