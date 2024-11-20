use crate::traits::FlatBulkNormSquared;
use crate::traits::FlatWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 15
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       6       0
//   Median:         6       9       0
//  Average:        10      18       0
//  Maximum:        63      83       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       6       0
//   Median:         6      12       0
//  Average:        10      20       0
//  Maximum:        63      89       0
impl std::ops::Div<flat_norm_squared> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: flat_norm_squared) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2        8        0
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
    //      f32       16       28        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       16       30        0
    //  no simd       16       34        0
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
    //      add/sub      mul      div
    // f32        6       16        0
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
    //      add/sub      mul      div
    // f32        7       17        0
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
    //      f32        2        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2        8        0
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
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
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
    //      f32        2        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2        8        0
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
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
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
    //      add/sub      mul      div
    // f32        6       16        0
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
    //      f32       17       29        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       17       35        0
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
    //      f32       63       80        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       63       83        0
    //  no simd       63       89        0
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
    //      add/sub      mul      div
    // f32        2        6        0
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
    //      add/sub      mul      div
    // f32        2        6        0
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
    //      f32       17       29        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       17       35        0
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
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
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
