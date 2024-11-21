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
//  Minimum:        11      15       0
//   Median:        24      36       0
//  Average:        27      40       0
//  Maximum:        91     121       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:        11      17       0
//   Median:        26      38       0
//  Average:        30      43       0
//  Maximum:        96     128       0
impl std::ops::Div<round_norm_squared> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       35        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       26       38        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       44        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       31       46        0
    //  no simd       34       51        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for AntiDualNum {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       22        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       15       23        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       15       23        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       34        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       23       35        0
    //  no simd       25       37        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       38        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       29       42        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl std::ops::DivAssign<round_norm_squared> for MultiVector {
    fn div_assign(&mut self, _rhs: round_norm_squared) {
        *self = self.round_norm_squared()
    }
}
impl RoundNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       89      118        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      121        0
    //  no simd       96      128        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for RoundPoint {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       17        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       44        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       31       46        0
    //  no simd       34       51        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<round_norm_squared> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm_squared) -> Self::Output {
        self.round_norm_squared()
    }
}
impl RoundNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       39        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       27       40        0
    //  no simd       30       43        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm_squared();
        let other = self.round_weight_norm_squared();
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self_2[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
