use crate::traits::FlatBulkNormSquared;
use crate::traits::FlatWeightNormSquared;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       5       0
//   Median:         5       9       0
//  Average:         6      10       0
//  Maximum:        62      73       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       6       0
//   Median:         5      12       0
//  Average:         6      14       0
//  Maximum:        62      90       0
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       12        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       12        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       12        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       13        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       13        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       13        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       13        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Flector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Line {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       12        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl std::ops::DivAssign<FlatNormSquaredPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: FlatNormSquaredPrefixOrPostfix) {
        *self = self.flat_norm_squared()
    }
}
impl FlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       66        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       62       73        0
    //  no simd       62       90        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Plane {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Sphere {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.flat_bulk_norm_squared()[scalar], self.flat_weight_norm_squared()[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
