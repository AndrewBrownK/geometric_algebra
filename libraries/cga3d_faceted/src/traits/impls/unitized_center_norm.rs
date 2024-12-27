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
//  Minimum:         5       9       1
//   Median:         9      13       1
//  Average:        17      22       1
//  Maximum:        94     109       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         5      11       1
//   Median:         9      16       1
//  Average:        17      25       1
//  Maximum:        94     119       1
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       12        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        1
    //  no simd        9       16        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for Circle {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       11        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for CircleRotor {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       10        1
    //  no simd        6       12        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for Dipole {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        9        1
    //  no simd        5       11        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for DipoleInversion {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        1
    //  no simd        9       16        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for MultiVector {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       94      104        1
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       94      109        1
    //  no simd       94      119        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for VersorEven {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       17        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
impl std::ops::Div<UnitizedCenterNormPrefixOrPostfix> for VersorOdd {
    type Output = f32;
    fn div(self, _rhs: UnitizedCenterNormPrefixOrPostfix) -> Self::Output {
        self.unitized_center_norm()
    }
}
impl UnitizedCenterNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        1
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       14        1
    //  no simd       10       17        1
    fn unitized_center_norm(self) -> f32 {
        return f32::powf(self.unitized_center_norm_squared(), 0.5);
    }
}
