use crate::traits::RoundBulkNorm;
use crate::traits::RoundWeightNorm;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 10
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         2       6       0
//   Median:         5      10       0
//  Average:        10      15       0
//  Maximum:        63      73       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         2       9       0
//   Median:         5      14       0
//  Average:        10      19       0
//  Maximum:        63      81       0
impl std::ops::Div<round_norm> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       13        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2        9        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2        9        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       12        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       14        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl std::ops::DivAssign<round_norm> for MultiVector {
    fn div_assign(&mut self, _rhs: round_norm) {
        *self = self.round_norm()
    }
}
impl RoundNorm for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       63       70        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       63       73        0
    //  no simd       63       81        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for RoundPoint {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2        9        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
impl std::ops::Div<round_norm> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: round_norm) -> Self::Output {
        self.round_norm()
    }
}
impl RoundNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       15        0
    fn round_norm(self) -> MultiVector {
        use crate::elements::*;
        let self_2 = self.round_bulk_norm();
        let other = self.round_weight_norm();
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
