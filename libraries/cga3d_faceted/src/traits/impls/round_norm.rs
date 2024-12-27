use crate::traits::RoundBulkNorm;
use crate::traits::RoundWeightNorm;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       3       0
//   Median:         5       8       0
//  Average:         6      10       0
//  Maximum:        62      76       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       3       0
//   Median:         5      10       0
//  Average:         6      12       0
//  Maximum:        62      86       0
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4        9        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       10        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       10        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       10        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiDipoleOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiDualNum {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiSphereOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        5        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4        9        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       11        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       11        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4        9        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl std::ops::DivAssign<RoundNormPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: RoundNormPrefixOrPostfix) {
        *self = self.round_norm()
    }
}
impl RoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       71        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       62       76        0
    //  no simd       62       86        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for RoundPoint {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        5        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
impl std::ops::Div<RoundNormPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn div(self, _rhs: RoundNormPrefixOrPostfix) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        6       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.round_bulk_norm()[scalar], self.round_weight_norm()[e12345]]),
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
