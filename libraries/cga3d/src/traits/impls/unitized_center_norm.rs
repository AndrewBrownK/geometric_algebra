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
//  Minimum:        15      24       1
//   Median:        30      44       1
//  Average:        37      52       1
//  Maximum:       123     154       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:        18      27       1
//   Median:        33      47       1
//  Average:        40      55       1
//  Maximum:       128     161       1
impl std::ops::Div<unitized_center_norm> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: unitized_center_norm) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       38        1
    //    simd3        1        1        0
    // Totals...
    // yes simd       25       39        1
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
    //      f32       33       48        1
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       34       50        1
    //  no simd       37       55        1
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
    //      f32       14       23        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       24        1
    //  no simd       18       27        1
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
    //      f32       15       24        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       16       25        1
    //  no simd       19       28        1
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
    //      f32       23       37        1
    //    simd3        1        1        0
    // Totals...
    // yes simd       24       38        1
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
    //      f32       29       43        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       44        1
    //  no simd       33       47        1
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
    //      f32      121      151        1
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd      123      154        1
    //  no simd      128      161        1
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
    //      f32       34       49        1
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       35       51        1
    //  no simd       38       56        1
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
    //      f32       30       44        1
    //    simd4        1        1        0
    // Totals...
    // yes simd       31       45        1
    //  no simd       34       48        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
