use crate::traits::FlatBulkNormSquared;
use crate::traits::FlatWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 16
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       4       0
//   Median:        12      24       0
//  Average:        18      29       0
//  Maximum:        99     124       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0      10       0
//   Median:        15      27       0
//  Average:        19      36       0
//  Maximum:       111     144       0
impl std::ops::Div<flat_norm_squared> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        8       17        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       39        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       25       57        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       18        0
    //  no simd        9       21        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       10       22        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        8       17        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       15       27        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for DualNum {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for FlatPoint {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        8       17        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for Flector {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       15       27        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for Line {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       18        0
    //  no simd        9       21        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for Motor {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       40        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       26       45        0
    //  no simd       26       58        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl std::ops::DivAssign<flat_norm_squared> for MultiVector {
    fn div_assign(&mut self, _rhs: flat_norm_squared) {
        *self = self.flat_norm_squared()
    }
}
impl FlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       94      116        0
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       99      124        0
    //  no simd      111      144        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for Plane {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       17       24        0
    //  no simd       17       33        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for Sphere {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       17       24        0
    //  no simd       17       33        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       40        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       26       45        0
    //  no simd       26       58        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
impl std::ops::Div<flat_norm_squared> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       15       27        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.flat_bulk_norm_squared();
        let other = self.flat_weight_norm_squared();
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
