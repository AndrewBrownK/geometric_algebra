// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 502
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         4      10       0
//  Average:         6      14       0
//  Maximum:       111     146       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      22       0
//  Average:        11      29       0
//  Maximum:       211     265       0
impl std::ops::Div<bulk_contraction> for AntiCircleRotor {
    type Output = bulk_contraction_partial<AntiCircleRotor>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       10       33        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().xyz().with_w(
                    (self[scalar] * right_dual[e12345])
                        - (self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e45] * right_dual[e321])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       32        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (right_dual.group3().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (right_dual.group2().www() * self.group2().xyz()).with_w(self[e42] * right_dual[e4315]),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(right_dual[e12345]) * self.group2(),
        );
    }
}
impl BulkContraction<AntiFlector> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       24        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]),
            ]) - (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<AntiLine> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e41] * right_dual[e235])
                - (self[e42] * right_dual[e315])
                - (self[e43] * right_dual[e125])
                - (self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435]),
        );
    }
}
impl BulkContraction<AntiMotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().xyz().with_w(
                    (self[scalar] * right_dual[e12345])
                        - (self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435]),
                ),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]),
            ]) - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<Dipole> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e41] * right_dual[e235])
                - (self[e42] * right_dual[e315])
                - (self[e43] * right_dual[e125])
                - (self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435])
                - (self[e45] * right_dual[e321])
                - (self[e15] * right_dual[e423])
                - (self[e25] * right_dual[e431])
                - (self[e35] * right_dual[e412]),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       25        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e41] * right_dual[e235])
                - (self[e42] * right_dual[e315])
                - (self[e43] * right_dual[e125])
                - (self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435])
                - (self[e45] * right_dual[e321])
                - (self[e15] * right_dual[e423])
                - (self[e25] * right_dual[e431])
                - (self[e35] * right_dual[e412]),
        );
    }
}
impl BulkContraction<DualNum> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]));
    }
}
impl BulkContraction<FlatPoint> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(
            // scalar
            -(self[e41] * right_dual[e235]) - (self[e42] * right_dual[e315]) - (self[e43] * right_dual[e125]) - (self[e45] * right_dual[e321]),
        );
    }
}
impl BulkContraction<Flector> for AntiCircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e41] * right_dual[e235]) - (self[e42] * right_dual[e315]) - (self[e43] * right_dual[e125]) - (self[e45] * right_dual[e321]),
        );
    }
}
impl BulkContraction<Motor> for AntiCircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]));
    }
}
impl BulkContraction<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       19       36        0
    //  no simd       25       63        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[scalar] * right_dual[e12345])
                    - (self[e41] * right_dual[e235])
                    - (self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e45] * right_dual[e321])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], right_dual[e4315]]) * self.group2().xyz().with_w(self[e42]))
                - (right_dual.group9().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e12345]) * self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e12345]) * self.group1().xyz(),
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
impl BulkContraction<RoundPoint> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], right_dual[e4315]]) * self.group2().xyz().with_w(self[e42]))
                - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(right_dual[e12345]) * self.group2(),
        );
    }
}
impl BulkContraction<VersorEven> for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       36        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (right_dual.group3().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (right_dual.group2().www() * self.group2().xyz()).with_w(self[e42] * right_dual[e4315]),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       10       38        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().xyz().with_w(
                    (self[scalar] * right_dual[e12345])
                        - (self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e45] * right_dual[e321])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
        );
    }
}
impl std::ops::Div<bulk_contraction> for AntiDipoleInversion {
    type Output = bulk_contraction_partial<AntiDipoleInversion>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       24        0
    //    simd3        0        3        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       30       57        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().xyz().with_w(
                    (self[e4] * right_dual[e12345])
                        - (self[e423] * right_dual[e415])
                        - (self[e431] * right_dual[e425])
                        - (self[e412] * right_dual[e435])
                        - (self[e415] * right_dual[e423])
                        - (self[e425] * right_dual[e431])
                        - (self[e435] * right_dual[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]) + (self[e1] * right_dual[e12345]),
                (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]) + (self[e2] * right_dual[e12345]),
                (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]) + (self[e3] * right_dual[e12345]),
                -(self[e435] * right_dual[e125]) - (self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) + (right_dual.group2().yzxw() * self.group0().zxy().with_w(self[e5]))
                - (right_dual.group2().zxyx() * self.group0().yzx().with_w(self[e415]))
                - (right_dual.group0().yzx() * self.group2().zxy()).with_w(self[e425] * right_dual[e315]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       37       57        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group3().yzx()) - (self.group0().yzx() * right_dual.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * right_dual[e3215]) + (self[e235] * right_dual[e1234]),
                (self[e431] * right_dual[e3215]) + (self[e315] * right_dual[e1234]),
                (self[e412] * right_dual[e3215]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual.group3().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                        - (self[e431] * right_dual[e25])
                        - (self[e412] * right_dual[e35])
                        - (self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12])
                        - (self[e321] * right_dual[e45])
                        - (self[e235] * right_dual[e41])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43]),
                )
                - (self.group2().zxy() * right_dual.group3().yzx()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(right_dual[e12345]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Scalar::from_groups(
            // scalar
            -(self[e423] * right_dual[e15]) - (self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkContraction<AntiFlector> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       24       40        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual.group1().yzx()) - (self.group0().yzx() * right_dual.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * right_dual[e3215],
                self[e431] * right_dual[e3215],
                self[e412] * right_dual[e3215],
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group1().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual.group1().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125])
                        - (self[e431] * right_dual[e25])
                        - (self[e412] * right_dual[e35])
                        - (self[e321] * right_dual[e45]),
                )
                - (self.group2().zxy() * right_dual.group1().yzx()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkContraction<AntiLine> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e321] * right_dual[e415]),
                (self[e423] * right_dual[e125]) + (self[e321] * right_dual[e425]),
                (self[e431] * right_dual[e235]) + (self[e321] * right_dual[e435]),
                -(self[e431] * right_dual[e425]) - (self[e412] * right_dual[e435]),
            ]) - (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[e423] * right_dual[e415]),
            // e5
            -(self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
        );
    }
}
impl BulkContraction<AntiMotor> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       18       42        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self
                    .group2()
                    .xyz()
                    .with_w((self[e4] * right_dual[e12345]) - (self[e423] * right_dual[e415]) - (self[e431] * right_dual[e425]) - (self[e412] * right_dual[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e321] * right_dual[e415]) + (self[e1] * right_dual[e12345]),
                (self[e321] * right_dual[e425]) + (self[e2] * right_dual[e12345]),
                (self[e321] * right_dual[e435]) + (self[e3] * right_dual[e12345]),
                -(self[e425] * right_dual[e315])
                    - (self[e435] * right_dual[e125])
                    - (self[e235] * right_dual[e415])
                    - (self[e315] * right_dual[e425])
                    - (self[e125] * right_dual[e435]),
            ]) + (self.group0().zxy() * right_dual.group1().yzx()).with_w(self[e5] * right_dual[e12345])
                - (right_dual.group1().zxyx() * self.group0().yzx().with_w(self[e415])),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       19        0
    //  no simd       17       35        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * right_dual[e3215],
                self[e431] * right_dual[e3215],
                self[e412] * right_dual[e3215],
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self[e125] * right_dual[e4315] * -1.0,
                self[e235] * right_dual[e4125] * -1.0,
                self[e315] * right_dual[e4235] * -1.0,
                (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]),
            ]) + (Simd32x4::from(right_dual[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual.group0().zxyx() * self.group2().yzx().with_w(self[e1])),
        );
    }
}
impl BulkContraction<Circle> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd        9       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e423] * right_dual[e15])
                - (self[e431] * right_dual[e25])
                - (self[e412] * right_dual[e35])
                - (self[e415] * right_dual[e23])
                - (self[e425] * right_dual[e31])
                - (self[e435] * right_dual[e12])
                - (self[e321] * right_dual[e45])
                - (self[e235] * right_dual[e41])
                - (self[e315] * right_dual[e42])
                - (self[e125] * right_dual[e43]),
        );
    }
}
impl BulkContraction<CircleRotor> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       18        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e41] * self[e235])
                - (right_dual[e42] * self[e315])
                - (right_dual[e43] * self[e125])
                - (right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435])
                - (right_dual[e45] * self[e321])
                - (right_dual[e15] * self[e423])
                - (right_dual[e25] * self[e431])
                - (right_dual[e35] * self[e412]),
        );
    }
}
impl BulkContraction<Dipole> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]),
                (self[e423] * right_dual[e125]) + (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]),
                (self[e431] * right_dual[e235]) + (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]),
                -(self[e412] * right_dual[e435]) - (self[e415] * right_dual[e423]) - (self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) - (self.group0().yzx() * right_dual.group2().zxy()).with_w(self[e423] * right_dual[e415])
                - (right_dual.group0().yzx() * self.group2().zxy()).with_w(self[e431] * right_dual[e425]),
            // e5
            -(self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       25       45        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) - (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e423] * self[e415])
                - (self.group0().yzx() * right_dual.group2().zxy()).with_w(right_dual[e431] * self[e425]),
            // e5
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
        );
    }
}
impl BulkContraction<DualNum> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e4]),
            // e15, e25, e35, e3215
            right_dual.group0().xx().with_zw(right_dual[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<FlatPoint> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e415] * right_dual[e321]),
                (self[e423] * right_dual[e125]) + (self[e425] * right_dual[e321]),
                (self[e431] * right_dual[e235]) + (self[e435] * right_dual[e321]),
                -(self[e425] * right_dual[e315]) - (self[e435] * right_dual[e125]),
            ]) - (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        );
    }
}
impl BulkContraction<Flector> for AntiDipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e415] * right_dual[e321]),
                (self[e423] * right_dual[e125]) + (self[e425] * right_dual[e321]),
                (self[e431] * right_dual[e235]) + (self[e435] * right_dual[e321]),
                -(self[e425] * right_dual[e315]) - (self[e435] * right_dual[e125]),
            ]) - (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        );
    }
}
impl BulkContraction<Line> for AntiDipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Scalar::from_groups(
            // scalar
            -(self[e423] * right_dual[e15])
                - (self[e431] * right_dual[e25])
                - (self[e412] * right_dual[e35])
                - (self[e415] * right_dual[e23])
                - (self[e425] * right_dual[e31])
                - (self[e435] * right_dual[e12]),
        );
    }
}
impl BulkContraction<Motor> for AntiDipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        6       31        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e3215], right_dual[e3215], right_dual[e3215], 1.0])
                * self.group0().with_w(
                    (self[e4] * right_dual[e3215])
                        - (self[e423] * right_dual[e15])
                        - (self[e431] * right_dual[e25])
                        - (self[e412] * right_dual[e35])
                        - (self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       50        0
    //    simd2        0        1        0
    //    simd3        4       12        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       44       69        0
    //  no simd       64      112        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e4] * right_dual[e3215]) + (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                    - (self[e423] * right_dual[e15])
                    - (self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]),
                (self[e423] * right_dual[e125]) + (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]),
                (self[e431] * right_dual[e235]) + (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]),
                -(self[e412] * right_dual[e435]) - (self[e415] * right_dual[e423]) - (self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group3().xyz().with_w(self[e4]))
                - (self.group0().yzx() * right_dual.group8().zxy()).with_w(self[e423] * right_dual[e415])
                - (right_dual.group7().yzx() * self.group2().zxy()).with_w(self[e431] * right_dual[e425]),
            // e5
            (self[e5] * right_dual[e12345])
                - (self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (right_dual.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group9().yzx()) - (self.group0().yzx() * right_dual.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e3215]) * self.group0()) + (Simd32x3::from(right_dual[e1234]) * self.group2().xyz())
                - (Simd32x3::from(self[e321]) * right_dual.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<RoundPoint> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        2        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       24       43        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * right_dual[e3215]) + (self[e235] * right_dual[e1234]),
                (self[e431] * right_dual[e3215]) + (self[e315] * right_dual[e1234]),
                (self[e412] * right_dual[e3215]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self[e125] * right_dual[e4315] * -1.0,
                self[e235] * right_dual[e4125] * -1.0,
                self[e315] * right_dual[e4235] * -1.0,
                (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
            ]) + (Simd32x4::from(right_dual[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual.group0().zxyx() * self.group2().yzx().with_w(self[e1])),
        );
    }
}
impl BulkContraction<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(right_dual[e12345]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<VersorEven> for AntiDipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       61        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group3().yzx()) - (self.group0().yzx() * right_dual.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * right_dual[e3215]) + (self[e235] * right_dual[e1234]),
                (self[e431] * right_dual[e3215]) + (self[e315] * right_dual[e1234]),
                (self[e412] * right_dual[e3215]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(right_dual[e3215]) * self.group1().xyz().with_w(self[e4]))
                + (right_dual.group3().zxyx() * self.group2().yzx().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                        - (self[e431] * right_dual[e25])
                        - (self[e412] * right_dual[e35])
                        - (self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12])
                        - (self[e321] * right_dual[e45])
                        - (self[e235] * right_dual[e41])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43]),
                )
                - (self.group2().zxy() * right_dual.group3().yzx()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        3        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       21       35        0
    //  no simd       30       62        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().xyz().with_w(
                    (self[e4] * right_dual[e12345])
                        - (self[e423] * right_dual[e415])
                        - (self[e431] * right_dual[e425])
                        - (self[e412] * right_dual[e435])
                        - (self[e415] * right_dual[e423])
                        - (self[e425] * right_dual[e431])
                        - (self[e435] * right_dual[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]) + (self[e1] * right_dual[e12345]),
                (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]) + (self[e2] * right_dual[e12345]),
                (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]) + (self[e3] * right_dual[e12345]),
                -(self[e435] * right_dual[e125]) - (self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) + (self.group0().zxy() * right_dual.group2().yzx()).with_w(self[e5] * right_dual[e12345])
                - (right_dual.group2().zxyx() * self.group0().yzx().with_w(self[e415]))
                - (self.group2().zxy() * right_dual.group0().yzx()).with_w(self[e425] * right_dual[e315]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for AntiDualNum {
    type Output = bulk_contraction_partial<AntiDualNum>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().xx().with_zw(self[e3215], self[scalar]) * right_dual.group0().with_w(right_dual[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * right_dual.group1().xyz().with_w(right_dual[e12345]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        7        0
    // no simd        0       28        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual.group3().xyz().with_w(right_dual[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiFlatPoint> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * right_dual[e45], 1.0]) * Simd32x2::from([-1.0, 0.0]));
    }
}
impl BulkContraction<AntiFlector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       22        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0().xx().with_zw(self[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(self[e3215] * right_dual[e45] * -1.0),
        );
    }
}
impl BulkContraction<AntiLine> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            self.group0().xx().with_zw(self[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<AntiMotor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            self.group0().xx().with_zw(self[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl BulkContraction<Circle> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkContraction<CircleRotor> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkContraction<Dipole> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e3215]) * right_dual.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e3215]) * right_dual.group1().xyz(),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       31        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e4]),
            // e15, e25, e35, e3215
            self.group0().xx().with_zw(self[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd2        0        1        0
    //    simd3        0        5        0
    //    simd4        0        9        0
    // Totals...
    // yes simd        1       24        0
    //  no simd        1       62        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e3215] * right_dual[e4]) + (self[scalar] * right_dual[e12345]), 0.0]),
            // e1, e2, e3, e4
            self.group0().xx().with_zw(self[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group4().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            self[e3215] * right_dual[e45] * -1.0,
            // e15, e25, e35, e45
            self.group0().xx().with_zw(self[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group6().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e3215]) * right_dual.group7(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e3215] * right_dual[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215]) * right_dual.group9().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215] * right_dual[e12345]),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<RoundPoint> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual.group0().xyz().with_w(right_dual[e1234]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkContraction<Scalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<Sphere> for AntiDualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Scalar::from_groups(/* scalar */ self[e3215] * right_dual[e4]);
    }
}
impl BulkContraction<VersorEven> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual.group3().xyz().with_w(right_dual[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * right_dual.group0().xyz().with_w(right_dual[e45]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       26        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e423], right_dual[e431], right_dual[e412], 1.0])
                * self.group0().xx().with_zw(self[e3215], (self[e3215] * right_dual[e4]) + (self[scalar] * right_dual[e12345])),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e3215]) * right_dual.group1().xyz().with_w(right_dual[e12345]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for AntiFlatPoint {
    type Output = bulk_contraction_partial<AntiFlatPoint>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        8       27        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * right_dual[e412]) + (self[e321] * right_dual[e415]),
                (self[e125] * right_dual[e423]) + (self[e321] * right_dual[e425]),
                (self[e235] * right_dual[e431]) + (self[e321] * right_dual[e435]),
                -(self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) - (right_dual.group0().yzx() * self.group0().zxy()).with_w(self[e235] * right_dual[e415]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       28        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e235] * right_dual[e1234],
                self[e315] * right_dual[e1234],
                self[e125] * right_dual[e1234],
                -(self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
            ]) - (self.group0().www() * right_dual.group3().xyz()).with_w(self[e235] * right_dual[e41]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiFlatPoint> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Scalar::from_groups(/* scalar */ self[e321] * right_dual[e45] * -1.0);
    }
}
impl BulkContraction<AntiFlector> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       22        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * right_dual.group1().xyz().with_w(right_dual[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<AntiLine> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       13        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * right_dual
                    .group0()
                    .with_w(-(self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435])),
        );
    }
}
impl BulkContraction<AntiMotor> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       19        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([right_dual[e415], right_dual[e425], right_dual[e435], 1.0])
                * self
                    .group0()
                    .www()
                    .with_w(-(self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435])),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e321]) * right_dual.group0().xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
        );
    }
}
impl BulkContraction<Circle> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkContraction<CircleRotor> for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e41] * self[e235]) - (right_dual[e42] * self[e315]) - (right_dual[e43] * self[e125]) - (right_dual[e45] * self[e321]),
        );
    }
}
impl BulkContraction<Dipole> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       22        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * right_dual[e412]) + (self[e321] * right_dual[e415]),
                (self[e125] * right_dual[e423]) + (self[e321] * right_dual[e425]),
                (self[e235] * right_dual[e431]) + (self[e321] * right_dual[e435]),
                -(self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) - (right_dual.group0().yzx() * self.group0().zxy()).with_w(self[e235] * right_dual[e415]),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiFlatPoint {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       27        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]),
                -(right_dual[e425] * self[e315]) - (right_dual[e435] * self[e125]),
            ]) - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e415])),
        );
    }
}
impl BulkContraction<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd2        0        1        0
    //    simd3        2       10        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       24        0
    //  no simd       19       54        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * right_dual.group6().xyz()).with_w(0.0) + (right_dual.group7().zxy() * self.group0().yzx()).with_w(0.0)
                - (right_dual.group7().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            -(self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            ((self.group0().yzx() * right_dual.group9().zxy()) - (self.group0().zxy() * right_dual.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321] * right_dual[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<RoundPoint> for AntiFlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        6       17        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group0().xyz()),
            // e15, e25, e35
            (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
        );
    }
}
impl BulkContraction<Scalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<VersorEven> for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       32        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e235] * right_dual[e1234],
                self[e315] * right_dual[e1234],
                self[e125] * right_dual[e1234],
                -(self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
            ]) - (self.group0().wwwx() * right_dual.group3().xyz().with_w(right_dual[e41])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiFlatPoint {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        6        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        8       32        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * right_dual[e412]) + (self[e321] * right_dual[e415]),
                (self[e125] * right_dual[e423]) + (self[e321] * right_dual[e425]),
                (self[e235] * right_dual[e431]) + (self[e321] * right_dual[e435]),
                -(self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e415])),
        );
    }
}
impl std::ops::Div<bulk_contraction> for AntiFlector {
    type Output = bulk_contraction_partial<AntiFlector>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       12       31        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e321] * right_dual[e415]) + (self[e1] * right_dual[e12345]),
                (self[e321] * right_dual[e425]) + (self[e2] * right_dual[e12345]),
                (self[e321] * right_dual[e435]) + (self[e3] * right_dual[e12345]),
                -(self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) + (right_dual.group0().zxy() * self.group0().yzx()).with_w(self[e5] * right_dual[e12345])
                - (right_dual.group0().yzx() * self.group0().zxy()).with_w(self[e235] * right_dual[e415]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       16       32        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43])
                    - (self[e321] * right_dual[e45]),
            ) + (right_dual.group2().www() * self.group0().xyz()).with_w(self[e1] * right_dual[e4235])
                - (self.group0().www() * right_dual.group3().xyz()).with_w(self[e235] * right_dual[e41]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Scalar::from_groups(/* scalar */ self[e321] * right_dual[e45] * -1.0);
    }
}
impl BulkContraction<AntiFlector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       26        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e4235], right_dual[e4315], right_dual[e4125], 1.0])
                * self
                    .group0()
                    .www()
                    .with_w((self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) - (self[e321] * right_dual[e45]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<AntiLine> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       13        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * right_dual
                    .group0()
                    .with_w(-(self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435])),
        );
    }
}
impl BulkContraction<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       22        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e1] * right_dual[e12345],
                self[e2] * right_dual[e12345],
                self[e3] * right_dual[e12345],
                -(self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) + (right_dual.group0() * self.group0().www().with_w(self[e5])),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        5       21        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e4235], right_dual[e4315], right_dual[e4125], 1.0])
                * self
                    .group0()
                    .www()
                    .with_w((self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Circle> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkContraction<CircleRotor> for AntiFlector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e41] * self[e235]) - (right_dual[e42] * self[e315]) - (right_dual[e43] * self[e125]) - (right_dual[e45] * self[e321]),
        );
    }
}
impl BulkContraction<Dipole> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       22        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e315] * right_dual[e412]) + (self[e321] * right_dual[e415]),
                (self[e125] * right_dual[e423]) + (self[e321] * right_dual[e425]),
                (self[e235] * right_dual[e431]) + (self[e321] * right_dual[e435]),
                -(self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) - (right_dual.group0().yzx() * self.group0().zxy()).with_w(self[e235] * right_dual[e415]),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiFlector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       27        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]),
                -(right_dual[e425] * self[e315]) - (right_dual[e435] * self[e125]),
            ]) - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e415])),
        );
    }
}
impl BulkContraction<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd2        0        1        0
    //    simd3        2       11        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       28       62        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43])
                    - (self[e321] * right_dual[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * right_dual.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz()).with_w(0.0)
                + (right_dual.group7().zxy() * self.group0().yzx()).with_w(0.0)
                - (right_dual.group7().yzx() * self.group0().zxy()).with_w(0.0),
            // e5
            (self[e5] * right_dual[e12345]) - (self[e235] * right_dual[e415]) - (self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            ((self.group0().yzx() * right_dual.group9().zxy()) - (self.group0().zxy() * right_dual.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321] * right_dual[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<RoundPoint> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        9       24        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e321] * right_dual[e4235] * -1.0,
                self[e321] * right_dual[e4315] * -1.0,
                self[e321] * right_dual[e4125] * -1.0,
                (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
            ]) + (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], right_dual[e4235]]) * self.group0().xyz().with_w(self[e1])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Scalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<VersorEven> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       16       36        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43])
                    - (self[e321] * right_dual[e45]),
            ) + (right_dual.group2().www() * self.group0().xyz()).with_w(self[e1] * right_dual[e4235])
                - (self.group0().wwwx() * right_dual.group3().xyz().with_w(right_dual[e41])),
            // e15, e25, e35, e3215
            ((self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       36        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e321] * right_dual[e415]) + (self[e1] * right_dual[e12345]),
                (self[e321] * right_dual[e425]) + (self[e2] * right_dual[e12345]),
                (self[e321] * right_dual[e435]) + (self[e3] * right_dual[e12345]),
                -(self[e315] * right_dual[e425]) - (self[e125] * right_dual[e435]),
            ]) + (right_dual.group0().zxyw() * self.group0().yzx().with_w(self[e5]))
                - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e415])),
        );
    }
}
impl std::ops::Div<bulk_contraction> for AntiLine {
    type Output = bulk_contraction_partial<AntiLine>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       33        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group0().with_w(
                    -(self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * right_dual.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * right_dual[e4315]) - (self[e15] * right_dual[e1234]),
                -(self[e23] * right_dual[e4125]) - (self[e25] * right_dual[e1234]),
                -(self[e31] * right_dual[e4235]) - (self[e35] * right_dual[e1234]),
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group3().zxyx() * self.group0().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<AntiFlector> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       20        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * right_dual[e4315] * -1.0,
                self[e23] * right_dual[e4125] * -1.0,
                self[e31] * right_dual[e4235] * -1.0,
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group1().zxyx() * self.group0().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<AntiLine> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Scalar::from_groups(/* scalar */ -(self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435]));
    }
}
impl BulkContraction<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        2       27        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group0().with_w(-(self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435])),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        5       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * right_dual[e4315] * -1.0,
                self[e23] * right_dual[e4125] * -1.0,
                self[e31] * right_dual[e4235] * -1.0,
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<Dipole> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435])
                - (self[e15] * right_dual[e423])
                - (self[e25] * right_dual[e431])
                - (self[e35] * right_dual[e412]),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiLine {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       21        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e423] * self[e15])
                - (right_dual[e431] * self[e25])
                - (right_dual[e412] * self[e35])
                - (right_dual[e415] * self[e23])
                - (right_dual[e425] * self[e31])
                - (right_dual[e435] * self[e12]),
        );
    }
}
impl BulkContraction<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd2        0        1        0
    //    simd3        0        6        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        9       24        0
    //  no simd       15       55        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0().yzx() * right_dual.group9().zxy()).with_w(0.0)
                - (Simd32x3::from(right_dual[e1234]) * self.group1()).with_w(0.0)
                - (self.group0().zxy() * right_dual.group9().yzx()).with_w(0.0),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            // e15, e25, e35, e45
            right_dual.group0().yy().with_zw(right_dual[e12345], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_dual[e12345]) * self.group0(),
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
impl BulkContraction<RoundPoint> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       17        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * right_dual[e4315]) - (self[e15] * right_dual[e1234]),
                -(self[e23] * right_dual[e4125]) - (self[e25] * right_dual[e1234]),
                -(self[e31] * right_dual[e4235]) - (self[e35] * right_dual[e1234]),
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<Scalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<VersorEven> for AntiLine {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       28        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * right_dual[e4315]) - (self[e15] * right_dual[e1234]),
                -(self[e23] * right_dual[e4125]) - (self[e25] * right_dual[e1234]),
                -(self[e31] * right_dual[e4235]) - (self[e35] * right_dual[e1234]),
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group3().zxyx() * self.group0().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        5       38        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group0().with_w(
                    -(self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for AntiMotor {
    type Output = bulk_contraction_partial<AntiMotor>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7       17        0
    //  no simd       12       31        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e23] * right_dual[e12345],
                self[e31] * right_dual[e12345],
                self[e12] * right_dual[e12345],
                -(self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
            ]) + (right_dual.group0() * self.group1().www()).with_w(self[scalar] * right_dual[e12345]),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e3215]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz())).with_w(self[e3215] * right_dual[e12345]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       12       36        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual.group3().xyz().with_w(right_dual[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * right_dual[e4315]) - (self[e15] * right_dual[e1234]),
                -(self[e23] * right_dual[e4125]) - (self[e25] * right_dual[e1234]),
                -(self[e31] * right_dual[e4235]) - (self[e35] * right_dual[e1234]),
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group3().zxyx() * self.group0().yzx().with_w(self[e15]))
                - (right_dual.group0() * self.group1().www()).with_w(self[e3215] * right_dual[e45]),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for AntiMotor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * right_dual[e45], 1.0]) * Simd32x2::from([-1.0, 0.0]));
    }
}
impl BulkContraction<AntiFlector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        1        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        9       30        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e1, e2, e3, e5
            (right_dual.group1().zxyx() * self.group0().yzx().with_w(self[e15])) + Simd32x3::from(0.0).with_w((self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]))
                - (self.group0().zxy() * right_dual.group1().yzx()).with_w(self[e3215] * right_dual[e45]),
        );
    }
}
impl BulkContraction<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        2       21        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435])),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group1().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       23        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w((self[scalar] * right_dual[e12345]) - (self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e3215]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz())).with_w(self[e3215] * right_dual[e12345]),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        2       13        0
    //  no simd        5       28        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * right_dual[e4315] * -1.0,
                self[e23] * right_dual[e4125] * -1.0,
                self[e31] * right_dual[e4235] * -1.0,
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<Circle> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       12        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            (right_dual.group0() * self.group1().www() * Simd32x3::from(-1.0)).with_w(self[e3215] * right_dual[e45] * -1.0),
        );
    }
}
impl BulkContraction<CircleRotor> for AntiMotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkContraction<Dipole> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       32        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0])
                * right_dual.group0().with_w(
                    -(self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        1        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        6       15        0
    //  no simd        6       38        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e3215], self[e3215], self[e3215], 1.0])
                * right_dual.group0().with_w(
                    (right_dual[e4] * self[e3215])
                        - (right_dual[e423] * self[e15])
                        - (right_dual[e431] * self[e25])
                        - (right_dual[e412] * self[e35])
                        - (right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd2        0        1        0
    //    simd3        2       12        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       33        0
    //  no simd       28       67        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[scalar] * right_dual[e12345]) + (self[e3215] * right_dual[e4])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0().yzx() * right_dual.group9().zxy()).with_w(0.0)
                - (Simd32x3::from(self[e3215]) * right_dual.group4()).with_w(0.0)
                - (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()).with_w(0.0)
                - (self.group0().zxy() * right_dual.group9().yzx()).with_w(0.0),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) - (self[e3215] * right_dual[e45]),
            // e15, e25, e35, e45
            ((Simd32x3::from(self[e3215]) * right_dual.group6().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * right_dual.group7()) + (Simd32x3::from(right_dual[e12345]) * self.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e3215] * right_dual[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e3215]) * right_dual.group9().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e3215] * right_dual[e12345]),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<RoundPoint> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual.group0().xyz().with_w(right_dual[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * right_dual[e4315]) - (self[e15] * right_dual[e1234]),
                -(self[e23] * right_dual[e4125]) - (self[e25] * right_dual[e1234]),
                -(self[e31] * right_dual[e4235]) - (self[e35] * right_dual[e1234]),
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<Sphere> for AntiMotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Scalar::from_groups(/* scalar */ self[e3215] * right_dual[e4]);
    }
}
impl BulkContraction<VersorEven> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        6       17        0
    //  no simd       12       40        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e3215]) * right_dual.group3().xyz().with_w(right_dual[e1234]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e15] * right_dual[e1234]) - (self[e3215] * right_dual[e41]),
                -(self[e25] * right_dual[e1234]) - (self[e3215] * right_dual[e42]),
                -(self[e35] * right_dual[e1234]) - (self[e3215] * right_dual[e43]),
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (right_dual.group3().zxyx() * self.group0().yzx().with_w(self[e15]))
                - (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e3215] * right_dual[e45]),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       16       37        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e3215]) * right_dual.group0().xyz().with_w(right_dual[e4]))
                + (Simd32x4::from(right_dual[e12345]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    -(self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e3215]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz())).with_w(self[e3215] * right_dual[e12345]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for AntiPlane {
    type Output = bulk_contraction_partial<AntiPlane>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       16        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiFlector> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(/* scalar */ (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]));
    }
}
impl BulkContraction<AntiMotor> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiPlane> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Scalar::from_groups(/* scalar */ (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]));
    }
}
impl BulkContraction<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        3       39        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            right_dual.group0().yy().with_zw(right_dual[e12345], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e5
            self[e5] * right_dual[e12345],
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
impl BulkContraction<RoundPoint> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        9        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Scalar::from_groups(
            // scalar
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkContraction<Scalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<VersorEven> for AntiPlane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       20        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl std::ops::Div<bulk_contraction> for AntiScalar {
    type Output = bulk_contraction_partial<AntiScalar>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for AntiScalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self[e12345]) * right_dual.group2(),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for AntiScalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       27        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<AntiDualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<AntiFlatPoint> for AntiScalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<AntiFlector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiLine> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiMotor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiPlane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<AntiScalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Scalar::from_groups(/* scalar */ self[e12345] * right_dual[scalar]);
    }
}
impl BulkContraction<Circle> for AntiScalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * right_dual.group2(),
        );
    }
}
impl BulkContraction<CircleRotor> for AntiScalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(self[e12345]) * right_dual.group2(),
        );
    }
}
impl BulkContraction<Dipole> for AntiScalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group2(),
        );
    }
}
impl BulkContraction<DipoleInversion> for AntiScalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       30        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * right_dual.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<DualNum> for AntiScalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<FlatPoint> for AntiScalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<Flector> for AntiScalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<Line> for AntiScalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<Motor> for AntiScalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0       19        0
    //  no simd        0       54        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e5
            self[e12345] * right_dual[e5],
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual.group3(),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group4(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group6(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group7(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group9(),
            // e1234
            self[e12345] * right_dual[e1234],
        );
    }
}
impl BulkContraction<Plane> for AntiScalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<RoundPoint> for AntiScalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1234
            self[e12345] * right_dual[e1234],
        );
    }
}
impl BulkContraction<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiScalar::from_groups(/* e12345 */ right_dual[e12345] * self[e12345]);
    }
}
impl BulkContraction<Sphere> for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e5
            self[e12345] * right_dual[e5],
        );
    }
}
impl BulkContraction<VersorEven> for AntiScalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<VersorOdd> for AntiScalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[e12345]) * right_dual.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl std::ops::Div<bulk_contraction> for Circle {
    type Output = bulk_contraction_partial<Circle>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       52        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().with_w(
                    -(self[e423] * right_dual[e415])
                        - (self[e431] * right_dual[e425])
                        - (self[e412] * right_dual[e435])
                        - (self[e415] * right_dual[e423])
                        - (self[e425] * right_dual[e431])
                        - (self[e435] * right_dual[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]),
                (self[e423] * right_dual[e125]) + (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]),
                (self[e431] * right_dual[e235]) + (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]),
                -(self[e415] * right_dual[e235]) - (self[e425] * right_dual[e315]) - (self[e435] * right_dual[e125]) - (self[e125] * right_dual[e435]),
            ]) - (self.group0().yzx() * right_dual.group2().zxy()).with_w(self[e235] * right_dual[e415])
                - (self.group2().zxy() * right_dual.group0().yzx()).with_w(self[e315] * right_dual[e425]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       29       52        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group3().yzx()) - (self.group0().yzx() * right_dual.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * right_dual[e3215]) + (self[e235] * right_dual[e1234]),
                (self[e431] * right_dual[e3215]) + (self[e315] * right_dual[e1234]),
                (self[e412] * right_dual[e3215]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
            ]) - (self.group2().zxy() * right_dual.group3().yzx()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkContraction<AntiDualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group2(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Scalar::from_groups(
            // scalar
            -(self[e423] * right_dual[e15]) - (self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkContraction<AntiFlector> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       17       36        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual.group1().yzx()) - (self.group0().yzx() * right_dual.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * right_dual[e3215],
                self[e431] * right_dual[e3215],
                self[e412] * right_dual[e3215],
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group1().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
            ]) - (self.group2().zxy() * right_dual.group1().yzx()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkContraction<AntiLine> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       24        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e321] * right_dual[e415]),
                (self[e423] * right_dual[e125]) + (self[e321] * right_dual[e425]),
                (self[e431] * right_dual[e235]) + (self[e321] * right_dual[e435]),
                -(self[e431] * right_dual[e425]) - (self[e412] * right_dual[e435]),
            ]) - (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[e423] * right_dual[e415]),
            // e5
            -(self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
        );
    }
}
impl BulkContraction<AntiMotor> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       37        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self
                    .group2()
                    .with_w(-(self[e423] * right_dual[e415]) - (self[e431] * right_dual[e425]) - (self[e412] * right_dual[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e321] * right_dual[e415]),
                (self[e423] * right_dual[e125]) + (self[e321] * right_dual[e425]),
                (self[e431] * right_dual[e235]) + (self[e321] * right_dual[e435]),
                -(self[e415] * right_dual[e235])
                    - (self[e425] * right_dual[e315])
                    - (self[e435] * right_dual[e125])
                    - (self[e315] * right_dual[e425])
                    - (self[e125] * right_dual[e435]),
            ]) - (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[e235] * right_dual[e415]),
        );
    }
}
impl BulkContraction<AntiPlane> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       14       28        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Dipole::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * right_dual[e3215],
                self[e431] * right_dual[e3215],
                self[e412] * right_dual[e3215],
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) + (self.group2().yzx() * right_dual.group0().zxy()) - (self.group2().zxy() * right_dual.group0().yzx()),
        );
    }
}
impl BulkContraction<Circle> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd        9       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e423] * right_dual[e15])
                - (self[e431] * right_dual[e25])
                - (self[e412] * right_dual[e35])
                - (self[e415] * right_dual[e23])
                - (self[e425] * right_dual[e31])
                - (self[e435] * right_dual[e12])
                - (self[e321] * right_dual[e45])
                - (self[e235] * right_dual[e41])
                - (self[e315] * right_dual[e42])
                - (self[e125] * right_dual[e43]),
        );
    }
}
impl BulkContraction<CircleRotor> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       12        0
    //  no simd        9       18        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e41] * self[e235])
                - (right_dual[e42] * self[e315])
                - (right_dual[e43] * self[e125])
                - (right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435])
                - (right_dual[e45] * self[e321])
                - (right_dual[e15] * self[e423])
                - (right_dual[e25] * self[e431])
                - (right_dual[e35] * self[e412]),
        );
    }
}
impl BulkContraction<Dipole> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) - (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e423] * self[e415])
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(right_dual[e431] * self[e425]),
            // e5
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
        );
    }
}
impl BulkContraction<DipoleInversion> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       25       45        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) - (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e423] * self[e415])
                - (self.group0().yzx() * right_dual.group2().zxy()).with_w(right_dual[e431] * self[e425]),
            // e5
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
        );
    }
}
impl BulkContraction<DualNum> for Circle {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual[e3215]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_dual[e3215]) * self.group1().xyz(),
        );
    }
}
impl BulkContraction<FlatPoint> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e315] * self[e412]) + (right_dual[e321] * self[e415]),
                (right_dual[e125] * self[e423]) + (right_dual[e321] * self[e425]),
                (right_dual[e235] * self[e431]) + (right_dual[e321] * self[e435]),
                -(right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) - (self.group0().yzx() * right_dual.group0().zxy()).with_w(right_dual[e235] * self[e415]),
        );
    }
}
impl BulkContraction<Flector> for Circle {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e315] * self[e412]) + (right_dual[e321] * self[e415]),
                (right_dual[e125] * self[e423]) + (right_dual[e321] * self[e425]),
                (right_dual[e235] * self[e431]) + (right_dual[e321] * self[e435]),
                -(right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) - (self.group0().yzx() * right_dual.group0().zxy()).with_w(right_dual[e235] * self[e415]),
        );
    }
}
impl BulkContraction<Line> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Scalar::from_groups(
            // scalar
            -(right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435])
                - (right_dual[e15] * self[e423])
                - (right_dual[e25] * self[e431])
                - (right_dual[e35] * self[e412]),
        );
    }
}
impl BulkContraction<Motor> for Circle {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       30        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e3215], right_dual[e3215], right_dual[e3215], 1.0])
                * self.group0().with_w(
                    -(right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       44        0
    //    simd2        0        1        0
    //    simd3        4       12        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       37       62        0
    //  no simd       54      102        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e423] * right_dual[e15])
                    - (self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]),
                (self[e423] * right_dual[e125]) + (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]),
                (self[e431] * right_dual[e235]) + (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]),
                -(self[e412] * right_dual[e435]) - (self[e415] * right_dual[e423]) - (self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) - (self.group0().yzx() * right_dual.group8().zxy()).with_w(self[e423] * right_dual[e415])
                - (self.group2().zxy() * right_dual.group7().yzx()).with_w(self[e431] * right_dual[e425]),
            // e5
            -(self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (right_dual.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group9().yzx()) - (self.group0().yzx() * right_dual.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e3215]) * self.group0()) + (Simd32x3::from(right_dual[e1234]) * self.group2()) - (Simd32x3::from(self[e321]) * right_dual.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<RoundPoint> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       20       35        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * right_dual[e3215]) + (self[e235] * right_dual[e1234]),
                (self[e431] * right_dual[e3215]) + (self[e315] * right_dual[e1234]),
                (self[e412] * right_dual[e3215]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) + (self.group2().yzx() * right_dual.group0().zxy()) - (self.group2().zxy() * right_dual.group0().yzx()),
        );
    }
}
impl BulkContraction<Scalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group2(),
        );
    }
}
impl BulkContraction<VersorEven> for Circle {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       29       56        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group3().yzx()) - (self.group0().yzx() * right_dual.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * right_dual[e3215]) + (self[e235] * right_dual[e1234]),
                (self[e431] * right_dual[e3215]) + (self[e315] * right_dual[e1234]),
                (self[e412] * right_dual[e3215]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
            ]) - (self.group2().zxy() * right_dual.group3().yzx()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkContraction<VersorOdd> for Circle {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       25       57        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().with_w(
                    -(self[e423] * right_dual[e415])
                        - (self[e431] * right_dual[e425])
                        - (self[e412] * right_dual[e435])
                        - (self[e415] * right_dual[e423])
                        - (self[e425] * right_dual[e431])
                        - (self[e435] * right_dual[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]),
                (self[e423] * right_dual[e125]) + (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]),
                (self[e431] * right_dual[e235]) + (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]),
                -(self[e415] * right_dual[e235]) - (self[e425] * right_dual[e315]) - (self[e435] * right_dual[e125]) - (self[e125] * right_dual[e435]),
            ]) - (self.group0().yzx() * right_dual.group2().zxy()).with_w(self[e235] * right_dual[e415])
                - (self.group2().zxy() * right_dual.group0().yzx()).with_w(self[e315] * right_dual[e425]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for CircleRotor {
    type Output = bulk_contraction_partial<CircleRotor>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       31        0
    //    simd3        1        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       35       62        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(right_dual[e12345]) * self.group0()) + (Simd32x3::from(self[e12345]) * right_dual.group0())).with_w(right_dual[e12345] * self[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(right_dual[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * right_dual.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (right_dual[e235] * self[e12345]) + (right_dual[e12345] * self[e235]),
                (right_dual[e315] * self[e12345]) + (right_dual[e12345] * self[e315]),
                (right_dual[e125] * self[e12345]) + (right_dual[e12345] * self[e125]),
                -(right_dual[e415] * self[e235])
                    - (right_dual[e425] * self[e315])
                    - (right_dual[e435] * self[e125])
                    - (right_dual[e235] * self[e415])
                    - (right_dual[e315] * self[e425])
                    - (right_dual[e125] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) - (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e423] * self[e415])
                - (self.group0().yzx() * right_dual.group2().zxy()).with_w(right_dual[e431] * self[e425]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        6        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       39       67        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self[e412] * right_dual[e4315]) + (self[e415] * right_dual[e1234]) + (self[e12345] * right_dual[e41]),
                (self[e423] * right_dual[e4125]) + (self[e425] * right_dual[e1234]) + (self[e12345] * right_dual[e42]),
                (self[e431] * right_dual[e4235]) + (self[e435] * right_dual[e1234]) + (self[e12345] * right_dual[e43]),
                -(self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
            ]) - (self.group0().yzx() * right_dual.group3().zxy()).with_w(self[e423] * right_dual[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e235] * right_dual[e1234]) + (self[e12345] * right_dual[e23]),
                (self[e315] * right_dual[e1234]) + (self[e12345] * right_dual[e31]),
                (self[e125] * right_dual[e1234]) + (self[e12345] * right_dual[e12]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) + (self.group0() * right_dual.group3().www()).with_w(self[e12345] * right_dual[e45])
                - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e3215]) * self.group1().xyz())
                + (self.group2().yzx() * right_dual.group3().zxy())
                - (self.group2().zxy() * right_dual.group3().yzx()))
            .with_w(self[e12345] * right_dual[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<AntiDualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual[e12345]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e5
            self.group2() * right_dual.group0().yy().with_zw(right_dual[e12345], right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e12345] * right_dual[e45]),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e15], right_dual[e25], right_dual[e35], 1.0])
                * self
                    .group2()
                    .www()
                    .with_w(-(self[e423] * right_dual[e15]) - (self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45])),
        );
    }
}
impl BulkContraction<AntiFlector> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd3        0        6        0
    //    simd4        6        4        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       27       44        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e412] * right_dual[e4315],
                self[e423] * right_dual[e4125],
                self[e431] * right_dual[e4235],
                -(self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
            ]) - (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[e423] * right_dual[e15]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]))
                + (self.group0() * right_dual.group1().www()).with_w(self[e12345] * right_dual[e45])
                - (self.group1().wwwx() * right_dual.group1().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e12345]) * right_dual.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[e3215]) * self.group1().xyz()).with_w(0.0)
                + (self.group2().yzx() * right_dual.group1().zxy()).with_w(0.0)
                - (self.group2().zxy() * right_dual.group1().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiLine> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       40        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual
                    .group1()
                    .with_w(-(self[e423] * right_dual[e415]) - (self[e431] * right_dual[e425]) - (self[e412] * right_dual[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e321] * right_dual[e415]),
                (self[e423] * right_dual[e125]) + (self[e321] * right_dual[e425]),
                (self[e431] * right_dual[e235]) + (self[e321] * right_dual[e435]),
                -(self[e415] * right_dual[e235])
                    - (self[e425] * right_dual[e315])
                    - (self[e435] * right_dual[e125])
                    - (self[e315] * right_dual[e425])
                    - (self[e125] * right_dual[e435]),
            ]) - (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[e235] * right_dual[e415]),
        );
    }
}
impl BulkContraction<AntiMotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        1        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       12       26        0
    //  no simd       20       44        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual[e12345]) * self.group0().with_w(self[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e12345]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz())).with_w(self[e321] * right_dual[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from([
                self[e12345] * right_dual[e235],
                self[e12345] * right_dual[e315],
                self[e12345] * right_dual[e125],
                -(self[e415] * right_dual[e235])
                    - (self[e425] * right_dual[e315])
                    - (self[e435] * right_dual[e125])
                    - (self[e235] * right_dual[e415])
                    - (self[e315] * right_dual[e425])
                    - (self[e125] * right_dual[e435]),
            ]) + (self.group2() * right_dual.group0().www().with_w(right_dual[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e315]) + (self[e321] * right_dual[e415]),
                (self[e423] * right_dual[e125]) + (self[e321] * right_dual[e425]),
                (self[e431] * right_dual[e235]) + (self[e321] * right_dual[e435]),
                -(self[e431] * right_dual[e425]) - (self[e412] * right_dual[e435]),
            ]) - (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[e423] * right_dual[e415]),
        );
    }
}
impl BulkContraction<AntiPlane> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       16       32        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e423] * right_dual[e3215],
                self[e431] * right_dual[e3215],
                self[e412] * right_dual[e3215],
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual[e3215]) * self.group1().xyz()).with_w(0.0) + (self.group2().yzx() * right_dual.group0().zxy()).with_w(0.0)
                - (self.group2().zxy() * right_dual.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<AntiScalar> for CircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Scalar::from_groups(/* scalar */ self[e12345] * right_dual[scalar]);
    }
}
impl BulkContraction<Circle> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       25        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().with_w(
                    -(self[e423] * right_dual[e15])
                        - (self[e431] * right_dual[e25])
                        - (self[e412] * right_dual[e35])
                        - (self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12])
                        - (self[e321] * right_dual[e45])
                        - (self[e235] * right_dual[e41])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43]),
                ),
        );
    }
}
impl BulkContraction<CircleRotor> for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       30        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().xyz().with_w(
                    (right_dual[scalar] * self[e12345])
                        - (right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e45] * self[e321])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
        );
    }
}
impl BulkContraction<Dipole> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       51        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().with_w(
                    -(right_dual[e423] * self[e415])
                        - (right_dual[e431] * self[e425])
                        - (right_dual[e412] * self[e435])
                        - (right_dual[e415] * self[e423])
                        - (right_dual[e425] * self[e431])
                        - (right_dual[e435] * self[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e415] * self[e235]) - (right_dual[e425] * self[e315]) - (right_dual[e435] * self[e125]) - (right_dual[e125] * self[e435]),
            ]) - (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e235] * self[e415])
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(right_dual[e315] * self[e425]),
        );
    }
}
impl BulkContraction<DipoleInversion> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       24        0
    //    simd3        0        3        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       30       61        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().xyz().with_w(
                    (right_dual[e4] * self[e12345])
                        - (right_dual[e423] * self[e415])
                        - (right_dual[e431] * self[e425])
                        - (right_dual[e412] * self[e435])
                        - (right_dual[e415] * self[e423])
                        - (right_dual[e425] * self[e431])
                        - (right_dual[e435] * self[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]) + (right_dual[e1] * self[e12345]),
                (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]) + (right_dual[e2] * self[e12345]),
                (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]) + (right_dual[e3] * self[e12345]),
                -(right_dual[e435] * self[e125]) - (right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) + (self.group2().yzxw() * right_dual.group0().zxy().with_w(right_dual[e5]))
                - (self.group2().zxyx() * right_dual.group0().yzx().with_w(right_dual[e415]))
                - (self.group0().yzx() * right_dual.group2().zxy()).with_w(right_dual[e425] * self[e315]),
        );
    }
}
impl BulkContraction<DualNum> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_dual.group0().xx().with_zw(right_dual[e3215], right_dual[scalar]) * self.group0().with_w(self[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual[e3215]) * self.group1().xyz().with_w(self[e12345]),
        );
    }
}
impl BulkContraction<FlatPoint> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e315] * self[e412]) + (right_dual[e321] * self[e415]),
                (right_dual[e125] * self[e423]) + (right_dual[e321] * self[e425]),
                (right_dual[e235] * self[e431]) + (right_dual[e321] * self[e435]),
                -(right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) - (self.group0().yzx() * right_dual.group0().zxy()).with_w(right_dual[e235] * self[e415]),
        );
    }
}
impl BulkContraction<Flector> for CircleRotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e321] * self[e415]) + (right_dual[e1] * self[e12345]),
                (right_dual[e321] * self[e425]) + (right_dual[e2] * self[e12345]),
                (right_dual[e321] * self[e435]) + (right_dual[e3] * self[e12345]),
                -(right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) + (self.group0().zxy() * right_dual.group0().yzx()).with_w(right_dual[e5] * self[e12345])
                - (self.group0().yzx() * right_dual.group0().zxy()).with_w(right_dual[e235] * self[e415]),
        );
    }
}
impl BulkContraction<Line> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       22        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group0().with_w(
                    -(right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * self.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<Motor> for CircleRotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                right_dual[e23] * self[e12345],
                right_dual[e31] * self[e12345],
                right_dual[e12] * self[e12345],
                -(right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
            ]) + (self.group0() * right_dual.group1().www()).with_w(right_dual[scalar] * self[e12345]),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz())).with_w(right_dual[e3215] * self[e12345]),
        );
    }
}
impl BulkContraction<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd2        0        1        0
    //    simd3        8       18        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       46       76        0
    //  no simd       80      134        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e12345] * right_dual[scalar])
                    - (self[e423] * right_dual[e15])
                    - (self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
                self[e12345] * right_dual[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]) + (self[e12345] * right_dual[e1]),
                (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]) + (self[e12345] * right_dual[e2]),
                (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]) + (self[e12345] * right_dual[e3]),
                -(self[e412] * right_dual[e435]) - (self[e415] * right_dual[e423]) - (self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) + (self.group0().zxy() * right_dual.group8().yzx()).with_w(self[e12345] * right_dual[e4])
                - (self.group0().yzx() * right_dual.group8().zxy()).with_w(self[e423] * right_dual[e415])
                - (right_dual.group7().yzx() * self.group2().zxy()).with_w(self[e431] * right_dual[e425]),
            // e5
            (self[e12345] * right_dual[e5])
                - (self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e315] * right_dual[e4125]) + (self[e12345] * right_dual[e15]),
                (self[e125] * right_dual[e4235]) + (self[e12345] * right_dual[e25]),
                (self[e235] * right_dual[e4315]) + (self[e12345] * right_dual[e35]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) + (right_dual.group9().www() * self.group1().xyz()).with_w(self[e12345] * right_dual[e45])
                - (right_dual.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * right_dual.group4()) + (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group9().yzx())
                - (self.group0().yzx() * right_dual.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e12345]) * right_dual.group5()) + (Simd32x3::from(right_dual[e3215]) * self.group0()) + (Simd32x3::from(right_dual[e1234]) * self.group2().xyz())
                - (Simd32x3::from(self[e321]) * right_dual.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * right_dual.group6()) + (Simd32x4::from(right_dual[e12345]) * self.group1()),
            // e423, e431, e412
            (Simd32x3::from(self[e12345]) * right_dual.group7()) + (Simd32x3::from(right_dual[e12345]) * self.group0()),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * right_dual.group8()) + (Simd32x3::from(right_dual[e12345]) * self.group2().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group9(),
            // e1234
            self[e12345] * right_dual[e1234],
        );
    }
}
impl BulkContraction<Plane> for CircleRotor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<RoundPoint> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        4        6        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       20       40        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e423] * right_dual[e3215]) + (self[e235] * right_dual[e1234]),
                (self[e431] * right_dual[e3215]) + (self[e315] * right_dual[e1234]),
                (self[e412] * right_dual[e3215]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) + (self.group2().yzx() * right_dual.group0().zxy()) - (self.group2().zxy() * right_dual.group0().yzx()))
                .with_w(self[e12345] * right_dual[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<Scalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(right_dual[e12345]) * self.group2(),
        );
    }
}
impl BulkContraction<Sphere> for CircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e5
            self[e12345] * right_dual[e5],
        );
    }
}
impl BulkContraction<VersorEven> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       22       40        0
    //  no simd       40       72        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self[e415] * right_dual[e1234]) + (self[e12345] * right_dual[e41]),
                (self[e425] * right_dual[e1234]) + (self[e12345] * right_dual[e42]),
                (self[e435] * right_dual[e1234]) + (self[e12345] * right_dual[e43]),
                -(self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
            ]) + (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e12345] * right_dual[scalar])
                - (self.group0().yzx() * right_dual.group3().zxy()).with_w(self[e423] * right_dual[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e235] * right_dual[e1234]) + (self[e12345] * right_dual[e23]),
                (self[e315] * right_dual[e1234]) + (self[e12345] * right_dual[e31]),
                (self[e125] * right_dual[e1234]) + (self[e12345] * right_dual[e12]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) + (self.group0() * right_dual.group3().www()).with_w(self[e12345] * right_dual[e45])
                - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e3215]) * self.group1().xyz())
                + (self.group2().yzx() * right_dual.group3().zxy())
                - (self.group2().zxy() * right_dual.group3().yzx()))
            .with_w(self[e12345] * right_dual[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<VersorOdd> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd3        1        5        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       40       72        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(self[e12345]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group0())).with_w(self[e12345] * right_dual[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * right_dual.group1()) + (Simd32x4::from(right_dual[e12345]) * self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                self[e12345] * right_dual[e235],
                self[e12345] * right_dual[e315],
                self[e12345] * right_dual[e125],
                -(self[e415] * right_dual[e235])
                    - (self[e425] * right_dual[e315])
                    - (self[e435] * right_dual[e125])
                    - (self[e235] * right_dual[e415])
                    - (self[e315] * right_dual[e425])
                    - (self[e125] * right_dual[e435]),
            ]) + (self.group2() * right_dual.group0().www().with_w(right_dual[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]) + (self[e12345] * right_dual[e1]),
                (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]) + (self[e12345] * right_dual[e2]),
                (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]) + (self[e12345] * right_dual[e3]),
                -(self[e412] * right_dual[e435]) - (self[e415] * right_dual[e423]) - (self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) + (self.group0().zxy() * right_dual.group2().yzx()).with_w(self[e12345] * right_dual[e4])
                - (self.group0().yzx() * right_dual.group2().zxy()).with_w(self[e423] * right_dual[e415])
                - (self.group2().zxy() * right_dual.group0().yzx()).with_w(self[e431] * right_dual[e425]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for Dipole {
    type Output = bulk_contraction_partial<Dipole>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        9       16        0
    //  no simd        9       32        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().with_w(
                    -(right_dual[e423] * self[e15])
                        - (right_dual[e431] * self[e25])
                        - (right_dual[e412] * self[e35])
                        - (right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12])
                        - (right_dual[e321] * self[e45])
                        - (right_dual[e235] * self[e41])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43]),
                ),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       32        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (right_dual.group3().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group2() * right_dual.group2().www()).with_w(self[e41] * right_dual[e4235]),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<AntiDualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(right_dual[e12345]) * self.group2(),
        );
    }
}
impl BulkContraction<AntiFlector> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       24        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]),
            ]) - (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<AntiLine> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -(self[e41] * right_dual[e235])
                - (self[e42] * right_dual[e315])
                - (self[e43] * right_dual[e125])
                - (self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435]),
        );
    }
}
impl BulkContraction<AntiMotor> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       25        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().with_w(
                    -(self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435]),
                ),
        );
    }
}
impl BulkContraction<AntiPlane> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]),
            ]) - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<Dipole> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e423] * self[e15])
                - (right_dual[e431] * self[e25])
                - (right_dual[e412] * self[e35])
                - (right_dual[e415] * self[e23])
                - (right_dual[e425] * self[e31])
                - (right_dual[e435] * self[e12])
                - (right_dual[e321] * self[e45])
                - (right_dual[e235] * self[e41])
                - (right_dual[e315] * self[e42])
                - (right_dual[e125] * self[e43]),
        );
    }
}
impl BulkContraction<DipoleInversion> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       25        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e423] * self[e15])
                - (right_dual[e431] * self[e25])
                - (right_dual[e412] * self[e35])
                - (right_dual[e415] * self[e23])
                - (right_dual[e425] * self[e31])
                - (right_dual[e435] * self[e12])
                - (right_dual[e321] * self[e45])
                - (right_dual[e235] * self[e41])
                - (right_dual[e315] * self[e42])
                - (right_dual[e125] * self[e43]),
        );
    }
}
impl BulkContraction<DualNum> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]));
    }
}
impl BulkContraction<FlatPoint> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(
            // scalar
            -(right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkContraction<Flector> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkContraction<Motor> for Dipole {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ (self.group0() * right_dual.group1().www()).with_w(right_dual[e3215] * self[e45]));
    }
}
impl BulkContraction<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       18       35        0
    //  no simd       24       62        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e41] * right_dual[e235])
                    - (self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e45] * right_dual[e321])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], right_dual[e4235]]) * self.group2().with_w(self[e41]))
                - (right_dual.group9().yzxy() * self.group1().zxy().with_w(self[e42])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e12345]) * self.group2().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e12345]) * self.group1().xyz(),
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
impl BulkContraction<RoundPoint> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], right_dual[e4235]]) * self.group2().with_w(self[e41]))
                - (right_dual.group0().yzxy() * self.group1().zxy().with_w(self[e42])),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<Scalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(right_dual[e12345]) * self.group2(),
        );
    }
}
impl BulkContraction<VersorEven> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       36        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (right_dual.group3().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group2() * right_dual.group2().www()).with_w(self[e41] * right_dual[e4235]),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
        );
    }
}
impl BulkContraction<VersorOdd> for Dipole {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        9       17        0
    //  no simd        9       37        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group2().with_w(
                    -(self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e45] * right_dual[e321])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
        );
    }
}
impl std::ops::Div<bulk_contraction> for DipoleInversion {
    type Output = bulk_contraction_partial<DipoleInversion>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       39       66        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (right_dual[e412] * self[e4315]) + (right_dual[e415] * self[e1234]) + (right_dual[e12345] * self[e41]),
                (right_dual[e423] * self[e4125]) + (right_dual[e425] * self[e1234]) + (right_dual[e12345] * self[e42]),
                (right_dual[e431] * self[e4235]) + (right_dual[e435] * self[e1234]) + (right_dual[e12345] * self[e43]),
                -(right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
            ]) - (right_dual.group0().yzx() * self.group3().zxy()).with_w(right_dual[e423] * self[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e235] * self[e1234]) + (right_dual[e12345] * self[e23]),
                (right_dual[e315] * self[e1234]) + (right_dual[e12345] * self[e31]),
                (right_dual[e125] * self[e1234]) + (right_dual[e12345] * self[e12]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) + (right_dual.group0() * self.group3().www()).with_w(right_dual[e12345] * self[e45])
                - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * right_dual.group1().xyz())
                + (right_dual.group2().yzx() * self.group3().zxy())
                - (right_dual.group2().zxy() * self.group3().yzx()))
            .with_w(right_dual[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        6        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       48       72        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * right_dual.group3().www().with_w(right_dual[e41]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234])
                        - (right_dual[e1234] * self[e45])
                        - (right_dual[e4315] * self[e42])
                        - (right_dual[e4125] * self[e43]),
                )
                - (self.group3().www() * right_dual.group3().xyz()).with_w(right_dual[e4235] * self[e41]),
            // e1, e2, e3, e5
            (right_dual.group3().zxyw() * self.group1().yzxw())
                + (self.group2().wwwz() * right_dual.group2().xyz().with_w(right_dual[e4125]))
                + (self.group0() * right_dual.group3().www()).with_w(right_dual[e4235] * self[e15])
                + (right_dual.group1().zxy() * self.group3().yzx()).with_w(right_dual[e4315] * self[e25])
                - (Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]))
                - (right_dual.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15]))
                - (right_dual.group3().yzx() * self.group1().zxy()).with_w(right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<AntiDualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            right_dual.group0().yy().with_zw(right_dual[e12345], right_dual[e5]) * self.group0().with_w(self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e5
            -(self[e4235] * right_dual[e15]) - (self[e4315] * right_dual[e25]) - (self[e4125] * right_dual[e35]) - (self[e3215] * right_dual[e45]),
        );
    }
}
impl BulkContraction<AntiFlector> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       48        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group1().xyz(),
            // e415, e425, e435, e321
            ((self.group3().zxy() * right_dual.group1().yzx()) - (self.group3().yzx() * right_dual.group1().zxy())).with_w(self[e1234] * right_dual[e3215]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]))
                + (right_dual.group1().www() * self.group3().xyz()).with_w(self[e1234] * right_dual[e45])
                - (right_dual.group1().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]))
                + (self.group2().wwwy() * right_dual.group0().xyz().with_w(right_dual[e4315]))
                + (right_dual.group1().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w((self[e35] * right_dual[e4125]) - (self[e4315] * right_dual[e25]) - (self[e4125] * right_dual[e35]) - (self[e3215] * right_dual[e45]))
                - (self.group1().zxy() * right_dual.group1().yzx()).with_w(self[e4235] * right_dual[e15]),
        );
    }
}
impl BulkContraction<AntiLine> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       31        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual
                    .group1()
                    .with_w(-(self[e4235] * right_dual[e415]) - (self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e4125] * right_dual[e315]) + (self[e3215] * right_dual[e415]),
                (self[e4235] * right_dual[e125]) + (self[e3215] * right_dual[e425]),
                (self[e4315] * right_dual[e235]) + (self[e3215] * right_dual[e435]),
                -(self[e42] * right_dual[e315]) - (self[e43] * right_dual[e125]) - (self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435]),
            ]) - (right_dual.group1().zxy() * self.group3().yzx()).with_w(self[e41] * right_dual[e235]),
        );
    }
}
impl BulkContraction<AntiMotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       12       26        0
    //  no simd       24       48        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e1234] * right_dual[e415],
                self[e1234] * right_dual[e425],
                self[e1234] * right_dual[e435],
                -(self[e41] * right_dual[e235])
                    - (self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435]),
            ]) + (self.group0() * right_dual.group0().www()).with_w(self[e1234] * right_dual[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e1234] * right_dual[e235],
                self[e1234] * right_dual[e315],
                self[e1234] * right_dual[e125],
                -(self[e4235] * right_dual[e415]) - (self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual.group0().xyz())
                + (Simd32x3::from(right_dual[e12345]) * self.group2().xyz())
                + (self.group3().zxy() * right_dual.group1().yzx())
                - (self.group3().yzx() * right_dual.group1().zxy()))
            .with_w(self[e1234] * right_dual[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiPlane> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       21        0
    //  no simd       17       39        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group0().xyz(),
            // e415, e425, e435, e321
            ((self.group3().zxy() * right_dual.group0().yzx()) - (self.group3().yzx() * right_dual.group0().zxy())).with_w(self[e1234] * right_dual[e3215]),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e4235] * right_dual[e3215],
                self[e4315] * right_dual[e3215],
                self[e4125] * right_dual[e3215],
                -(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]),
            ]) - (right_dual.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e12] * right_dual[e4315] * -1.0,
                self[e23] * right_dual[e4125] * -1.0,
                self[e31] * right_dual[e4235] * -1.0,
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]))
                + (right_dual.group0().zxyx() * self.group1().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<Circle> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (self.group3().yzxy() * right_dual.group1().zxy().with_w(right_dual[e42]))
                + (right_dual.group2() * self.group2().www()).with_w(right_dual[e41] * self[e4235]),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<CircleRotor> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (self.group3().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41]))
                + (self.group2().www() * right_dual.group2().xyz()).with_w(right_dual[e42] * self[e4315]),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<Dipole> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       29       50        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group3().yzx()) - (right_dual.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e423] * self[e3215]) + (right_dual[e235] * self[e1234]),
                (right_dual[e431] * self[e3215]) + (right_dual[e315] * self[e1234]),
                (right_dual[e412] * self[e3215]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
            ]) - (right_dual.group2().zxy() * self.group3().yzx()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkContraction<DipoleInversion> for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        5        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       60        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group3().yzx()) - (right_dual.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e423] * self[e3215]) + (right_dual[e235] * self[e1234]),
                (right_dual[e431] * self[e3215]) + (right_dual[e315] * self[e1234]),
                (right_dual[e412] * self[e3215]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual.group1().xyz().with_w(right_dual[e4]))
                + (self.group3().zxyx() * right_dual.group2().yzx().with_w(right_dual[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                        - (right_dual[e431] * self[e25])
                        - (right_dual[e412] * self[e35])
                        - (right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12])
                        - (right_dual[e321] * self[e45])
                        - (right_dual[e235] * self[e41])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43]),
                )
                - (right_dual.group2().zxy() * self.group3().yzx()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkContraction<DualNum> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]),
        );
    }
}
impl BulkContraction<FlatPoint> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        9       20        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                right_dual[e235] * self[e1234],
                right_dual[e315] * self[e1234],
                right_dual[e125] * self[e1234],
                -(right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
            ]) - (right_dual.group0().www() * self.group3().xyz()).with_w(right_dual[e235] * self[e41]),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Flector> for DipoleInversion {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        1        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       16       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43])
                    - (right_dual[e321] * self[e45]),
            ) + (self.group2().www() * right_dual.group0().xyz()).with_w(right_dual[e1] * self[e4235])
                - (right_dual.group0().www() * self.group3().xyz()).with_w(right_dual[e235] * self[e41]),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Line> for DipoleInversion {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e12] * self[e4315]) + (right_dual[e15] * self[e1234]),
                (right_dual[e23] * self[e4125]) + (right_dual[e25] * self[e1234]),
                (right_dual[e31] * self[e4235]) + (right_dual[e35] * self[e1234]),
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (self.group3().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<Motor> for DipoleInversion {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e12] * self[e4315]) + (right_dual[e15] * self[e1234]),
                (right_dual[e23] * self[e4125]) + (right_dual[e25] * self[e1234]),
                (right_dual[e31] * self[e4235]) + (right_dual[e35] * self[e1234]),
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) + (self.group0() * right_dual.group1().www()).with_w(right_dual[e3215] * self[e45])
                - (self.group3().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       39        0
    //    simd2        0        1        0
    //    simd3        8       19        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       43       70        0
    //  no simd       89      142        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1234] * right_dual[e5]) + (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4])
                    - (self[e41] * right_dual[e235])
                    - (self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e45] * right_dual[e321])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_dual.group3())
                + (self.group3().yzxy() * right_dual.group5().zxy().with_w(right_dual[e42]))
                + (self.group0() * right_dual.group9().www()).with_w(self[e4235] * right_dual[e41])
                + (self.group1().yzx() * right_dual.group9().zxy()).with_w(self[e4125] * right_dual[e43])
                - (Simd32x4::from(right_dual[e1234]) * self.group2().xyz().with_w(self[e45]))
                - (right_dual.group9().yzxz() * self.group1().zxy().with_w(self[e43]))
                - (right_dual.group4() * self.group3().www()).with_w(self[e41] * right_dual[e4235])
                - (right_dual.group5().yzx() * self.group3().zxy()).with_w(self[e42] * right_dual[e4315]),
            // e5
            (self[e45] * right_dual[e3215]) + (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125])
                - (self[e4235] * right_dual[e15])
                - (self[e4315] * right_dual[e25])
                - (self[e4125] * right_dual[e35])
                - (self[e3215] * right_dual[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * right_dual[e315]) + (self[e3215] * right_dual[e415]),
                (self[e4235] * right_dual[e125]) + (self[e3215] * right_dual[e425]),
                (self[e4315] * right_dual[e235]) + (self[e3215] * right_dual[e435]),
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group2().xyz().with_w(self[e45]))
                - (right_dual.group8().zxy() * self.group3().yzx()).with_w(self[e4235] * right_dual[e415]),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group6().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group0()) + (right_dual.group7().zxy() * self.group3().yzx())
                - (right_dual.group7().yzx() * self.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual.group8()) + (Simd32x3::from(self[e3215]) * right_dual.group7()) + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz())
                - (Simd32x3::from(right_dual[e321]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group9().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual.group9().zxy().with_w(right_dual[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group9().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group3().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
            // e1234
            self[e1234] * right_dual[e12345],
        );
    }
}
impl BulkContraction<Plane> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(
            // scalar
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkContraction<RoundPoint> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        2        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       25       45        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group0().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual.group0().zxy().with_w(right_dual[e1234])),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e4235] * right_dual[e3215],
                self[e4315] * right_dual[e3215],
                self[e4125] * right_dual[e3215],
                -(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]) - (self[e45] * right_dual[e1234]),
            ]) - (right_dual.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(self[e12] * right_dual[e4315]) - (self[e15] * right_dual[e1234]),
                -(self[e23] * right_dual[e4125]) - (self[e25] * right_dual[e1234]),
                -(self[e31] * right_dual[e4235]) - (self[e35] * right_dual[e1234]),
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]),
            ]) + (Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]))
                + (right_dual.group0().zxyx() * self.group1().yzx().with_w(self[e15])),
        );
    }
}
impl BulkContraction<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<Sphere> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Scalar::from_groups(
            // scalar
            (self[e1234] * right_dual[e5]) + (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
        );
    }
}
impl BulkContraction<VersorEven> for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10       13        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       48       76        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(
                (self[e4235] * right_dual[e41]) + (self[e4315] * right_dual[e42]) + (self[e4125] * right_dual[e43])
                    - (self[e42] * right_dual[e4315])
                    - (self[e43] * right_dual[e4125])
                    - (self[e45] * right_dual[e1234]),
            ) + (right_dual.group3().www() * self.group3().xyz()).with_w(self[e1234] * right_dual[e45])
                - (right_dual.group3().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual[e3215]) * self.group0().with_w(self[e45]))
                + (self.group2().wwwy() * right_dual.group2().xyz().with_w(right_dual[e4315]))
                + (right_dual.group3().zxyx() * self.group1().yzx().with_w(self[e15]))
                + (self.group3().yzx() * right_dual.group1().zxy()).with_w(self[e35] * right_dual[e4125])
                - (Simd32x4::from(self[e3215]) * right_dual.group0().xyz().with_w(right_dual[e45]))
                - (self.group3().zxyz() * right_dual.group1().yzx().with_w(right_dual[e35]))
                - (right_dual.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group1().zxy() * right_dual.group3().yzx()).with_w(self[e4235] * right_dual[e15]),
        );
    }
}
impl BulkContraction<VersorOdd> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       47       76        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group3().yzxy() * right_dual.group0().zxy().with_w(right_dual[e2]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e45] * right_dual[e321])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                )
                + (self.group0() * right_dual.group0().www()).with_w(self[e1234] * right_dual[e5])
                + (self.group2().www() * right_dual.group1().xyz()).with_w(self[e4235] * right_dual[e1])
                - (self.group3().zxy() * right_dual.group0().yzx()).with_w(self[e41] * right_dual[e235]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e1234] * right_dual[e235]) + (self[e3215] * right_dual[e423]),
                (self[e1234] * right_dual[e315]) + (self[e3215] * right_dual[e431]),
                (self[e1234] * right_dual[e125]) + (self[e3215] * right_dual[e412]),
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group1())
                - (self.group3().xyzx() * right_dual.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[e12345]) * self.group2().xyz())
                + (self.group3().zxy() * right_dual.group2().yzx())
                - (self.group3().yzx() * right_dual.group2().zxy()))
            .with_w(self[e1234] * right_dual[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl std::ops::Div<bulk_contraction> for DualNum {
    type Output = bulk_contraction_partial<DualNum>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       23        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * right_dual.group0().with_w(right_dual[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e5
            right_dual.group2() * self.group0().yy().with_zw(self[e12345], self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        7        0
    // no simd        0       28        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0().yy().with_zw(self[e12345], self[e5]) * right_dual.group0().with_w(right_dual[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<AntiDualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([(right_dual[e5] * self[e12345]) + (right_dual[e12345] * self[e5]), right_dual[e12345] * self[e12345]]),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<AntiFlector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiLine> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiMotor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([right_dual[e235], right_dual[e315], right_dual[e125], 1.0])
                * self.group0().yy().with_zw(self[e12345], (self[e5] * right_dual[e12345]) + (self[e12345] * right_dual[e5])),
        );
    }
}
impl BulkContraction<AntiPlane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<AntiScalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Scalar::from_groups(/* scalar */ self[e12345] * right_dual[scalar]);
    }
}
impl BulkContraction<Circle> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * right_dual.group2(),
        );
    }
}
impl BulkContraction<CircleRotor> for DualNum {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(self[e12345]) * right_dual.group2(),
        );
    }
}
impl BulkContraction<Dipole> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group2(),
        );
    }
}
impl BulkContraction<DipoleInversion> for DualNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       30        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(self[e12345]) * right_dual.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<DualNum> for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<FlatPoint> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<Flector> for DualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<Line> for DualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e15, e25, e35
            Simd32x3::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<Motor> for DualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd2        0        1        0
    //    simd3        0        6        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        2       22        0
    //  no simd        2       56        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self[e5] * right_dual[e1234]) + (self[e12345] * right_dual[scalar]), self[e12345] * right_dual[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e5
            (self[e5] * right_dual[e12345]) + (self[e12345] * right_dual[e5]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual.group3(),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group4(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group6(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group7(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group9(),
            // e1234
            self[e12345] * right_dual[e1234],
        );
    }
}
impl BulkContraction<Plane> for DualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<RoundPoint> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(self[e5] * right_dual[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(self[e12345] * right_dual[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<Sphere> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e5
            self[e12345] * right_dual[e5],
        );
    }
}
impl BulkContraction<VersorEven> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       34        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([right_dual[e41], right_dual[e42], right_dual[e43], 1.0])
                * self.group0().yy().with_zw(self[e12345], (self[e5] * right_dual[e1234]) + (self[e12345] * right_dual[scalar])),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[e12345]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<VersorOdd> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       34        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([right_dual[e235], right_dual[e315], right_dual[e125], 1.0])
                * self.group0().yy().with_zw(self[e12345], (self[e5] * right_dual[e12345]) + (self[e12345] * right_dual[e5])),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl std::ops::Div<bulk_contraction> for FlatPoint {
    type Output = bulk_contraction_partial<FlatPoint>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       20        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_dual[e12345] * self[e45]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15], self[e25], self[e35], 1.0])
                * right_dual
                    .group2()
                    .www()
                    .with_w(-(right_dual[e423] * self[e15]) - (right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45])),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       24        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (right_dual[e4235] * self[e15]) + (right_dual[e4315] * self[e25]) + (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45]),
        );
    }
}
impl BulkContraction<AntiDualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiFlector> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
            0.0,
        ]));
    }
}
impl BulkContraction<AntiMotor> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiPlane> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
            0.0,
        ]));
    }
}
impl BulkContraction<Dipole> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       14        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e423] * self[e15]) - (right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkContraction<DipoleInversion> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       19        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e423] * self[e15]) - (right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkContraction<DualNum> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([right_dual[e3215] * self[e45], 1.0]) * Simd32x2::from([1.0, 0.0]));
    }
}
impl BulkContraction<FlatPoint> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(/* scalar */ right_dual[e321] * self[e45] * -1.0);
    }
}
impl BulkContraction<Flector> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(/* scalar */ right_dual[e321] * self[e45] * -1.0);
    }
}
impl BulkContraction<Motor> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([right_dual[e3215] * self[e45], 1.0]) * Simd32x2::from([1.0, 0.0]));
    }
}
impl BulkContraction<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        6       42        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e15] * right_dual[e423]) - (self[e25] * right_dual[e431]) - (self[e35] * right_dual[e412]) - (self[e45] * right_dual[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e12345]) * self.group0(),
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
impl BulkContraction<RoundPoint> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       17        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
        );
    }
}
impl BulkContraction<Scalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<VersorEven> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       28        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group0() * Simd32x4::from(-1.0),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
        );
    }
}
impl BulkContraction<VersorOdd> for FlatPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       25        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(self[e45] * right_dual[e12345]),
            // e15, e25, e35, scalar
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w(-(self[e15] * right_dual[e423]) - (self[e25] * right_dual[e431]) - (self[e35] * right_dual[e412]) - (self[e45] * right_dual[e321])),
        );
    }
}
impl std::ops::Div<bulk_contraction> for Flector {
    type Output = bulk_contraction_partial<Flector>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd3        0        7        0
    //    simd4        6        4        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       27       47        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                right_dual[e412] * self[e4315],
                right_dual[e423] * self[e4125],
                right_dual[e431] * self[e4235],
                -(right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
            ]) - (right_dual.group0().yzx() * self.group1().zxy()).with_w(right_dual[e423] * self[e15]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(-(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]))
                + (right_dual.group0() * self.group1().www()).with_w(right_dual[e12345] * self[e45])
                - (right_dual.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual[e12345]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * right_dual.group1().xyz()).with_w(0.0)
                + (right_dual.group2().yzx() * self.group1().zxy()).with_w(0.0)
                - (right_dual.group2().zxy() * self.group1().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       31       56        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual.group3().yzx() * self.group1().zxy()) - (right_dual.group3().zxy() * self.group1().yzx())).with_w(right_dual[e1234] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * right_dual.group3().www().with_w(right_dual[e41]))
                + Simd32x3::from(0.0).with_w((right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]))
                - (self.group1().www() * right_dual.group3().xyz()).with_w(right_dual[e1234] * self[e45]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((right_dual[e4315] * self[e25]) + (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45]) - (right_dual[e35] * self[e4125]))
                + (right_dual.group1().zxy() * self.group1().yzx()).with_w(right_dual[e4235] * self[e15])
                - (Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]))
                - (right_dual.group2().wwwy() * self.group0().xyz().with_w(self[e4315]))
                - (self.group1().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<AntiDualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
            0.0,
        ]));
    }
}
impl BulkContraction<AntiFlector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       16       28        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_dual.group1().yzx() * self.group1().zxy()) - (right_dual.group1().zxy() * self.group1().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            (right_dual.group1().wwwx() * self.group1().xyz().with_w(self[e15]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e4315] * self[e25]) + (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45])
                        - (right_dual[e25] * self[e4315])
                        - (right_dual[e35] * self[e4125])
                        - (right_dual[e45] * self[e3215]),
                )
                - (self.group1().wwwx() * right_dual.group1().xyz().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<AntiLine> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * right_dual[e315]) + (self[e3215] * right_dual[e415]),
                (self[e4235] * right_dual[e125]) + (self[e3215] * right_dual[e425]),
                (self[e4315] * right_dual[e235]) + (self[e3215] * right_dual[e435]),
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) - (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
        );
    }
}
impl BulkContraction<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * right_dual[e315]) + (self[e3215] * right_dual[e415]),
                (self[e4235] * right_dual[e125]) + (self[e3215] * right_dual[e425]),
                (self[e4315] * right_dual[e235]) + (self[e3215] * right_dual[e435]),
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group0())
                - (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       23        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((self.group1().zxy() * right_dual.group0().yzx()) - (self.group1().yzx() * right_dual.group0().zxy())).with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                self[e3215] * right_dual[e4235] * -1.0,
                self[e3215] * right_dual[e4315] * -1.0,
                self[e3215] * right_dual[e4125] * -1.0,
                (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
            ]) + (right_dual.group0().wwwx() * self.group1().xyz().with_w(self[e15])),
        );
    }
}
impl BulkContraction<Circle> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]),
            ]) + (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<CircleRotor> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       24        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]),
            ]) + (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<Dipole> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       17       38        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * self.group1().yzx()) - (right_dual.group0().yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * self[e3215],
                right_dual[e431] * self[e3215],
                right_dual[e412] * self[e3215],
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
            ]) - (right_dual.group2().zxy() * self.group1().yzx()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkContraction<DipoleInversion> for Flector {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        4        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       24       47        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * self.group1().yzx()) - (right_dual.group0().yzx() * self.group1().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * self[e3215],
                right_dual[e431] * self[e3215],
                right_dual[e412] * self[e3215],
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group1().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual.group1().xyz().with_w(right_dual[e4]))
                + (self.group1().zxyx() * right_dual.group2().yzx().with_w(right_dual[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125])
                        - (right_dual[e431] * self[e25])
                        - (right_dual[e412] * self[e35])
                        - (right_dual[e321] * self[e45]),
                )
                - (right_dual.group2().zxy() * self.group1().yzx()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkContraction<DualNum> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            right_dual.group0().xx().with_zw(right_dual[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(right_dual[e3215] * self[e45]),
        );
    }
}
impl BulkContraction<FlatPoint> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       18        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[e321]) * self.group1().xyz().with_w(self[e45]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group1().zxy()) - (right_dual.group0().zxy() * self.group1().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Flector> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       26        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 1.0])
                * right_dual
                    .group0()
                    .www()
                    .with_w((right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) - (right_dual[e321] * self[e45]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group1().zxy()) - (right_dual.group0().zxy() * self.group1().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Line> for Flector {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e12] * self[e4315],
                right_dual[e23] * self[e4125],
                right_dual[e31] * self[e4235],
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (self.group1().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<Motor> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        1        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        9       30        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(-(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]))
                + (right_dual.group0().zxy() * self.group1().yzx()).with_w(right_dual[e3215] * self[e45])
                - (self.group1().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       38        0
    //    simd2        0        1        0
    //    simd3        4       14        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       59        0
    //  no simd       50      106        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412])
                    - (self[e45] * right_dual[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(self[e15] * right_dual[e1234]) - (self[e4125] * right_dual[e31]),
                -(self[e25] * right_dual[e1234]) - (self[e4235] * right_dual[e12]),
                -(self[e35] * right_dual[e1234]) - (self[e4315] * right_dual[e23]),
                (self[e4315] * right_dual[e42]) + (self[e4125] * right_dual[e43]),
            ]) + (self.group1().yzxx() * right_dual.group5().zxy().with_w(right_dual[e41]))
                - (right_dual.group4() * self.group1().www()).with_w(self[e45] * right_dual[e1234]),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215])
                - (self[e4235] * right_dual[e15])
                - (self[e4315] * right_dual[e25])
                - (self[e4125] * right_dual[e35])
                - (self[e3215] * right_dual[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * right_dual[e315]) + (self[e3215] * right_dual[e415]),
                (self[e4235] * right_dual[e125]) + (self[e3215] * right_dual[e425]),
                (self[e4315] * right_dual[e235]) + (self[e3215] * right_dual[e435]),
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group0())
                - (right_dual.group8().zxy() * self.group1().yzx()).with_w(self[e4235] * right_dual[e415]),
            // e41, e42, e43
            (right_dual.group7().zxy() * self.group1().yzx()) - (right_dual.group7().yzx() * self.group1().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * right_dual.group7()) - (Simd32x3::from(right_dual[e321]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((self.group1().zxy() * right_dual.group9().yzx()) - (self.group1().yzx() * right_dual.group9().zxy())).with_w(self[e3215] * right_dual[e1234] * -1.0),
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<Plane> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(/* scalar */ (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]));
    }
}
impl BulkContraction<RoundPoint> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        2        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       18        0
    //  no simd        9       39        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group1().zxy() * right_dual.group0().yzx()) - (self.group1().yzx() * right_dual.group0().zxy())).with_w(self[e3215] * right_dual[e1234] * -1.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group0().xyz())).with_w(self[e45] * right_dual[e1234] * -1.0),
            // e1, e2, e3, e5
            Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w((self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl BulkContraction<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<Sphere> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Scalar::from_groups(
            // scalar
            (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
        );
    }
}
impl BulkContraction<VersorEven> for Flector {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       31       60        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group1().zxy() * right_dual.group3().yzx()) - (self.group1().yzx() * right_dual.group3().zxy())).with_w(self[e3215] * right_dual[e1234] * -1.0),
            // e235, e315, e125, e4
            (self.group1().xyzx() * right_dual.group3().www().with_w(right_dual[e41]))
                + Simd32x3::from(0.0).with_w((self[e4315] * right_dual[e42]) + (self[e4125] * right_dual[e43]))
                - (self.group1().www() * right_dual.group3().xyz()).with_w(self[e45] * right_dual[e1234]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w((self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]) - (self[e3215] * right_dual[e45]))
                + (self.group1().yzx() * right_dual.group1().zxy()).with_w(self[e15] * right_dual[e4235])
                - (self.group1().zxyy() * right_dual.group1().yzx().with_w(right_dual[e25]))
                - (self.group1().wwwz() * right_dual.group0().xyz().with_w(right_dual[e35]))
                - (right_dual.group2().wwwx() * self.group0().xyz().with_w(self[e4235])),
        );
    }
}
impl BulkContraction<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        4        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       34       56        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group1().yzxx() * right_dual.group0().zxy().with_w(right_dual[e1]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412])
                        - (self[e45] * right_dual[e321]),
                )
                - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            (right_dual.group0() * self.group1().www().with_w(self[e45])) + Simd32x3::from(0.0).with_w(-(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]))
                - (self.group1().xyzx() * right_dual.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * right_dual.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[e12345]) * self.group0().xyz()).with_w(0.0)
                + (self.group1().zxy() * right_dual.group2().yzx()).with_w(0.0)
                - (self.group1().yzx() * right_dual.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl std::ops::Div<bulk_contraction> for Line {
    type Output = bulk_contraction_partial<Line>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        6        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       13       45        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self
                    .group1()
                    .with_w(-(right_dual[e423] * self[e415]) - (right_dual[e431] * self[e425]) - (right_dual[e412] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e321] * self[e415]),
                (right_dual[e423] * self[e125]) + (right_dual[e321] * self[e425]),
                (right_dual[e431] * self[e235]) + (right_dual[e321] * self[e435]),
                -(right_dual[e415] * self[e235])
                    - (right_dual[e425] * self[e315])
                    - (right_dual[e435] * self[e125])
                    - (right_dual[e315] * self[e425])
                    - (right_dual[e125] * self[e435]),
            ]) - (right_dual.group0().yzx() * self.group1().zxy()).with_w(right_dual[e235] * self[e415]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       37        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], 1.0])
                * self
                    .group1()
                    .with_w(-(right_dual[e4235] * self[e415]) - (right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual[e4125] * self[e315]) + (right_dual[e3215] * self[e415]),
                (right_dual[e4235] * self[e125]) + (right_dual[e3215] * self[e425]),
                (right_dual[e4315] * self[e235]) + (right_dual[e3215] * self[e435]),
                -(right_dual[e42] * self[e315]) - (right_dual[e43] * self[e125]) - (right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435]),
            ]) - (self.group1().zxy() * right_dual.group3().yzx()).with_w(right_dual[e41] * self[e235]),
        );
    }
}
impl BulkContraction<AntiDualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<AntiFlector> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e4125] * self[e315]) + (right_dual[e3215] * self[e415]),
                (right_dual[e4235] * self[e125]) + (right_dual[e3215] * self[e425]),
                (right_dual[e4315] * self[e235]) + (right_dual[e3215] * self[e435]),
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) - (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e415])),
        );
    }
}
impl BulkContraction<AntiLine> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            0.0,
        ]));
    }
}
impl BulkContraction<AntiMotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       30        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self.group1().with_w(
                    -(self[e415] * right_dual[e235])
                        - (self[e425] * right_dual[e315])
                        - (self[e435] * right_dual[e125])
                        - (self[e235] * right_dual[e415])
                        - (self[e315] * right_dual[e425])
                        - (self[e125] * right_dual[e435]),
                ),
        );
    }
}
impl BulkContraction<AntiPlane> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
        );
    }
}
impl BulkContraction<Circle> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd        5       10        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e41] * self[e235])
                - (right_dual[e42] * self[e315])
                - (right_dual[e43] * self[e125])
                - (right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435]),
        );
    }
}
impl BulkContraction<CircleRotor> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       14        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            -(right_dual[e41] * self[e235])
                - (right_dual[e42] * self[e315])
                - (right_dual[e43] * self[e125])
                - (right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435]),
        );
    }
}
impl BulkContraction<Dipole> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       28        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e321] * self[e415]),
                (right_dual[e423] * self[e125]) + (right_dual[e321] * self[e425]),
                (right_dual[e431] * self[e235]) + (right_dual[e321] * self[e435]),
                -(right_dual[e431] * self[e425]) - (right_dual[e412] * self[e435]),
            ]) - (right_dual.group0().yzx() * self.group1().zxy()).with_w(right_dual[e423] * self[e415]),
            // e5
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
        );
    }
}
impl BulkContraction<DipoleInversion> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       33        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e321] * self[e415]),
                (right_dual[e423] * self[e125]) + (right_dual[e321] * self[e425]),
                (right_dual[e431] * self[e235]) + (right_dual[e321] * self[e435]),
                -(right_dual[e431] * self[e425]) - (right_dual[e412] * self[e435]),
            ]) - (right_dual.group0().yzx() * self.group1().zxy()).with_w(right_dual[e423] * self[e415]),
            // e5
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
        );
    }
}
impl BulkContraction<DualNum> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            right_dual.group0().xx().with_zw(right_dual[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<FlatPoint> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([right_dual[e321], right_dual[e321], right_dual[e321], 1.0])
                * self
                    .group0()
                    .with_w(-(right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435])),
        );
    }
}
impl BulkContraction<Flector> for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([right_dual[e321], right_dual[e321], right_dual[e321], 1.0])
                * self
                    .group0()
                    .with_w(-(right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435])),
        );
    }
}
impl BulkContraction<Line> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Scalar::from_groups(/* scalar */ -(right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435]));
    }
}
impl BulkContraction<Motor> for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        2       23        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435])),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group1().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       31        0
    //    simd2        0        1        0
    //    simd3        0        6        0
    //    simd4        2        7        0
    // Totals...
    // yes simd       20       45        0
    //  no simd       26       79        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * right_dual[e321]) + (self[e315] * right_dual[e412]),
                (self[e425] * right_dual[e321]) + (self[e125] * right_dual[e423]),
                (self[e435] * right_dual[e321]) + (self[e235] * right_dual[e431]),
                -(self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) - (self.group1().zxy() * right_dual.group7().yzx()).with_w(self[e415] * right_dual[e423]),
            // e5
            -(self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (right_dual.group9().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e1234]) * self.group1(),
            // e415, e425, e435, e321
            right_dual.group0().yy().with_zw(right_dual[e12345], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<RoundPoint> for Line {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], 1.0])
                * self
                    .group1()
                    .with_w(-(self[e415] * right_dual[e4235]) - (self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125])),
            // e15, e25, e35
            (Simd32x3::from(right_dual[e3215]) * self.group0()) + (self.group1().yzx() * right_dual.group0().zxy()) - (self.group1().zxy() * right_dual.group0().yzx()),
        );
    }
}
impl BulkContraction<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_dual[e12345]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<VersorEven> for Line {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        5        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       41        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], 1.0])
                * self
                    .group1()
                    .with_w(-(self[e415] * right_dual[e4235]) - (self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e31]) - (self[e435] * right_dual[e12]) - (self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]),
            ]) - (self.group1().zxy() * right_dual.group3().yzx()).with_w(self[e415] * right_dual[e23]),
        );
    }
}
impl BulkContraction<VersorOdd> for Line {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        8        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       13       50        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([right_dual[e12345], right_dual[e12345], right_dual[e12345], 1.0])
                * self
                    .group1()
                    .with_w(-(self[e415] * right_dual[e423]) - (self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e415] * right_dual[e321]) + (self[e315] * right_dual[e412]),
                (self[e425] * right_dual[e321]) + (self[e125] * right_dual[e423]),
                (self[e435] * right_dual[e321]) + (self[e235] * right_dual[e431]),
                -(self[e425] * right_dual[e315])
                    - (self[e435] * right_dual[e125])
                    - (self[e235] * right_dual[e415])
                    - (self[e315] * right_dual[e425])
                    - (self[e125] * right_dual[e435]),
            ]) - (self.group1().zxy() * right_dual.group0().yzx()).with_w(self[e415] * right_dual[e235]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for Motor {
    type Output = bulk_contraction_partial<Motor>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        1        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       20       47        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * right_dual.group0().with_w(right_dual[e12345]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz())).with_w(right_dual[e321] * self[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from([
                right_dual[e12345] * self[e235],
                right_dual[e12345] * self[e315],
                right_dual[e12345] * self[e125],
                -(right_dual[e415] * self[e235])
                    - (right_dual[e425] * self[e315])
                    - (right_dual[e435] * self[e125])
                    - (right_dual[e235] * self[e415])
                    - (right_dual[e315] * self[e425])
                    - (right_dual[e125] * self[e435]),
            ]) + (right_dual.group2() * self.group0().www().with_w(self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e321] * self[e415]),
                (right_dual[e423] * self[e125]) + (right_dual[e321] * self[e425]),
                (right_dual[e431] * self[e235]) + (right_dual[e321] * self[e435]),
                -(right_dual[e431] * self[e425]) - (right_dual[e412] * self[e435]),
            ]) - (right_dual.group0().yzx() * self.group1().zxy()).with_w(right_dual[e423] * self[e415]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       24       52        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                right_dual[e1234] * self[e415],
                right_dual[e1234] * self[e425],
                right_dual[e1234] * self[e435],
                -(right_dual[e41] * self[e235])
                    - (right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435]),
            ]) + (right_dual.group0() * self.group0().www()).with_w(right_dual[e1234] * self[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e1234] * self[e235],
                right_dual[e1234] * self[e315],
                right_dual[e1234] * self[e125],
                -(right_dual[e4235] * self[e415]) - (right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e3215]) * self.group0().xyz())
                + (Simd32x3::from(self[e12345]) * right_dual.group2().xyz())
                + (right_dual.group3().zxy() * self.group1().yzx())
                - (right_dual.group3().yzx() * self.group1().zxy()))
            .with_w(right_dual[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<AntiDualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 1.0])
                * right_dual.group0().yy().with_zw(right_dual[e12345], (right_dual[e5] * self[e12345]) + (right_dual[e12345] * self[e5])),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for Motor {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<AntiFlector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e4125] * self[e315]) + (right_dual[e3215] * self[e415]),
                (right_dual[e4235] * self[e125]) + (right_dual[e3215] * self[e425]),
                (right_dual[e4315] * self[e235]) + (right_dual[e3215] * self[e435]),
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group0())
                - (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiLine> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       28        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group1().with_w(
                    -(right_dual[e415] * self[e235])
                        - (right_dual[e425] * self[e315])
                        - (right_dual[e435] * self[e125])
                        - (right_dual[e235] * self[e415])
                        - (right_dual[e315] * self[e425])
                        - (right_dual[e125] * self[e435]),
                ),
        );
    }
}
impl BulkContraction<AntiMotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(right_dual[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group0().xyz())).with_w(right_dual[e12345] * self[e12345]),
            // e235, e315, e125, e5
            (Simd32x4::from(right_dual[e12345]) * self.group1())
                + (Simd32x4::from(self[e12345]) * right_dual.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e415] * self[e235])
                        - (right_dual[e425] * self[e315])
                        - (right_dual[e435] * self[e125])
                        - (right_dual[e235] * self[e415])
                        - (right_dual[e315] * self[e425])
                        - (right_dual[e125] * self[e435]),
                ),
        );
    }
}
impl BulkContraction<AntiPlane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<AntiScalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Scalar::from_groups(/* scalar */ self[e12345] * right_dual[scalar]);
    }
}
impl BulkContraction<Circle> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       21        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().with_w(
                    -(right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435]),
                ),
        );
    }
}
impl BulkContraction<CircleRotor> for Motor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().xyz().with_w(
                    (right_dual[scalar] * self[e12345])
                        - (right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435]),
                ),
        );
    }
}
impl BulkContraction<Dipole> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       39        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual
                    .group2()
                    .with_w(-(right_dual[e423] * self[e415]) - (right_dual[e431] * self[e425]) - (right_dual[e412] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e321] * self[e415]),
                (right_dual[e423] * self[e125]) + (right_dual[e321] * self[e425]),
                (right_dual[e431] * self[e235]) + (right_dual[e321] * self[e435]),
                -(right_dual[e415] * self[e235])
                    - (right_dual[e425] * self[e315])
                    - (right_dual[e435] * self[e125])
                    - (right_dual[e315] * self[e425])
                    - (right_dual[e125] * self[e435]),
            ]) - (right_dual.group0().yzx() * self.group1().zxy()).with_w(right_dual[e235] * self[e415]),
        );
    }
}
impl BulkContraction<DipoleInversion> for Motor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd3        0        3        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       18       49        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual
                    .group2()
                    .xyz()
                    .with_w((right_dual[e4] * self[e12345]) - (right_dual[e423] * self[e415]) - (right_dual[e431] * self[e425]) - (right_dual[e412] * self[e435])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e321] * self[e415]) + (right_dual[e1] * self[e12345]),
                (right_dual[e321] * self[e425]) + (right_dual[e2] * self[e12345]),
                (right_dual[e321] * self[e435]) + (right_dual[e3] * self[e12345]),
                -(right_dual[e425] * self[e315])
                    - (right_dual[e435] * self[e125])
                    - (right_dual[e235] * self[e415])
                    - (right_dual[e315] * self[e425])
                    - (right_dual[e125] * self[e435]),
            ]) + (right_dual.group0().zxy() * self.group1().yzx()).with_w(right_dual[e5] * self[e12345])
                - (self.group1().zxyx() * right_dual.group0().yzx().with_w(right_dual[e415])),
        );
    }
}
impl BulkContraction<DualNum> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(right_dual[scalar] * self[e12345]),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual[e3215]) * self.group0(),
        );
    }
}
impl BulkContraction<FlatPoint> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([self[e415], self[e425], self[e435], 1.0])
                * right_dual
                    .group0()
                    .www()
                    .with_w(-(right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435])),
        );
    }
}
impl BulkContraction<Flector> for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       22        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e1] * self[e12345],
                right_dual[e2] * self[e12345],
                right_dual[e3] * self[e12345],
                -(right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) + (self.group0() * right_dual.group0().www().with_w(right_dual[e5])),
        );
    }
}
impl BulkContraction<Line> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       19        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual
                    .group0()
                    .with_w(-(right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435])),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<Motor> for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       23        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w((right_dual[scalar] * self[e12345]) - (right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435])),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz())).with_w(right_dual[e3215] * self[e12345]),
        );
    }
}
impl BulkContraction<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        0
    //    simd2        0        1        0
    //    simd3        4       13        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       59        0
    //  no simd       50      104        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e12345] * right_dual[scalar]) + (self[e5] * right_dual[e1234])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
                self[e12345] * right_dual[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e415] * right_dual[e321]) + (self[e12345] * right_dual[e1]),
                (self[e425] * right_dual[e321]) + (self[e12345] * right_dual[e2]),
                (self[e435] * right_dual[e321]) + (self[e12345] * right_dual[e3]),
                -(self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) + (right_dual.group7().zxy() * self.group1().yzx()).with_w(self[e12345] * right_dual[e4])
                - (right_dual.group7().yzx() * self.group1().zxy()).with_w(self[e415] * right_dual[e423]),
            // e5
            (self[e12345] * right_dual[e5]) + (self[e5] * right_dual[e12345])
                - (self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e12345] * right_dual[e15]) + (self[e315] * right_dual[e4125]),
                (self[e12345] * right_dual[e25]) + (self[e125] * right_dual[e4235]),
                (self[e12345] * right_dual[e35]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) + (self.group0() * right_dual.group9().www().with_w(right_dual[e45]))
                - (right_dual.group9().yzxx() * self.group1().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * right_dual.group4()) + (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(self[e12345]) * right_dual.group5()) + (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e12345]) * right_dual.group6().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group0().xyz())).with_w(self[e12345] * right_dual[e321]),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group7(),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * right_dual.group8()) + (Simd32x3::from(right_dual[e12345]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group9(),
            // e1234
            self[e12345] * right_dual[e1234],
        );
    }
}
impl BulkContraction<Plane> for Motor {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<RoundPoint> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        2        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       30        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual[e1234]) * self.group0().xyz().with_w(self[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], 1.0])
                * self
                    .group1()
                    .xyz()
                    .with_w(-(self[e415] * right_dual[e4235]) - (self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) + (self.group1().yzx() * right_dual.group0().zxy()) - (self.group1().zxy() * right_dual.group0().yzx()))
                .with_w(self[e12345] * right_dual[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[e12345]) * self.group1(),
        );
    }
}
impl BulkContraction<Sphere> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e5
            self[e12345] * right_dual[e5],
        );
    }
}
impl BulkContraction<VersorEven> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       28       57        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * right_dual.group2().www().with_w(right_dual[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12])
                        - (self[e235] * right_dual[e41])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43]),
                )
                + (self.group0().www() * right_dual.group0().xyz()).with_w(self[e5] * right_dual[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e235] * right_dual[e1234],
                self[e315] * right_dual[e1234],
                self[e125] * right_dual[e1234],
                -(self[e415] * right_dual[e4235]) - (self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e3215]) * self.group0().xyz())
                + (self.group1().yzx() * right_dual.group3().zxy())
                - (self.group1().zxy() * right_dual.group3().yzx()))
            .with_w(self[e12345] * right_dual[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<VersorOdd> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        9        0
    // Totals...
    // yes simd       14       26        0
    //  no simd       28       57        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e12345]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group0().xyz())).with_w(self[e12345] * right_dual[e321]),
            // e235, e315, e125, e5
            (Simd32x4::from(self[e12345]) * right_dual.group2())
                + (Simd32x4::from(right_dual[e12345]) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * right_dual[e235])
                        - (self[e425] * right_dual[e315])
                        - (self[e435] * right_dual[e125])
                        - (self[e235] * right_dual[e415])
                        - (self[e315] * right_dual[e425])
                        - (self[e125] * right_dual[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e12345] * right_dual[e1]) + (self[e315] * right_dual[e412]),
                (self[e12345] * right_dual[e2]) + (self[e125] * right_dual[e423]),
                (self[e12345] * right_dual[e3]) + (self[e235] * right_dual[e431]),
                -(self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]),
            ]) + (self.group0() * right_dual.group1().www().with_w(right_dual[e4]))
                - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e415])),
        );
    }
}
impl std::ops::Div<bulk_contraction> for MultiVector {
    type Output = bulk_contraction_partial<MultiVector>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd3        8       17        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       46       71        0
    //  no simd       80      123        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e12345] * self[scalar])
                    - (right_dual[e423] * self[e15])
                    - (right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
                right_dual[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]) + (right_dual[e12345] * self[e1]),
                (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]) + (right_dual[e12345] * self[e2]),
                (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]) + (right_dual[e12345] * self[e3]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) + (right_dual.group0().zxy() * self.group8().yzx()).with_w(right_dual[e12345] * self[e4])
                - (right_dual.group0().yzx() * self.group8().zxy()).with_w(right_dual[e423] * self[e415])
                - (self.group7().yzx() * right_dual.group2().zxy()).with_w(right_dual[e431] * self[e425]),
            // e5
            (right_dual[e12345] * self[e5])
                - (right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e315] * self[e4125]) + (right_dual[e12345] * self[e15]),
                (right_dual[e125] * self[e4235]) + (right_dual[e12345] * self[e25]),
                (right_dual[e235] * self[e4315]) + (right_dual[e12345] * self[e35]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) + (self.group9().www() * right_dual.group1().xyz()).with_w(right_dual[e12345] * self[e45])
                - (self.group9().yzxx() * right_dual.group2().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e12345]) * self.group4()) + (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group9().yzx())
                - (right_dual.group0().yzx() * self.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e12345]) * self.group5()) + (Simd32x3::from(self[e3215]) * right_dual.group0()) + (Simd32x3::from(self[e1234]) * right_dual.group2().xyz())
                - (Simd32x3::from(right_dual[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(right_dual[e12345]) * self.group6()) + (Simd32x4::from(self[e12345]) * right_dual.group1()),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e12345]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual.group0()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e12345]) * self.group8()) + (Simd32x3::from(self[e12345]) * right_dual.group2().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group9(),
            // e1234
            right_dual[e12345] * self[e1234],
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        8       17        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       43       65        0
    //  no simd       89      132        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e1234] * self[e5]) + (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4])
                    - (right_dual[e41] * self[e235])
                    - (right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_dual.group2().xyz().with_w(right_dual[e45]))
                + (self.group9().yzxz() * right_dual.group1().zxy().with_w(right_dual[e43]))
                + (self.group4() * right_dual.group3().www()).with_w(right_dual[e41] * self[e4235])
                + (self.group5().yzx() * right_dual.group3().zxy()).with_w(right_dual[e42] * self[e4315])
                - (Simd32x4::from(right_dual[e1234]) * self.group3())
                - (right_dual.group3().yzxy() * self.group5().zxy().with_w(self[e42]))
                - (right_dual.group0() * self.group9().www()).with_w(right_dual[e4235] * self[e41])
                - (right_dual.group1().yzx() * self.group9().zxy()).with_w(right_dual[e4125] * self[e43]),
            // e5
            (right_dual[e4235] * self[e15]) + (right_dual[e4315] * self[e25]) + (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45])
                - (right_dual[e45] * self[e3215])
                - (right_dual[e15] * self[e4235])
                - (right_dual[e25] * self[e4315])
                - (right_dual[e35] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e4125] * self[e315]) + (right_dual[e3215] * self[e415]),
                (right_dual[e4235] * self[e125]) + (right_dual[e3215] * self[e425]),
                (right_dual[e4315] * self[e235]) + (right_dual[e3215] * self[e435]),
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group2().xyz().with_w(right_dual[e45]))
                - (self.group8().zxy() * right_dual.group3().yzx()).with_w(right_dual[e4235] * self[e415]),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group0()) + (self.group7().zxy() * right_dual.group3().yzx())
                - (self.group7().yzx() * right_dual.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e1234]) * self.group8()) + (Simd32x3::from(right_dual[e3215]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz())
                - (Simd32x3::from(self[e321]) * right_dual.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group3().yzxw() * self.group9().zxy().with_w(self[e1234])) - (self.group9().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group3().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
            // e1234
            right_dual[e1234] * self[e12345],
        );
    }
}
impl BulkContraction<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(right_dual[e5] * self[e1234]) + (right_dual[e12345] * self[scalar]), right_dual[e12345] * self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e5
            (right_dual[e5] * self[e12345]) + (right_dual[e12345] * self[e5]),
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e12345]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group9(),
            // e1234
            right_dual[e12345] * self[e1234],
        );
    }
}
impl BulkContraction<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       20        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual[e15] * self[e423]) - (right_dual[e25] * self[e431]) - (right_dual[e35] * self[e412]) - (right_dual[e45] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e5
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual.group0(),
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
impl BulkContraction<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       35        0
    //    simd3        4       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       51        0
    //  no simd       50       88        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412])
                    - (right_dual[e45] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e15] * self[e1234]) + (right_dual[e4125] * self[e31]),
                (right_dual[e25] * self[e1234]) + (right_dual[e4235] * self[e12]),
                (right_dual[e35] * self[e1234]) + (right_dual[e4315] * self[e23]),
                -(right_dual[e4315] * self[e42]) - (right_dual[e4125] * self[e43]),
            ]) + (self.group4() * right_dual.group1().www()).with_w(right_dual[e45] * self[e1234])
                - (right_dual.group1().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (right_dual[e4235] * self[e15]) + (right_dual[e4315] * self[e25]) + (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45])
                - (right_dual[e15] * self[e4235])
                - (right_dual[e25] * self[e4315])
                - (right_dual[e35] * self[e4125])
                - (right_dual[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e4125] * self[e315]) + (right_dual[e3215] * self[e415]),
                (right_dual[e4235] * self[e125]) + (right_dual[e3215] * self[e425]),
                (right_dual[e4315] * self[e235]) + (right_dual[e3215] * self[e435]),
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group0())
                - (self.group8().zxy() * right_dual.group1().yzx()).with_w(right_dual[e4235] * self[e415]),
            // e41, e42, e43
            (self.group7().zxy() * right_dual.group1().yzx()) - (self.group7().yzx() * right_dual.group1().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e3215]) * self.group7()) - (Simd32x3::from(self[e321]) * right_dual.group1().xyz()),
            // e415, e425, e435, e321
            ((right_dual.group1().yzx() * self.group9().zxy()) - (right_dual.group1().zxy() * self.group9().yzx())).with_w(right_dual[e3215] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group1().xyz(),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        6        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       20       39        0
    //  no simd       26       63        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e315] * self[e412]),
                (right_dual[e425] * self[e321]) + (right_dual[e125] * self[e423]),
                (right_dual[e435] * self[e321]) + (right_dual[e235] * self[e431]),
                -(right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) - (right_dual.group1().zxy() * self.group7().yzx()).with_w(right_dual[e415] * self[e423]),
            // e5
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (self.group9().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * right_dual.group1(),
            // e415, e425, e435, e321
            self.group0().yy().with_zw(self[e12345], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        4       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       53        0
    //  no simd       50       90        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e12345] * self[scalar]) + (right_dual[e5] * self[e1234])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
                right_dual[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e12345] * self[e1]),
                (right_dual[e425] * self[e321]) + (right_dual[e12345] * self[e2]),
                (right_dual[e435] * self[e321]) + (right_dual[e12345] * self[e3]),
                -(right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) + (self.group7().zxy() * right_dual.group1().yzx()).with_w(right_dual[e12345] * self[e4])
                - (self.group7().yzx() * right_dual.group1().zxy()).with_w(right_dual[e415] * self[e423]),
            // e5
            (right_dual[e12345] * self[e5]) + (right_dual[e5] * self[e12345])
                - (right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e12345] * self[e15]) + (right_dual[e315] * self[e4125]),
                (right_dual[e12345] * self[e25]) + (right_dual[e125] * self[e4235]),
                (right_dual[e12345] * self[e35]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) + (right_dual.group0() * self.group9().www().with_w(self[e45]))
                - (self.group9().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e12345]) * self.group4()) + (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e12345]) * self.group5()) + (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual[e12345]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group0().xyz())).with_w(right_dual[e12345] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group7(),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e12345]) * self.group8()) + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group9(),
            // e1234
            right_dual[e12345] * self[e1234],
        );
    }
}
impl BulkContraction<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        4        9        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       20       38        0
    //  no simd       34       68        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e42] * right_dual[e4315]) - (self[e43] * right_dual[e4125]),
            ]) - (right_dual.group0().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (right_dual.group0().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (self.group7().zxy() * right_dual.group0().yzx()) - (self.group7().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e3215]) * self.group7()) - (Simd32x3::from(self[e321]) * right_dual.group0().xyz()),
            // e415, e425, e435, e321
            ((self.group9().zxy() * right_dual.group0().yzx()) - (self.group9().yzx() * right_dual.group0().zxy())).with_w(self[e1234] * right_dual[e3215]),
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group0().xyz(),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group0().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<AntiScalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Scalar::from_groups(/* scalar */ self[e12345] * right_dual[scalar]);
    }
}
impl BulkContraction<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       24       44        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual[e41] * self[e235])
                    - (right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4235]]) * right_dual.group2().with_w(right_dual[e41]))
                + (self.group9().yzxy() * right_dual.group1().zxy().with_w(right_dual[e42])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual.group2().with_w(right_dual[e45]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual.group1().xyz(),
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
impl BulkContraction<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       25       49        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[scalar] * self[e12345])
                    - (right_dual[e41] * self[e235])
                    - (right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4315]]) * right_dual.group2().xyz().with_w(right_dual[e42]))
                + (self.group9().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            // e15, e25, e35, e45
            Simd32x4::from(self[e12345]) * right_dual.group2().xyz().with_w(right_dual[e45]),
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual.group1().xyz(),
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
impl BulkContraction<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       42        0
    //    simd3        4       12        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       57        0
    //  no simd       54       90        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual[e423] * self[e15])
                    - (right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) - (right_dual.group0().yzx() * self.group8().zxy()).with_w(right_dual[e423] * self[e415])
                - (right_dual.group2().zxy() * self.group7().yzx()).with_w(right_dual[e431] * self[e425]),
            // e5
            -(right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (self.group9().yzxx() * right_dual.group2().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group9().yzx()) - (right_dual.group0().yzx() * self.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * right_dual.group0()) + (Simd32x3::from(self[e1234]) * right_dual.group2()) - (Simd32x3::from(right_dual[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd3        4       11        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       44       65        0
    //  no simd       64      105        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e4] * self[e3215]) + (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                    - (right_dual[e423] * self[e15])
                    - (right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group3().xyz().with_w(right_dual[e4]))
                - (right_dual.group0().yzx() * self.group8().zxy()).with_w(right_dual[e423] * self[e415])
                - (self.group7().yzx() * right_dual.group2().zxy()).with_w(right_dual[e431] * self[e425]),
            // e5
            (right_dual[e5] * self[e12345])
                - (right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (self.group9().yzxx() * right_dual.group2().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group9().yzx()) - (right_dual.group0().yzx() * self.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * right_dual.group0()) + (Simd32x3::from(self[e1234]) * right_dual.group2().xyz())
                - (Simd32x3::from(right_dual[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        1       14        0
    //  no simd        1       37        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(right_dual[e3215] * self[e4]) + (right_dual[scalar] * self[e12345]), 0.0]),
            // e1, e2, e3, e4
            right_dual.group0().xx().with_zw(right_dual[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group4().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e5
            right_dual[e3215] * self[e45],
            // e15, e25, e35, e45
            right_dual.group0().xx().with_zw(right_dual[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group6().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_dual[e3215]) * self.group7(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual[e3215] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[e3215]) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual[e3215] * self[e12345]),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        2        8        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       19       36        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual[e321]) * self.group6().xyz()).with_w(0.0) + (self.group7().zxy() * right_dual.group0().yzx()).with_w(0.0)
                - (self.group7().yzx() * right_dual.group0().zxy()).with_w(0.0),
            // e5
            -(right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            ((right_dual.group0().yzx() * self.group9().zxy()) - (right_dual.group0().zxy() * self.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual[e321] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd3        2        9        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       28       48        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43])
                    - (right_dual[e321] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual[e321]) * self.group6().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz()).with_w(0.0)
                + (self.group7().zxy() * right_dual.group0().yzx()).with_w(0.0)
                - (self.group7().yzx() * right_dual.group0().zxy()).with_w(0.0),
            // e5
            (right_dual[e5] * self[e12345]) - (right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            ((right_dual.group0().yzx() * self.group9().zxy()) - (right_dual.group0().zxy() * self.group9().yzx())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual[e321] * self[e12345]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e12345]) * right_dual.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        0        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       33        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -(right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e1234]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().zxy() * self.group9().yzx()).with_w(0.0)
                - (right_dual.group0().yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            // e15, e25, e35, e45
            self.group0().yy().with_zw(self[e12345], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e12345]) * right_dual.group0(),
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
impl BulkContraction<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        2        9        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       28       49        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[scalar] * self[e12345]) + (right_dual[e3215] * self[e4])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual[e3215]) * self.group4()).with_w(0.0)
                + (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()).with_w(0.0)
                + (right_dual.group0().zxy() * self.group9().yzx()).with_w(0.0)
                - (right_dual.group0().yzx() * self.group9().zxy()).with_w(0.0),
            // e5
            (right_dual[e3215] * self[e45]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual[e3215]) * self.group6().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz())).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e3215]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual[e3215] * self[e1234]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[e3215]) * self.group9().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual[e3215] * self[e12345]),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       94        0
    //    simd2        0        1        0
    //    simd3       20       35        0
    //    simd4       20       16        0
    // Totals...
    // yes simd      111      146        0
    //  no simd      211      265        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[scalar] * self[e12345])
                    + (right_dual[e12345] * self[scalar])
                    + (right_dual[e1] * self[e4235])
                    + (right_dual[e2] * self[e4315])
                    + (right_dual[e3] * self[e4125])
                    + (right_dual[e4] * self[e3215])
                    + (right_dual[e5] * self[e1234])
                    + (right_dual[e4235] * self[e1])
                    + (right_dual[e4315] * self[e2])
                    + (right_dual[e4125] * self[e3])
                    + (right_dual[e3215] * self[e4])
                    + (right_dual[e1234] * self[e5])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e41] * self[e235])
                    - (right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e423] * self[e15])
                    - (right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
                right_dual[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e15] * self[e1234]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e4125] * self[e31]),
                (right_dual[e25] * self[e1234]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e4235] * self[e12]),
                (right_dual[e35] * self[e1234]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e4315] * self[e23]),
                -(right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]) - (right_dual[e1234] * self[e45]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group1())
                + (Simd32x4::from(self[e12345]) * right_dual.group1())
                + (self.group9().yzxx() * right_dual.group5().zxy().with_w(right_dual[e41]))
                + (self.group4() * right_dual.group9().www()).with_w(right_dual[e45] * self[e1234])
                + (right_dual.group7().zxy() * self.group8().yzx()).with_w(right_dual[e42] * self[e4315])
                + (right_dual.group8().yzx() * self.group7().zxy()).with_w(right_dual[e43] * self[e4125])
                - (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], right_dual[e4125]]) * self.group3().xyz().with_w(self[e43]))
                - (right_dual.group9().yzxy() * self.group5().zxy().with_w(self[e42]))
                - (right_dual.group4() * self.group9().www()).with_w(right_dual[e423] * self[e415])
                - (right_dual.group5().yzx() * self.group9().zxy()).with_w(right_dual[e431] * self[e425])
                - (right_dual.group7().yzx() * self.group8().zxy()).with_w(right_dual[e412] * self[e435])
                - (right_dual.group8().zxy() * self.group7().yzx()).with_w(right_dual[e4235] * self[e41]),
            // e5
            (right_dual[e12345] * self[e5])
                + (right_dual[e5] * self[e12345])
                + (right_dual[e4235] * self[e15])
                + (right_dual[e4315] * self[e25])
                + (right_dual[e4125] * self[e35])
                + (right_dual[e3215] * self[e45])
                - (right_dual[e15] * self[e4235])
                - (right_dual[e25] * self[e4315])
                - (right_dual[e35] * self[e4125])
                - (right_dual[e45] * self[e3215])
                - (right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]) + (right_dual[e4125] * self[e315]) + (right_dual[e3215] * self[e415]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]) + (right_dual[e4235] * self[e125]) + (right_dual[e3215] * self[e425]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]) + (right_dual[e4315] * self[e235]) + (right_dual[e3215] * self[e435]),
                -(right_dual[e435] * self[e4125]) - (right_dual[e4235] * self[e415]) - (right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group3())
                + (Simd32x4::from(self[e12345]) * right_dual.group3())
                - (self.group9().yzxx() * right_dual.group8().zxy().with_w(right_dual[e415]))
                - (self.group8().zxy() * right_dual.group9().yzx()).with_w(right_dual[e425] * self[e4315]),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e12345]) * self.group4())
                + (Simd32x3::from(right_dual[e1234]) * self.group6().xyz())
                + (Simd32x3::from(self[e12345]) * right_dual.group4())
                + (Simd32x3::from(self[e1234]) * right_dual.group6().xyz())
                + (right_dual.group7().zxy() * self.group9().yzx())
                + (self.group7().zxy() * right_dual.group9().yzx())
                - (right_dual.group7().yzx() * self.group9().zxy())
                - (self.group7().yzx() * right_dual.group9().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e12345]) * self.group5())
                + (Simd32x3::from(right_dual[e3215]) * self.group7())
                + (Simd32x3::from(right_dual[e1234]) * self.group8())
                + (Simd32x3::from(self[e12345]) * right_dual.group5())
                + (Simd32x3::from(self[e3215]) * right_dual.group7())
                + (Simd32x3::from(self[e1234]) * right_dual.group8())
                - (Simd32x3::from(right_dual[e321]) * self.group9().xyz())
                - (Simd32x3::from(self[e321]) * right_dual.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(right_dual[e12345]) * self.group6())
                + (Simd32x4::from(self[e12345]) * right_dual.group6())
                + (right_dual.group9().yzxw() * self.group9().zxy().with_w(self[e1234]))
                - (right_dual.group9().zxy() * self.group9().yzx()).with_w(right_dual[e1234] * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e12345]) * self.group7()) + (Simd32x3::from(self[e12345]) * right_dual.group7()) + (Simd32x3::from(self[e1234]) * right_dual.group9().xyz())
                - (Simd32x3::from(right_dual[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e12345]) * self.group8()) + (Simd32x3::from(right_dual[e3215]) * self.group9().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group8())
                - (Simd32x3::from(self[e3215]) * right_dual.group9().xyz()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_dual[e12345]) * self.group9()) + (Simd32x4::from(self[e12345]) * right_dual.group9()),
            // e1234
            (right_dual[e12345] * self[e1234]) + (right_dual[e1234] * self[e12345]),
        );
    }
}
impl BulkContraction<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       21        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            self.group0().yy().with_zw(self[e12345], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e5
            right_dual[e5] * self[e12345],
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
impl BulkContraction<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        0
    //    simd3        6       11        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       25       45        0
    //  no simd       49       85        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]) + (self[e5] * right_dual[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e3215]) + (self[e31] * right_dual[e4125]),
                (self[e42] * right_dual[e3215]) + (self[e12] * right_dual[e4235]),
                (self[e43] * right_dual[e3215]) + (self[e23] * right_dual[e4315]),
                -(self[e45] * right_dual[e1234]) - (self[e43] * right_dual[e4125]),
            ]) - (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], right_dual[e4315]]) * self.group3().xyz().with_w(self[e42]))
                - (right_dual.group0().yzxx() * self.group5().zxy().with_w(self[e41])),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) - (right_dual.group0().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group6().xyz()) + (self.group7().zxy() * right_dual.group0().yzx()) - (self.group7().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e3215]) * self.group7()) + (Simd32x3::from(right_dual[e1234]) * self.group8()) - (Simd32x3::from(self[e321]) * right_dual.group0().xyz()),
            // e415, e425, e435, e321
            (self.group9().zxy() * right_dual.group0().yzx()).with_w(self[e1234] * right_dual[e3215])
                - (self.group9().yzxw() * right_dual.group0().zxy().with_w(right_dual[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group0().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1234
            self[e12345] * right_dual[e1234],
        );
    }
}
impl BulkContraction<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(right_dual[e12345]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e5
            right_dual[e12345] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e12345]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(right_dual[e12345]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e12345]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(right_dual[e12345]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e12345]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group9(),
            // e1234
            right_dual[e12345] * self[e1234],
        );
    }
}
impl BulkContraction<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       14        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]) + (self[e1234] * right_dual[e5]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e5
            self[e12345] * right_dual[e5],
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
impl BulkContraction<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       39        0
    //    simd3        8       18        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       44       68        0
    //  no simd       90      137        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[e12345] * right_dual[scalar])
                    + (self[e1] * right_dual[e4235])
                    + (self[e2] * right_dual[e4315])
                    + (self[e3] * right_dual[e4125])
                    + (self[e4] * right_dual[e3215])
                    + (self[e5] * right_dual[e1234])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e423] * right_dual[e15])
                    - (self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * right_dual.group2().xyz().with_w(right_dual[e45]))
                + (self.group9().yzxz() * right_dual.group1().zxy().with_w(right_dual[e43]))
                + (self.group4() * right_dual.group3().www()).with_w(self[e4235] * right_dual[e41])
                + (self.group5().yzx() * right_dual.group3().zxy()).with_w(self[e4315] * right_dual[e42])
                - (right_dual.group3().yzxx() * self.group5().zxy().with_w(self[e41]))
                - (self.group9().zxy() * right_dual.group1().yzx()).with_w(self[e43] * right_dual[e4125])
                - (self.group9().www() * right_dual.group0().xyz()).with_w(self[e45] * right_dual[e1234])
                - (right_dual.group2().www() * self.group3().xyz()).with_w(self[e42] * right_dual[e4315]),
            // e5
            (self[e15] * right_dual[e4235]) + (self[e25] * right_dual[e4315]) + (self[e35] * right_dual[e4125]) + (self[e45] * right_dual[e3215])
                - (self[e4235] * right_dual[e15])
                - (self[e4315] * right_dual[e25])
                - (self[e4125] * right_dual[e35])
                - (self[e3215] * right_dual[e45]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e415] * right_dual[e3215]) + (self[e315] * right_dual[e4125]),
                (self[e425] * right_dual[e3215]) + (self[e125] * right_dual[e4235]),
                (self[e435] * right_dual[e3215]) + (self[e235] * right_dual[e4315]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group2().xyz().with_w(right_dual[e45]))
                - (right_dual.group3().yzxx() * self.group8().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * right_dual.group0().xyz())
                + (Simd32x3::from(right_dual[e1234]) * self.group6().xyz())
                + (self.group7().zxy() * right_dual.group3().yzx())
                - (self.group7().yzx() * right_dual.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e12345]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[e1234]) * self.group8()) + (Simd32x3::from(right_dual[e3215]) * self.group7())
                - (Simd32x3::from(self[e321]) * right_dual.group3().xyz()),
            // e415, e425, e435, e321
            (self.group9().zxy() * right_dual.group3().yzx()).with_w(self[e1234] * right_dual[e3215])
                - (self.group9().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group9().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group9().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group3().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
            // e1234
            self[e12345] * right_dual[e1234],
        );
    }
}
impl BulkContraction<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       53        0
    //    simd3        8       15        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       53       78        0
    //  no simd       90      138        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self[scalar] * right_dual[e12345])
                    + (self[e4235] * right_dual[e1])
                    + (self[e4315] * right_dual[e2])
                    + (self[e4125] * right_dual[e3])
                    + (self[e3215] * right_dual[e4])
                    + (self[e1234] * right_dual[e5])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412])
                    - (self[e45] * right_dual[e321])
                    - (self[e41] * right_dual[e235])
                    - (self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435]),
                self[e12345] * right_dual[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e1] * right_dual[e12345]) + (self[e415] * right_dual[e321]) + (self[e321] * right_dual[e415]) + (self[e315] * right_dual[e412]),
                (self[e2] * right_dual[e12345]) + (self[e425] * right_dual[e321]) + (self[e321] * right_dual[e425]) + (self[e125] * right_dual[e423]),
                (self[e3] * right_dual[e12345]) + (self[e435] * right_dual[e321]) + (self[e321] * right_dual[e435]) + (self[e235] * right_dual[e431]),
                -(self[e415] * right_dual[e423]) - (self[e425] * right_dual[e431]) - (self[e435] * right_dual[e412]) - (self[e412] * right_dual[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group3())
                + (self.group7().zxy() * right_dual.group2().yzx()).with_w(self[e4] * right_dual[e12345])
                - (self.group7().yzx() * right_dual.group2().zxy()).with_w(self[e423] * right_dual[e415])
                - (self.group8().zxy() * right_dual.group0().yzx()).with_w(self[e431] * right_dual[e425]),
            // e5
            (self[e12345] * right_dual[e5]) + (self[e5] * right_dual[e12345])
                - (self[e415] * right_dual[e235])
                - (self[e425] * right_dual[e315])
                - (self[e435] * right_dual[e125])
                - (self[e235] * right_dual[e415])
                - (self[e315] * right_dual[e425])
                - (self[e125] * right_dual[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (self[e4125] * right_dual[e315]) + (self[e3215] * right_dual[e415]),
                (self[e4235] * right_dual[e125]) + (self[e3215] * right_dual[e425]),
                (self[e4315] * right_dual[e235]) + (self[e3215] * right_dual[e435]),
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group3())
                - (self.group9().yzxx() * right_dual.group2().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group4()) + (self.group9().yzx() * right_dual.group0().zxy())
                - (self.group9().zxy() * right_dual.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * right_dual.group0().xyz())
                + (Simd32x3::from(self[e1234]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e12345]) * self.group5())
                - (Simd32x3::from(right_dual[e321]) * self.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * right_dual.group1()) + (Simd32x4::from(right_dual[e12345]) * self.group6()),
            // e423, e431, e412
            (Simd32x3::from(self[e12345]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group7()),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * right_dual.group2().xyz()) + (Simd32x3::from(right_dual[e12345]) * self.group8()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group9(),
            // e1234
            self[e1234] * right_dual[e12345],
        );
    }
}
impl std::ops::Div<bulk_contraction> for Plane {
    type Output = bulk_contraction_partial<Plane>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        5       15        0
    //  no simd       16       39        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * self[e3215],
                right_dual[e431] * self[e3215],
                right_dual[e412] * self[e3215],
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * right_dual.group1().xyz()).with_w(0.0) + (right_dual.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        6       25        0
    //  no simd       17       51        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual.group3().yzx() * self.group0().zxy()) - (right_dual.group3().zxy() * self.group0().yzx())).with_w(right_dual[e1234] * self[e3215] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                right_dual[e4235] * self[e3215] * -1.0,
                right_dual[e4315] * self[e3215] * -1.0,
                right_dual[e4125] * self[e3215] * -1.0,
                (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]),
            ]) + (self.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e12] * self[e4315],
                right_dual[e23] * self[e4125],
                right_dual[e31] * self[e4235],
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]))
                - (self.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<AntiDualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<AntiFlatPoint> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
            0.0,
        ]));
    }
}
impl BulkContraction<AntiFlector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        9       24        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_dual.group1().yzx() * self.group0().zxy()) - (right_dual.group1().zxy() * self.group0().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                right_dual[e3215] * self[e4235],
                right_dual[e3215] * self[e4315],
                right_dual[e3215] * self[e4125],
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
            ]) - (self.group0().wwwx() * right_dual.group1().xyz().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<AntiLine> for Plane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
        );
    }
}
impl BulkContraction<AntiMotor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
        );
    }
}
impl BulkContraction<AntiPlane> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Line::from_groups(
            // e415, e425, e435
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkContraction<Circle> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]),
            ]) + (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<CircleRotor> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       24        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]),
            ]) + (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<Dipole> for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       14       34        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * self[e3215],
                right_dual[e431] * self[e3215],
                right_dual[e412] * self[e3215],
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * right_dual.group1().xyz()) + (right_dual.group2().yzx() * self.group0().zxy()) - (right_dual.group2().zxy() * self.group0().yzx()),
        );
    }
}
impl BulkContraction<DipoleInversion> for Plane {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        3        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       17       46        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * self[e3215],
                right_dual[e431] * self[e3215],
                right_dual[e412] * self[e3215],
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                right_dual[e125] * self[e4315] * -1.0,
                right_dual[e235] * self[e4125] * -1.0,
                right_dual[e315] * self[e4235] * -1.0,
                (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]),
            ]) + (Simd32x4::from(self[e3215]) * right_dual.group1().xyz().with_w(right_dual[e4]))
                + (self.group0().zxyx() * right_dual.group2().yzx().with_w(right_dual[e1])),
        );
    }
}
impl BulkContraction<DualNum> for Plane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            right_dual.group0().xx().with_zw(right_dual[e3215], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<FlatPoint> for Plane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual[e321]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
        );
    }
}
impl BulkContraction<Flector> for Plane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        5       25        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 1.0])
                * right_dual
                    .group0()
                    .www()
                    .with_w((right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Line> for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e12] * self[e4315],
                right_dual[e23] * self[e4125],
                right_dual[e31] * self[e4235],
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<Motor> for Plane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        6        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       29        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e12] * self[e4315],
                right_dual[e23] * self[e4125],
                right_dual[e31] * self[e4235],
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        0
    //    simd2        0        1        0
    //    simd3        4       12        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       20       47        0
    //  no simd       34       90        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]),
            ]) + (self.group0().yzxx() * right_dual.group5().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual.group8().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (right_dual.group7().zxy() * self.group0().yzx()) - (right_dual.group7().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * right_dual.group7()) - (Simd32x3::from(right_dual[e321]) * self.group0().xyz()),
            // e415, e425, e435, e321
            ((right_dual.group9().yzx() * self.group0().zxy()) - (right_dual.group9().zxy() * self.group0().yzx())).with_w(right_dual[e1234] * self[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1234
            0.0,
        );
    }
}
impl BulkContraction<Plane> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(/* scalar */ (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]));
    }
}
impl BulkContraction<RoundPoint> for Plane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        6       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy())).with_w(self[e3215] * right_dual[e1234] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkContraction<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[e12345]) * self.group0());
    }
}
impl BulkContraction<Sphere> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Scalar::from_groups(
            // scalar
            (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
        );
    }
}
impl BulkContraction<VersorEven> for Plane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        7        0
    // Totals...
    // yes simd        6       26        0
    //  no simd       17       55        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((self.group0().zxy() * right_dual.group3().yzx()) - (self.group0().yzx() * right_dual.group3().zxy())).with_w(self[e3215] * right_dual[e1234] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e3215] * right_dual[e4235] * -1.0,
                self[e3215] * right_dual[e4315] * -1.0,
                self[e3215] * right_dual[e4125] * -1.0,
                (self[e4315] * right_dual[e42]) + (self[e4125] * right_dual[e43]),
            ]) + (self.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e4315] * right_dual[e12],
                self[e4125] * right_dual[e23],
                self[e4235] * right_dual[e31],
                -(self[e4125] * right_dual[e35]) - (self[e3215] * right_dual[e45]),
            ]) - (self.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15]))
                - (self.group0().wwwy() * right_dual.group0().xyz().with_w(right_dual[e25])),
        );
    }
}
impl BulkContraction<VersorOdd> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       14        0
    //    simd3        0        3        0
    //    simd4        4        7        0
    // Totals...
    // yes simd        7       24        0
    //  no simd       19       51        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e4125] * right_dual[e431] * -1.0,
                self[e4235] * right_dual[e412] * -1.0,
                self[e4315] * right_dual[e423] * -1.0,
                (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
            ]) + (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e1])),
            // e23, e31, e12, e45
            Simd32x4::from([
                self[e3215] * right_dual[e423],
                self[e3215] * right_dual[e431],
                self[e3215] * right_dual[e412],
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) - (self.group0().xyzx() * right_dual.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(self[e3215]) * right_dual.group1().xyz()).with_w(0.0) + (self.group0().zxy() * right_dual.group2().yzx()).with_w(0.0)
                - (self.group0().yzx() * right_dual.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
        );
    }
}
impl std::ops::Div<bulk_contraction> for RoundPoint {
    type Output = bulk_contraction_partial<RoundPoint>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e5
            right_dual[e12345] * self[e5],
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       17        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            (right_dual[e1234] * self[e5]) + (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
        );
    }
}
impl BulkContraction<AntiDualNum> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e5
            right_dual[e12345] * self[e5],
        );
    }
}
impl BulkContraction<AntiFlector> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
        );
    }
}
impl BulkContraction<AntiMotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e5
            right_dual[e12345] * self[e5],
        );
    }
}
impl BulkContraction<AntiPlane> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Scalar::from_groups(
            // scalar
            (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
        );
    }
}
impl BulkContraction<DualNum> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Scalar::from_groups(/* scalar */ right_dual[e3215] * self[e4]);
    }
}
impl BulkContraction<Motor> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(/* scalar */ right_dual[e3215] * self[e4]);
    }
}
impl BulkContraction<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       15        0
    //  no simd        4       32        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]) + (right_dual[e1234] * self[e5]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e5
            right_dual[e12345] * self[e5],
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
impl BulkContraction<RoundPoint> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Scalar::from_groups(
            // scalar
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkContraction<Scalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e5
            right_dual[e12345] * self[e5],
        );
    }
}
impl BulkContraction<VersorEven> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        4       21        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Scalar::from_groups(
            // scalar
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkContraction<VersorOdd> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       21        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e5
            self[e5] * right_dual[e12345],
        );
    }
}
impl std::ops::Div<bulk_contraction> for Scalar {
    type Output = bulk_contraction_partial<Scalar>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(/* scalar */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkContraction<AntiDualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Scalar::from_groups(/* scalar */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkContraction<AntiMotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(/* scalar */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkContraction<MultiVector> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       23        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Scalar::from_groups(/* scalar */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkContraction<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Scalar::from_groups(/* scalar */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkContraction<VersorOdd> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       17        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Scalar::from_groups(/* scalar */ self[scalar] * right_dual[e12345]);
    }
}
impl std::ops::Div<bulk_contraction> for Sphere {
    type Output = bulk_contraction_partial<Sphere>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        7        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       20       46        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e423] * self[e3215]) + (right_dual[e235] * self[e1234]),
                (right_dual[e431] * self[e3215]) + (right_dual[e315] * self[e1234]),
                (right_dual[e412] * self[e3215]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual.group1().xyz()) + (right_dual.group2().yzx() * self.group0().zxy()) - (right_dual.group2().zxy() * self.group0().yzx()))
                .with_w(right_dual[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       11       27        0
    //  no simd       25       55        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_dual.group3().yzxw() * self.group0().zxy().with_w(self[e1234])) - (self.group0().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e235, e315, e125, e4
            Simd32x4::from([
                right_dual[e4235] * self[e3215] * -1.0,
                right_dual[e4315] * self[e3215] * -1.0,
                right_dual[e4125] * self[e3215] * -1.0,
                (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (self.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e12] * self[e4315]) + (right_dual[e15] * self[e1234]),
                (right_dual[e23] * self[e4125]) + (right_dual[e25] * self[e1234]),
                (right_dual[e31] * self[e4235]) + (right_dual[e35] * self[e1234]),
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]))
                - (self.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<AntiDualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(right_dual[e5] * self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(right_dual[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e5
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
        );
    }
}
impl BulkContraction<AntiFlector> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        9       33        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group1().xyz(),
            // e415, e425, e435, e321
            ((right_dual.group1().yzx() * self.group0().zxy()) - (right_dual.group1().zxy() * self.group0().yzx())).with_w(right_dual[e3215] * self[e1234]),
            // e235, e315, e125, e4
            ((Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group1().xyz())).with_w(right_dual[e45] * self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215])),
        );
    }
}
impl BulkContraction<AntiLine> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       25        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual
                    .group1()
                    .with_w(-(right_dual[e415] * self[e4235]) - (right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125])),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * right_dual.group0()) + (right_dual.group1().yzx() * self.group0().zxy()) - (right_dual.group1().zxy() * self.group0().yzx()),
        );
    }
}
impl BulkContraction<AntiMotor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       33        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[e1234]) * right_dual.group0().xyz().with_w(right_dual[e5]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual
                    .group1()
                    .xyz()
                    .with_w(-(right_dual[e415] * self[e4235]) - (right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual.group0().xyz()) + (right_dual.group1().yzx() * self.group0().zxy()) - (right_dual.group1().zxy() * self.group0().yzx()))
                .with_w(right_dual[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
        );
    }
}
impl BulkContraction<AntiPlane> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       20        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group0().xyz(),
            // e415, e425, e435, e321
            ((right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx())).with_w(right_dual[e3215] * self[e1234]),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkContraction<Circle> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4235]]) * right_dual.group2().with_w(right_dual[e41]))
                + (self.group0().yzxy() * right_dual.group1().zxy().with_w(right_dual[e42])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<CircleRotor> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4315]]) * right_dual.group2().xyz().with_w(right_dual[e42]))
                + (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<Dipole> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       20       40        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e423] * self[e3215]) + (right_dual[e235] * self[e1234]),
                (right_dual[e431] * self[e3215]) + (right_dual[e315] * self[e1234]),
                (right_dual[e412] * self[e3215]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(self[e3215]) * right_dual.group1().xyz()) + (right_dual.group2().yzx() * self.group0().zxy()) - (right_dual.group2().zxy() * self.group0().yzx()),
        );
    }
}
impl BulkContraction<DipoleInversion> for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        4        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       11       27        0
    //  no simd       24       53        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e423] * self[e3215]) + (right_dual[e235] * self[e1234]),
                (right_dual[e431] * self[e3215]) + (right_dual[e315] * self[e1234]),
                (right_dual[e412] * self[e3215]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                right_dual[e125] * self[e4315] * -1.0,
                right_dual[e235] * self[e4125] * -1.0,
                right_dual[e315] * self[e4235] * -1.0,
                (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
            ]) + (Simd32x4::from(self[e3215]) * right_dual.group1().xyz().with_w(right_dual[e4]))
                + (self.group0().zxyx() * right_dual.group2().yzx().with_w(right_dual[e1])),
        );
    }
}
impl BulkContraction<DualNum> for Sphere {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(right_dual[e3215]) * self.group0().xyz().with_w(self[e1234]));
    }
}
impl BulkContraction<FlatPoint> for Sphere {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiLine::from_groups(
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e321]) * self.group0().xyz()),
            // e15, e25, e35
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
        );
    }
}
impl BulkContraction<Flector> for Sphere {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        9       27        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                right_dual[e321] * self[e4235] * -1.0,
                right_dual[e321] * self[e4315] * -1.0,
                right_dual[e321] * self[e4125] * -1.0,
                (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
            ]) + (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4235]]) * right_dual.group0().xyz().with_w(right_dual[e1])),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Line> for Sphere {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e12] * self[e4315]) + (right_dual[e15] * self[e1234]),
                (right_dual[e23] * self[e4125]) + (right_dual[e25] * self[e1234]),
                (right_dual[e31] * self[e4235]) + (right_dual[e35] * self[e1234]),
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<Motor> for Sphere {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e3215]) * self.group0().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e12] * self[e4315]) + (right_dual[e15] * self[e1234]),
                (right_dual[e23] * self[e4125]) + (right_dual[e25] * self[e1234]),
                (right_dual[e31] * self[e4235]) + (right_dual[e35] * self[e1234]),
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       29        0
    //    simd2        0        1        0
    //    simd3        6       13        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       25       51        0
    //  no simd       49      102        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]) + (right_dual[e5] * self[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e45] * self[e1234]) + (right_dual[e43] * self[e4125]),
            ]) + (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e4315]]) * right_dual.group3().xyz().with_w(right_dual[e42]))
                + (self.group0().yzxx() * right_dual.group5().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (self.group0().yzxx() * right_dual.group8().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group6().xyz()) + (right_dual.group7().zxy() * self.group0().yzx()) - (right_dual.group7().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e3215]) * right_dual.group7()) + (Simd32x3::from(self[e1234]) * right_dual.group8()) - (Simd32x3::from(right_dual[e321]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_dual.group9().yzxw() * self.group0().zxy().with_w(self[e1234])) - (right_dual.group9().zxy() * self.group0().yzx()).with_w(right_dual[e1234] * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group9().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1234
            right_dual[e12345] * self[e1234],
        );
    }
}
impl BulkContraction<Plane> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(
            // scalar
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkContraction<RoundPoint> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        2        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd       10       25        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (right_dual.group0().yzxw() * self.group0().zxy().with_w(self[e1234])) - (right_dual.group0().zxy() * self.group0().yzx()).with_w(right_dual[e1234] * self[e3215]),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkContraction<Scalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e1234
            right_dual[e12345] * self[e1234],
        );
    }
}
impl BulkContraction<Sphere> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Scalar::from_groups(
            // scalar
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkContraction<VersorEven> for Sphere {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        3        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       25       59        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()),
            // e415, e425, e435, e321
            (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e1234] * right_dual[e3215])
                - (self.group0().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e235, e315, e125, e4
            Simd32x4::from([
                self[e3215] * right_dual[e4235] * -1.0,
                self[e3215] * right_dual[e4315] * -1.0,
                self[e3215] * right_dual[e4125] * -1.0,
                (self[e4315] * right_dual[e42]) + (self[e4125] * right_dual[e43]) + (self[e1234] * right_dual[e45]),
            ]) + (self.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self[e4315] * right_dual[e12]) + (self[e1234] * right_dual[e15]),
                (self[e4125] * right_dual[e23]) + (self[e1234] * right_dual[e25]),
                (self[e4235] * right_dual[e31]) + (self[e1234] * right_dual[e35]),
                -(self[e4125] * right_dual[e35]) - (self[e3215] * right_dual[e45]),
            ]) - (self.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15]))
                - (self.group0().wwwy() * right_dual.group0().xyz().with_w(right_dual[e25])),
        );
    }
}
impl BulkContraction<VersorOdd> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        2        3        0
    //    simd4        3        8        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       24       59        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self[e4125] * right_dual[e431] * -1.0,
                self[e4235] * right_dual[e412] * -1.0,
                self[e4315] * right_dual[e423] * -1.0,
                (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]) + (self[e1234] * right_dual[e5]),
            ]) + (Simd32x4::from([self[e1234], self[e1234], self[e1234], right_dual[e2]]) * right_dual.group1().xyz().with_w(self[e4315]))
                + (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e1])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e3215] * right_dual[e423]) + (self[e1234] * right_dual[e235]),
                (self[e3215] * right_dual[e431]) + (self[e1234] * right_dual[e315]),
                (self[e3215] * right_dual[e412]) + (self[e1234] * right_dual[e125]),
                -(self[e4315] * right_dual[e425]) - (self[e4125] * right_dual[e435]),
            ]) - (self.group0().xyzx() * right_dual.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e3215]) * right_dual.group1().xyz()) + (self.group0().zxy() * right_dual.group2().yzx()) - (self.group0().yzx() * right_dual.group2().zxy()))
                .with_w(self[e1234] * right_dual[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group0(),
        );
    }
}
impl std::ops::Div<bulk_contraction> for VersorEven {
    type Output = bulk_contraction_partial<VersorEven>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd3        1        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       23       40        0
    //  no simd       40       67        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(right_dual[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group0())).with_w(right_dual[e12345] * self[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(right_dual[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * right_dual.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                right_dual[e12345] * self[e235],
                right_dual[e12345] * self[e315],
                right_dual[e12345] * self[e125],
                -(right_dual[e415] * self[e235])
                    - (right_dual[e425] * self[e315])
                    - (right_dual[e435] * self[e125])
                    - (right_dual[e235] * self[e415])
                    - (right_dual[e315] * self[e425])
                    - (right_dual[e125] * self[e435]),
            ]) + (right_dual.group2() * self.group0().www().with_w(self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]) + (right_dual[e12345] * self[e1]),
                (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]) + (right_dual[e12345] * self[e2]),
                (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]) + (right_dual[e12345] * self[e3]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) + (right_dual.group0().zxy() * self.group2().yzx()).with_w(right_dual[e12345] * self[e4])
                - (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e423] * self[e415])
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(right_dual[e431] * self[e425]),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       23       37        0
    //  no simd       47       72        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual.group3().yzxy() * self.group0().zxy().with_w(self[e2]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e45] * self[e321])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                )
                + (right_dual.group0() * self.group0().www()).with_w(right_dual[e1234] * self[e5])
                + (right_dual.group2().www() * self.group1().xyz()).with_w(right_dual[e4235] * self[e1])
                - (right_dual.group3().zxy() * self.group0().yzx()).with_w(right_dual[e41] * self[e235]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e1234] * self[e235]) + (right_dual[e3215] * self[e423]),
                (right_dual[e1234] * self[e315]) + (right_dual[e3215] * self[e431]),
                (right_dual[e1234] * self[e125]) + (right_dual[e3215] * self[e412]),
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group1())
                - (right_dual.group3().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e3215]) * self.group1().xyz())
                + (Simd32x3::from(self[e12345]) * right_dual.group2().xyz())
                + (right_dual.group3().zxy() * self.group2().yzx())
                - (right_dual.group3().yzx() * self.group2().zxy()))
            .with_w(right_dual[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 1.0])
                * right_dual.group0().yy().with_zw(right_dual[e12345], (right_dual[e5] * self[e12345]) + (right_dual[e12345] * self[e5])),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(right_dual[e45] * self[e12345]),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e15] * self[e423]) - (right_dual[e25] * self[e431]) - (right_dual[e35] * self[e412]) - (right_dual[e45] * self[e321])),
        );
    }
}
impl BulkContraction<AntiFlector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        4        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       13       19        0
    //  no simd       34       48        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412])
                        - (right_dual[e45] * self[e321]),
                )
                - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e15])),
            // e23, e31, e12, e45
            (self.group0() * right_dual.group1().www().with_w(right_dual[e45])) + Simd32x3::from(0.0).with_w(-(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]))
                - (right_dual.group1().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual[e3215]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e12345]) * right_dual.group0().xyz()).with_w(0.0)
                + (right_dual.group1().zxy() * self.group2().yzx()).with_w(0.0)
                - (right_dual.group1().yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group1(),
        );
    }
}
impl BulkContraction<AntiLine> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       40        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual
                    .group1()
                    .with_w(-(right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412])),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e315] * self[e412]),
                (right_dual[e425] * self[e321]) + (right_dual[e125] * self[e423]),
                (right_dual[e435] * self[e321]) + (right_dual[e235] * self[e431]),
                -(right_dual[e425] * self[e315])
                    - (right_dual[e435] * self[e125])
                    - (right_dual[e235] * self[e415])
                    - (right_dual[e315] * self[e425])
                    - (right_dual[e125] * self[e435]),
            ]) - (right_dual.group1().zxy() * self.group0().yzx()).with_w(right_dual[e415] * self[e235]),
        );
    }
}
impl BulkContraction<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       28       49        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual[e12345]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group0().xyz())).with_w(right_dual[e12345] * self[e321]),
            // e235, e315, e125, e5
            (Simd32x4::from(right_dual[e12345]) * self.group2())
                + (Simd32x4::from(self[e12345]) * right_dual.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e415] * self[e235])
                        - (right_dual[e425] * self[e315])
                        - (right_dual[e435] * self[e125])
                        - (right_dual[e235] * self[e415])
                        - (right_dual[e315] * self[e425])
                        - (right_dual[e125] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e12345] * self[e1]) + (right_dual[e315] * self[e412]),
                (right_dual[e12345] * self[e2]) + (right_dual[e125] * self[e423]),
                (right_dual[e12345] * self[e3]) + (right_dual[e235] * self[e431]),
                -(right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) + (right_dual.group0() * self.group1().www().with_w(self[e4]))
                - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
        );
    }
}
impl BulkContraction<AntiPlane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       14        0
    //    simd3        0        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        7       21        0
    //  no simd       19       39        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                right_dual[e4125] * self[e431] * -1.0,
                right_dual[e4235] * self[e412] * -1.0,
                right_dual[e4315] * self[e423] * -1.0,
                (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
            ]) + (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e1])),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e3215] * self[e423],
                right_dual[e3215] * self[e431],
                right_dual[e3215] * self[e412],
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) - (right_dual.group0().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_dual[e3215]) * self.group1().xyz()).with_w(0.0) + (right_dual.group0().zxy() * self.group2().yzx()).with_w(0.0)
                - (right_dual.group0().yzx() * self.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<AntiScalar> for VersorEven {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_contraction(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Scalar::from_groups(/* scalar */ right_dual[scalar] * self[e12345]);
    }
}
impl BulkContraction<Circle> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       14        0
    //  no simd        9       25        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().with_w(
                    -(right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e45] * self[e321])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
        );
    }
}
impl BulkContraction<CircleRotor> for VersorEven {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       30        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().xyz().with_w(
                    (right_dual[scalar] * self[e12345])
                        - (right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e45] * self[e321])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
        );
    }
}
impl BulkContraction<Dipole> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       51        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().with_w(
                    -(right_dual[e423] * self[e415])
                        - (right_dual[e431] * self[e425])
                        - (right_dual[e412] * self[e435])
                        - (right_dual[e415] * self[e423])
                        - (right_dual[e425] * self[e431])
                        - (right_dual[e435] * self[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e412] * self[e315]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e423] * self[e125]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e431] * self[e235]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e415] * self[e235]) - (right_dual[e425] * self[e315]) - (right_dual[e435] * self[e125]) - (right_dual[e125] * self[e435]),
            ]) - (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e235] * self[e415])
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(right_dual[e315] * self[e425]),
        );
    }
}
impl BulkContraction<DipoleInversion> for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        4        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       21       35        0
    //  no simd       30       61        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e12345]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[e12345]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group2().xyz().with_w(
                    (right_dual[e4] * self[e12345])
                        - (right_dual[e423] * self[e415])
                        - (right_dual[e431] * self[e425])
                        - (right_dual[e412] * self[e435])
                        - (right_dual[e415] * self[e423])
                        - (right_dual[e425] * self[e431])
                        - (right_dual[e435] * self[e412]),
                ),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]) + (right_dual[e1] * self[e12345]),
                (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]) + (right_dual[e2] * self[e12345]),
                (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]) + (right_dual[e3] * self[e12345]),
                -(right_dual[e435] * self[e125]) - (right_dual[e235] * self[e415]) - (right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) + (right_dual.group0().zxy() * self.group2().yzx()).with_w(right_dual[e5] * self[e12345])
                - (self.group2().zxyx() * right_dual.group0().yzx().with_w(right_dual[e415]))
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(right_dual[e425] * self[e315]),
        );
    }
}
impl BulkContraction<DualNum> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        1       12        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e423], self[e431], self[e412], 1.0])
                * right_dual
                    .group0()
                    .xx()
                    .with_zw(right_dual[e3215], (right_dual[e3215] * self[e4]) + (right_dual[scalar] * self[e12345])),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual[e3215]) * self.group1().xyz().with_w(self[e12345]),
        );
    }
}
impl BulkContraction<FlatPoint> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e315] * self[e412]) + (right_dual[e321] * self[e415]),
                (right_dual[e125] * self[e423]) + (right_dual[e321] * self[e425]),
                (right_dual[e235] * self[e431]) + (right_dual[e321] * self[e435]),
                -(right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) - (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        );
    }
}
impl BulkContraction<Flector> for VersorEven {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e321] * self[e415]) + (right_dual[e1] * self[e12345]),
                (right_dual[e321] * self[e425]) + (right_dual[e2] * self[e12345]),
                (right_dual[e321] * self[e435]) + (right_dual[e3] * self[e12345]),
                -(right_dual[e315] * self[e425]) - (right_dual[e125] * self[e435]),
            ]) + (self.group0().zxyw() * right_dual.group0().yzx().with_w(right_dual[e5]))
                - (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e415])),
        );
    }
}
impl BulkContraction<Line> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       22        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self[e12345], self[e12345], self[e12345], 1.0])
                * right_dual.group0().with_w(
                    -(right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
            // e15, e25, e35, e3215
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkContraction<Motor> for VersorEven {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x4::from(right_dual[e3215]) * self.group0().xyz().with_w(self[e4]))
                + (Simd32x4::from(self[e12345]) * right_dual.group0())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
            // e15, e25, e35, e3215
            ((Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group1().xyz())).with_w(right_dual[e3215] * self[e12345]),
        );
    }
}
impl BulkContraction<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       55        0
    //    simd2        0        1        0
    //    simd3        8       17        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       53       82        0
    //  no simd       90      144        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[scalar] * self[e12345])
                    + (right_dual[e4235] * self[e1])
                    + (right_dual[e4315] * self[e2])
                    + (right_dual[e4125] * self[e3])
                    + (right_dual[e3215] * self[e4])
                    + (right_dual[e1234] * self[e5])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e41] * self[e235])
                    - (right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435]),
                right_dual[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e1] * self[e12345]) + (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]),
                (right_dual[e2] * self[e12345]) + (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]),
                (right_dual[e3] * self[e12345]) + (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]),
                -(right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]) - (right_dual[e412] * self[e435]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group3())
                + (right_dual.group7().zxy() * self.group2().yzx()).with_w(right_dual[e4] * self[e12345])
                - (right_dual.group7().yzx() * self.group2().zxy()).with_w(right_dual[e423] * self[e415])
                - (right_dual.group8().zxy() * self.group0().yzx()).with_w(right_dual[e431] * self[e425]),
            // e5
            (right_dual[e12345] * self[e5]) + (right_dual[e5] * self[e12345])
                - (right_dual[e415] * self[e235])
                - (right_dual[e425] * self[e315])
                - (right_dual[e435] * self[e125])
                - (right_dual[e235] * self[e415])
                - (right_dual[e315] * self[e425])
                - (right_dual[e125] * self[e435]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e4125] * self[e315]) + (right_dual[e3215] * self[e415]),
                (right_dual[e4235] * self[e125]) + (right_dual[e3215] * self[e425]),
                (right_dual[e4315] * self[e235]) + (right_dual[e3215] * self[e435]),
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) + (Simd32x4::from(self[e12345]) * right_dual.group3())
                - (right_dual.group9().yzxx() * self.group2().zxy().with_w(self[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group4()) + (right_dual.group9().yzx() * self.group0().zxy())
                - (right_dual.group9().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e3215]) * self.group0().xyz())
                + (Simd32x3::from(right_dual[e1234]) * self.group2().xyz())
                + (Simd32x3::from(self[e12345]) * right_dual.group5())
                - (Simd32x3::from(self[e321]) * right_dual.group9().xyz()),
            // e415, e425, e435, e321
            (Simd32x4::from(right_dual[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * right_dual.group6()),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group7()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e12345]) * self.group2().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group8()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group9(),
            // e1234
            right_dual[e1234] * self[e12345],
        );
    }
}
impl BulkContraction<Plane> for VersorEven {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[e12345]) * right_dual.group0());
    }
}
impl BulkContraction<RoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       19        0
    //    simd3        2        3        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       11       27        0
    //  no simd       24       48        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                right_dual[e4125] * self[e431] * -1.0,
                right_dual[e4235] * self[e412] * -1.0,
                right_dual[e4315] * self[e423] * -1.0,
                (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]) + (right_dual[e1234] * self[e5]),
            ]) + (Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], self[e2]]) * self.group1().xyz().with_w(right_dual[e4315]))
                + (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e1])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e3215] * self[e423]) + (right_dual[e1234] * self[e235]),
                (right_dual[e3215] * self[e431]) + (right_dual[e1234] * self[e315]),
                (right_dual[e3215] * self[e412]) + (right_dual[e1234] * self[e125]),
                -(right_dual[e4315] * self[e425]) - (right_dual[e4125] * self[e435]),
            ]) - (right_dual.group0().xyzx() * self.group1().wwwx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e3215]) * self.group1().xyz()) + (right_dual.group0().zxy() * self.group2().yzx()) - (right_dual.group0().yzx() * self.group2().zxy()))
                .with_w(right_dual[e1234] * self[e12345]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group0(),
        );
    }
}
impl BulkContraction<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[e12345]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<Sphere> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e12345]) * right_dual.group0(),
            // e5
            right_dual[e5] * self[e12345],
        );
    }
}
impl BulkContraction<VersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       77        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0().zxyw() * right_dual.group3().yzx().with_w(right_dual[scalar]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215])
                        - (self[e431] * right_dual[e25])
                        - (self[e412] * right_dual[e35])
                        - (self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12])
                        - (self[e321] * right_dual[e45])
                        - (self[e235] * right_dual[e41])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43]),
                )
                + (self.group0().www() * right_dual.group0().xyz()).with_w(self[e5] * right_dual[e1234])
                + (right_dual.group2().www() * self.group1().xyz()).with_w(self[e1] * right_dual[e4235])
                - (self.group0().yzxx() * right_dual.group3().zxy().with_w(right_dual[e15])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e12345] * right_dual[e23]) + (self[e235] * right_dual[e1234]),
                (self[e12345] * right_dual[e31]) + (self[e315] * right_dual[e1234]),
                (self[e12345] * right_dual[e12]) + (self[e125] * right_dual[e1234]),
                -(self[e425] * right_dual[e4315]) - (self[e435] * right_dual[e4125]),
            ]) + (self.group0() * right_dual.group3().www().with_w(right_dual[e45]))
                - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e12345]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e3215]) * self.group1().xyz())
                + (self.group2().yzx() * right_dual.group3().zxy())
                - (self.group2().zxy() * right_dual.group3().yzx()))
            .with_w(self[e12345] * right_dual[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e12345]) * right_dual.group3(),
        );
    }
}
impl BulkContraction<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd3        1        4        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       48       77        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(right_dual[e12345]) * self.group0().xyz()) + (Simd32x3::from(self[e12345]) * right_dual.group0().xyz())).with_w(right_dual[e12345] * self[e12345]),
            // e415, e425, e435, e321
            (Simd32x4::from(right_dual[e12345]) * self.group1()) + (Simd32x4::from(self[e12345]) * right_dual.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from(right_dual[e12345]) * self.group2())
                + (Simd32x4::from(self[e12345]) * right_dual.group2())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e415] * self[e235])
                        - (right_dual[e425] * self[e315])
                        - (right_dual[e435] * self[e125])
                        - (right_dual[e235] * self[e415])
                        - (right_dual[e315] * self[e425])
                        - (right_dual[e125] * self[e435]),
                ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e415] * self[e321]) + (right_dual[e321] * self[e415]) + (right_dual[e315] * self[e412]) + (right_dual[e1] * self[e12345]),
                (right_dual[e425] * self[e321]) + (right_dual[e321] * self[e425]) + (right_dual[e125] * self[e423]) + (right_dual[e2] * self[e12345]),
                (right_dual[e435] * self[e321]) + (right_dual[e321] * self[e435]) + (right_dual[e235] * self[e431]) + (right_dual[e3] * self[e12345]),
                -(right_dual[e412] * self[e435]) - (right_dual[e415] * self[e423]) - (right_dual[e425] * self[e431]) - (right_dual[e435] * self[e412]),
            ]) + (right_dual.group0().zxyw() * self.group2().yzx().with_w(self[e4]))
                + (right_dual.group0().www() * self.group3().xyz()).with_w(right_dual[e4] * self[e12345])
                - (right_dual.group0().yzxx() * self.group2().zxy().with_w(self[e415]))
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(right_dual[e431] * self[e425]),
        );
    }
}
impl std::ops::Div<bulk_contraction> for VersorOdd {
    type Output = bulk_contraction_partial<VersorOdd>;
    fn div(self, _rhs: bulk_contraction) -> Self::Output {
        bulk_contraction_partial(self)
    }
}
impl BulkContraction<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        8        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       40       67        0
    fn bulk_contraction(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (right_dual[e415] * self[e1234]) + (right_dual[e12345] * self[e41]),
                (right_dual[e425] * self[e1234]) + (right_dual[e12345] * self[e42]),
                (right_dual[e435] * self[e1234]) + (right_dual[e12345] * self[e43]),
                -(right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
            ]) + (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e12345] * self[scalar])
                - (right_dual.group0().yzx() * self.group3().zxy()).with_w(right_dual[e423] * self[e15]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e235] * self[e1234]) + (right_dual[e12345] * self[e23]),
                (right_dual[e315] * self[e1234]) + (right_dual[e12345] * self[e31]),
                (right_dual[e125] * self[e1234]) + (right_dual[e12345] * self[e12]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) + (right_dual.group0() * self.group3().www()).with_w(right_dual[e12345] * self[e45])
                - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * right_dual.group1().xyz())
                + (right_dual.group2().yzx() * self.group3().zxy())
                - (right_dual.group2().zxy() * self.group3().yzx()))
            .with_w(right_dual[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiDipoleInversion> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       48       72        0
    fn bulk_contraction(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * right_dual.group3().www().with_w(right_dual[e41]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234])
                        - (right_dual[e4235] * self[e41])
                        - (right_dual[e4315] * self[e42])
                        - (right_dual[e4125] * self[e43]),
                )
                - (self.group3().www() * right_dual.group3().xyz()).with_w(right_dual[e1234] * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (right_dual.group3().zxyz() * self.group1().yzx().with_w(self[e35]))
                + (self.group2().wwwy() * right_dual.group2().xyz().with_w(right_dual[e4315]))
                + (right_dual.group1().zxy() * self.group3().yzx()).with_w(right_dual[e4235] * self[e15])
                - (Simd32x4::from(self[e3215]) * right_dual.group0().with_w(right_dual[e45]))
                - (right_dual.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15]))
                - (right_dual.group3().yzx() * self.group1().zxy()).with_w(right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_contraction(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], 1.0])
                * right_dual
                    .group0()
                    .yy()
                    .with_zw(right_dual[e12345], (right_dual[e5] * self[e1234]) + (right_dual[e12345] * self[scalar])),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiFlatPoint> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_contraction(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e5
            -(right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]),
        );
    }
}
impl BulkContraction<AntiFlector> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       48        0
    fn bulk_contraction(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group1().xyz(),
            // e415, e425, e435, e321
            ((right_dual.group1().yzx() * self.group3().zxy()) - (right_dual.group1().zxy() * self.group3().yzx())).with_w(right_dual[e3215] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(-(right_dual[e4315] * self[e42]) - (right_dual[e4125] * self[e43]))
                + (right_dual.group1().www() * self.group3().xyz()).with_w(right_dual[e45] * self[e1234])
                - (right_dual.group1().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            (right_dual.group1().zxyy() * self.group1().yzx().with_w(self[e25]))
                + (right_dual.group1().wwwz() * self.group0().xyz().with_w(self[e35]))
                + (self.group2().wwwx() * right_dual.group0().xyz().with_w(right_dual[e4235]))
                + Simd32x3::from(0.0).with_w((right_dual[e3215] * self[e45]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]) - (right_dual[e45] * self[e3215]))
                - (right_dual.group1().yzx() * self.group1().zxy()).with_w(right_dual[e15] * self[e4235]),
        );
    }
}
impl BulkContraction<AntiLine> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       31        0
    fn bulk_contraction(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual
                    .group1()
                    .with_w(-(right_dual[e415] * self[e4235]) - (right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125])),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e31]) - (right_dual[e435] * self[e12]) - (right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]),
            ]) - (right_dual.group1().zxy() * self.group3().yzx()).with_w(right_dual[e415] * self[e23]),
        );
    }
}
impl BulkContraction<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       28       49        0
    fn bulk_contraction(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual.group0() * self.group2().www().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12])
                        - (right_dual[e235] * self[e41])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43]),
                )
                + (right_dual.group0().www() * self.group0().xyz()).with_w(right_dual[e5] * self[e1234]),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e235] * self[e1234],
                right_dual[e315] * self[e1234],
                right_dual[e125] * self[e1234],
                -(right_dual[e415] * self[e4235]) - (right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group1()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * right_dual.group0().xyz())
                + (right_dual.group1().yzx() * self.group3().zxy())
                - (right_dual.group1().zxy() * self.group3().yzx()))
            .with_w(right_dual[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<AntiPlane> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       21        0
    //  no simd       17       39        0
    fn bulk_contraction(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e1234]) * right_dual.group0().xyz(),
            // e415, e425, e435, e321
            ((right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx())).with_w(right_dual[e3215] * self[e1234]),
            // e235, e315, e125, e4
            Simd32x4::from([
                right_dual[e3215] * self[e4235],
                right_dual[e3215] * self[e4315],
                right_dual[e3215] * self[e4125],
                -(right_dual[e4315] * self[e42]) - (right_dual[e4125] * self[e43]),
            ]) - (right_dual.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e4315] * self[e12] * -1.0,
                right_dual[e4125] * self[e23] * -1.0,
                right_dual[e4235] * self[e31] * -1.0,
                (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45]),
            ]) + (right_dual.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + (right_dual.group0().wwwy() * self.group0().xyz().with_w(self[e25])),
        );
    }
}
impl BulkContraction<Circle> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       15       24        0
    fn bulk_contraction(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (self.group3().yzxy() * right_dual.group1().zxy().with_w(right_dual[e42]))
                + (right_dual.group2() * self.group2().www()).with_w(right_dual[e41] * self[e4235]),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<CircleRotor> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       28        0
    fn bulk_contraction(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * self[e3215]) - (right_dual[e31] * self[e4125]),
                -(right_dual[e42] * self[e3215]) - (right_dual[e12] * self[e4235]),
                -(right_dual[e43] * self[e3215]) - (right_dual[e23] * self[e4315]),
                (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234]),
            ]) + (self.group3().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41]))
                + (self.group2().www() * right_dual.group2().xyz()).with_w(right_dual[e42] * self[e4315]),
            // e5
            -(right_dual[e45] * self[e3215]) - (right_dual[e15] * self[e4235]) - (right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<Dipole> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       29       50        0
    fn bulk_contraction(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group3().yzx()) - (right_dual.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e423] * self[e3215]) + (right_dual[e235] * self[e1234]),
                (right_dual[e431] * self[e3215]) + (right_dual[e315] * self[e1234]),
                (right_dual[e412] * self[e3215]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
            ]) - (right_dual.group2().zxy() * self.group3().yzx()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkContraction<DipoleInversion> for VersorOdd {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        5        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       60        0
    fn bulk_contraction(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()) + (right_dual.group0().zxy() * self.group3().yzx()) - (right_dual.group0().yzx() * self.group3().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e423] * self[e3215]) + (right_dual[e235] * self[e1234]),
                (right_dual[e431] * self[e3215]) + (right_dual[e315] * self[e1234]),
                (right_dual[e412] * self[e3215]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, scalar
            (Simd32x4::from(self[e3215]) * right_dual.group1().xyz().with_w(right_dual[e4]))
                + (self.group3().zxyx() * right_dual.group2().yzx().with_w(right_dual[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                        - (right_dual[e431] * self[e25])
                        - (right_dual[e412] * self[e35])
                        - (right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12])
                        - (right_dual[e321] * self[e45])
                        - (right_dual[e235] * self[e41])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43]),
                )
                - (right_dual.group2().zxy() * self.group3().yzx()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkContraction<DualNum> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_contraction(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[e3215]) * self.group0().xyz().with_w(self[e45]),
        );
    }
}
impl BulkContraction<FlatPoint> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       20        0
    fn bulk_contraction(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                right_dual[e235] * self[e1234],
                right_dual[e315] * self[e1234],
                right_dual[e125] * self[e1234],
                -(right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
            ]) - (right_dual.group0().wwwx() * self.group3().xyz().with_w(self[e41])),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Flector> for VersorOdd {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       28        0
    fn bulk_contraction(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43])
                    - (right_dual[e321] * self[e45]),
            ) + (self.group2().www() * right_dual.group0().xyz()).with_w(right_dual[e1] * self[e4235])
                - (right_dual.group0().wwwx() * self.group3().xyz().with_w(self[e41])),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx())).with_w(0.0),
        );
    }
}
impl BulkContraction<Line> for VersorOdd {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_contraction(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e12] * self[e4315]) + (right_dual[e15] * self[e1234]),
                (right_dual[e23] * self[e4125]) + (right_dual[e25] * self[e1234]),
                (right_dual[e31] * self[e4235]) + (right_dual[e35] * self[e1234]),
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) - (self.group3().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<Motor> for VersorOdd {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       28        0
    fn bulk_contraction(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[e3215]) * self.group3().xyz().with_w(self[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (right_dual[e15] * self[e1234]) + (right_dual[e3215] * self[e41]),
                (right_dual[e25] * self[e1234]) + (right_dual[e3215] * self[e42]),
                (right_dual[e35] * self[e1234]) + (right_dual[e3215] * self[e43]),
                -(right_dual[e25] * self[e4315]) - (right_dual[e35] * self[e4125]),
            ]) + (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e3215] * self[e45])
                - (self.group3().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl BulkContraction<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        0
    //    simd2        0        1        0
    //    simd3        8       20        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       44       72        0
    //  no simd       90      143        0
    fn bulk_contraction(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
            // e15, e25, e35, e45
            other.group8().with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group7(),
            // e23, e31, e12
            other.group6().xyz(),
            // e415, e425, e435, e321
            other.group5().with_w(other[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            other.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group1().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e12345] * self[scalar])
                    + (right_dual[e1] * self[e4235])
                    + (right_dual[e2] * self[e4315])
                    + (right_dual[e3] * self[e4125])
                    + (right_dual[e4] * self[e3215])
                    + (right_dual[e5] * self[e1234])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e423] * self[e15])
                    - (right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group3().yzxx() * right_dual.group5().zxy().with_w(right_dual[e41]))
                + (right_dual.group9().zxy() * self.group1().yzx()).with_w(right_dual[e43] * self[e4125])
                + (right_dual.group9().www() * self.group0().xyz()).with_w(right_dual[e45] * self[e1234])
                + (self.group2().www() * right_dual.group3().xyz()).with_w(right_dual[e42] * self[e4315])
                - (Simd32x4::from(right_dual[e1234]) * self.group2().xyz().with_w(self[e45]))
                - (right_dual.group9().yzxz() * self.group1().zxy().with_w(self[e43]))
                - (right_dual.group4() * self.group3().www()).with_w(right_dual[e4235] * self[e41])
                - (right_dual.group5().yzx() * self.group3().zxy()).with_w(right_dual[e4315] * self[e42]),
            // e5
            (right_dual[e4235] * self[e15]) + (right_dual[e4315] * self[e25]) + (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45])
                - (right_dual[e15] * self[e4235])
                - (right_dual[e25] * self[e4315])
                - (right_dual[e35] * self[e4125])
                - (right_dual[e45] * self[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * self[e3215]) + (right_dual[e315] * self[e4125]),
                (right_dual[e425] * self[e3215]) + (right_dual[e125] * self[e4235]),
                (right_dual[e435] * self[e3215]) + (right_dual[e235] * self[e4315]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) + (Simd32x4::from(right_dual[e12345]) * self.group2().xyz().with_w(self[e45]))
                - (self.group3().yzxx() * right_dual.group8().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e12345]) * self.group0().xyz())
                + (Simd32x3::from(self[e1234]) * right_dual.group6().xyz())
                + (right_dual.group7().zxy() * self.group3().yzx())
                - (right_dual.group7().yzx() * self.group3().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e12345]) * self.group1().xyz()) + (Simd32x3::from(self[e1234]) * right_dual.group8()) + (Simd32x3::from(self[e3215]) * right_dual.group7())
                - (Simd32x3::from(right_dual[e321]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group9().yzxw() * self.group3().zxy().with_w(self[e1234])) - (right_dual.group9().zxy() * self.group3().yzx()).with_w(right_dual[e1234] * self[e3215]),
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group9().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * self.group3().xyz()) - (Simd32x3::from(self[e3215]) * right_dual.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
            // e1234
            right_dual[e12345] * self[e1234],
        );
    }
}
impl BulkContraction<Plane> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_contraction(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(
            // scalar
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkContraction<RoundPoint> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd3        1        3        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       25       45        0
    fn bulk_contraction(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group0().yzxw() * self.group3().zxy().with_w(self[e1234])) - (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e1234] * self[e3215]),
            // e235, e315, e125, e4
            Simd32x4::from([
                right_dual[e3215] * self[e4235],
                right_dual[e3215] * self[e4315],
                right_dual[e3215] * self[e4125],
                -(right_dual[e4315] * self[e42]) - (right_dual[e4125] * self[e43]) - (right_dual[e1234] * self[e45]),
            ]) - (right_dual.group0().xyzx() * self.group3().www().with_w(self[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                -(right_dual[e4315] * self[e12]) - (right_dual[e1234] * self[e15]),
                -(right_dual[e4125] * self[e23]) - (right_dual[e1234] * self[e25]),
                -(right_dual[e4235] * self[e31]) - (right_dual[e1234] * self[e35]),
                (right_dual[e4125] * self[e35]) + (right_dual[e3215] * self[e45]),
            ]) + (right_dual.group0().zxyx() * self.group1().yzx().with_w(self[e15]))
                + (right_dual.group0().wwwy() * self.group0().xyz().with_w(self[e25])),
        );
    }
}
impl BulkContraction<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_contraction(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual[e12345]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[e12345]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[e12345]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
impl BulkContraction<Sphere> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_contraction(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Scalar::from_groups(
            // scalar
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkContraction<VersorEven> for VersorOdd {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        5        0
    //    simd4       10       13        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       48       76        0
    fn bulk_contraction(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e1234]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e1234]) * self.group3().xyz()),
            // e415, e425, e435, e321
            (right_dual.group3().yzxw() * self.group3().zxy().with_w(self[e1234])) - (self.group3().yzxw() * right_dual.group3().zxy().with_w(right_dual[e1234])),
            // e235, e315, e125, e4
            (self.group3().xyzx() * right_dual.group3().www().with_w(right_dual[e41]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e42] * self[e4315]) + (right_dual[e43] * self[e4125]) + (right_dual[e45] * self[e1234])
                        - (right_dual[e4235] * self[e41])
                        - (right_dual[e4315] * self[e42])
                        - (right_dual[e4125] * self[e43]),
                )
                - (self.group3().www() * right_dual.group3().xyz()).with_w(right_dual[e1234] * self[e45]),
            // e1, e2, e3, e5
            (Simd32x4::from(right_dual[e3215]) * self.group0().xyz().with_w(self[e45]))
                + (right_dual.group3().zxyz() * self.group1().yzx().with_w(self[e35]))
                + (self.group2().wwwy() * right_dual.group2().xyz().with_w(right_dual[e4315]))
                + (right_dual.group1().zxy() * self.group3().yzx()).with_w(right_dual[e4235] * self[e15])
                - (Simd32x4::from(self[e3215]) * right_dual.group0().xyz().with_w(right_dual[e45]))
                - (right_dual.group2().wwwy() * self.group2().xyz().with_w(self[e4315]))
                - (self.group3().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15]))
                - (right_dual.group3().yzx() * self.group1().zxy()).with_w(right_dual[e35] * self[e4125]),
        );
    }
}
impl BulkContraction<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       77        0
    fn bulk_contraction(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group2().xyz().with_w(other[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (right_dual.group0().zxyw() * self.group3().yzx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215])
                        - (right_dual[e431] * self[e25])
                        - (right_dual[e412] * self[e35])
                        - (right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12])
                        - (right_dual[e321] * self[e45])
                        - (right_dual[e235] * self[e41])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43]),
                )
                + (right_dual.group0().www() * self.group0().xyz()).with_w(right_dual[e5] * self[e1234])
                + (self.group2().www() * right_dual.group1().xyz()).with_w(right_dual[e1] * self[e4235])
                - (right_dual.group0().yzxx() * self.group3().zxy().with_w(self[e15])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (right_dual[e12345] * self[e23]) + (right_dual[e235] * self[e1234]),
                (right_dual[e12345] * self[e31]) + (right_dual[e315] * self[e1234]),
                (right_dual[e12345] * self[e12]) + (right_dual[e125] * self[e1234]),
                -(right_dual[e425] * self[e4315]) - (right_dual[e435] * self[e4125]),
            ]) + (right_dual.group0() * self.group3().www().with_w(self[e45]))
                - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e12345]) * self.group2().xyz())
                + (Simd32x3::from(self[e3215]) * right_dual.group1().xyz())
                + (right_dual.group2().yzx() * self.group3().zxy())
                - (right_dual.group2().zxy() * self.group3().yzx()))
            .with_w(right_dual[e12345] * self[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * self.group3(),
        );
    }
}
