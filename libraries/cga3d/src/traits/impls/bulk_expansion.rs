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
//  Maximum:       111     145       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      22       0
//  Average:        11      29       0
//  Maximum:       211     265       0
impl std::ops::Div<bulk_expansion> for AntiCircleRotor {
    type Output = bulk_expansion_partial<AntiCircleRotor>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       10       33        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[e235], right_dual[e315], right_dual[e125], 1.0])
                * self.group2().www().with_w(
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
impl BulkExpansion<AntiDipoleInversion> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        3        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       30       58        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[e15], right_dual[e25], right_dual[e35], 1.0])
                * self.group2().www().with_w(
                    (self[scalar] * right_dual[e1234])
                        - (self[e41] * right_dual[e23])
                        - (self[e42] * right_dual[e31])
                        - (self[e43] * right_dual[e12])
                        - (self[e23] * right_dual[e41])
                        - (self[e31] * right_dual[e42])
                        - (self[e12] * right_dual[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]) + (self[scalar] * right_dual[e4235]),
                (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]) + (self[scalar] * right_dual[e4315]),
                (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]) + (self[scalar] * right_dual[e4125]),
                -(self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (self.group0().yzx() * right_dual.group2().zxy()).with_w(self[scalar] * right_dual[e3215])
                - (right_dual.group2().yzxx() * self.group0().zxy().with_w(self[e23]))
                - (right_dual.group0().zxy() * self.group2().yzx()).with_w(self[e31] * right_dual[e25]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Motor::from_groups(
            // e415, e425, e435, e12345
            right_dual.group0().xx().with_zw(right_dual[e5], right_dual[e12345]) * self.group0().with_w(self[scalar]),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[e5]) * self.group1().xyz().with_w(self[scalar]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]),
            ]) - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiFlector> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[scalar] * right_dual[e4235]),
                (self[e31] * right_dual[e45]) + (self[scalar] * right_dual[e4315]),
                (self[e12] * right_dual[e45]) + (self[scalar] * right_dual[e4125]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]),
            ]) + (self.group0().yzx() * right_dual.group0().zxy()).with_w(self[scalar] * right_dual[e3215])
                - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiLine> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       28        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group0().with_w(
                    -(self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * self.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiMotor> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[scalar] * right_dual[e415],
                self[scalar] * right_dual[e425],
                self[scalar] * right_dual[e435],
                -(self[e41] * right_dual[e235])
                    - (self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435]),
            ]) + (self.group0() * right_dual.group1().www()).with_w(self[scalar] * right_dual[e12345]),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[e5]) * self.group1().xyz())).with_w(self[scalar] * right_dual[e5]),
        );
    }
}
impl BulkExpansion<AntiPlane> for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiScalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(right_dual[scalar]) * self.group2(),
        );
    }
}
impl BulkExpansion<Circle> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       25       45        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group2().with_w(
                    -(self[e41] * right_dual[e23])
                        - (self[e42] * right_dual[e31])
                        - (self[e43] * right_dual[e12])
                        - (self[e23] * right_dual[e41])
                        - (self[e31] * right_dual[e42])
                        - (self[e12] * right_dual[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]),
                -(self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group0().zxy() * right_dual.group2().yzx()).with_w(self[e23] * right_dual[e15])
                - (right_dual.group0().zxy() * self.group2().yzx()).with_w(self[e31] * right_dual[e25]),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       30        0
    //    simd3        1        3        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       35       59        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(right_dual[scalar]) * self.group0()) + (Simd32x3::from(self[scalar]) * right_dual.group0())).with_w(right_dual[scalar] * self[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(right_dual[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * right_dual.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (right_dual[e15] * self[scalar]) + (right_dual[scalar] * self[e15]),
                (right_dual[e25] * self[scalar]) + (right_dual[scalar] * self[e25]),
                (right_dual[e35] * self[scalar]) + (right_dual[scalar] * self[e35]),
                -(right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (self.group2().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23]))
                - (self.group0().zxy() * right_dual.group2().yzx()).with_w(right_dual[e31] * self[e25]),
        );
    }
}
impl BulkExpansion<Dipole> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd        9       31        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group2().with_w(
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
impl BulkExpansion<DipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        7        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       24       41        0
    //  no simd       39       70        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self[e42] * right_dual[e3]) + (self[e23] * right_dual[e4]) + (self[scalar] * right_dual[e423]),
                (self[e43] * right_dual[e1]) + (self[e31] * right_dual[e4]) + (self[scalar] * right_dual[e431]),
                (self[e41] * right_dual[e2]) + (self[e12] * right_dual[e4]) + (self[scalar] * right_dual[e412]),
                -(self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e45] * right_dual[e321])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
            ]) - (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e41] * right_dual[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * right_dual[e4]) + (self[scalar] * right_dual[e415]),
                (self[e25] * right_dual[e4]) + (self[scalar] * right_dual[e425]),
                (self[e35] * right_dual[e4]) + (self[scalar] * right_dual[e435]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) + (self.group0() * right_dual.group3().www()).with_w(self[scalar] * right_dual[e321])
                - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e5]) * self.group1().xyz())
                + (self.group2().zxy() * right_dual.group3().yzx())
                - (self.group2().yzx() * right_dual.group3().zxy()))
            .with_w(self[scalar] * right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3().xyz().with_w(right_dual[e4]),
        );
    }
}
impl BulkExpansion<DualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       23        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual[scalar]) * self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            right_dual.group0().yy().with_zw(right_dual[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group2().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e3215]),
        );
    }
}
impl BulkExpansion<FlatPoint> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[e235], right_dual[e315], right_dual[e125], 1.0])
                * self
                    .group2()
                    .www()
                    .with_w(-(self[e41] * right_dual[e235]) - (self[e42] * right_dual[e315]) - (self[e43] * right_dual[e125]) - (self[e45] * right_dual[e321])),
        );
    }
}
impl BulkExpansion<Flector> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        3        6        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        9       23        0
    //  no simd       24       53        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e42] * right_dual[e3],
                self[e43] * right_dual[e1],
                self[e41] * right_dual[e2],
                -(self[e42] * right_dual[e315]) - (self[e43] * right_dual[e125]) - (self[e45] * right_dual[e321]),
            ]) - (self.group0().zxy() * right_dual.group1().yzx()).with_w(self[e41] * right_dual[e235]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]))
                + (self.group0() * right_dual.group1().www()).with_w(self[scalar] * right_dual[e321])
                - (self.group1().wwwx() * right_dual.group1().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * right_dual.group0().xyz())
                + (Simd32x3::from(right_dual[e5]) * self.group1().xyz())
                + (self.group2().zxy() * right_dual.group1().yzx())
                - (self.group2().yzx() * right_dual.group1().zxy()))
            .with_w(self[scalar] * right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(1.0).with_w(0.0) * self.group2().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<Line> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       34        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual
                    .group1()
                    .with_w(-(self[e41] * right_dual[e23]) - (self[e42] * right_dual[e31]) - (self[e43] * right_dual[e12])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e45] * right_dual[e23]),
                (self[e43] * right_dual[e15]) + (self[e45] * right_dual[e31]),
                (self[e41] * right_dual[e25]) + (self[e45] * right_dual[e12]),
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group0().zxy() * right_dual.group1().yzx()).with_w(self[e15] * right_dual[e23]),
        );
    }
}
impl BulkExpansion<Motor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        1        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       20       44        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual[scalar]) * self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            ((Simd32x3::from(self[scalar]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group1().xyz())).with_w(self[e45] * right_dual[scalar]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self[e15] * right_dual[scalar]) + (self[scalar] * right_dual[e15]),
                (self[e25] * right_dual[scalar]) + (self[scalar] * right_dual[e25]),
                (self[e35] * right_dual[scalar]) + (self[scalar] * right_dual[e35]),
                -(self[e41] * right_dual[e23]) - (self[e42] * right_dual[e31]) - (self[e43] * right_dual[e12]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e45] * right_dual[e23],
                self[e45] * right_dual[e31],
                self[e45] * right_dual[e12],
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (right_dual.group1().zxyw() * self.group0().yzx().with_w(self[scalar]))
                - (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd2        0        1        0
    //    simd3        8       16        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       46       74        0
    //  no simd       80      134        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                self[scalar] * right_dual[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e5
            self[scalar] * right_dual[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(self[scalar]) * right_dual.group3()) + (Simd32x4::from(right_dual[scalar]) * self.group2().xyz().with_w(self[e45])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * right_dual.group4()) + (Simd32x3::from(right_dual[scalar]) * self.group0()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * right_dual.group5()) + (Simd32x3::from(right_dual[scalar]) * self.group1().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * right_dual[e4]) + (self[scalar] * right_dual[e415]),
                (self[e25] * right_dual[e4]) + (self[scalar] * right_dual[e425]),
                (self[e35] * right_dual[e4]) + (self[scalar] * right_dual[e435]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) + (Simd32x4::from([right_dual[e5], right_dual[e5], right_dual[e5], right_dual[e321]]) * self.group0().with_w(self[scalar]))
                - (self.group1().wwwx() * right_dual.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * right_dual.group7()) + (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group1().zxy())
                - (self.group0().zxy() * right_dual.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * right_dual.group8()) + (Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (self.group2().zxy() * right_dual.group1().yzx())
                - (self.group2().yzx() * right_dual.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]) + (self[scalar] * right_dual[e4235]),
                (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]) + (self[scalar] * right_dual[e4315]),
                (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]) + (self[scalar] * right_dual[e4125]),
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e35] * right_dual[e12]),
            ]) + (self.group0().yzx() * right_dual.group3().zxy()).with_w(self[scalar] * right_dual[e3215])
                - (self.group2().yzxy() * right_dual.group4().zxy().with_w(right_dual[e31]))
                - (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e15] * right_dual[e23]),
            // e1234
            (self[scalar] * right_dual[e1234])
                - (self[e41] * right_dual[e23])
                - (self[e42] * right_dual[e31])
                - (self[e43] * right_dual[e12])
                - (self[e23] * right_dual[e41])
                - (self[e31] * right_dual[e42])
                - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<Plane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       16       32        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e41] * right_dual[e5],
                self[e42] * right_dual[e5],
                self[e43] * right_dual[e5],
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e235, e315, e125, e4
            (Simd32x3::from(right_dual[e5]) * self.group1().xyz()).with_w(0.0) + (self.group2().zxy() * right_dual.group0().yzx()).with_w(0.0)
                - (self.group2().yzx() * right_dual.group0().zxy()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_dual.group0(),
        );
    }
}
impl BulkExpansion<RoundPoint> for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1234
            self[scalar] * right_dual[e1234],
        );
    }
}
impl BulkExpansion<Scalar> for AntiCircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * right_dual[e12345]);
    }
}
impl BulkExpansion<Sphere> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       20       39        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * right_dual[e5]) + (self[e15] * right_dual[e4]),
                (self[e42] * right_dual[e5]) + (self[e25] * right_dual[e4]),
                (self[e43] * right_dual[e5]) + (self[e35] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e235, e315, e125, e4
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (self.group2().zxy() * right_dual.group0().yzx()) - (self.group2().yzx() * right_dual.group0().zxy()))
                .with_w(self[scalar] * right_dual[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_dual.group0().xyz().with_w(right_dual[e5]),
        );
    }
}
impl BulkExpansion<VersorEven> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd3        1        4        0
    //    simd4        5        8        0
    // Totals...
    // yes simd       23       40        0
    //  no simd       40       72        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
            ((Simd32x3::from(self[scalar]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group0())).with_w(self[scalar] * right_dual[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(self[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[scalar] * right_dual[e15],
                self[scalar] * right_dual[e25],
                self[scalar] * right_dual[e35],
                -(self[e41] * right_dual[e23])
                    - (self[e42] * right_dual[e31])
                    - (self[e43] * right_dual[e12])
                    - (self[e23] * right_dual[e41])
                    - (self[e31] * right_dual[e42])
                    - (self[e12] * right_dual[e43]),
            ]) + (self.group2() * right_dual.group0().www().with_w(right_dual[e1234])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]) + (self[scalar] * right_dual[e4235]),
                (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]) + (self[scalar] * right_dual[e4315]),
                (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]) + (self[scalar] * right_dual[e4125]),
                -(self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (self.group0().yzx() * right_dual.group2().zxy()).with_w(self[scalar] * right_dual[e3215])
                - (right_dual.group2().yzxx() * self.group0().zxy().with_w(self[e23]))
                - (self.group2().yzx() * right_dual.group0().zxy()).with_w(self[e31] * right_dual[e25]),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       22       40        0
    //  no simd       40       72        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
            Simd32x4::from([
                (self[e23] * right_dual[e4]) + (self[scalar] * right_dual[e423]),
                (self[e31] * right_dual[e4]) + (self[scalar] * right_dual[e431]),
                (self[e12] * right_dual[e4]) + (self[scalar] * right_dual[e412]),
                -(self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e45] * right_dual[e321])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
            ]) + (self.group0().yzx() * right_dual.group3().zxy()).with_w(self[scalar] * right_dual[e12345])
                - (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e41] * right_dual[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * right_dual[e4]) + (self[scalar] * right_dual[e415]),
                (self[e25] * right_dual[e4]) + (self[scalar] * right_dual[e425]),
                (self[e35] * right_dual[e4]) + (self[scalar] * right_dual[e435]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) + (self.group0() * right_dual.group2().www()).with_w(self[scalar] * right_dual[e321])
                - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e5]) * self.group1().xyz())
                + (self.group2().zxy() * right_dual.group3().yzx())
                - (self.group2().yzx() * right_dual.group3().zxy()))
            .with_w(self[scalar] * right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiDipoleInversion {
    type Output = bulk_expansion_partial<AntiDipoleInversion>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       31        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * right_dual[e235]) + (self[e3] * right_dual[e425]),
                (self[e4] * right_dual[e315]) + (self[e1] * right_dual[e435]),
                (self[e4] * right_dual[e125]) + (self[e2] * right_dual[e415]),
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group3().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315]))
                - (right_dual.group0() * self.group3().www()).with_w(self[e1] * right_dual[e235]),
            // e1234
            (self[e4] * right_dual[e321]) + (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       37       57        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * right_dual[e15]) + (self[e5] * right_dual[e41]),
                (self[e4] * right_dual[e25]) + (self[e5] * right_dual[e42]),
                (self[e4] * right_dual[e35]) + (self[e5] * right_dual[e43]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group3().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e12345
            (self.group3().wwwx() * right_dual.group1().xyz().with_w(right_dual[e4235]))
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
                + (self.group3().yzx() * right_dual.group2().zxy()).with_w(self[e4] * right_dual[e3215])
                - (right_dual.group2().yzxx() * self.group3().zxy().with_w(self[e423])),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e5]) * self.group3().xyz().with_w(self[e4]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       20        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[e4] * right_dual[e15],
                self[e4] * right_dual[e25],
                self[e4] * right_dual[e35],
                -(self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
            ]) - (right_dual.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((self.group3().yzx() * right_dual.group0().zxy()) - (self.group3().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiFlector> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       16       28        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[e4]) * right_dual.group0().xyz().with_w(right_dual[e3215]))
                + Simd32x3::from(0.0).with_w(
                    (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125])
                        - (self[e431] * right_dual[e25])
                        - (self[e412] * right_dual[e35])
                        - (self[e321] * right_dual[e45]),
                )
                - (right_dual.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((self.group3().yzx() * right_dual.group0().zxy()) - (self.group3().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiLine> for AntiDipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * right_dual[e235]) + (self[e3] * right_dual[e425]),
                (self[e4] * right_dual[e315]) + (self[e1] * right_dual[e435]),
                (self[e4] * right_dual[e125]) + (self[e2] * right_dual[e415]),
                -(self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]),
            ]) - (self.group3().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiMotor> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e5]) * self.group3().xyz().with_w(self[e4]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * right_dual[e235]) + (self[e3] * right_dual[e425]),
                (self[e4] * right_dual[e315]) + (self[e1] * right_dual[e435]),
                (self[e4] * right_dual[e125]) + (self[e2] * right_dual[e415]),
                -(self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]),
            ]) + (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                - (self.group3().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiPlane> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(
            // e12345
            (self[e4] * right_dual[e3215]) + (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]),
        );
    }
}
impl BulkExpansion<AntiScalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(right_dual[scalar]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<Circle> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       29       44        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * right_dual[e15]) + (self[e5] * right_dual[e41]),
                (self[e4] * right_dual[e25]) + (self[e5] * right_dual[e42]),
                (self[e4] * right_dual[e35]) + (self[e5] * right_dual[e43]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group3().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e2] * right_dual[e35]) + (self[e5] * right_dual[e23]),
                (self[e3] * right_dual[e15]) + (self[e5] * right_dual[e31]),
                (self[e1] * right_dual[e25]) + (self[e5] * right_dual[e12]),
                -(self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e321] * right_dual[e45])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
            ]) - (right_dual.group2().yzx() * self.group3().zxy()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        3        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       24       39        0
    //  no simd       39       63        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (right_dual[e42] * self[e3]) + (right_dual[e23] * self[e4]) + (right_dual[scalar] * self[e423]),
                (right_dual[e43] * self[e1]) + (right_dual[e31] * self[e4]) + (right_dual[scalar] * self[e431]),
                (right_dual[e41] * self[e2]) + (right_dual[e12] * self[e4]) + (right_dual[scalar] * self[e412]),
                -(right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
            ]) - (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e41] * self[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e15] * self[e4]) + (right_dual[scalar] * self[e415]),
                (right_dual[e25] * self[e4]) + (right_dual[scalar] * self[e425]),
                (right_dual[e35] * self[e4]) + (right_dual[scalar] * self[e435]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) + (right_dual.group0() * self.group3().www()).with_w(right_dual[scalar] * self[e321])
                - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[scalar]) * self.group2().xyz())
                + (Simd32x3::from(self[e5]) * right_dual.group1().xyz())
                + (right_dual.group2().zxy() * self.group3().yzx())
                - (right_dual.group2().yzx() * self.group3().zxy()))
            .with_w(right_dual[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3().xyz().with_w(self[e4]),
        );
    }
}
impl BulkExpansion<Dipole> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       30        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e4] * right_dual[e235]) + (self[e3] * right_dual[e425]),
                (self[e4] * right_dual[e315]) + (self[e1] * right_dual[e435]),
                (self[e4] * right_dual[e125]) + (self[e2] * right_dual[e415]),
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group3().wwwx() * right_dual.group0().with_w(right_dual[e235]))
                - (self.group3().yzx() * right_dual.group1().zxy()).with_w(self[e2] * right_dual[e315]),
            // e1234
            (self[e4] * right_dual[e321]) + (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        7        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       48       75        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (right_dual.group3().zxyw() * self.group3().yzx().with_w(self[e4])) - (self.group3().zxyw() * right_dual.group3().yzx().with_w(right_dual[e4])),
            // e15, e25, e35, e1234
            (self.group3().xyzx() * right_dual.group3().www().with_w(right_dual[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                        - (right_dual[e4] * self[e321])
                        - (right_dual[e2] * self[e431])
                        - (right_dual[e3] * self[e412]),
                )
                - (self.group3().www() * right_dual.group3().xyz()).with_w(right_dual[e1] * self[e423]),
            // e4235, e4315, e4125, e3215
            (right_dual.group3().yzxw() * self.group1().zxyw())
                + (self.group2().wwwz() * right_dual.group2().xyz().with_w(right_dual[e3]))
                + (self.group0() * right_dual.group3().www()).with_w(right_dual[e1] * self[e235])
                + (right_dual.group1().yzx() * self.group3().zxy()).with_w(right_dual[e2] * self[e315])
                - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (right_dual.group2().wwwy() * self.group2().xyz().with_w(self[e2]))
                - (self.group3().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235]))
                - (right_dual.group3().zxy() * self.group1().yzx()).with_w(right_dual[e125] * self[e3]),
        );
    }
}
impl BulkExpansion<DualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            right_dual.group0().yy().with_zw(right_dual[scalar], right_dual[e3215]) * self.group0().with_w(self[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[scalar]) * self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3().xyz().with_w(self[e4]),
        );
    }
}
impl BulkExpansion<FlatPoint> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e235], right_dual[e315], right_dual[e125], 1.0])
                * self
                    .group2()
                    .www()
                    .with_w(-(self[e1] * right_dual[e235]) - (self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321])),
            // e1234
            self[e4] * right_dual[e321],
        );
    }
}
impl BulkExpansion<Flector> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       48        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group1().xyz(),
            // e23, e31, e12, e45
            ((self.group3().yzx() * right_dual.group1().zxy()) - (self.group3().zxy() * right_dual.group1().yzx())).with_w(self[e4] * right_dual[e5]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(-(self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]))
                + (right_dual.group1().www() * self.group3().xyz()).with_w(self[e4] * right_dual[e321])
                - (right_dual.group1().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (self.group2().wwwy() * right_dual.group0().xyz().with_w(right_dual[e2]))
                + (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w((self[e125] * right_dual[e3]) - (self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]))
                - (self.group1().yzx() * right_dual.group1().zxy()).with_w(self[e1] * right_dual[e235]),
        );
    }
}
impl BulkExpansion<Line> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       25        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual.group1().with_w(-(self[e1] * right_dual[e23]) - (self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e2] * right_dual[e35]) + (self[e5] * right_dual[e23]),
                (self[e3] * right_dual[e15]) + (self[e5] * right_dual[e31]),
                (self[e1] * right_dual[e25]) + (self[e5] * right_dual[e12]),
                -(self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e415] * right_dual[e23]) - (self[e425] * right_dual[e31]) - (self[e435] * right_dual[e12]),
            ]) - (right_dual.group1().yzx() * self.group3().zxy()).with_w(self[e423] * right_dual[e15]),
        );
    }
}
impl BulkExpansion<Motor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       12       26        0
    //  no simd       24       48        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e4] * right_dual[e23],
                self[e4] * right_dual[e31],
                self[e4] * right_dual[e12],
                -(self[e423] * right_dual[e15])
                    - (self[e431] * right_dual[e25])
                    - (self[e412] * right_dual[e35])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12]),
            ]) + (self.group0() * right_dual.group0().www()).with_w(self[e4] * right_dual[e3215]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e4] * right_dual[e15],
                self[e4] * right_dual[e25],
                self[e4] * right_dual[e35],
                -(self[e1] * right_dual[e23]) - (self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group0().xyz())
                + (Simd32x3::from(right_dual[scalar]) * self.group2().xyz())
                + (self.group3().yzx() * right_dual.group1().zxy())
                - (self.group3().zxy() * right_dual.group1().yzx()))
            .with_w(self[e5] * right_dual[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3().xyz().with_w(self[e4]),
        );
    }
}
impl BulkExpansion<MultiVector> for AntiDipoleInversion {
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
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3().xyz().with_w(self[e4]),
            // e5
            self[e5] * right_dual[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[e5]) * self.group3().xyz().with_w(self[e4])) - (Simd32x4::from(self[e5]) * right_dual.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12
            (self.group3().yzx() * right_dual.group1().zxy()) - (self.group3().zxy() * right_dual.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * right_dual[e15]) + (self[e5] * right_dual[e41]),
                (self[e4] * right_dual[e25]) + (self[e5] * right_dual[e42]),
                (self[e4] * right_dual[e35]) + (self[e5] * right_dual[e43]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group1())
                - (right_dual.group3().www() * self.group3().xyz()).with_w(self[e1] * right_dual[e23]),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group5()) + (Simd32x3::from(right_dual[scalar]) * self.group0()) + (right_dual.group4().yzx() * self.group3().zxy())
                - (right_dual.group4().zxy() * self.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group5()) + (Simd32x3::from(right_dual[scalar]) * self.group2().xyz()) + (self.group3().yzx() * right_dual.group3().zxy())
                - (self.group3().zxy() * right_dual.group3().yzx()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (right_dual.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (right_dual.group8() * self.group2().www()).with_w(self[e235] * right_dual[e1])
                + (self.group3().zxy() * right_dual.group6().yzx()).with_w(self[e125] * right_dual[e3])
                - (self.group3().yzxw() * right_dual.group6().zxyw())
                - (self.group3().wwwx() * right_dual.group7().with_w(right_dual[e235]))
                - (self.group1().yzx() * right_dual.group1().zxy()).with_w(self[e2] * right_dual[e315])
                - (right_dual.group1().www() * self.group2().xyz()).with_w(self[e3] * right_dual[e125]),
            // e1234
            (self[e4] * right_dual[e321]) + (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412])
                - (self[e423] * right_dual[e1])
                - (self[e431] * right_dual[e2])
                - (self[e412] * right_dual[e3])
                - (self[e321] * right_dual[e4]),
        );
    }
}
impl BulkExpansion<Plane> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       21        0
    //  no simd       17       39        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group0().xyz(),
            // e23, e31, e12, e45
            ((self.group3().yzx() * right_dual.group0().zxy()) - (self.group3().zxy() * right_dual.group0().yzx())).with_w(self[e4] * right_dual[e5]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e1] * right_dual[e5],
                self[e2] * right_dual[e5],
                self[e3] * right_dual[e5],
                -(self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]),
            ]) - (right_dual.group0().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e425] * right_dual[e3] * -1.0,
                self[e435] * right_dual[e1] * -1.0,
                self[e415] * right_dual[e2] * -1.0,
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<RoundPoint> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiScalar::from_groups(
            // e12345
            (self[e4] * right_dual[e3215]) + (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkExpansion<Sphere> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       25       44        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (self.group3().yzx() * right_dual.group0().zxy()).with_w(self[e4] * right_dual[e5]) - (self.group3().zxyw() * right_dual.group0().yzxw()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e1] * right_dual[e5],
                self[e2] * right_dual[e5],
                self[e3] * right_dual[e5],
                -(self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]) - (self[e321] * right_dual[e4]),
            ]) - (right_dual.group0().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<VersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        7        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       47       76        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group3().zxyy() * right_dual.group0().yzx().with_w(right_dual[e4315]))
                + Simd32x3::from(0.0).with_w(
                    (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
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
                + (self.group0() * right_dual.group0().www()).with_w(self[e4] * right_dual[e3215])
                + (self.group2().www() * right_dual.group1().xyz()).with_w(self[e1] * right_dual[e4235])
                - (self.group3().yzx() * right_dual.group0().zxy()).with_w(self[e423] * right_dual[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * right_dual[e15]) + (self[e5] * right_dual[e41]),
                (self[e4] * right_dual[e25]) + (self[e5] * right_dual[e42]),
                (self[e4] * right_dual[e35]) + (self[e5] * right_dual[e43]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group1())
                - (self.group3().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * self.group2().xyz())
                + (self.group3().yzx() * right_dual.group2().zxy())
                - (self.group3().zxy() * right_dual.group2().yzx()))
            .with_w(self[e5] * right_dual[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3().xyz().with_w(self[e4]),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        7        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       48       76        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (self.group3().yzx() * right_dual.group3().zxy()).with_w(self[e4] * right_dual[e5]) - (self.group3().zxyw() * right_dual.group3().yzxw()),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412])
                    - (self[e431] * right_dual[e2])
                    - (self[e412] * right_dual[e3])
                    - (self[e321] * right_dual[e4]),
            ) + (right_dual.group2().www() * self.group3().xyz()).with_w(self[e4] * right_dual[e321])
                - (right_dual.group3().xyzx() * self.group3().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (self.group2().wwwy() * right_dual.group2().xyz().with_w(right_dual[e2]))
                + (right_dual.group3().yzxx() * self.group1().zxy().with_w(self[e235]))
                + (self.group3().zxy() * right_dual.group1().yzx()).with_w(self[e125] * right_dual[e3])
                - (Simd32x4::from(self[e5]) * right_dual.group0().xyz().with_w(right_dual[e321]))
                - (self.group3().yzxz() * right_dual.group1().zxy().with_w(right_dual[e125]))
                - (self.group1().yzx() * right_dual.group3().zxy()).with_w(self[e1] * right_dual[e235])
                - (right_dual.group3().www() * self.group2().xyz()).with_w(self[e2] * right_dual[e315]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiDualNum {
    type Output = bulk_expansion_partial<AntiDualNum>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for AntiDualNum {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self[scalar]) * right_dual.group2(),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for AntiDualNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       27        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiFlector> for AntiDualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<AntiLine> for AntiDualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<AntiMotor> for AntiDualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<AntiPlane> for AntiDualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiScalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<Circle> for AntiDualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * right_dual.group2(),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       29        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * right_dual.group0().with_w(right_dual[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            self.group0().yy().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group2().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual[scalar] * self[e3215]),
        );
    }
}
impl BulkExpansion<Dipole> for AntiDualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group2(),
        );
    }
}
impl BulkExpansion<DipoleInversion> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       31        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0().yy().with_zw(self[scalar], self[e3215]) * right_dual.group0().with_w(right_dual[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * right_dual.group2().xyz().with_w(right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3().xyz().with_w(right_dual[e4]),
        );
    }
}
impl BulkExpansion<DualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (right_dual[e3215] * self[scalar]) + (right_dual[scalar] * self[e3215]),
            right_dual[scalar] * self[scalar],
        ]));
    }
}
impl BulkExpansion<FlatPoint> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<Flector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<Line> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<Motor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from([right_dual[e15], right_dual[e25], right_dual[e35], 1.0])
                * self.group0().yy().with_zw(self[scalar], (self[e3215] * right_dual[scalar]) + (self[scalar] * right_dual[e3215])),
        );
    }
}
impl BulkExpansion<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd2        0        1        0
    //    simd3        0        6        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        2       23        0
    //  no simd        2       57        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
            Simd32x2::from([self[scalar] * right_dual[scalar], (self[e3215] * right_dual[e4]) + (self[scalar] * right_dual[e12345])]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e5
            self[scalar] * right_dual[e5],
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group3(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group4(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * right_dual.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group6(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * right_dual.group7(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e4235], right_dual[e4315], right_dual[e4125], 1.0])
                * self.group0().yy().with_zw(self[scalar], (self[e3215] * right_dual[scalar]) + (self[scalar] * right_dual[e3215])),
            // e1234
            self[scalar] * right_dual[e1234],
        );
    }
}
impl BulkExpansion<Plane> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<RoundPoint> for AntiDualNum {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1234
            self[scalar] * right_dual[e1234],
        );
    }
}
impl BulkExpansion<Scalar> for AntiDualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * right_dual[e12345]);
    }
}
impl BulkExpansion<Sphere> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e3215] * right_dual[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
        );
    }
}
impl BulkExpansion<VersorEven> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       34        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e4235], right_dual[e4315], right_dual[e4125], 1.0])
                * self.group0().yy().with_zw(self[scalar], (self[e3215] * right_dual[scalar]) + (self[scalar] * right_dual[e3215])),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       34        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
            Simd32x4::from([right_dual[e423], right_dual[e431], right_dual[e412], 1.0])
                * self.group0().yy().with_zw(self[scalar], (self[e3215] * right_dual[e4]) + (self[scalar] * right_dual[e12345])),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * right_dual.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiFlatPoint {
    type Output = bulk_expansion_partial<AntiFlatPoint>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       16        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
            -(self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e321] * right_dual[e5], 1.0]) * Simd32x2::from([1.0, 0.0]));
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(/* e12345 */ self[e321] * right_dual[e45] * -1.0);
    }
}
impl BulkExpansion<AntiFlector> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiScalar::from_groups(/* e12345 */ self[e321] * right_dual[e45] * -1.0);
    }
}
impl BulkExpansion<AntiMotor> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e321] * right_dual[e5], 1.0]) * Simd32x2::from([1.0, 0.0]));
    }
}
impl BulkExpansion<AntiScalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<Circle> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiScalar::from_groups(
            // e12345
            -(self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       17        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual[scalar] * self[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], 1.0])
                * right_dual
                    .group2()
                    .www()
                    .with_w(-(right_dual[e41] * self[e235]) - (right_dual[e42] * self[e315]) - (right_dual[e43] * self[e125]) - (right_dual[e45] * self[e321])),
        );
    }
}
impl BulkExpansion<DipoleInversion> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       29        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e235], self[e315], self[e125], 1.0])
                * right_dual
                    .group2()
                    .www()
                    .with_w((right_dual[e1] * self[e235]) + (right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234
            right_dual[e4] * self[e321] * -1.0,
        );
    }
}
impl BulkExpansion<DualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<Flector> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e235] * right_dual[e1]) + (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]),
            0.0,
        ]));
    }
}
impl BulkExpansion<Motor> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        6       22        0
    //  no simd        6       44        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                -(self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
            ]),
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
            Simd32x3::from(0.0).with_w(self[e321] * right_dual[scalar]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w((self[e235] * right_dual[e1]) + (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234
            self[e321] * right_dual[e4] * -1.0,
        );
    }
}
impl BulkExpansion<Plane> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e235] * right_dual[e1]) + (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]),
            0.0,
        ]));
    }
}
impl BulkExpansion<Sphere> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       18        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w((self[e235] * right_dual[e1]) + (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234
            self[e321] * right_dual[e4] * -1.0,
        );
    }
}
impl BulkExpansion<VersorEven> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       25        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[e321] * right_dual[scalar]),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w(-(self[e235] * right_dual[e41]) - (self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45])),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       30        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w((self[e235] * right_dual[e1]) + (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1234
            self[e321] * right_dual[e4] * -1.0,
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiFlector {
    type Output = bulk_expansion_partial<AntiFlector>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       27        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group1().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315]))
                - (right_dual.group0() * self.group1().www()).with_w(self[e1] * right_dual[e235]),
            // e1234
            (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       24       44        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (right_dual.group0().yzx() * self.group1().zxy()) - (right_dual.group0().zxy() * self.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e5] * right_dual[e41],
                self[e5] * right_dual[e42],
                self[e5] * right_dual[e43],
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group1().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e12345
            (self.group1().yzxx() * right_dual.group2().zxy().with_w(right_dual[e4235]))
                + (self.group1().wwwy() * right_dual.group1().xyz().with_w(right_dual[e4315]))
                + Simd32x3::from(0.0).with_w(
                    (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43])
                        - (self[e321] * right_dual[e45]),
                )
                - (self.group1().zxy() * right_dual.group2().yzx()).with_w(self[e235] * right_dual[e41]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Flector::from_groups(
            // e15, e25, e35, e45
            right_dual.group0().xx().with_zw(right_dual[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e321] * right_dual[e5]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       18        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_dual[e45]) * self.group1().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            ((self.group1().yzx() * right_dual.group0().zxy()) - (self.group1().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiFlector> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       26        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[e45], right_dual[e45], right_dual[e45], 1.0])
                * self
                    .group1()
                    .xyz()
                    .with_w((self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) - (self[e321] * right_dual[e45]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            ((self.group1().yzx() * right_dual.group0().zxy()) - (self.group1().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiLine> for AntiFlector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        5       15        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]),
            ]) - (self.group1().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiMotor> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        1        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        9       30        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(-(self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]))
                + (self.group1().zxy() * right_dual.group0().yzx()).with_w(self[e321] * right_dual[e5])
                - (self.group1().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiPlane> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(/* e12345 */ (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]));
    }
}
impl BulkExpansion<AntiScalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Circle> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       17       32        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            (right_dual.group0().yzx() * self.group1().zxy()) - (right_dual.group0().zxy() * self.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e5] * right_dual[e41],
                self[e5] * right_dual[e42],
                self[e5] * right_dual[e43],
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group1().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e2] * right_dual[e35]) + (self[e5] * right_dual[e23]),
                (self[e3] * right_dual[e15]) + (self[e5] * right_dual[e31]),
                (self[e1] * right_dual[e25]) + (self[e5] * right_dual[e12]),
                -(self[e315] * right_dual[e42]) - (self[e125] * right_dual[e43]) - (self[e321] * right_dual[e45]),
            ]) - (right_dual.group2().yzx() * self.group1().zxy()).with_w(self[e235] * right_dual[e41]),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        3        6        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        9       23        0
    //  no simd       24       53        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                right_dual[e42] * self[e3],
                right_dual[e43] * self[e1],
                right_dual[e41] * self[e2],
                -(right_dual[e42] * self[e315]) - (right_dual[e43] * self[e125]) - (right_dual[e45] * self[e321]),
            ]) - (right_dual.group0().zxy() * self.group1().yzx()).with_w(right_dual[e41] * self[e235]),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]))
                + (right_dual.group0() * self.group1().www()).with_w(right_dual[scalar] * self[e321])
                - (right_dual.group1().wwwx() * self.group1().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[scalar]) * self.group0().xyz())
                + (Simd32x3::from(self[e5]) * right_dual.group1().xyz())
                + (right_dual.group2().zxy() * self.group1().yzx())
                - (right_dual.group2().yzx() * self.group1().zxy()))
            .with_w(right_dual[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group2().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<Dipole> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       26        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group1().wwwx() * right_dual.group0().with_w(right_dual[e235]))
                - (self.group1().yzx() * right_dual.group1().zxy()).with_w(self[e2] * right_dual[e315]),
            // e1234
            (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        7        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       31       59        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((right_dual.group3().zxy() * self.group1().yzx()) - (right_dual.group3().yzx() * self.group1().zxy())).with_w(right_dual[e4] * self[e5] * -1.0),
            // e15, e25, e35, e1234
            (self.group1().xyzx() * right_dual.group3().www().with_w(right_dual[e423])) + Simd32x3::from(0.0).with_w((right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]))
                - (self.group1().www() * right_dual.group3().xyz()).with_w(right_dual[e4] * self[e321]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w((right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]) - (right_dual[e125] * self[e3]))
                + (right_dual.group1().yzx() * self.group1().zxy()).with_w(right_dual[e1] * self[e235])
                - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (right_dual.group2().wwwy() * self.group0().xyz().with_w(self[e2]))
                - (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<DualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<FlatPoint> for AntiFlector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(right_dual[e235] * self[e1]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5]),
            0.0,
        ]));
    }
}
impl BulkExpansion<Flector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       16       28        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((right_dual.group1().zxy() * self.group1().yzx()) - (right_dual.group1().yzx() * self.group1().zxy())).with_w(0.0),
            // e15, e25, e35, e3215
            (right_dual.group1().wwwx() * self.group1().xyz().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321])
                        - (right_dual[e315] * self[e2])
                        - (right_dual[e125] * self[e3])
                        - (right_dual[e321] * self[e5]),
                )
                - (self.group1().wwwx() * right_dual.group1().xyz().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<Line> for AntiFlector {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e2] * right_dual[e35]) + (self[e5] * right_dual[e23]),
                (self[e3] * right_dual[e15]) + (self[e5] * right_dual[e31]),
                (self[e1] * right_dual[e25]) + (self[e5] * right_dual[e12]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group1().zxyx() * right_dual.group1().yzx().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<Motor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e2] * right_dual[e35]) + (self[e5] * right_dual[e23]),
                (self[e3] * right_dual[e15]) + (self[e5] * right_dual[e31]),
                (self[e1] * right_dual[e25]) + (self[e5] * right_dual[e12]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group0())
                - (self.group1().zxyx() * right_dual.group1().yzx().with_w(right_dual[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        0
    //    simd2        0        1        0
    //    simd3        6       19        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       26       55        0
    //  no simd       56      115        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43])
                    - (self[e321] * right_dual[e45]),
            ]),
            // e1, e2, e3, e4
            right_dual.group0().xx().with_zw(right_dual[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e5
            self[e5] * right_dual[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group1().xyz())).with_w(self[e5] * right_dual[e4] * -1.0),
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (self.group1().yzx() * right_dual.group1().zxy()) - (self.group1().zxy() * right_dual.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]))
                + (right_dual.group4() * self.group1().www()).with_w(self[e321] * right_dual[scalar])
                - (right_dual.group3().www() * self.group1().xyz()).with_w(self[e1] * right_dual[e23]),
            // e423, e431, e412
            (right_dual.group4().yzx() * self.group1().zxy()) - (right_dual.group4().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group5()) + (Simd32x3::from(right_dual[scalar]) * self.group0().xyz()) + (self.group1().yzx() * right_dual.group3().zxy())
                - (self.group1().zxy() * right_dual.group3().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w((self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]) - (self[e5] * right_dual[e321]))
                + (self.group1().zxy() * right_dual.group6().yzx()).with_w(self[e235] * right_dual[e1])
                - (self.group1().wwwx() * right_dual.group7().with_w(right_dual[e235]))
                - (self.group1().yzx() * right_dual.group6().zxy()).with_w(self[e3] * right_dual[e125])
                - (right_dual.group1().www() * self.group0().xyz()).with_w(self[e2] * right_dual[e315]),
            // e1234
            (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) - (self[e321] * right_dual[e4]),
        );
    }
}
impl BulkExpansion<Plane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       23        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((self.group1().yzx() * right_dual.group0().zxy()) - (self.group1().zxy() * right_dual.group0().yzx())).with_w(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([
                self[e5] * right_dual[e1] * -1.0,
                self[e5] * right_dual[e2] * -1.0,
                self[e5] * right_dual[e3] * -1.0,
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]),
            ]) + (right_dual.group0().wwwx() * self.group1().xyz().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<RoundPoint> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        9        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiScalar::from_groups(
            // e12345
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkExpansion<Sphere> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        2        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       17        0
    //  no simd        9       38        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group1().yzx() * right_dual.group0().zxy()) - (self.group1().zxy() * right_dual.group0().yzx())).with_w(self[e5] * right_dual[e4] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group0().xyz())).with_w(self[e321] * right_dual[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w((self[e235] * right_dual[e1]) + (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl BulkExpansion<VersorEven> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        3        4        0
    //    simd4        4       11        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       31       65        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group1().zxyx() * right_dual.group0().yzx().with_w(right_dual[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43])
                        - (self[e321] * right_dual[e45]),
                )
                - (right_dual.group0().zxyx() * self.group1().yzx().with_w(self[e235])),
            // e415, e425, e435, e321
            (right_dual.group0() * self.group1().www().with_w(self[e321])) + Simd32x3::from(0.0).with_w(-(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]))
                - (self.group1().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * self.group0().xyz())
                + (self.group1().yzx() * right_dual.group2().zxy())
                - (self.group1().zxy() * right_dual.group2().yzx()))
            .with_w(self[e5] * right_dual[scalar]),
            // e1, e2, e3, e4
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        1        6        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       31       60        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group1().yzx() * right_dual.group3().zxy()) - (self.group1().zxy() * right_dual.group3().yzx())).with_w(self[e5] * right_dual[e4] * -1.0),
            // e15, e25, e35, e1234
            (self.group1().xyzx() * right_dual.group2().www().with_w(right_dual[e423])) + Simd32x3::from(0.0).with_w((self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]))
                - (right_dual.group3() * self.group1().www().with_w(self[e321])),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w((self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]) + (self[e321] * right_dual[e5]) - (self[e5] * right_dual[e321]))
                + (self.group1().zxy() * right_dual.group1().yzx()).with_w(self[e235] * right_dual[e1])
                - (self.group1().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315]))
                - (self.group1().wwwz() * right_dual.group0().xyz().with_w(right_dual[e125]))
                - (right_dual.group3().www() * self.group0().xyz()).with_w(self[e1] * right_dual[e235]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiLine {
    type Output = bulk_expansion_partial<AntiLine>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       17        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            -(self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435])
                - (self[e15] * right_dual[e423])
                - (self[e25] * right_dual[e431])
                - (self[e35] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       30        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e35] * right_dual[e42]),
                (self[e31] * right_dual[e45]) + (self[e15] * right_dual[e43]),
                (self[e12] * right_dual[e45]) + (self[e25] * right_dual[e41]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group1().yzx() * right_dual.group0().zxy()).with_w(self[e23] * right_dual[e15]),
            // e1234
            -(self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            right_dual.group0().xx().with_zw(right_dual[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e45], right_dual[e45], right_dual[e45], 1.0])
                * self.group0().with_w(-(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35])),
        );
    }
}
impl BulkExpansion<AntiFlector> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e45], right_dual[e45], right_dual[e45], 1.0])
                * self.group0().with_w(-(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35])),
        );
    }
}
impl BulkExpansion<AntiLine> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(/* e12345 */ -(self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435]));
    }
}
impl BulkExpansion<AntiMotor> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        2       23        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(-(self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group1().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiScalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Circle> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       22        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e35] * right_dual[e42]),
                (self[e31] * right_dual[e45]) + (self[e15] * right_dual[e43]),
                (self[e12] * right_dual[e45]) + (self[e25] * right_dual[e41]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group1().yzx() * right_dual.group0().zxy()).with_w(self[e23] * right_dual[e15]),
            // e1234
            -(self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        6        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       42        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group1().with_w(-(right_dual[e41] * self[e23]) - (right_dual[e42] * self[e31]) - (right_dual[e43] * self[e12])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e45] * self[e23]),
                (right_dual[e43] * self[e15]) + (right_dual[e45] * self[e31]),
                (right_dual[e41] * self[e25]) + (right_dual[e45] * self[e12]),
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().zxy() * self.group1().yzx()).with_w(right_dual[e15] * self[e23]),
        );
    }
}
impl BulkExpansion<Dipole> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(
            // e12345
            -(self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435])
                - (self[e15] * right_dual[e423])
                - (self[e25] * right_dual[e431])
                - (self[e35] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        3        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       40        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e4]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self.group1().with_w(-(right_dual[e1] * self[e23]) - (right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (right_dual[e2] * self[e35]) + (right_dual[e5] * self[e23]),
                (right_dual[e3] * self[e15]) + (right_dual[e5] * self[e31]),
                (right_dual[e1] * self[e25]) + (right_dual[e5] * self[e12]),
                -(right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e415] * self[e23]) - (right_dual[e425] * self[e31]) - (right_dual[e435] * self[e12]),
            ]) - (self.group1().yzx() * right_dual.group3().zxy()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkExpansion<DualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Flector> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (right_dual[e2] * self[e35]) + (right_dual[e5] * self[e23]),
                (right_dual[e3] * self[e15]) + (right_dual[e5] * self[e31]),
                (right_dual[e1] * self[e25]) + (right_dual[e5] * self[e12]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group1().zxyx() * self.group1().yzx().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<Line> for AntiLine {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(right_dual[e23] * self[e15])
                - (right_dual[e31] * self[e25])
                - (right_dual[e12] * self[e35])
                - (right_dual[e15] * self[e23])
                - (right_dual[e25] * self[e31])
                - (right_dual[e35] * self[e12]),
            0.0,
        ]));
    }
}
impl BulkExpansion<Motor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       30        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group1().with_w(
                    -(self[e23] * right_dual[e15])
                        - (self[e31] * right_dual[e25])
                        - (self[e12] * right_dual[e35])
                        - (self[e15] * right_dual[e23])
                        - (self[e25] * right_dual[e31])
                        - (self[e35] * right_dual[e12]),
                ),
        );
    }
}
impl BulkExpansion<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd2        0        1        0
    //    simd3        2        8        0
    //    simd4        1        7        0
    // Totals...
    // yes simd       19       42        0
    //  no simd       26       80        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                -(self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            right_dual.group0().xx().with_zw(right_dual[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self.group1().with_w(-(self[e23] * right_dual[e1]) - (self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3])),
            // e423, e431, e412
            Simd32x3::from(right_dual[e4]) * self.group0(),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group0()) + (self.group1().zxy() * right_dual.group1().yzx()) - (self.group1().yzx() * right_dual.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e35] * right_dual[e42]),
                (self[e31] * right_dual[e45]) + (self[e15] * right_dual[e43]),
                (self[e12] * right_dual[e45]) + (self[e25] * right_dual[e41]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group1().yzx() * right_dual.group4().zxy()).with_w(self[e23] * right_dual[e15]),
            // e1234
            -(self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<Plane> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e23] * right_dual[e5]) + (self[e35] * right_dual[e2]),
                (self[e31] * right_dual[e5]) + (self[e15] * right_dual[e3]),
                (self[e12] * right_dual[e5]) + (self[e25] * right_dual[e1]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (right_dual.group0().zxyx() * self.group1().yzx().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<Sphere> for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        8       23        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e4]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self.group1().with_w(-(self[e23] * right_dual[e1]) - (self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3])),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group0()) + (self.group1().zxy() * right_dual.group0().yzx()) - (self.group1().yzx() * right_dual.group0().zxy()),
        );
    }
}
impl BulkExpansion<VersorEven> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        8        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       13       50        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group1().with_w(-(self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e35] * right_dual[e42]),
                (self[e31] * right_dual[e45]) + (self[e15] * right_dual[e43]),
                (self[e12] * right_dual[e45]) + (self[e25] * right_dual[e41]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group1().yzx() * right_dual.group0().zxy()).with_w(self[e23] * right_dual[e15]),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        5        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       41        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e4]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self.group1().with_w(-(self[e23] * right_dual[e1]) - (self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e23] * right_dual[e5]) + (self[e35] * right_dual[e2]),
                (self[e31] * right_dual[e5]) + (self[e15] * right_dual[e3]),
                (self[e12] * right_dual[e5]) + (self[e25] * right_dual[e1]),
                -(self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435]) - (self[e15] * right_dual[e423]) - (self[e25] * right_dual[e431]) - (self[e35] * right_dual[e412]),
            ]) - (self.group1().yzx() * right_dual.group3().zxy()).with_w(self[e23] * right_dual[e415]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiMotor {
    type Output = bulk_expansion_partial<AntiMotor>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        6       29        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[e235], right_dual[e315], right_dual[e125], 1.0])
                * self.group0().www().with_w(
                    (self[scalar] * right_dual[e12345])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd3        0        3        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       18       46        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[e15], right_dual[e25], right_dual[e35], 1.0])
                * self
                    .group0()
                    .www()
                    .with_w((self[scalar] * right_dual[e1234]) - (self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[scalar] * right_dual[e4235]),
                (self[e31] * right_dual[e45]) + (self[scalar] * right_dual[e4315]),
                (self[e12] * right_dual[e45]) + (self[scalar] * right_dual[e4125]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (right_dual.group0().yzx() * self.group1().zxy()).with_w(self[scalar] * right_dual[e3215])
                - (right_dual.group0().zxy() * self.group1().yzx()).with_w(self[e23] * right_dual[e15]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[e5]) * self.group0(),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([right_dual[e45], right_dual[e45], right_dual[e45], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w(-(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35])),
        );
    }
}
impl BulkExpansion<AntiFlector> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       22        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[scalar] * right_dual[e4235],
                self[scalar] * right_dual[e4315],
                self[scalar] * right_dual[e4125],
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]),
            ]) + (self.group0() * right_dual.group0().www().with_w(right_dual[e3215])),
        );
    }
}
impl BulkExpansion<AntiLine> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        2       25        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual
                    .group0()
                    .with_w(-(self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiMotor> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       23        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[e415], right_dual[e425], right_dual[e435], 1.0])
                * self
                    .group0()
                    .www()
                    .with_w((self[scalar] * right_dual[e12345]) - (self[e23] * right_dual[e415]) - (self[e31] * right_dual[e425]) - (self[e12] * right_dual[e435])),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[e5]) * self.group0().xyz())).with_w(self[scalar] * right_dual[e5]),
        );
    }
}
impl BulkExpansion<AntiPlane> for AntiMotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiScalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Circle> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       33        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual
                    .group2()
                    .with_w(-(self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e35] * right_dual[e42]),
                (self[e31] * right_dual[e45]) + (self[e15] * right_dual[e43]),
                (self[e12] * right_dual[e45]) + (self[e25] * right_dual[e41]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (right_dual.group0().zxy() * self.group1().yzx()).with_w(self[e23] * right_dual[e15]),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        1        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       20       44        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * right_dual.group0().with_w(right_dual[scalar]),
            // e23, e31, e12, e45
            ((Simd32x3::from(right_dual[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group1().xyz())).with_w(right_dual[e45] * self[scalar]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (right_dual[e15] * self[scalar]) + (right_dual[scalar] * self[e15]),
                (right_dual[e25] * self[scalar]) + (right_dual[scalar] * self[e25]),
                (right_dual[e35] * self[scalar]) + (right_dual[scalar] * self[e35]),
                -(right_dual[e41] * self[e23]) - (right_dual[e42] * self[e31]) - (right_dual[e43] * self[e12]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e45] * self[e23],
                right_dual[e45] * self[e31],
                right_dual[e45] * self[e12],
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (self.group1().zxyw() * right_dual.group0().yzx().with_w(right_dual[scalar]))
                - (self.group1().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<Dipole> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       27        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group2().with_w(
                    -(self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                ),
        );
    }
}
impl BulkExpansion<DipoleInversion> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        3        6        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       28        0
    //  no simd       24       55        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                right_dual[e4] * self[e23],
                right_dual[e4] * self[e31],
                right_dual[e4] * self[e12],
                -(right_dual[e423] * self[e15])
                    - (right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12]),
            ]) + (right_dual.group0() * self.group0().www()).with_w(right_dual[e4] * self[e3215]),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e4] * self[e15],
                right_dual[e4] * self[e25],
                right_dual[e4] * self[e35],
                -(right_dual[e1] * self[e23]) - (right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group0().xyz())
                + (Simd32x3::from(self[scalar]) * right_dual.group2().xyz())
                + (right_dual.group3().yzx() * self.group1().zxy())
                - (right_dual.group3().zxy() * self.group1().yzx()))
            .with_w(right_dual[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3().xyz().with_w(right_dual[e4]),
        );
    }
}
impl BulkExpansion<DualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        1       12        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from([self[e15], self[e25], self[e35], 1.0])
                * right_dual
                    .group0()
                    .yy()
                    .with_zw(right_dual[scalar], (right_dual[e3215] * self[scalar]) + (right_dual[scalar] * self[e3215])),
        );
    }
}
impl BulkExpansion<FlatPoint> for AntiMotor {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<Flector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (right_dual[e2] * self[e35]) + (right_dual[e5] * self[e23]),
                (right_dual[e3] * self[e15]) + (right_dual[e5] * self[e31]),
                (right_dual[e1] * self[e25]) + (right_dual[e5] * self[e12]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group0())
                - (right_dual.group1().zxyx() * self.group1().yzx().with_w(self[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<Line> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       22        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group1().with_w(
                    -(right_dual[e23] * self[e15])
                        - (right_dual[e31] * self[e25])
                        - (right_dual[e12] * self[e35])
                        - (right_dual[e15] * self[e23])
                        - (right_dual[e25] * self[e31])
                        - (right_dual[e35] * self[e12]),
                ),
        );
    }
}
impl BulkExpansion<Motor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((Simd32x3::from(right_dual[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz())).with_w(right_dual[scalar] * self[scalar]),
            // e15, e25, e35, e3215
            (Simd32x4::from(right_dual[scalar]) * self.group1())
                + (Simd32x4::from(self[scalar]) * right_dual.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e23] * self[e15])
                        - (right_dual[e31] * self[e25])
                        - (right_dual[e12] * self[e35])
                        - (right_dual[e15] * self[e23])
                        - (right_dual[e25] * self[e31])
                        - (right_dual[e35] * self[e12]),
                ),
        );
    }
}
impl BulkExpansion<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd2        0        1        0
    //    simd3        6       13        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       26       53        0
    //  no simd       50      104        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                self[scalar] * right_dual[scalar],
                (self[scalar] * right_dual[e12345]) + (self[e3215] * right_dual[e4])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e5
            self[scalar] * right_dual[e5],
            // e15, e25, e35, e45
            ((Simd32x3::from(self[scalar]) * right_dual.group3().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group1().xyz())).with_w(self[scalar] * right_dual[e45]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group4(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * right_dual.group5()) + (Simd32x3::from(right_dual[scalar]) * self.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e15] * right_dual[e4],
                self[e25] * right_dual[e4],
                self[e35] * right_dual[e4],
                -(self[e23] * right_dual[e1]) - (self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group6()),
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * right_dual.group7()) + (Simd32x3::from(right_dual[e4]) * self.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * right_dual.group8()) + (Simd32x3::from(right_dual[e5]) * self.group0().xyz()) + (self.group1().zxy() * right_dual.group1().yzx())
                - (self.group1().yzx() * right_dual.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[scalar] * right_dual[e4235],
                self[scalar] * right_dual[e4315],
                self[scalar] * right_dual[e4125],
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (self.group0() * right_dual.group3().www().with_w(right_dual[e3215]))
                + (self.group1().zxyw() * right_dual.group4().yzx().with_w(right_dual[scalar]))
                - (self.group1().yzxx() * right_dual.group4().zxy().with_w(right_dual[e23])),
            // e1234
            (self[scalar] * right_dual[e1234]) - (self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<Plane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self[e23] * right_dual[e5]) + (self[e35] * right_dual[e2]),
                (self[e31] * right_dual[e5]) + (self[e15] * right_dual[e3]),
                (self[e12] * right_dual[e5]) + (self[e25] * right_dual[e1]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (right_dual.group0().zxyx() * self.group1().yzx().with_w(self[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_dual.group0(),
        );
    }
}
impl BulkExpansion<RoundPoint> for AntiMotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1234
            self[scalar] * right_dual[e1234],
        );
    }
}
impl BulkExpansion<Scalar> for AntiMotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * right_dual[e12345]);
    }
}
impl BulkExpansion<Sphere> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        8       29        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual[e4]) * self.group0().xyz().with_w(self[e3215]),
            // e415, e425, e435, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 1.0])
                * self.group1().xyz().with_w(-(self[e23] * right_dual[e1]) - (self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3])),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group0().xyz()) + (self.group1().zxy() * right_dual.group0().yzx()) - (self.group1().yzx() * right_dual.group0().zxy()))
                .with_w(self[scalar] * right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
        );
    }
}
impl BulkExpansion<VersorEven> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        4        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       11       28        0
    //  no simd       25       57        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            ((Simd32x3::from(self[scalar]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group0().xyz())).with_w(self[scalar] * right_dual[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e15] * right_dual[scalar],
                self[e25] * right_dual[scalar],
                self[e35] * right_dual[scalar],
                -(self[e23] * right_dual[e41]) - (self[e31] * right_dual[e42]) - (self[e12] * right_dual[e43]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e35] * right_dual[e42],
                self[e15] * right_dual[e43],
                self[e25] * right_dual[e41],
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (self.group0() * right_dual.group1().www().with_w(right_dual[e3215]))
                + (self.group0().www() * right_dual.group3().xyz()).with_w(self[e3215] * right_dual[scalar])
                - (self.group1().yzx() * right_dual.group0().zxy()).with_w(self[e23] * right_dual[e15]),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       28       57        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
            (self.group0() * right_dual.group3().www().with_w(right_dual[e12345]))
                + Simd32x3::from(0.0).with_w(
                    -(self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435])
                        - (self[e15] * right_dual[e423])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412]),
                )
                + (self.group0().www() * right_dual.group0().xyz()).with_w(self[e3215] * right_dual[e4]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e15] * right_dual[e4],
                self[e25] * right_dual[e4],
                self[e35] * right_dual[e4],
                -(self[e23] * right_dual[e1]) - (self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[scalar]) * right_dual.group2().xyz())
                + (Simd32x3::from(right_dual[e5]) * self.group0().xyz())
                + (self.group1().zxy() * right_dual.group3().yzx())
                - (self.group1().yzx() * right_dual.group3().zxy()))
            .with_w(self[scalar] * right_dual[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiPlane {
    type Output = bulk_expansion_partial<AntiPlane>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       27        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group0().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315]))
                - (right_dual.group0() * self.group0().www()).with_w(self[e1] * right_dual[e235]),
            // e1234
            (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for AntiPlane {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        6       21        0
    //  no simd       17       43        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e5] * right_dual[e41],
                self[e5] * right_dual[e42],
                self[e5] * right_dual[e43],
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group0().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self[e3] * right_dual[e25] * -1.0,
                self[e1] * right_dual[e35] * -1.0,
                self[e2] * right_dual[e15] * -1.0,
                (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
            ]) + (self.group0().yzxx() * right_dual.group2().zxy().with_w(right_dual[e4235]))
                + (self.group0().wwwy() * right_dual.group1().xyz().with_w(right_dual[e4315])),
        );
    }
}
impl BulkExpansion<AntiDualNum> for AntiPlane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            right_dual.group0().xx().with_zw(right_dual[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for AntiPlane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       16        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_dual[e45]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
        );
    }
}
impl BulkExpansion<AntiFlector> for AntiPlane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        5       25        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[e45], right_dual[e45], right_dual[e45], 1.0])
                * self
                    .group0()
                    .xyz()
                    .with_w((self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            ((self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiLine> for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        5       15        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiMotor> for AntiPlane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        6        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       29        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e2] * right_dual[e315]) - (self[e3] * right_dual[e125]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiPlane> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(/* e12345 */ (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]));
    }
}
impl BulkExpansion<AntiScalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<Circle> for AntiPlane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       14       28        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Circle::from_groups(
            // e423, e431, e412
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e5] * right_dual[e41],
                self[e5] * right_dual[e42],
                self[e5] * right_dual[e43],
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group0().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (right_dual.group2().zxy() * self.group0().yzx()) - (right_dual.group2().yzx() * self.group0().zxy()),
        );
    }
}
impl BulkExpansion<CircleRotor> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       16       36        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e41] * self[e5],
                right_dual[e42] * self[e5],
                right_dual[e43] * self[e5],
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125, e4
            (Simd32x3::from(self[e5]) * right_dual.group1().xyz()).with_w(0.0) + (right_dual.group2().zxy() * self.group0().yzx()).with_w(0.0)
                - (right_dual.group2().yzx() * self.group0().zxy()).with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[scalar]) * self.group0(),
        );
    }
}
impl BulkExpansion<Dipole> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       26        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group0().wwwx() * right_dual.group0().with_w(right_dual[e235]))
                - (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[e2] * right_dual[e315]),
            // e1234
            (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        5        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        6       26        0
    //  no simd       17       54        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((right_dual.group3().zxy() * self.group0().yzx()) - (right_dual.group3().yzx() * self.group0().zxy())).with_w(right_dual[e4] * self[e5] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                right_dual[e1] * self[e5] * -1.0,
                right_dual[e2] * self[e5] * -1.0,
                right_dual[e3] * self[e5] * -1.0,
                (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]),
            ]) + (self.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e425] * self[e3],
                right_dual[e435] * self[e1],
                right_dual[e415] * self[e2],
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<DualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<FlatPoint> for AntiPlane {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(right_dual[e235] * self[e1]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5]),
            0.0,
        ]));
    }
}
impl BulkExpansion<Flector> for AntiPlane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        9       24        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((right_dual.group1().zxy() * self.group0().yzx()) - (right_dual.group1().yzx() * self.group0().zxy())).with_w(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([
                right_dual[e5] * self[e1],
                right_dual[e5] * self[e2],
                right_dual[e5] * self[e3],
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5]),
            ]) - (self.group0().wwwx() * right_dual.group1().xyz().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<Line> for AntiPlane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (right_dual[e23] * self[e5]) + (right_dual[e35] * self[e2]),
                (right_dual[e31] * self[e5]) + (right_dual[e15] * self[e3]),
                (right_dual[e12] * self[e5]) + (right_dual[e25] * self[e1]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (self.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<Motor> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (right_dual[e23] * self[e5]) + (right_dual[e35] * self[e2]),
                (right_dual[e31] * self[e5]) + (right_dual[e15] * self[e3]),
                (right_dual[e12] * self[e5]) + (right_dual[e25] * self[e1]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (self.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e23])),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[scalar]) * self.group0(),
        );
    }
}
impl BulkExpansion<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       24        0
    //    simd2        0        1        0
    //    simd3        5       15        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       15       47        0
    //  no simd       34       99        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
            ]),
            // e1, e2, e3, e4
            right_dual.group0().xx().with_zw(right_dual[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e5
            self[e5] * right_dual[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group1().xyz())).with_w(self[e5] * right_dual[e4] * -1.0),
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e5] * right_dual[e41],
                self[e5] * right_dual[e42],
                self[e5] * right_dual[e43],
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (right_dual.group3().www() * self.group0().xyz()).with_w(self[e1] * right_dual[e23]),
            // e423, e431, e412
            (right_dual.group4().yzx() * self.group0().zxy()) - (right_dual.group4().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group5()) + (self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group0().wwwx() * right_dual.group7().with_w(right_dual[e235]))
                - (self.group0().yzx() * right_dual.group6().zxy()).with_w(self[e2] * right_dual[e315]),
            // e1234
            (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
        );
    }
}
impl BulkExpansion<Plane> for AntiPlane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiLine::from_groups(
            // e23, e31, e12
            (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
            // e15, e25, e35
            (Simd32x3::from(right_dual[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkExpansion<RoundPoint> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        9        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiScalar::from_groups(
            // e12345
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkExpansion<Sphere> for AntiPlane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        6       24        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx())).with_w(self[e5] * right_dual[e4] * -1.0),
            // e15, e25, e35
            (Simd32x3::from(right_dual[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkExpansion<VersorEven> for AntiPlane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        2        3        0
    //    simd4        2        9        0
    // Totals...
    // yes simd        7       27        0
    //  no simd       17       60        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e2] * right_dual[e43] * -1.0,
                self[e3] * right_dual[e41] * -1.0,
                self[e1] * right_dual[e42] * -1.0,
                (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e5] * right_dual[e1234]),
            ]) + (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e4235])),
            // e415, e425, e435, e321
            Simd32x4::from([
                self[e5] * right_dual[e41],
                self[e5] * right_dual[e42],
                self[e5] * right_dual[e43],
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group0().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (self.group0().yzx() * right_dual.group2().zxy()) - (self.group0().zxy() * right_dual.group2().yzx()))
                .with_w(self[e5] * right_dual[scalar]),
            // e1, e2, e3, e4
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().www().with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<VersorOdd> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        7        0
    // Totals...
    // yes simd        6       26        0
    //  no simd       17       55        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            ((self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx())).with_w(self[e5] * right_dual[e4] * -1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e5] * right_dual[e1] * -1.0,
                self[e5] * right_dual[e2] * -1.0,
                self[e5] * right_dual[e3] * -1.0,
                (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]),
            ]) + (self.group0().xyzx() * right_dual.group2().www().with_w(right_dual[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self[e3] * right_dual[e425],
                self[e1] * right_dual[e435],
                self[e2] * right_dual[e415],
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235]))
                - (self.group0().wwwy() * right_dual.group0().xyz().with_w(right_dual[e315])),
        );
    }
}
impl std::ops::Div<bulk_expansion> for AntiScalar {
    type Output = bulk_expansion_partial<AntiScalar>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * right_dual[scalar]);
    }
}
impl BulkExpansion<CircleRotor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiScalar::from_groups(/* e12345 */ right_dual[scalar] * self[e12345]);
    }
}
impl BulkExpansion<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiScalar::from_groups(/* e12345 */ right_dual[scalar] * self[e12345]);
    }
}
impl BulkExpansion<Motor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiScalar::from_groups(/* e12345 */ right_dual[scalar] * self[e12345]);
    }
}
impl BulkExpansion<MultiVector> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       23        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * right_dual[scalar]);
    }
}
impl BulkExpansion<VersorEven> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       17        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * right_dual[scalar]);
    }
}
impl std::ops::Div<bulk_expansion> for Circle {
    type Output = bulk_expansion_partial<Circle>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiDipoleInversion> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       22        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<AntiDualNum> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]));
    }
}
impl BulkExpansion<AntiFlatPoint> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * right_dual[e15]) - (self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkExpansion<AntiFlector> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * right_dual[e15]) - (self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkExpansion<AntiMotor> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]));
    }
}
impl BulkExpansion<AntiScalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group2(),
        );
    }
}
impl BulkExpansion<Circle> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd        9       14        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<CircleRotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       29        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().with_w(
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
impl BulkExpansion<DipoleInversion> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       35        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(right_dual[e4] * self[e235]) - (right_dual[e3] * self[e425]),
                -(right_dual[e4] * self[e315]) - (right_dual[e1] * self[e435]),
                -(right_dual[e4] * self[e125]) - (right_dual[e2] * self[e415]),
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group3().wwwx() * self.group0().with_w(self[e235]))
                + (right_dual.group3().yzx() * self.group1().zxy()).with_w(right_dual[e2] * self[e315]),
            // e1234
            -(right_dual[e4] * self[e321]) - (right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<DualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group2(),
        );
    }
}
impl BulkExpansion<Flector> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       16        0
    //  no simd       11       27        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group1().wwwx() * self.group0().with_w(self[e235]))
                + (right_dual.group1().yzx() * self.group1().zxy()).with_w(right_dual[e2] * self[e315]),
            // e1234
            -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<Line> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435])
                - (right_dual[e15] * self[e423])
                - (right_dual[e25] * self[e431])
                - (right_dual[e35] * self[e412]),
        );
    }
}
impl BulkExpansion<Motor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       25        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().with_w(
                    -(right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
        );
    }
}
impl BulkExpansion<MultiVector> for Circle {
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
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
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
            ]),
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
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e321] * right_dual[e5]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from([right_dual[e5], right_dual[e5], right_dual[e5], right_dual[e1]]) * self.group0().with_w(self[e235]))
                + (right_dual.group1().yzxy() * self.group1().zxy().with_w(self[e315])),
            // e1234
            -(self[e423] * right_dual[e1]) - (self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]) - (self[e321] * right_dual[e4]),
        );
    }
}
impl BulkExpansion<Plane> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       15        0
    //  no simd       11       23        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group0().wwwx() * self.group0().with_w(self[e235]))
                + (right_dual.group0().yzx() * self.group1().zxy()).with_w(right_dual[e2] * self[e315]),
            // e1234
            -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<Sphere> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       24        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e321] * right_dual[e5]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from([right_dual[e5], right_dual[e5], right_dual[e5], right_dual[e1]]) * self.group0().with_w(self[e235]))
                + (right_dual.group0().yzxy() * self.group1().zxy().with_w(self[e315])),
            // e1234
            -(self[e423] * right_dual[e1]) - (self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]) - (self[e321] * right_dual[e4]),
        );
    }
}
impl BulkExpansion<VersorEven> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        9       17        0
    //  no simd        9       37        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().with_w(
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
impl BulkExpansion<VersorOdd> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       36        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e321] * right_dual[e5]) + (self[e125] * right_dual[e3]),
            ]) + (right_dual.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * right_dual.group2().www()).with_w(self[e235] * right_dual[e1]),
            // e1234
            -(self[e423] * right_dual[e1]) - (self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]) - (self[e321] * right_dual[e4]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for CircleRotor {
    type Output = bulk_expansion_partial<CircleRotor>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiDipoleInversion> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       22        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<AntiDualNum> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]));
    }
}
impl BulkExpansion<AntiFlatPoint> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * right_dual[e15]) - (self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkExpansion<AntiFlector> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * right_dual[e15]) - (self[e431] * right_dual[e25]) - (self[e412] * right_dual[e35]) - (self[e321] * right_dual[e45]),
        );
    }
}
impl BulkExpansion<AntiMotor> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]));
    }
}
impl BulkExpansion<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(right_dual[scalar]) * self.group2(),
        );
    }
}
impl BulkExpansion<Circle> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd        9       14        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<CircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       30        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], 1.0])
                * right_dual.group2().www().with_w(
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
impl BulkExpansion<DipoleInversion> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       35        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(right_dual[e4] * self[e235]) - (right_dual[e3] * self[e425]),
                -(right_dual[e4] * self[e315]) - (right_dual[e1] * self[e435]),
                -(right_dual[e4] * self[e125]) - (right_dual[e2] * self[e415]),
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * right_dual.group3().www()).with_w(right_dual[e1] * self[e235]),
            // e1234
            -(right_dual[e4] * self[e321]) - (right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<DualNum> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(right_dual[scalar]) * self.group2(),
        );
    }
}
impl BulkExpansion<Flector> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       16        0
    //  no simd       11       27        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * right_dual.group1().www()).with_w(right_dual[e1] * self[e235]),
            // e1234
            -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<Line> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435])
                - (right_dual[e15] * self[e423])
                - (right_dual[e25] * self[e431])
                - (right_dual[e35] * self[e412]),
        );
    }
}
impl BulkExpansion<Motor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       26        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], 1.0])
                * right_dual.group0().www().with_w(
                    (right_dual[scalar] * self[e12345])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                ),
        );
    }
}
impl BulkExpansion<MultiVector> for CircleRotor {
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
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
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
            ]),
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
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e235])),
            // e1234
            -(self[e423] * right_dual[e1]) - (self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]) - (self[e321] * right_dual[e4]),
        );
    }
}
impl BulkExpansion<Plane> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       15        0
    //  no simd       11       23        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group0().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group0() * right_dual.group0().www()).with_w(right_dual[e1] * self[e235]),
            // e1234
            -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<Sphere> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       24        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e235])),
            // e1234
            -(self[e423] * right_dual[e1]) - (self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]) - (self[e321] * right_dual[e4]),
        );
    }
}
impl BulkExpansion<VersorEven> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       10       38        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().xyz().with_w(
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
                ),
        );
    }
}
impl BulkExpansion<VersorOdd> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       36        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e321]))
                + (right_dual.group3().yzxx() * self.group1().zxy().with_w(self[e235])),
            // e1234
            -(self[e423] * right_dual[e1]) - (self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]) - (self[e321] * right_dual[e4]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for Dipole {
    type Output = bulk_expansion_partial<Dipole>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       21        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<AntiDipoleInversion> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       42        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]),
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e35] * right_dual[e12]),
            ]) - (self.group0().zxy() * right_dual.group2().yzx()).with_w(self[e15] * right_dual[e23])
                - (self.group2().yzx() * right_dual.group0().zxy()).with_w(self[e25] * right_dual[e31]),
            // e1234
            -(self[e41] * right_dual[e23])
                - (self[e42] * right_dual[e31])
                - (self[e43] * right_dual[e12])
                - (self[e23] * right_dual[e41])
                - (self[e31] * right_dual[e42])
                - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for Dipole {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_dual[e5]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[e5]) * self.group1().xyz(),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]),
            ]) - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiFlector> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]),
            ]) - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiLine> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(
            // e12345
            -(self[e41] * right_dual[e235])
                - (self[e42] * right_dual[e315])
                - (self[e43] * right_dual[e125])
                - (self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435]),
        );
    }
}
impl BulkExpansion<AntiMotor> for Dipole {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       30        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[e5], right_dual[e5], right_dual[e5], 1.0])
                * self.group0().with_w(
                    -(self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiScalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(right_dual[scalar]) * self.group2(),
        );
    }
}
impl BulkExpansion<Circle> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       25       34        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().zxy() * self.group2().yzx()).with_w(right_dual[e15] * self[e23])
                - (right_dual.group2().yzx() * self.group0().zxy()).with_w(right_dual[e25] * self[e31]),
            // e1234
            -(right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<CircleRotor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       25       49        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().with_w(
                    -(right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().zxy() * self.group2().yzx()).with_w(right_dual[e23] * self[e15])
                - (self.group0().zxy() * right_dual.group2().yzx()).with_w(right_dual[e31] * self[e25]),
        );
    }
}
impl BulkExpansion<Dipole> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<DipoleInversion> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       29       55        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e4] * self[e15]) + (right_dual[e5] * self[e41]),
                (right_dual[e4] * self[e25]) + (right_dual[e5] * self[e42]),
                (right_dual[e4] * self[e35]) + (right_dual[e5] * self[e43]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (right_dual[e2] * self[e35]) + (right_dual[e5] * self[e23]),
                (right_dual[e3] * self[e15]) + (right_dual[e5] * self[e31]),
                (right_dual[e1] * self[e25]) + (right_dual[e5] * self[e12]),
                -(right_dual[e431] * self[e25])
                    - (right_dual[e412] * self[e35])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e321] * self[e45])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
            ]) - (self.group2().yzx() * right_dual.group3().zxy()).with_w(right_dual[e423] * self[e15]),
        );
    }
}
impl BulkExpansion<DualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(right_dual[scalar]) * self.group2(),
        );
    }
}
impl BulkExpansion<FlatPoint> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkExpansion<Flector> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       17       36        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e5] * self[e41],
                right_dual[e5] * self[e42],
                right_dual[e5] * self[e43],
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group1().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (right_dual[e2] * self[e35]) + (right_dual[e5] * self[e23]),
                (right_dual[e3] * self[e15]) + (right_dual[e5] * self[e31]),
                (right_dual[e1] * self[e25]) + (right_dual[e5] * self[e12]),
                -(right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
            ]) - (self.group2().yzx() * right_dual.group1().zxy()).with_w(right_dual[e235] * self[e41]),
        );
    }
}
impl BulkExpansion<Line> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e35] * self[e42]),
                (right_dual[e31] * self[e45]) + (right_dual[e15] * self[e43]),
                (right_dual[e12] * self[e45]) + (right_dual[e25] * self[e41]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group1().yzx() * self.group0().zxy()).with_w(right_dual[e23] * self[e15]),
            // e1234
            -(right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<Motor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       37        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().with_w(-(right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e35] * self[e42]),
                (right_dual[e31] * self[e45]) + (right_dual[e15] * self[e43]),
                (right_dual[e12] * self[e45]) + (right_dual[e25] * self[e41]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (self.group0().zxy() * right_dual.group1().yzx()).with_w(right_dual[e23] * self[e15]),
        );
    }
}
impl BulkExpansion<MultiVector> for Dipole {
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
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group2().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[scalar]) * self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * right_dual[e5]) + (self[e15] * right_dual[e4]),
                (self[e42] * right_dual[e5]) + (self[e25] * right_dual[e4]),
                (self[e43] * right_dual[e5]) + (self[e35] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (self.group2().zxy() * right_dual.group1().yzx()) - (self.group2().yzx() * right_dual.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]),
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e35] * right_dual[e12]),
            ]) - (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e15] * right_dual[e23])
                - (self.group2().yzx() * right_dual.group4().zxy()).with_w(self[e25] * right_dual[e31]),
            // e1234
            -(self[e41] * right_dual[e23])
                - (self[e42] * right_dual[e31])
                - (self[e43] * right_dual[e12])
                - (self[e23] * right_dual[e41])
                - (self[e31] * right_dual[e42])
                - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<Plane> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       14       28        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Circle::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e5] * self[e41],
                right_dual[e5] * self[e42],
                right_dual[e5] * self[e43],
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (self.group2().zxy() * right_dual.group0().yzx()) - (self.group2().yzx() * right_dual.group0().zxy()),
        );
    }
}
impl BulkExpansion<Sphere> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       20       34        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * right_dual[e5]) + (self[e15] * right_dual[e4]),
                (self[e42] * right_dual[e5]) + (self[e25] * right_dual[e4]),
                (self[e43] * right_dual[e5]) + (self[e35] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (self.group2().zxy() * right_dual.group0().yzx()) - (self.group2().yzx() * right_dual.group0().zxy()),
        );
    }
}
impl BulkExpansion<VersorEven> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       25       57        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().with_w(
                    -(self[e41] * right_dual[e23])
                        - (self[e42] * right_dual[e31])
                        - (self[e43] * right_dual[e12])
                        - (self[e23] * right_dual[e41])
                        - (self[e31] * right_dual[e42])
                        - (self[e12] * right_dual[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]),
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e35] * right_dual[e12]),
            ]) - (self.group0().zxy() * right_dual.group2().yzx()).with_w(self[e15] * right_dual[e23])
                - (self.group2().yzx() * right_dual.group0().zxy()).with_w(self[e25] * right_dual[e31]),
        );
    }
}
impl BulkExpansion<VersorOdd> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       29       56        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * right_dual[e5]) + (self[e15] * right_dual[e4]),
                (self[e42] * right_dual[e5]) + (self[e25] * right_dual[e4]),
                (self[e43] * right_dual[e5]) + (self[e35] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self[e23] * right_dual[e5]) + (self[e35] * right_dual[e2]),
                (self[e31] * right_dual[e5]) + (self[e15] * right_dual[e3]),
                (self[e12] * right_dual[e5]) + (self[e25] * right_dual[e1]),
                -(self[e42] * right_dual[e315])
                    - (self[e43] * right_dual[e125])
                    - (self[e23] * right_dual[e415])
                    - (self[e31] * right_dual[e425])
                    - (self[e12] * right_dual[e435])
                    - (self[e45] * right_dual[e321])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412]),
            ]) - (self.group2().yzx() * right_dual.group3().zxy()).with_w(self[e41] * right_dual[e235]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for DipoleInversion {
    type Output = bulk_expansion_partial<DipoleInversion>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       21        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<AntiDipoleInversion> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       42        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (self.group2().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23]))
                - (self.group0().zxy() * right_dual.group2().yzx()).with_w(right_dual[e31] * self[e25]),
            // e1234
            -(right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_dual[e5]) * self.group0().with_w(self[e1234]),
            // e235, e315, e125, e5
            right_dual.group0().xx().with_zw(right_dual[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]),
            ]) - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiFlector> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]),
                -(self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]),
            ]) - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiLine> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(
            // e12345
            -(self[e41] * right_dual[e235])
                - (self[e42] * right_dual[e315])
                - (self[e43] * right_dual[e125])
                - (self[e23] * right_dual[e415])
                - (self[e31] * right_dual[e425])
                - (self[e12] * right_dual[e435]),
        );
    }
}
impl BulkExpansion<AntiMotor> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        6       31        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[e5], right_dual[e5], right_dual[e5], 1.0])
                * self.group0().with_w(
                    (self[e1234] * right_dual[e5])
                        - (self[e41] * right_dual[e235])
                        - (self[e42] * right_dual[e315])
                        - (self[e43] * right_dual[e125])
                        - (self[e23] * right_dual[e415])
                        - (self[e31] * right_dual[e425])
                        - (self[e12] * right_dual[e435]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiScalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<Circle> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       25       34        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().zxy() * self.group2().yzx()).with_w(right_dual[e15] * self[e23])
                - (right_dual.group2().yzx() * self.group0().zxy()).with_w(right_dual[e25] * self[e31]),
            // e1234
            -(right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<CircleRotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        3        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       30       54        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 1.0])
                * right_dual.group2().www().with_w(
                    (right_dual[scalar] * self[e1234])
                        - (right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]) + (right_dual[scalar] * self[e4235]),
                (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]) + (right_dual[scalar] * self[e4315]),
                (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]) + (right_dual[scalar] * self[e4125]),
                -(right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[scalar] * self[e3215])
                - (self.group2().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23]))
                - (self.group0().zxy() * right_dual.group2().yzx()).with_w(right_dual[e31] * self[e25]),
        );
    }
}
impl BulkExpansion<Dipole> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd        9       20        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(
            // e12345
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
impl BulkExpansion<DipoleInversion> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        5        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       60        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e4] * self[e15]) + (right_dual[e5] * self[e41]),
                (right_dual[e4] * self[e25]) + (right_dual[e5] * self[e42]),
                (right_dual[e4] * self[e35]) + (right_dual[e5] * self[e43]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            (right_dual.group3().wwwx() * self.group1().xyz().with_w(self[e4235]))
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
                + (right_dual.group3().yzx() * self.group2().zxy()).with_w(right_dual[e4] * self[e3215])
                - (self.group2().yzxx() * right_dual.group3().zxy().with_w(right_dual[e423])),
        );
    }
}
impl BulkExpansion<DualNum> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       17        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<FlatPoint> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkExpansion<Flector> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       24       40        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e5] * self[e41],
                right_dual[e5] * self[e42],
                right_dual[e5] * self[e43],
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group1().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            (right_dual.group1().yzxx() * self.group2().zxy().with_w(self[e4235]))
                + (right_dual.group1().wwwy() * self.group1().xyz().with_w(self[e4315]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43])
                        - (right_dual[e321] * self[e45]),
                )
                - (right_dual.group1().zxy() * self.group2().yzx()).with_w(right_dual[e235] * self[e41]),
        );
    }
}
impl BulkExpansion<Line> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e35] * self[e42]),
                (right_dual[e31] * self[e45]) + (right_dual[e15] * self[e43]),
                (right_dual[e12] * self[e45]) + (right_dual[e25] * self[e41]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group1().yzx() * self.group0().zxy()).with_w(right_dual[e23] * self[e15]),
            // e1234
            -(right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<Motor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd3        0        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       18       42        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 1.0])
                * right_dual
                    .group0()
                    .www()
                    .with_w((right_dual[scalar] * self[e1234]) - (right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[scalar] * self[e4235]),
                (right_dual[e31] * self[e45]) + (right_dual[scalar] * self[e4315]),
                (right_dual[e12] * self[e45]) + (right_dual[scalar] * self[e4125]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (self.group0().yzx() * right_dual.group1().zxy()).with_w(right_dual[scalar] * self[e3215])
                - (self.group0().zxy() * right_dual.group1().yzx()).with_w(right_dual[e23] * self[e15]),
        );
    }
}
impl BulkExpansion<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd2        0        1        0
    //    simd3        4       11        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       44       68        0
    //  no simd       64      112        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[scalar]) * self.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * right_dual[e5]) + (self[e15] * right_dual[e4]),
                (self[e42] * right_dual[e5]) + (self[e25] * right_dual[e4]),
                (self[e43] * right_dual[e5]) + (self[e35] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (self.group2().zxy() * right_dual.group1().yzx()) - (self.group2().yzx() * right_dual.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e42] * right_dual[e35]) + (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]),
                (self[e43] * right_dual[e15]) + (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]),
                (self[e41] * right_dual[e25]) + (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]),
                -(self[e23] * right_dual[e15]) - (self[e31] * right_dual[e25]) - (self[e12] * right_dual[e35]) - (self[e35] * right_dual[e12]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group3())
                - (self.group2().yzxy() * right_dual.group4().zxy().with_w(right_dual[e31]))
                - (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e15] * right_dual[e23]),
            // e1234
            (self[e1234] * right_dual[scalar])
                - (self[e41] * right_dual[e23])
                - (self[e42] * right_dual[e31])
                - (self[e43] * right_dual[e12])
                - (self[e23] * right_dual[e41])
                - (self[e31] * right_dual[e42])
                - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<Plane> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       19        0
    //  no simd       17       35        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e5] * self[e41],
                right_dual[e5] * self[e42],
                right_dual[e5] * self[e43],
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                right_dual[e3] * self[e25] * -1.0,
                right_dual[e1] * self[e35] * -1.0,
                right_dual[e2] * self[e15] * -1.0,
                (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
            ]) + (right_dual.group0().yzxx() * self.group2().zxy().with_w(self[e4235]))
                + (right_dual.group0().wwwy() * self.group1().xyz().with_w(self[e4315])),
        );
    }
}
impl BulkExpansion<Sphere> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       24       42        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group0().zxy()) - (self.group0().zxy() * right_dual.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * right_dual[e5]) + (self[e15] * right_dual[e4]),
                (self[e42] * right_dual[e5]) + (self[e25] * right_dual[e4]),
                (self[e43] * right_dual[e5]) + (self[e35] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group0().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self[e25] * right_dual[e3] * -1.0,
                self[e35] * right_dual[e1] * -1.0,
                self[e15] * right_dual[e2] * -1.0,
                (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
            ]) + (Simd32x4::from(right_dual[e5]) * self.group1().xyz().with_w(self[e1234]))
                + (right_dual.group0().yzxx() * self.group2().zxy().with_w(self[e4235])),
        );
    }
}
impl BulkExpansion<VersorEven> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        3        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       21       35        0
    //  no simd       30       62        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group2().xyz().with_w(
                    (self[e1234] * right_dual[scalar])
                        - (self[e41] * right_dual[e23])
                        - (self[e42] * right_dual[e31])
                        - (self[e43] * right_dual[e12])
                        - (self[e23] * right_dual[e41])
                        - (self[e31] * right_dual[e42])
                        - (self[e12] * right_dual[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e23] * right_dual[e45]) + (self[e45] * right_dual[e23]) + (self[e35] * right_dual[e42]) + (self[e4235] * right_dual[scalar]),
                (self[e31] * right_dual[e45]) + (self[e45] * right_dual[e31]) + (self[e15] * right_dual[e43]) + (self[e4315] * right_dual[scalar]),
                (self[e12] * right_dual[e45]) + (self[e45] * right_dual[e12]) + (self[e25] * right_dual[e41]) + (self[e4125] * right_dual[scalar]),
                -(self[e12] * right_dual[e35]) - (self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (self.group0().yzx() * right_dual.group2().zxy()).with_w(self[e3215] * right_dual[scalar])
                - (right_dual.group2().yzxx() * self.group0().zxy().with_w(self[e23]))
                - (self.group2().yzx() * right_dual.group0().zxy()).with_w(self[e31] * right_dual[e25]),
        );
    }
}
impl BulkExpansion<VersorOdd> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       37       61        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (self.group0().yzx() * right_dual.group3().zxy()) - (self.group0().zxy() * right_dual.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e41] * right_dual[e5]) + (self[e15] * right_dual[e4]),
                (self[e42] * right_dual[e5]) + (self[e25] * right_dual[e4]),
                (self[e43] * right_dual[e5]) + (self[e35] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (self.group1().wwwx() * right_dual.group3().xyzx()),
            // e235, e315, e125, e12345
            (Simd32x4::from(right_dual[e5]) * self.group1().xyz().with_w(self[e1234]))
                + (right_dual.group3().yzxx() * self.group2().zxy().with_w(self[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4])
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
                - (self.group2().yzx() * right_dual.group3().zxy()).with_w(self[e41] * right_dual[e235]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for DualNum {
    type Output = bulk_expansion_partial<DualNum>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        7        0
    // no simd        0       28        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e1234]),
            // e235, e315, e125, e5
            self.group0().xx().with_zw(self[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<Circle> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e5]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e5]) * right_dual.group1().xyz(),
        );
    }
}
impl BulkExpansion<CircleRotor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xx().with_zw(self[e5], self[e12345]) * right_dual.group0().with_w(right_dual[scalar]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * right_dual.group1().xyz().with_w(right_dual[scalar]),
        );
    }
}
impl BulkExpansion<Dipole> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkExpansion<DipoleInversion> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       31        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * right_dual.group3().xyz().with_w(right_dual[e4]) * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkExpansion<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<FlatPoint> for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([right_dual[e321] * self[e5], 1.0]) * Simd32x2::from([-1.0, 0.0]));
    }
}
impl BulkExpansion<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       22        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0().xx().with_zw(self[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_dual[e321] * self[e5] * -1.0),
        );
    }
}
impl BulkExpansion<Line> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            self.group0().xx().with_zw(self[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(right_dual[scalar] * self[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * right_dual.group0(),
        );
    }
}
impl BulkExpansion<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0       10        0
    // Totals...
    // yes simd        1       19        0
    //  no simd        1       56        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
            Simd32x2::from([0.0, (self[e5] * right_dual[e1234]) + (self[e12345] * right_dual[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5] * right_dual[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * right_dual.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0().xx().with_zw(self[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group4().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e5]) * right_dual.group5(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * right_dual.group7().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<Plane> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            self.group0().xx().with_zw(self[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl BulkExpansion<RoundPoint> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiScalar::from_groups(/* e12345 */ self[e5] * right_dual[e1234]);
    }
}
impl BulkExpansion<Sphere> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[e5]) * right_dual.group0() * Simd32x4::from(-1.0));
    }
}
impl BulkExpansion<VersorEven> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       26        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[e41], right_dual[e42], right_dual[e43], 1.0])
                * self.group0().xx().with_zw(self[e5], (self[e5] * right_dual[e1234]) + (self[e12345] * right_dual[scalar])),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * right_dual.group1().xyz().with_w(right_dual[scalar]),
        );
    }
}
impl BulkExpansion<VersorOdd> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * right_dual.group3() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * right_dual.group0().xyz().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Div<bulk_expansion> for FlatPoint {
    type Output = bulk_expansion_partial<FlatPoint>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       15        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e423] * self[e15]) - (right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<AntiScalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<Circle> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<CircleRotor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<Dipole> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       14        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e423] * self[e15]) - (right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        3        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       31        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                right_dual[e4] * self[e15],
                right_dual[e4] * self[e25],
                right_dual[e4] * self[e35],
                -(right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
            ]) - (self.group0().wwwx() * right_dual.group3().xyz().with_w(right_dual[e423])),
            // e235, e315, e125, e5
            ((right_dual.group3().yzx() * self.group0().zxy()) - (right_dual.group3().zxy() * self.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<DualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiScalar::from_groups(/* e12345 */ right_dual[e321] * self[e45] * -1.0);
    }
}
impl BulkExpansion<Flector> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       22        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e45]) * right_dual.group1().xyz().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            ((right_dual.group1().yzx() * self.group0().zxy()) - (right_dual.group1().zxy() * self.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<Line> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e45], self[e45], self[e45], 1.0])
                * right_dual
                    .group0()
                    .with_w(-(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35])),
        );
    }
}
impl BulkExpansion<Motor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       19        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e45], self[e45], self[e45], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35])),
        );
    }
}
impl BulkExpansion<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd2        0        1        0
    //    simd3        2        6        0
    //    simd4        1        5        0
    // Totals...
    // yes simd       10       26        0
    //  no simd       17       54        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                -(self[e15] * right_dual[e423]) - (self[e25] * right_dual[e431]) - (self[e35] * right_dual[e412]) - (self[e45] * right_dual[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual[e4]) * self.group0().xyz()) - (Simd32x3::from(self[e45]) * right_dual.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (self.group0().zxy() * right_dual.group1().yzx()) - (self.group0().yzx() * right_dual.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * right_dual[e42]) + (self[e45] * right_dual[e23]),
                (self[e15] * right_dual[e43]) + (self[e45] * right_dual[e31]),
                (self[e25] * right_dual[e41]) + (self[e45] * right_dual[e12]),
                -(self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group0().yzxx() * right_dual.group4().zxy().with_w(right_dual[e23])),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<Plane> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       16        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e45]) * right_dual.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
        );
    }
}
impl BulkExpansion<Sphere> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(right_dual[e4]) * self.group0().xyz()) - (Simd32x3::from(self[e45]) * right_dual.group0().xyz()),
            // e235, e315, e125
            (self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy()),
        );
    }
}
impl BulkExpansion<VersorEven> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        6        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        8       32        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * right_dual[e42]) + (self[e45] * right_dual[e23]),
                (self[e15] * right_dual[e43]) + (self[e45] * right_dual[e31]),
                (self[e25] * right_dual[e41]) + (self[e45] * right_dual[e12]),
                -(self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<VersorOdd> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       32        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[e15] * right_dual[e4],
                self[e25] * right_dual[e4],
                self[e35] * right_dual[e4],
                -(self[e25] * right_dual[e431]) - (self[e35] * right_dual[e412]) - (self[e45] * right_dual[e321]),
            ]) - (self.group0().wwwx() * right_dual.group3().xyz().with_w(right_dual[e423])),
            // e235, e315, e125, e5
            ((self.group0().zxy() * right_dual.group3().yzx()) - (self.group0().yzx() * right_dual.group3().zxy())).with_w(0.0),
        );
    }
}
impl std::ops::Div<bulk_expansion> for Flector {
    type Output = bulk_expansion_partial<Flector>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       15        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e423] * self[e15]) - (right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Circle> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<CircleRotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[scalar] * self[e4235]),
                (right_dual[e31] * self[e45]) + (right_dual[scalar] * self[e4315]),
                (right_dual[e12] * self[e45]) + (right_dual[scalar] * self[e4125]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]),
            ]) + (right_dual.group0().yzx() * self.group0().zxy()).with_w(right_dual[scalar] * self[e3215])
                - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<Dipole> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       14        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e423] * self[e15]) - (right_dual[e431] * self[e25]) - (right_dual[e412] * self[e35]) - (right_dual[e321] * self[e45]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        3        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       16       35        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(right_dual[e4]) * self.group0().xyz().with_w(self[e3215]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125])
                        - (right_dual[e431] * self[e25])
                        - (right_dual[e412] * self[e35])
                        - (right_dual[e321] * self[e45]),
                )
                - (self.group0().wwwx() * right_dual.group3().xyz().with_w(right_dual[e423])),
            // e235, e315, e125, e5
            ((right_dual.group3().yzx() * self.group0().zxy()) - (right_dual.group3().zxy() * self.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<FlatPoint> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiScalar::from_groups(/* e12345 */ right_dual[e321] * self[e45] * -1.0);
    }
}
impl BulkExpansion<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       26        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e45], self[e45], self[e45], 1.0])
                * right_dual
                    .group1()
                    .xyz()
                    .with_w((right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) - (right_dual[e321] * self[e45]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            ((right_dual.group1().yzx() * self.group0().zxy()) - (right_dual.group1().zxy() * self.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e45], self[e45], self[e45], 1.0])
                * right_dual
                    .group0()
                    .with_w(-(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35])),
        );
    }
}
impl BulkExpansion<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       22        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[scalar] * self[e4235],
                right_dual[scalar] * self[e4315],
                right_dual[scalar] * self[e4125],
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]),
            ]) + (right_dual.group0() * self.group0().www().with_w(self[e3215])),
        );
    }
}
impl BulkExpansion<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd2        0        1        0
    //    simd3        2        6        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       15       31        0
    //  no simd       25       62        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4])
                    - (self[e15] * right_dual[e423])
                    - (self[e25] * right_dual[e431])
                    - (self[e35] * right_dual[e412])
                    - (self[e45] * right_dual[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual[e4]) * self.group0().xyz()) - (Simd32x3::from(self[e45]) * right_dual.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (self.group0().zxy() * right_dual.group1().yzx()) - (self.group0().yzx() * right_dual.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * right_dual[e42]) + (self[e45] * right_dual[e23]),
                (self[e15] * right_dual[e43]) + (self[e45] * right_dual[e31]),
                (self[e25] * right_dual[e41]) + (self[e45] * right_dual[e12]),
                -(self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group1())
                - (self.group0().yzxx() * right_dual.group4().zxy().with_w(right_dual[e23])),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        5       21        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e45], self[e45], self[e45], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w((right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            ((right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx())).with_w(0.0),
        );
    }
}
impl BulkExpansion<Sphere> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        9       23        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self[e45] * right_dual[e1] * -1.0,
                self[e45] * right_dual[e2] * -1.0,
                self[e45] * right_dual[e3] * -1.0,
                (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
            ]) + (right_dual.group0().wwwx() * self.group0().xyz().with_w(self[e4235])),
            // e235, e315, e125, e5
            ((self.group0().zxy() * right_dual.group0().yzx()) - (self.group0().yzx() * right_dual.group0().zxy())).with_w(0.0),
        );
    }
}
impl BulkExpansion<VersorEven> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       36        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e45] * right_dual[e23]) + (self[e4235] * right_dual[scalar]),
                (self[e45] * right_dual[e31]) + (self[e4315] * right_dual[scalar]),
                (self[e45] * right_dual[e12]) + (self[e4125] * right_dual[scalar]),
                -(self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]),
            ]) + (right_dual.group0().yzxw() * self.group0().zxy().with_w(self[e3215]))
                - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23])),
        );
    }
}
impl BulkExpansion<VersorOdd> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       16       36        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (right_dual.group3().wwwx() * self.group0().xyz().with_w(self[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4])
                        - (self[e25] * right_dual[e431])
                        - (self[e35] * right_dual[e412])
                        - (self[e45] * right_dual[e321]),
                )
                - (self.group0().wwwx() * right_dual.group3().xyz().with_w(right_dual[e423])),
            // e235, e315, e125, e5
            ((self.group0().zxy() * right_dual.group3().yzx()) - (self.group0().yzx() * right_dual.group3().zxy())).with_w(0.0),
        );
    }
}
impl std::ops::Div<bulk_expansion> for Line {
    type Output = bulk_expansion_partial<Line>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiDipoleInversion> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       18        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e41] * self[e235])
                - (right_dual[e42] * self[e315])
                - (right_dual[e43] * self[e125])
                - (right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435]),
        );
    }
}
impl BulkExpansion<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Circle> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd        5       10        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return AntiScalar::from_groups(
            // e12345
            -(right_dual[e41] * self[e235])
                - (right_dual[e42] * self[e315])
                - (right_dual[e43] * self[e125])
                - (right_dual[e23] * self[e415])
                - (right_dual[e31] * self[e425])
                - (right_dual[e12] * self[e435]),
        );
    }
}
impl BulkExpansion<CircleRotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       30        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group0().with_w(
                    -(right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * right_dual.group2().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       27        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(right_dual[e4] * self[e235]) - (right_dual[e3] * self[e425]),
                -(right_dual[e4] * self[e315]) - (right_dual[e1] * self[e435]),
                -(right_dual[e4] * self[e125]) - (right_dual[e2] * self[e415]),
                (right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]),
            ]) + (right_dual.group3().yzxx() * self.group0().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_dual[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Flector> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       20        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]),
            ]) + (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<Line> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiScalar::from_groups(/* e12345 */ -(right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435]));
    }
}
impl BulkExpansion<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        2       27        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group0().with_w(-(right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        1        7        0
    // Totals...
    // yes simd       10       27        0
    //  no simd       13       55        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                -(self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
            ]),
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
            right_dual.group0().xx().with_zw(right_dual[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e235])),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<Plane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        5       16        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]),
            ]) + (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<Sphere> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<VersorEven> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        5       38        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([right_dual[scalar], right_dual[scalar], right_dual[scalar], 1.0])
                * self.group0().with_w(
                    -(self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12])
                        - (self[e235] * right_dual[e41])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * self.group1().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<VersorOdd> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       28        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (right_dual.group3().yzxx() * self.group0().zxy().with_w(self[e235])),
        );
    }
}
impl std::ops::Div<bulk_expansion> for Motor {
    type Output = bulk_expansion_partial<Motor>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       35        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e5], self[e5], self[e5], 1.0])
                * right_dual.group0().with_w(
                    (right_dual[e1234] * self[e5])
                        - (right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<Circle> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       26        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e5], self[e5], self[e5], 1.0])
                * right_dual.group0().with_w(
                    -(right_dual[e41] * self[e235])
                        - (right_dual[e42] * self[e315])
                        - (right_dual[e43] * self[e125])
                        - (right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<CircleRotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                right_dual[scalar] * self[e415],
                right_dual[scalar] * self[e425],
                right_dual[scalar] * self[e435],
                -(right_dual[e41] * self[e235])
                    - (right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435]),
            ]) + (right_dual.group0() * self.group1().www()).with_w(right_dual[scalar] * self[e12345]),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * right_dual.group1().xyz())).with_w(right_dual[scalar] * self[e5]),
        );
    }
}
impl BulkExpansion<Dipole> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       18        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl BulkExpansion<DipoleInversion> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       12       39        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * right_dual.group3().xyz().with_w(right_dual[e4]) * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(right_dual[e4] * self[e235]) - (right_dual[e3] * self[e425]),
                -(right_dual[e4] * self[e315]) - (right_dual[e1] * self[e435]),
                -(right_dual[e4] * self[e125]) - (right_dual[e2] * self[e415]),
                (right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]),
            ]) + (right_dual.group3().yzxx() * self.group0().zxy().with_w(self[e235]))
                - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321])),
        );
    }
}
impl BulkExpansion<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[scalar]) * self.group1(),
        );
    }
}
impl BulkExpansion<FlatPoint> for Motor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([right_dual[e321] * self[e5], 1.0]) * Simd32x2::from([-1.0, 0.0]));
    }
}
impl BulkExpansion<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        1        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        9       30        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e235])) + Simd32x3::from(0.0).with_w((right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]))
                - (right_dual.group1().zxy() * self.group0().yzx()).with_w(right_dual[e321] * self[e5]),
        );
    }
}
impl BulkExpansion<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(-(right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group1().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       23        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e415], self[e425], self[e435], 1.0])
                * right_dual
                    .group0()
                    .www()
                    .with_w((right_dual[scalar] * self[e12345]) - (right_dual[e23] * self[e415]) - (right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435])),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * right_dual.group0().xyz())).with_w(right_dual[scalar] * self[e5]),
        );
    }
}
impl BulkExpansion<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd2        0        1        0
    //    simd3        2        7        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       15       34        0
    //  no simd       25       67        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                (self[e12345] * right_dual[scalar]) + (self[e5] * right_dual[e1234])
                    - (self[e415] * right_dual[e23])
                    - (self[e425] * right_dual[e31])
                    - (self[e435] * right_dual[e12])
                    - (self[e235] * right_dual[e41])
                    - (self[e315] * right_dual[e42])
                    - (self[e125] * right_dual[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5] * right_dual[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * right_dual.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e5]) * right_dual.group4()) + (Simd32x3::from(right_dual[scalar]) * self.group0().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group5()) + (Simd32x3::from(right_dual[scalar]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e235]))
                - (right_dual.group7() * self.group1().www()).with_w(self[e5] * right_dual[e321]),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        2       13        0
    //  no simd        5       28        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e2] * self[e315]) + (right_dual[e3] * self[e125]),
            ]) + (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<RoundPoint> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiScalar::from_groups(/* e12345 */ self[e5] * right_dual[e1234]);
    }
}
impl BulkExpansion<Sphere> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * right_dual.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<VersorEven> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       16       37        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[e5]) * right_dual.group0().xyz().with_w(right_dual[e1234]))
                + (Simd32x4::from(right_dual[scalar]) * self.group0())
                + Simd32x3::from(0.0).with_w(
                    -(self[e415] * right_dual[e23])
                        - (self[e425] * right_dual[e31])
                        - (self[e435] * right_dual[e12])
                        - (self[e235] * right_dual[e41])
                        - (self[e315] * right_dual[e42])
                        - (self[e125] * right_dual[e43]),
                ),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group1().xyz())).with_w(self[e5] * right_dual[scalar]),
        );
    }
}
impl BulkExpansion<VersorOdd> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        6       17        0
    //  no simd       12       40        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * right_dual.group3() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e235] * right_dual[e4]) - (self[e5] * right_dual[e423]),
                -(self[e315] * right_dual[e4]) - (self[e5] * right_dual[e431]),
                -(self[e125] * right_dual[e4]) - (self[e5] * right_dual[e412]),
                (self[e315] * right_dual[e2]) + (self[e125] * right_dual[e3]),
            ]) + (right_dual.group3().yzxx() * self.group0().zxy().with_w(self[e235]))
                - (self.group0().yzx() * right_dual.group3().zxy()).with_w(self[e5] * right_dual[e321]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for MultiVector {
    type Output = bulk_expansion_partial<MultiVector>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        3        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       25       52        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
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
                0.0,
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
            ]),
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
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235])),
            // e1234
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       47        0
    //    simd3        4        9        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       44       63        0
    //  no simd       64      102        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group2().xyz().with_w(right_dual[e45]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * right_dual.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e41] * self[e5]) + (right_dual[e15] * self[e4]),
                (right_dual[e42] * self[e5]) + (right_dual[e25] * self[e4]),
                (right_dual[e43] * self[e5]) + (right_dual[e35] * self[e4]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group1().zxy()) - (right_dual.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (right_dual.group2().zxy() * self.group1().yzx()) - (right_dual.group2().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e35] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group3())
                - (right_dual.group2().yzxy() * self.group4().zxy().with_w(self[e31]))
                - (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e15] * self[e23]),
            // e1234
            (right_dual[e1234] * self[scalar])
                - (right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        1       26        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (right_dual[e5] * self[e1234]) + (right_dual[e12345] * self[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            right_dual[e5] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e5]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            right_dual.group0().xx().with_zw(right_dual[e5], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group4().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_dual[e5]) * self.group5(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e5]) * self.group7().with_w(self[e321]),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        2        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       17       36        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(right_dual[e15] * self[e423]) - (right_dual[e25] * self[e431]) - (right_dual[e35] * self[e412]) - (right_dual[e45] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e45]) * self.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (right_dual.group0().zxy() * self.group1().yzx()) - (right_dual.group0().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e35] * self[e42]) + (right_dual[e45] * self[e23]),
                (right_dual[e15] * self[e43]) + (right_dual[e45] * self[e31]),
                (right_dual[e25] * self[e41]) + (right_dual[e45] * self[e12]),
                -(right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().yzxx() * self.group4().zxy().with_w(self[e23])),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd3        2        4        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       25       48        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
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
                0.0,
                (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412])
                    - (right_dual[e45] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e45]) * self.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (right_dual.group0().zxy() * self.group1().yzx()) - (right_dual.group0().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e35] * self[e42]) + (right_dual[e45] * self[e23]),
                (right_dual[e15] * self[e43]) + (right_dual[e45] * self[e31]),
                (right_dual[e25] * self[e41]) + (right_dual[e45] * self[e12]),
                -(right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1())
                - (right_dual.group0().yzxx() * self.group4().zxy().with_w(self[e23])),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        3        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       39        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
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
                0.0,
                -(right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
            ]),
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
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (self.group1().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        2        5        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       25       49        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
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
                0.0,
                (right_dual[e12345] * self[scalar]) + (right_dual[e5] * self[e1234])
                    - (right_dual[e415] * self[e23])
                    - (right_dual[e425] * self[e31])
                    - (right_dual[e435] * self[e12])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            right_dual[e5] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e5]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_dual[e5]) * self.group4()) + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group5()) + (Simd32x3::from(self[scalar]) * right_dual.group1().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) + (self.group7() * right_dual.group1().www()).with_w(right_dual[e5] * self[e321])
                - (self.group1().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]),
            ]),
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
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       33        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(right_dual[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e5
            self[e5] * right_dual[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(right_dual[scalar]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group9(),
            // e1234
            self[e1234] * right_dual[scalar],
        );
    }
}
impl BulkExpansion<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       42        0
    //    simd3        4       10        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       55        0
    //  no simd       54       84        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group2().with_w(right_dual[e45]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * right_dual.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e41] * self[e5]) + (right_dual[e15] * self[e4]),
                (right_dual[e42] * self[e5]) + (right_dual[e25] * self[e4]),
                (right_dual[e43] * self[e5]) + (right_dual[e35] * self[e4]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group1().zxy()) - (right_dual.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (right_dual.group2().zxy() * self.group1().yzx()) - (right_dual.group2().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e15] * self[e23])
                - (right_dual.group2().yzx() * self.group4().zxy()).with_w(right_dual[e25] * self[e31]),
            // e1234
            -(right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       46        0
    //    simd3        8       14        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       46       68        0
    //  no simd       80      120        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
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
                right_dual[scalar] * self[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e5
            right_dual[scalar] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[scalar]) * self.group3()) + (Simd32x4::from(self[scalar]) * right_dual.group2().xyz().with_w(right_dual[e45])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[scalar]) * self.group4()) + (Simd32x3::from(self[scalar]) * right_dual.group0()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[scalar]) * self.group5()) + (Simd32x3::from(self[scalar]) * right_dual.group1().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e15] * self[e4]) + (right_dual[scalar] * self[e415]),
                (right_dual[e25] * self[e4]) + (right_dual[scalar] * self[e425]),
                (right_dual[e35] * self[e4]) + (right_dual[scalar] * self[e435]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) + (Simd32x4::from([self[e5], self[e5], self[e5], self[e321]]) * right_dual.group0().with_w(right_dual[scalar]))
                - (right_dual.group1().wwwx() * self.group1().xyzx()),
            // e423, e431, e412
            (Simd32x3::from(right_dual[scalar]) * self.group7()) + (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group1().zxy())
                - (right_dual.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[scalar]) * self.group8()) + (Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (right_dual.group2().zxy() * self.group1().yzx())
                - (right_dual.group2().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]) + (right_dual[scalar] * self[e4235]),
                (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]) + (right_dual[scalar] * self[e4315]),
                (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]) + (right_dual[scalar] * self[e4125]),
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e35] * self[e12]),
            ]) + (right_dual.group0().yzx() * self.group3().zxy()).with_w(right_dual[scalar] * self[e3215])
                - (right_dual.group2().yzxy() * self.group4().zxy().with_w(self[e31]))
                - (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e15] * self[e23]),
            // e1234
            (right_dual[scalar] * self[e1234])
                - (right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       24       50        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
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
                0.0,
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
            ]),
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
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e321] * self[e5]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from([self[e5], self[e5], self[e5], self[e1]]) * right_dual.group0().with_w(right_dual[e235]))
                - (self.group1().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315])),
            // e1234
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        8       18        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       43       66        0
    //  no simd       89      135        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3().xyz().with_w(right_dual[e4]),
            // e5
            right_dual[e5] * self[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[e5]) * self.group1()) - (Simd32x4::from(self[e5]) * right_dual.group3().xyz().with_w(right_dual[e4])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (right_dual.group3().zxy() * self.group1().yzx()) - (right_dual.group3().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e4] * self[e15]) + (right_dual[e5] * self[e41]),
                (right_dual[e4] * self[e25]) + (right_dual[e5] * self[e42]),
                (right_dual[e4] * self[e35]) + (right_dual[e5] * self[e43]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1())
                - (self.group3().www() * right_dual.group3().xyz()).with_w(right_dual[e1] * self[e23]),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group5()) + (Simd32x3::from(self[scalar]) * right_dual.group0()) + (self.group4().yzx() * right_dual.group3().zxy())
                - (self.group4().zxy() * right_dual.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group5()) + (Simd32x3::from(self[scalar]) * right_dual.group2().xyz()) + (right_dual.group3().yzx() * self.group3().zxy())
                - (right_dual.group3().zxy() * self.group3().yzx()),
            // e4235, e4315, e4125, e3215
            (right_dual.group3().yzxw() * self.group6().zxyw())
                + (right_dual.group3().wwwx() * self.group7().with_w(self[e235]))
                + (right_dual.group1().yzx() * self.group1().zxy()).with_w(right_dual[e2] * self[e315])
                + (self.group1().www() * right_dual.group2().xyz()).with_w(right_dual[e3] * self[e125])
                - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (self.group1().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315]))
                - (self.group8() * right_dual.group2().www()).with_w(right_dual[e235] * self[e1])
                - (right_dual.group3().zxy() * self.group6().yzx()).with_w(right_dual[e125] * self[e3]),
            // e1234
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                - (right_dual[e4] * self[e321])
                - (right_dual[e1] * self[e423])
                - (right_dual[e2] * self[e431])
                - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       16        0
    //  no simd        2       37        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([right_dual[scalar] * self[scalar], (right_dual[e3215] * self[e4]) + (right_dual[scalar] * self[e12345])]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e5
            right_dual[scalar] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[scalar]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(right_dual[scalar]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(right_dual[scalar]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(right_dual[scalar]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 1.0])
                * right_dual
                    .group0()
                    .yy()
                    .with_zw(right_dual[scalar], (right_dual[e3215] * self[scalar]) + (right_dual[scalar] * self[e3215])),
            // e1234
            right_dual[scalar] * self[e1234],
        );
    }
}
impl BulkExpansion<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        6       21        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45]),
            ]),
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
            Simd32x3::from(0.0).with_w(right_dual[e321] * self[scalar]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group0().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e235] * self[e1]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5])),
            // e1234
            right_dual[e321] * self[e4],
        );
    }
}
impl BulkExpansion<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd3        6       16        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       47        0
    //  no simd       56       97        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
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
                0.0,
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                    - (right_dual[e235] * self[e41])
                    - (right_dual[e315] * self[e42])
                    - (right_dual[e125] * self[e43])
                    - (right_dual[e321] * self[e45]),
            ]),
            // e1, e2, e3, e4
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e5
            right_dual[e5] * self[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group1().xyz())).with_w(right_dual[e5] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group1().xyz(),
            // e23, e31, e12
            (right_dual.group1().zxy() * self.group1().yzx()) - (right_dual.group1().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(-(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]))
                + (self.group4() * right_dual.group1().www()).with_w(right_dual[e321] * self[scalar])
                - (self.group3().www() * right_dual.group1().xyz()).with_w(right_dual[e1] * self[e23]),
            // e423, e431, e412
            (self.group4().yzx() * right_dual.group1().zxy()) - (self.group4().zxy() * right_dual.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group5()) + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz()) + (right_dual.group1().yzx() * self.group3().zxy())
                - (right_dual.group1().zxy() * self.group3().yzx()),
            // e4235, e4315, e4125, e3215
            (right_dual.group1().wwwx() * self.group7().with_w(self[e235]))
                + Simd32x3::from(0.0).with_w((right_dual[e5] * self[e321]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5]))
                + (right_dual.group1().yzx() * self.group6().zxy()).with_w(right_dual[e3] * self[e125])
                + (self.group1().www() * right_dual.group0().xyz()).with_w(right_dual[e2] * self[e315])
                - (right_dual.group1().zxy() * self.group6().yzx()).with_w(right_dual[e235] * self[e1]),
            // e1234
            (right_dual[e321] * self[e4]) - (right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd3        2        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       19       34        0
    //  no simd       26       58        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual.group1().with_w(-(right_dual[e23] * self[e1]) - (right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3])),
            // e423, e431, e412
            Simd32x3::from(self[e4]) * right_dual.group0(),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group0()) + (right_dual.group1().zxy() * self.group1().yzx()) - (right_dual.group1().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e35] * self[e42]),
                (right_dual[e31] * self[e45]) + (right_dual[e15] * self[e43]),
                (right_dual[e12] * self[e45]) + (right_dual[e25] * self[e41]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group1().yzx() * self.group4().zxy()).with_w(right_dual[e23] * self[e15]),
            // e1234
            -(right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        6       11        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       26       47        0
    //  no simd       50       90        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
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
                right_dual[scalar] * self[scalar],
                (right_dual[scalar] * self[e12345]) + (right_dual[e3215] * self[e4])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e5
            right_dual[scalar] * self[e5],
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual[scalar]) * self.group3().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group1().xyz())).with_w(right_dual[scalar] * self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_dual[scalar]) * self.group4(),
            // e23, e31, e12
            (Simd32x3::from(right_dual[scalar]) * self.group5()) + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz()),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e15] * self[e4],
                right_dual[e25] * self[e4],
                right_dual[e35] * self[e4],
                -(right_dual[e23] * self[e1]) - (right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group6()),
            // e423, e431, e412
            (Simd32x3::from(right_dual[scalar]) * self.group7()) + (Simd32x3::from(self[e4]) * right_dual.group0().xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[scalar]) * self.group8()) + (Simd32x3::from(self[e5]) * right_dual.group0().xyz()) + (right_dual.group1().zxy() * self.group1().yzx())
                - (right_dual.group1().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[scalar] * self[e4235],
                right_dual[scalar] * self[e4315],
                right_dual[scalar] * self[e4125],
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (right_dual.group0() * self.group3().www().with_w(self[e3215]))
                + (right_dual.group1().zxyw() * self.group4().yzx().with_w(self[scalar]))
                - (right_dual.group1().yzxx() * self.group4().zxy().with_w(self[e23])),
            // e1234
            (right_dual[scalar] * self[e1234]) - (right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       93        0
    //    simd2        0        1        0
    //    simd3       20       34        0
    //    simd4       20       17        0
    // Totals...
    // yes simd      111      145        0
    //  no simd      211      265        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                right_dual[scalar] * self[scalar],
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
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_dual[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * right_dual.group1()),
            // e5
            (right_dual[scalar] * self[e5]) + (right_dual[e5] * self[scalar]),
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[scalar]) * self.group3()) + (Simd32x4::from(right_dual[e5]) * self.group1()) + (Simd32x4::from(self[scalar]) * right_dual.group3())
                - (Simd32x4::from(self[e5]) * right_dual.group1()),
            // e41, e42, e43
            (Simd32x3::from(right_dual[scalar]) * self.group4()) + (Simd32x3::from(self[scalar]) * right_dual.group4()) + (Simd32x3::from(self[e4]) * right_dual.group1().xyz())
                - (Simd32x3::from(right_dual[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[scalar]) * self.group5()) + (Simd32x3::from(self[scalar]) * right_dual.group5()) + (right_dual.group1().zxy() * self.group1().yzx())
                - (right_dual.group1().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e4] * self[e15]) + (right_dual[e5] * self[e41]) + (right_dual[e15] * self[e4]) + (right_dual[e41] * self[e5]),
                (right_dual[e4] * self[e25]) + (right_dual[e5] * self[e42]) + (right_dual[e25] * self[e4]) + (right_dual[e42] * self[e5]),
                (right_dual[e4] * self[e35]) + (right_dual[e5] * self[e43]) + (right_dual[e35] * self[e4]) + (right_dual[e43] * self[e5]),
                -(right_dual[e1] * self[e23]) - (right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]) - (right_dual[e12] * self[e3]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group6())
                + (Simd32x4::from(self[scalar]) * right_dual.group6())
                - (self.group1().xyzy() * right_dual.group3().www().with_w(right_dual[e31]))
                - (self.group3().www() * right_dual.group1().xyz()).with_w(right_dual[e23] * self[e1]),
            // e423, e431, e412
            (Simd32x3::from(right_dual[scalar]) * self.group7())
                + (Simd32x3::from(right_dual[e4]) * self.group5())
                + (Simd32x3::from(self[scalar]) * right_dual.group7())
                + (Simd32x3::from(self[e4]) * right_dual.group5())
                + (right_dual.group4().yzx() * self.group1().zxy())
                + (self.group4().yzx() * right_dual.group1().zxy())
                - (right_dual.group4().zxy() * self.group1().yzx())
                - (self.group4().zxy() * right_dual.group1().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[scalar]) * self.group8())
                + (Simd32x3::from(right_dual[e5]) * self.group5())
                + (Simd32x3::from(self[scalar]) * right_dual.group8())
                + (Simd32x3::from(self[e5]) * right_dual.group5())
                + (right_dual.group1().yzx() * self.group3().zxy())
                + (right_dual.group3().zxy() * self.group1().yzx())
                - (right_dual.group1().zxy() * self.group3().yzx())
                - (right_dual.group3().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e2] * self[e435]) + (right_dual[e5] * self[e423]) + (right_dual[e45] * self[e23]) + (right_dual[e425] * self[e3]),
                (right_dual[e3] * self[e415]) + (right_dual[e5] * self[e431]) + (right_dual[e45] * self[e31]) + (right_dual[e435] * self[e1]),
                (right_dual[e1] * self[e425]) + (right_dual[e5] * self[e412]) + (right_dual[e45] * self[e12]) + (right_dual[e415] * self[e2]),
                -(right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]) - (right_dual[e321] * self[e5]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group9())
                + (Simd32x4::from(self[scalar]) * right_dual.group9())
                + (right_dual.group5() * self.group3().www()).with_w(right_dual[e2] * self[e315])
                + (right_dual.group8() * self.group1().www()).with_w(right_dual[e3] * self[e125])
                + (right_dual.group4().yzx() * self.group3().zxy()).with_w(right_dual[e1] * self[e235])
                + (self.group4().yzx() * right_dual.group3().zxy()).with_w(right_dual[e5] * self[e321])
                - (Simd32x4::from([self[e5], self[e5], self[e5], self[e25]]) * right_dual.group7().with_w(right_dual[e31]))
                - (self.group1().yzxz() * right_dual.group6().zxy().with_w(right_dual[e125]))
                - (self.group3().yzxx() * right_dual.group4().zxy().with_w(right_dual[e23]))
                - (self.group8() * right_dual.group1().www()).with_w(right_dual[e235] * self[e1])
                - (self.group4().zxy() * right_dual.group3().yzx()).with_w(right_dual[e12] * self[e35])
                - (right_dual.group1().zxy() * self.group6().yzx()).with_w(right_dual[e315] * self[e2]),
            // e1234
            (right_dual[scalar] * self[e1234])
                + (right_dual[e321] * self[e4])
                + (right_dual[e423] * self[e1])
                + (right_dual[e431] * self[e2])
                + (right_dual[e412] * self[e3])
                + (right_dual[e1234] * self[scalar])
                - (right_dual[e1] * self[e423])
                - (right_dual[e2] * self[e431])
                - (right_dual[e3] * self[e412])
                - (right_dual[e4] * self[e321])
                - (right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       24        0
    //    simd3        5       12        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       15       41        0
    //  no simd       34       80        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
            ]),
            // e1, e2, e3, e4
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e5
            right_dual[e5] * self[scalar],
            // e15, e25, e35, e45
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group0().xyz())).with_w(right_dual[e5] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group0().xyz(),
            // e23, e31, e12
            (right_dual.group0().zxy() * self.group1().yzx()) - (right_dual.group0().yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e5] * self[e41],
                right_dual[e5] * self[e42],
                right_dual[e5] * self[e43],
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (self.group3().www() * right_dual.group0().xyz()).with_w(right_dual[e1] * self[e23]),
            // e423, e431, e412
            (self.group4().yzx() * right_dual.group0().zxy()) - (self.group4().zxy() * right_dual.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group5()) + (right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group0().wwwx() * self.group7().with_w(self[e235]))
                + (right_dual.group0().yzx() * self.group6().zxy()).with_w(right_dual[e2] * self[e315]),
            // e1234
            -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
        );
    }
}
impl BulkExpansion<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        4       15        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
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
                0.0,
                (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]) + (self[e5] * right_dual[e1234]),
            ]),
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
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1234
            self[scalar] * right_dual[e1234],
        );
    }
}
impl BulkExpansion<Scalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiScalar::from_groups(/* e12345 */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkExpansion<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd3        6       10        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       25       43        0
    //  no simd       49       84        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
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
                0.0,
                (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]) + (self[e1234] * right_dual[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e5
            self[scalar] * right_dual[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[e5]) * self.group1()) - (Simd32x4::from(self[e5]) * right_dual.group0()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (self.group1().yzx() * right_dual.group0().zxy()) - (self.group1().zxy() * right_dual.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * right_dual[e4]) + (self[e41] * right_dual[e5]),
                (self[e25] * right_dual[e4]) + (self[e42] * right_dual[e5]),
                (self[e35] * right_dual[e4]) + (self[e43] * right_dual[e5]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (right_dual.group0().xyzx() * self.group3().www().with_w(self[e23])),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group5()) + (self.group4().yzx() * right_dual.group0().zxy()) - (self.group4().zxy() * right_dual.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group5()) + (self.group3().zxy() * right_dual.group0().yzx()) - (self.group3().yzx() * right_dual.group0().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self[e425] * right_dual[e3]) - (self[e235] * right_dual[e4]),
                -(self[e435] * right_dual[e1]) - (self[e315] * right_dual[e4]),
                -(self[e415] * right_dual[e2]) - (self[e125] * right_dual[e4]),
                (self[e321] * right_dual[e5]) + (self[e125] * right_dual[e3]),
            ]) + (Simd32x4::from([right_dual[e5], right_dual[e5], right_dual[e5], right_dual[e1]]) * self.group7().with_w(self[e235]))
                + (right_dual.group0().yzxy() * self.group6().zxy().with_w(self[e315])),
            // e1234
            -(self[e321] * right_dual[e4]) - (self[e423] * right_dual[e1]) - (self[e431] * right_dual[e2]) - (self[e412] * right_dual[e3]),
        );
    }
}
impl BulkExpansion<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       53        0
    //    simd3        8       15        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       53       78        0
    //  no simd       90      138        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
                self[scalar] * right_dual[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e5
            self[e5] * right_dual[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(self[scalar]) * right_dual.group2().xyz().with_w(right_dual[e45])) + (Simd32x4::from(right_dual[scalar]) * self.group3()),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group4()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e5] * right_dual[e41]) + (self[e415] * right_dual[scalar]),
                (self[e5] * right_dual[e42]) + (self[e425] * right_dual[scalar]),
                (self[e5] * right_dual[e43]) + (self[e435] * right_dual[scalar]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) + (self.group1().www() * right_dual.group2().xyz()).with_w(self[e321] * right_dual[scalar])
                - (self.group1().xyzx() * right_dual.group1().wwwx()),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group7()) + (self.group1().zxy() * right_dual.group0().yzx())
                - (self.group1().yzx() * right_dual.group0().zxy()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (Simd32x3::from(right_dual[scalar]) * self.group8()) + (self.group1().yzx() * right_dual.group2().zxy())
                - (self.group1().zxy() * right_dual.group2().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e35] * right_dual[e42]) + (self[e45] * right_dual[e23]) + (self[e23] * right_dual[e45]) + (self[e4235] * right_dual[scalar]),
                (self[e15] * right_dual[e43]) + (self[e45] * right_dual[e31]) + (self[e31] * right_dual[e45]) + (self[e4315] * right_dual[scalar]),
                (self[e25] * right_dual[e41]) + (self[e45] * right_dual[e12]) + (self[e12] * right_dual[e45]) + (self[e4125] * right_dual[scalar]),
                -(self[e15] * right_dual[e23]) - (self[e25] * right_dual[e31]) - (self[e35] * right_dual[e12]) - (self[e12] * right_dual[e35]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group3())
                + (self.group4().yzx() * right_dual.group2().zxy()).with_w(self[e3215] * right_dual[scalar])
                - (right_dual.group2().yzxx() * self.group4().zxy().with_w(self[e23]))
                - (self.group3().yzx() * right_dual.group0().zxy()).with_w(self[e31] * right_dual[e25]),
            // e1234
            (self[scalar] * right_dual[e1234]) + (self[e1234] * right_dual[scalar])
                - (self[e41] * right_dual[e23])
                - (self[e42] * right_dual[e31])
                - (self[e43] * right_dual[e12])
                - (self[e23] * right_dual[e41])
                - (self[e31] * right_dual[e42])
                - (self[e12] * right_dual[e43]),
        );
    }
}
impl BulkExpansion<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd3        8       17        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       44       67        0
    //  no simd       90      137        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3(),
            // e5
            self[scalar] * right_dual[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[e5]) * self.group1()) - (Simd32x4::from(self[e5]) * right_dual.group3()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (self.group1().yzx() * right_dual.group3().zxy()) - (self.group1().zxy() * right_dual.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e15] * right_dual[e4]) + (self[e41] * right_dual[e5]),
                (self[e25] * right_dual[e4]) + (self[e42] * right_dual[e5]),
                (self[e35] * right_dual[e4]) + (self[e43] * right_dual[e5]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1())
                - (right_dual.group3().xyzx() * self.group3().www().with_w(self[e23])),
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * right_dual.group0().xyz()) + (Simd32x3::from(right_dual[e4]) * self.group5()) + (self.group4().yzx() * right_dual.group3().zxy())
                - (self.group4().zxy() * right_dual.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * right_dual.group2().xyz()) + (Simd32x3::from(right_dual[e5]) * self.group5()) + (self.group3().zxy() * right_dual.group3().yzx())
                - (self.group3().yzx() * right_dual.group3().zxy()),
            // e4235, e4315, e4125, e3215
            (self.group6().zxyw() * right_dual.group3().yzx().with_w(right_dual[e5]))
                + (self.group7() * right_dual.group2().www()).with_w(self[e235] * right_dual[e1])
                + (self.group1().zxy() * right_dual.group1().yzx()).with_w(self[e315] * right_dual[e2])
                + (self.group1().www() * right_dual.group2().xyz()).with_w(self[e125] * right_dual[e3])
                - (Simd32x4::from(self[e5]) * right_dual.group0().xyz().with_w(right_dual[e321]))
                - (self.group1().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315]))
                - (self.group8() * right_dual.group3().www()).with_w(self[e1] * right_dual[e235])
                - (self.group6().yzx() * right_dual.group3().zxy()).with_w(self[e3] * right_dual[e125]),
            // e1234
            (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321])
                - (self[e321] * right_dual[e4])
                - (self[e423] * right_dual[e1])
                - (self[e431] * right_dual[e2])
                - (self[e412] * right_dual[e3]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for Plane {
    type Output = bulk_expansion_partial<Plane>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<CircleRotor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<DipoleInversion> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       19        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e4] * self[e3215]) + (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]),
        );
    }
}
impl BulkExpansion<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<Flector> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(/* e12345 */ (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]));
    }
}
impl BulkExpansion<Motor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        3       30        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]),
            ]),
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
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1234
            0.0,
        );
    }
}
impl BulkExpansion<Plane> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiScalar::from_groups(/* e12345 */ (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]));
    }
}
impl BulkExpansion<Sphere> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
        );
    }
}
impl BulkExpansion<VersorEven> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(right_dual[scalar]) * self.group0());
    }
}
impl BulkExpansion<VersorOdd> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       20        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for RoundPoint {
    type Output = bulk_expansion_partial<RoundPoint>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       31        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235])),
            // e1234
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for RoundPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       24       50        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e41] * self[e5]) + (right_dual[e15] * self[e4]),
                (right_dual[e42] * self[e5]) + (right_dual[e25] * self[e4]),
                (right_dual[e43] * self[e5]) + (right_dual[e35] * self[e4]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                right_dual[e25] * self[e3] * -1.0,
                right_dual[e35] * self[e1] * -1.0,
                right_dual[e15] * self[e2] * -1.0,
                (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
            ]) + (Simd32x4::from(self[e5]) * right_dual.group1().xyz().with_w(right_dual[e1234]))
                + (self.group0().yzxx() * right_dual.group2().zxy().with_w(right_dual[e4235])),
        );
    }
}
impl BulkExpansion<AntiDualNum> for RoundPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(right_dual[e5]) * self.group0());
    }
}
impl BulkExpansion<AntiFlatPoint> for RoundPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e45]) * self.group0().xyz()),
            // e235, e315, e125
            (right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy()),
        );
    }
}
impl BulkExpansion<AntiFlector> for RoundPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        9       27        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                right_dual[e45] * self[e1] * -1.0,
                right_dual[e45] * self[e2] * -1.0,
                right_dual[e45] * self[e3] * -1.0,
                (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
            ]) + (self.group0().wwwx() * right_dual.group0().xyz().with_w(right_dual[e4235])),
            // e235, e315, e125, e5
            ((right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiLine> for RoundPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiMotor> for RoundPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e5]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (self.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiPlane> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
        );
    }
}
impl BulkExpansion<AntiScalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e5
            self[e5] * right_dual[scalar],
        );
    }
}
impl BulkExpansion<Circle> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       20       34        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e41] * self[e5]) + (right_dual[e15] * self[e4]),
                (right_dual[e42] * self[e5]) + (right_dual[e25] * self[e4]),
                (right_dual[e43] * self[e5]) + (right_dual[e35] * self[e4]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (right_dual.group2().zxy() * self.group0().yzx()) - (right_dual.group2().yzx() * self.group0().zxy()),
        );
    }
}
impl BulkExpansion<CircleRotor> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       20       43        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e41] * self[e5]) + (right_dual[e15] * self[e4]),
                (right_dual[e42] * self[e5]) + (right_dual[e25] * self[e4]),
                (right_dual[e43] * self[e5]) + (right_dual[e35] * self[e4]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group0().xyzx()),
            // e235, e315, e125, e4
            ((Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (right_dual.group2().zxy() * self.group0().yzx()) - (right_dual.group2().yzx() * self.group0().zxy()))
                .with_w(right_dual[scalar] * self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(right_dual[scalar]) * self.group0().xyz().with_w(self[e5]),
        );
    }
}
impl BulkExpansion<Dipole> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       30        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e321] * self[e5]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from([self[e5], self[e5], self[e5], self[e1]]) * right_dual.group0().with_w(right_dual[e235]))
                - (self.group0().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315])),
            // e1234
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        4        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       25       58        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (right_dual.group3().zxyw() * self.group0().yzxw()) - (right_dual.group3().yzx() * self.group0().zxy()).with_w(right_dual[e4] * self[e5]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                right_dual[e1] * self[e5] * -1.0,
                right_dual[e2] * self[e5] * -1.0,
                right_dual[e3] * self[e5] * -1.0,
                (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
            ]) + (self.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<DualNum> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(right_dual[e3215] * self[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(right_dual[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group0(),
        );
    }
}
impl BulkExpansion<FlatPoint> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e235] * self[e1]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5])),
            // e1234
            right_dual[e321] * self[e4],
        );
    }
}
impl BulkExpansion<Flector> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        9       33        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group1().xyz(),
            // e23, e31, e12, e45
            ((right_dual.group1().zxy() * self.group0().yzx()) - (right_dual.group1().yzx() * self.group0().zxy())).with_w(right_dual[e5] * self[e4]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_dual[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group1().xyz())).with_w(right_dual[e321] * self[e4]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e235] * self[e1]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5])),
        );
    }
}
impl BulkExpansion<Line> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        8       19        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual.group1().with_w(-(right_dual[e23] * self[e1]) - (right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3])),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group0()) + (right_dual.group1().zxy() * self.group0().yzx()) - (right_dual.group1().yzx() * self.group0().zxy()),
        );
    }
}
impl BulkExpansion<Motor> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       33        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[e4]) * right_dual.group0().xyz().with_w(right_dual[e3215]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual
                    .group1()
                    .xyz()
                    .with_w(-(right_dual[e23] * self[e1]) - (right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3])),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group0().xyz()) + (right_dual.group1().zxy() * self.group0().yzx()) - (right_dual.group1().yzx() * self.group0().zxy()))
                .with_w(right_dual[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group0(),
        );
    }
}
impl BulkExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        0
    //    simd2        0        1        0
    //    simd3        6       12        0
    //    simd4        4        9        0
    // Totals...
    // yes simd       25       50        0
    //  no simd       49      102        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]) + (right_dual[e1234] * self[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e5
            right_dual[scalar] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[e5]) * self.group0()) - (Simd32x4::from(self[e5]) * right_dual.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group0().xyz()),
            // e23, e31, e12
            (right_dual.group1().zxy() * self.group0().yzx()) - (right_dual.group1().yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e15] * self[e4]) + (right_dual[e41] * self[e5]),
                (right_dual[e25] * self[e4]) + (right_dual[e42] * self[e5]),
                (right_dual[e35] * self[e4]) + (right_dual[e43] * self[e5]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (self.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e23])),
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group5()) + (right_dual.group4().yzx() * self.group0().zxy()) - (right_dual.group4().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * right_dual.group5()) + (right_dual.group3().zxy() * self.group0().yzx()) - (right_dual.group3().yzx() * self.group0().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e321] * self[e5]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from([self[e5], self[e5], self[e5], self[e1]]) * right_dual.group7().with_w(right_dual[e235]))
                - (self.group0().yzxy() * right_dual.group6().zxy().with_w(right_dual[e315])),
            // e1234
            (right_dual[e321] * self[e4]) + (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]),
        );
    }
}
impl BulkExpansion<Plane> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       20        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group0().xyz(),
            // e23, e31, e12, e45
            ((right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy())).with_w(right_dual[e5] * self[e4]),
            // e15, e25, e35
            (Simd32x3::from(right_dual[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkExpansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiScalar::from_groups(
            // e12345
            (self[e1] * right_dual[e4235]) + (self[e2] * right_dual[e4315]) + (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]) + (self[e5] * right_dual[e1234]),
        );
    }
}
impl BulkExpansion<Sphere> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd       10       24        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (right_dual.group0().zxy() * self.group0().yzx()).with_w(right_dual[e5] * self[e4]) - (right_dual.group0().yzxw() * self.group0().zxy().with_w(self[e5])),
            // e15, e25, e35
            (Simd32x3::from(right_dual[e5]) * self.group0().xyz()) - (Simd32x3::from(self[e5]) * right_dual.group0().xyz()),
        );
    }
}
impl BulkExpansion<VersorEven> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        2        3        0
    //    simd4        3        8        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       24       59        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self[e2] * right_dual[e43] * -1.0,
                self[e3] * right_dual[e41] * -1.0,
                self[e1] * right_dual[e42] * -1.0,
                (self[e3] * right_dual[e4125]) + (self[e4] * right_dual[e3215]) + (self[e5] * right_dual[e1234]),
            ]) + (self.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e4235]))
                + (self.group0().wwwy() * right_dual.group1().xyz().with_w(right_dual[e4315])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e4] * right_dual[e15]) + (self[e5] * right_dual[e41]),
                (self[e4] * right_dual[e25]) + (self[e5] * right_dual[e42]),
                (self[e4] * right_dual[e35]) + (self[e5] * right_dual[e43]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) - (self.group0().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group1().xyz()) + (self.group0().yzx() * right_dual.group2().zxy()) - (self.group0().zxy() * right_dual.group2().yzx()))
                .with_w(self[e5] * right_dual[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group0(),
        );
    }
}
impl BulkExpansion<VersorOdd> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        3        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       25       59        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (self.group0().yzxw() * right_dual.group3().zxy().with_w(right_dual[e5])) - (self.group0().zxy() * right_dual.group3().yzx()).with_w(self[e5] * right_dual[e4]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self[e5] * right_dual[e1] * -1.0,
                self[e5] * right_dual[e2] * -1.0,
                self[e5] * right_dual[e3] * -1.0,
                (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321]),
            ]) + (self.group0().xyzx() * right_dual.group2().www().with_w(right_dual[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self[e3] * right_dual[e425]) + (self[e4] * right_dual[e235]),
                (self[e1] * right_dual[e435]) + (self[e4] * right_dual[e315]),
                (self[e2] * right_dual[e415]) + (self[e4] * right_dual[e125]),
                -(self[e3] * right_dual[e125]) - (self[e5] * right_dual[e321]),
            ]) - (Simd32x4::from([self[e5], self[e5], self[e5], right_dual[e315]]) * right_dual.group0().xyz().with_w(self[e2]))
                - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235])),
        );
    }
}
impl std::ops::Div<bulk_expansion> for Scalar {
    type Output = bulk_expansion_partial<Scalar>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for Scalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self[scalar]) * right_dual.group2(),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for Scalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       27        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl BulkExpansion<AntiDualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiFlatPoint> for Scalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiFlector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<AntiLine> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<AntiMotor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<AntiPlane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Scalar::from_groups(/* scalar */ right_dual[scalar] * self[scalar]);
    }
}
impl BulkExpansion<Circle> for Scalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * right_dual.group2(),
        );
    }
}
impl BulkExpansion<CircleRotor> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(self[scalar]) * right_dual.group2(),
        );
    }
}
impl BulkExpansion<Dipole> for Scalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group2(),
        );
    }
}
impl BulkExpansion<DipoleInversion> for Scalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       30        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(self[scalar]) * right_dual.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl BulkExpansion<DualNum> for Scalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<FlatPoint> for Scalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<Flector> for Scalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<Line> for Scalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<Motor> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(self[scalar]) * right_dual.group1(),
        );
    }
}
impl BulkExpansion<MultiVector> for Scalar {
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
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
            Simd32x2::from(self[scalar]) * right_dual.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e5
            right_dual[e5] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group3(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group4(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * right_dual.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group6(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * right_dual.group7(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * right_dual.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group9(),
            // e1234
            right_dual[e1234] * self[scalar],
        );
    }
}
impl BulkExpansion<Plane> for Scalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<RoundPoint> for Scalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1234
            self[scalar] * right_dual[e1234],
        );
    }
}
impl BulkExpansion<Scalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiScalar::from_groups(/* e12345 */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkExpansion<Sphere> for Scalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e5
            right_dual[e5] * self[scalar],
        );
    }
}
impl BulkExpansion<VersorEven> for Scalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * right_dual.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl BulkExpansion<VersorOdd> for Scalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * right_dual.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
impl std::ops::Div<bulk_expansion> for Sphere {
    type Output = bulk_expansion_partial<Sphere>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiDualNum> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return AntiScalar::from_groups(/* e12345 */ right_dual[e5] * self[e1234]);
    }
}
impl BulkExpansion<AntiMotor> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(/* e12345 */ right_dual[e5] * self[e1234]);
    }
}
impl BulkExpansion<AntiScalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1234
            right_dual[scalar] * self[e1234],
        );
    }
}
impl BulkExpansion<CircleRotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1234
            right_dual[scalar] * self[e1234],
        );
    }
}
impl BulkExpansion<DipoleInversion> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        4       20        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e4] * self[e3215]) + (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkExpansion<DualNum> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1234
            right_dual[scalar] * self[e1234],
        );
    }
}
impl BulkExpansion<Flector> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkExpansion<Motor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1234
            right_dual[scalar] * self[e1234],
        );
    }
}
impl BulkExpansion<MultiVector> for Sphere {
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
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
                (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]) + (right_dual[e5] * self[e1234]),
            ]),
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
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1234
            right_dual[scalar] * self[e1234],
        );
    }
}
impl BulkExpansion<Plane> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkExpansion<Sphere> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e1] * self[e4235]) + (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]) + (right_dual[e5] * self[e1234]),
        );
    }
}
impl BulkExpansion<VersorEven> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       21        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e1234
            self[e1234] * right_dual[scalar],
        );
    }
}
impl BulkExpansion<VersorOdd> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        4       21        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * right_dual[e1]) + (self[e4315] * right_dual[e2]) + (self[e4125] * right_dual[e3]) + (self[e3215] * right_dual[e4]) + (self[e1234] * right_dual[e5]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for VersorEven {
    type Output = bulk_expansion_partial<VersorEven>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       31        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (self.group3().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235])),
            // e1234
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       37       57        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e41] * self[e5]) + (right_dual[e15] * self[e4]),
                (right_dual[e42] * self[e5]) + (right_dual[e25] * self[e4]),
                (right_dual[e43] * self[e5]) + (right_dual[e35] * self[e4]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[e5]) * right_dual.group1().xyz().with_w(right_dual[e1234]))
                + (self.group3().yzxx() * right_dual.group2().zxy().with_w(right_dual[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4])
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
                - (right_dual.group2().yzx() * self.group3().zxy()).with_w(right_dual[e41] * self[e235]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e5]) * self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e5]) * self.group0().xyz().with_w(self[e321]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       20        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                right_dual[e15] * self[e4],
                right_dual[e25] * self[e4],
                right_dual[e35] * self[e4],
                -(right_dual[e25] * self[e431]) - (right_dual[e35] * self[e412]) - (right_dual[e45] * self[e321]),
            ]) - (right_dual.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((right_dual.group0().zxy() * self.group3().yzx()) - (right_dual.group0().yzx() * self.group3().zxy())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiFlector> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       16       28        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group3().wwwx() * right_dual.group0().xyz().with_w(right_dual[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412])
                        - (right_dual[e45] * self[e321]),
                )
                - (right_dual.group0().wwwx() * self.group3().xyz().with_w(self[e423])),
            // e235, e315, e125, e5
            ((right_dual.group0().zxy() * self.group3().yzx()) - (right_dual.group0().yzx() * self.group3().zxy())).with_w(0.0),
        );
    }
}
impl BulkExpansion<AntiLine> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) - (self.group3().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiMotor> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_dual[e5]) * self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e235] * self[e4]) + (right_dual[e5] * self[e423]),
                (right_dual[e315] * self[e4]) + (right_dual[e5] * self[e431]),
                (right_dual[e125] * self[e4]) + (right_dual[e5] * self[e412]),
                -(right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]),
            ]) + (right_dual.group0().yzx() * self.group3().zxy()).with_w(right_dual[e5] * self[e321])
                - (self.group3().yzxx() * right_dual.group0().zxy().with_w(right_dual[e235])),
        );
    }
}
impl BulkExpansion<AntiPlane> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]),
        );
    }
}
impl BulkExpansion<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       17        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[scalar]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<Circle> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd3        2        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       29       44        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) + (right_dual.group0().yzx() * self.group3().zxy()) - (right_dual.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e41] * self[e5]) + (right_dual[e15] * self[e4]),
                (right_dual[e42] * self[e5]) + (right_dual[e25] * self[e4]),
                (right_dual[e43] * self[e5]) + (right_dual[e35] * self[e4]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (right_dual[e23] * self[e5]) + (right_dual[e35] * self[e2]),
                (right_dual[e31] * self[e5]) + (right_dual[e15] * self[e3]),
                (right_dual[e12] * self[e5]) + (right_dual[e25] * self[e1]),
                -(right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
            ]) - (right_dual.group2().yzx() * self.group3().zxy()).with_w(right_dual[e41] * self[e235]),
        );
    }
}
impl BulkExpansion<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        3        7        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       40       64        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (right_dual[e23] * self[e4]) + (right_dual[scalar] * self[e423]),
                (right_dual[e31] * self[e4]) + (right_dual[scalar] * self[e431]),
                (right_dual[e12] * self[e4]) + (right_dual[scalar] * self[e412]),
                -(right_dual[e42] * self[e315])
                    - (right_dual[e43] * self[e125])
                    - (right_dual[e23] * self[e415])
                    - (right_dual[e31] * self[e425])
                    - (right_dual[e12] * self[e435])
                    - (right_dual[e45] * self[e321])
                    - (right_dual[e15] * self[e423])
                    - (right_dual[e25] * self[e431])
                    - (right_dual[e35] * self[e412]),
            ]) + (right_dual.group0().yzx() * self.group3().zxy()).with_w(right_dual[scalar] * self[e12345])
                - (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e41] * self[e235]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e15] * self[e4]) + (right_dual[scalar] * self[e415]),
                (right_dual[e25] * self[e4]) + (right_dual[scalar] * self[e425]),
                (right_dual[e35] * self[e4]) + (right_dual[scalar] * self[e435]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) + (right_dual.group0() * self.group2().www()).with_w(right_dual[scalar] * self[e321])
                - (right_dual.group1().wwwx() * self.group3().xyzx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[scalar]) * self.group2().xyz())
                + (Simd32x3::from(self[e5]) * right_dual.group1().xyz())
                + (right_dual.group2().zxy() * self.group3().yzx())
                - (right_dual.group2().yzx() * self.group3().zxy()))
            .with_w(right_dual[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<Dipole> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       30        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e425] * self[e3]) + (right_dual[e235] * self[e4]),
                (right_dual[e435] * self[e1]) + (right_dual[e315] * self[e4]),
                (right_dual[e415] * self[e2]) + (right_dual[e125] * self[e4]),
                -(right_dual[e321] * self[e5]) - (right_dual[e125] * self[e3]),
            ]) - (self.group3().yzxy() * right_dual.group1().zxy().with_w(right_dual[e315]))
                - (right_dual.group0() * self.group2().www()).with_w(right_dual[e235] * self[e1]),
            // e1234
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
        );
    }
}
impl BulkExpansion<DipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        8        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       48       75        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (right_dual.group3().zxyw() * self.group3().yzxw()) - (right_dual.group3().yzx() * self.group3().zxy()).with_w(right_dual[e4] * self[e5]),
            // e15, e25, e35, e1234
            (self.group3().xyzx() * right_dual.group3().www().with_w(right_dual[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                        - (right_dual[e1] * self[e423])
                        - (right_dual[e2] * self[e431])
                        - (right_dual[e3] * self[e412]),
                )
                - (self.group2().www() * right_dual.group3().xyz()).with_w(right_dual[e4] * self[e321]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_dual[e5]) * self.group0().xyz().with_w(self[e321]))
                + (right_dual.group3().yzxz() * self.group1().zxy().with_w(self[e125]))
                + (right_dual.group1().yzx() * self.group3().zxy()).with_w(right_dual[e1] * self[e235])
                + (self.group3().www() * right_dual.group2().xyz()).with_w(right_dual[e2] * self[e315])
                - (Simd32x4::from(self[e5]) * right_dual.group0().with_w(right_dual[e321]))
                - (right_dual.group2().wwwy() * self.group2().xyz().with_w(self[e2]))
                - (self.group3().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235]))
                - (right_dual.group3().zxy() * self.group1().yzx()).with_w(right_dual[e125] * self[e3]),
        );
    }
}
impl BulkExpansion<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       20        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], 1.0])
                * right_dual
                    .group0()
                    .yy()
                    .with_zw(right_dual[scalar], (right_dual[e3215] * self[e4]) + (right_dual[scalar] * self[e12345])),
            // e415, e425, e435, e321
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[scalar]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<FlatPoint> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e235] * self[e1]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5])),
            // e1234
            right_dual[e321] * self[e4],
        );
    }
}
impl BulkExpansion<Flector> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        1        5        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       31       48        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group1().xyz(),
            // e23, e31, e12, e45
            ((right_dual.group1().zxy() * self.group3().yzx()) - (right_dual.group1().yzx() * self.group3().zxy())).with_w(right_dual[e5] * self[e4]),
            // e15, e25, e35, e1234
            (self.group3() * right_dual.group1().www().with_w(right_dual[e321])) + Simd32x3::from(0.0).with_w(-(right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]))
                - (right_dual.group1().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (right_dual.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (right_dual.group1().wwwz() * self.group0().xyz().with_w(self[e125]))
                + Simd32x3::from(0.0).with_w((right_dual[e5] * self[e321]) - (right_dual[e315] * self[e2]) - (right_dual[e125] * self[e3]) - (right_dual[e321] * self[e5]))
                + (self.group3().www() * right_dual.group0().xyz()).with_w(right_dual[e1] * self[e235])
                - (right_dual.group1().zxy() * self.group1().yzx()).with_w(right_dual[e235] * self[e1]),
        );
    }
}
impl BulkExpansion<Line> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       25        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 1.0])
                * right_dual.group1().with_w(-(right_dual[e23] * self[e1]) - (right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (right_dual[e23] * self[e5]) + (right_dual[e35] * self[e2]),
                (right_dual[e31] * self[e5]) + (right_dual[e15] * self[e3]),
                (right_dual[e12] * self[e5]) + (right_dual[e25] * self[e1]),
                -(right_dual[e31] * self[e425]) - (right_dual[e12] * self[e435]) - (right_dual[e15] * self[e423]) - (right_dual[e25] * self[e431]) - (right_dual[e35] * self[e412]),
            ]) - (right_dual.group1().yzx() * self.group3().zxy()).with_w(right_dual[e23] * self[e415]),
        );
    }
}
impl BulkExpansion<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       28       49        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (right_dual.group0() * self.group3().www().with_w(self[e12345]))
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e23] * self[e415])
                        - (right_dual[e31] * self[e425])
                        - (right_dual[e12] * self[e435])
                        - (right_dual[e15] * self[e423])
                        - (right_dual[e25] * self[e431])
                        - (right_dual[e35] * self[e412]),
                )
                + (right_dual.group0().www() * self.group0().xyz()).with_w(right_dual[e3215] * self[e4]),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e15] * self[e4],
                right_dual[e25] * self[e4],
                right_dual[e35] * self[e4],
                -(right_dual[e23] * self[e1]) - (right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group1()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[scalar]) * self.group2().xyz())
                + (Simd32x3::from(self[e5]) * right_dual.group0().xyz())
                + (right_dual.group1().zxy() * self.group3().yzx())
                - (right_dual.group1().yzx() * self.group3().zxy()))
            .with_w(right_dual[scalar] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       40        0
    //    simd2        0        1        0
    //    simd3        8       19        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       44       71        0
    //  no simd       90      143        0
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3(),
            // e5
            right_dual[scalar] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[e5]) * self.group3()) - (Simd32x4::from(self[e5]) * right_dual.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12
            (right_dual.group1().zxy() * self.group3().yzx()) - (right_dual.group1().yzx() * self.group3().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e15] * self[e4]) + (right_dual[e41] * self[e5]),
                (right_dual[e25] * self[e4]) + (right_dual[e42] * self[e5]),
                (right_dual[e35] * self[e4]) + (right_dual[e43] * self[e5]),
                -(right_dual[e31] * self[e2]) - (right_dual[e12] * self[e3]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group1())
                - (self.group3().xyzx() * right_dual.group3().www().with_w(right_dual[e23])),
            // e423, e431, e412
            (Simd32x3::from(right_dual[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e4]) * right_dual.group5()) + (right_dual.group4().yzx() * self.group3().zxy())
                - (right_dual.group4().zxy() * self.group3().yzx()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[scalar]) * self.group2().xyz()) + (Simd32x3::from(self[e5]) * right_dual.group5()) + (right_dual.group3().zxy() * self.group3().yzx())
                - (right_dual.group3().yzx() * self.group3().zxy()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_dual[e5]) * self.group0().xyz().with_w(self[e321]))
                + (right_dual.group1().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (right_dual.group8() * self.group3().www()).with_w(right_dual[e1] * self[e235])
                + (right_dual.group6().yzx() * self.group3().zxy()).with_w(right_dual[e3] * self[e125])
                - (right_dual.group6().zxyw() * self.group3().yzx().with_w(self[e5]))
                - (right_dual.group7() * self.group2().www()).with_w(right_dual[e235] * self[e1])
                - (right_dual.group1().zxy() * self.group1().yzx()).with_w(right_dual[e315] * self[e2])
                - (right_dual.group1().www() * self.group2().xyz()).with_w(right_dual[e125] * self[e3]),
            // e1234
            (right_dual[e321] * self[e4]) + (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3])
                - (right_dual[e1] * self[e423])
                - (right_dual[e2] * self[e431])
                - (right_dual[e3] * self[e412])
                - (right_dual[e4] * self[e321]),
        );
    }
}
impl BulkExpansion<Plane> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       14        0
    //    simd3        1        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       21        0
    //  no simd       17       39        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * right_dual.group0().xyz(),
            // e23, e31, e12, e45
            ((right_dual.group0().zxy() * self.group3().yzx()) - (right_dual.group0().yzx() * self.group3().zxy())).with_w(right_dual[e5] * self[e4]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                right_dual[e5] * self[e1],
                right_dual[e5] * self[e2],
                right_dual[e5] * self[e3],
                -(right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]),
            ]) - (right_dual.group0().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e3] * self[e425] * -1.0,
                right_dual[e1] * self[e435] * -1.0,
                right_dual[e2] * self[e415] * -1.0,
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e235]))
                + (right_dual.group0().wwwy() * self.group0().xyz().with_w(self[e315])),
        );
    }
}
impl BulkExpansion<RoundPoint> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return AntiScalar::from_groups(
            // e12345
            (right_dual[e4235] * self[e1]) + (right_dual[e4315] * self[e2]) + (right_dual[e4125] * self[e3]) + (right_dual[e3215] * self[e4]) + (right_dual[e1234] * self[e5]),
        );
    }
}
impl BulkExpansion<Sphere> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        3        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       25       44        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (right_dual.group0().zxy() * self.group3().yzx()).with_w(right_dual[e5] * self[e4]) - (right_dual.group0().yzxw() * self.group3().zxy().with_w(self[e5])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                right_dual[e5] * self[e1],
                right_dual[e5] * self[e2],
                right_dual[e5] * self[e3],
                -(right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]) - (right_dual[e4] * self[e321]),
            ]) - (right_dual.group0().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(right_dual[e3] * self[e425]) - (right_dual[e4] * self[e235]),
                -(right_dual[e1] * self[e435]) - (right_dual[e4] * self[e315]),
                -(right_dual[e2] * self[e415]) - (right_dual[e4] * self[e125]),
                (right_dual[e3] * self[e125]) + (right_dual[e5] * self[e321]),
            ]) + (Simd32x4::from([right_dual[e5], right_dual[e5], right_dual[e5], self[e315]]) * self.group0().xyz().with_w(right_dual[e2]))
                + (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e235])),
        );
    }
}
impl BulkExpansion<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       77        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(right_dual[scalar]) * self.group0())
                + (self.group3().wwwx() * right_dual.group1().xyz().with_w(right_dual[e4235]))
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
                + (self.group3().zxy() * right_dual.group0().yzx()).with_w(self[e5] * right_dual[e1234])
                - (self.group3().yzx() * right_dual.group0().zxy()).with_w(self[e423] * right_dual[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e5] * right_dual[e41]) + (self[e4] * right_dual[e15]),
                (self[e5] * right_dual[e42]) + (self[e4] * right_dual[e25]),
                (self[e5] * right_dual[e43]) + (self[e4] * right_dual[e35]),
                -(self[e2] * right_dual[e31]) - (self[e3] * right_dual[e12]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group1())
                - (self.group3().xyzx() * right_dual.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * self.group2().xyz())
                + (self.group3().yzx() * right_dual.group2().zxy())
                - (self.group3().zxy() * right_dual.group2().yzx()))
            .with_w(self[e5] * right_dual[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<VersorOdd> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        7        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       48       76        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group3().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (self.group3().yzxw() * right_dual.group3().zxy().with_w(right_dual[e5])) - (right_dual.group3().yzxw() * self.group3().zxy().with_w(self[e5])),
            // e15, e25, e35, e1234
            (self.group3().xyzx() * right_dual.group2().www().with_w(right_dual[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                        - (right_dual[e2] * self[e431])
                        - (right_dual[e3] * self[e412])
                        - (right_dual[e4] * self[e321]),
                )
                - (right_dual.group3().xyzx() * self.group2().www().with_w(self[e423])),
            // e4235, e4315, e4125, e3215
            (right_dual.group3().yzxz() * self.group1().zxy().with_w(self[e125]))
                + (right_dual.group1().yzx() * self.group3().zxy()).with_w(right_dual[e5] * self[e321])
                + (right_dual.group2().www() * self.group0().xyz()).with_w(right_dual[e2] * self[e315])
                + (self.group3().www() * right_dual.group2().xyz()).with_w(right_dual[e1] * self[e235])
                - (Simd32x4::from(self[e5]) * right_dual.group0().xyz().with_w(right_dual[e321]))
                - (self.group3().yzxx() * right_dual.group1().zxy().with_w(right_dual[e235]))
                - (right_dual.group3().zxy() * self.group1().yzx()).with_w(right_dual[e315] * self[e2])
                - (right_dual.group3().www() * self.group2().xyz()).with_w(right_dual[e125] * self[e3]),
        );
    }
}
impl std::ops::Div<bulk_expansion> for VersorOdd {
    type Output = bulk_expansion_partial<VersorOdd>;
    fn div(self, _rhs: bulk_expansion) -> Self::Output {
        bulk_expansion_partial(self)
    }
}
impl BulkExpansion<AntiCircleRotor> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       10       33        0
    fn bulk_expansion(self, other: AntiCircleRotor) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group2().xyz().with_w(
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
                ),
        );
    }
}
impl BulkExpansion<AntiDipoleInversion> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        3        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       30       58        0
    fn bulk_expansion(self, other: AntiDipoleInversion) -> Self::Output {
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
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group2().xyz().with_w(
                    (right_dual[e1234] * self[scalar])
                        - (right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]) + (right_dual[e4235] * self[scalar]),
                (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]) + (right_dual[e4315] * self[scalar]),
                (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]) + (right_dual[e4125] * self[scalar]),
                -(right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[e3215] * self[scalar])
                - (self.group2().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23]))
                - (right_dual.group2().yzx() * self.group0().zxy()).with_w(right_dual[e31] * self[e25]),
        );
    }
}
impl BulkExpansion<AntiDualNum> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       10        0
    fn bulk_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = DualNum::from_groups(/* e5, e12345 */ other.group0());
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[e41], self[e42], self[e43], 1.0])
                * right_dual.group0().xx().with_zw(right_dual[e5], (right_dual[e5] * self[e1234]) + (right_dual[e12345] * self[scalar])),
            // e235, e315, e125, e5
            Simd32x4::from(right_dual[e5]) * self.group1().xyz().with_w(self[scalar]),
        );
    }
}
impl BulkExpansion<AntiFlatPoint> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bulk_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e35] * self[e42]) + (right_dual[e45] * self[e23]),
                (right_dual[e15] * self[e43]) + (right_dual[e45] * self[e31]),
                (right_dual[e25] * self[e41]) + (right_dual[e45] * self[e12]),
                -(right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiFlector> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       12       28        0
    fn bulk_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e45] * self[e23]) + (right_dual[e4235] * self[scalar]),
                (right_dual[e45] * self[e31]) + (right_dual[e4315] * self[scalar]),
                (right_dual[e45] * self[e12]) + (right_dual[e4125] * self[scalar]),
                -(right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (self.group0().yzxw() * right_dual.group0().zxy().with_w(right_dual[e3215]))
                - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl BulkExpansion<AntiLine> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       28        0
    fn bulk_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(
            // e415, e425, e435
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group0().with_w(
                    -(right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12])
                        - (right_dual[e235] * self[e41])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43]),
                ),
            // e235, e315, e125, e5
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<AntiMotor> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn bulk_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(right_dual[e5]) * self.group0().xyz().with_w(self[e1234]))
                + (Simd32x4::from(self[scalar]) * right_dual.group0())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e415] * self[e23])
                        - (right_dual[e425] * self[e31])
                        - (right_dual[e435] * self[e12])
                        - (right_dual[e235] * self[e41])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43]),
                ),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group1().xyz())).with_w(right_dual[e5] * self[scalar]),
        );
    }
}
impl BulkExpansion<AntiPlane> for VersorOdd {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bulk_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * right_dual.group0());
    }
}
impl BulkExpansion<AntiScalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       17        0
    fn bulk_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = Scalar::from_groups(/* scalar */ other[e12345] * -1.0);
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[scalar]) * self.group3(),
        );
    }
}
impl BulkExpansion<Circle> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       25       45        0
    fn bulk_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            other.group2(),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group2().with_w(
                    -(right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e42] * self[e35]) + (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]),
                (right_dual[e43] * self[e15]) + (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]),
                (right_dual[e41] * self[e25]) + (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]),
                -(right_dual[e23] * self[e15]) - (right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group0().zxy() * self.group2().yzx()).with_w(right_dual[e15] * self[e23])
                - (right_dual.group2().yzx() * self.group0().zxy()).with_w(right_dual[e25] * self[e31]),
        );
    }
}
impl BulkExpansion<CircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd3        1        4        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       40       64        0
    fn bulk_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(right_dual[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group0())).with_w(right_dual[scalar] * self[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(right_dual[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * right_dual.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                right_dual[scalar] * self[e15],
                right_dual[scalar] * self[e25],
                right_dual[scalar] * self[e35],
                -(right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
            ]) + (right_dual.group2() * self.group0().www().with_w(self[e1234])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]) + (right_dual[scalar] * self[e4235]),
                (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]) + (right_dual[scalar] * self[e4315]),
                (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]) + (right_dual[scalar] * self[e4125]),
                -(right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (right_dual.group0().yzx() * self.group2().zxy()).with_w(right_dual[scalar] * self[e3215])
                - (self.group2().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23]))
                - (right_dual.group2().yzx() * self.group0().zxy()).with_w(right_dual[e31] * self[e25]),
        );
    }
}
impl BulkExpansion<Dipole> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd        9       31        0
    fn bulk_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
        );
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual.group2().with_w(
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
impl BulkExpansion<DipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        3        8        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       47       75        0
    fn bulk_expansion(self, other: DipoleInversion) -> Self::Output {
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (right_dual.group3().zxyy() * self.group0().yzx().with_w(self[e4315]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
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
                + (right_dual.group0() * self.group0().www()).with_w(right_dual[e4] * self[e3215])
                + (right_dual.group2().www() * self.group1().xyz()).with_w(right_dual[e1] * self[e4235])
                - (right_dual.group3().yzx() * self.group0().zxy()).with_w(right_dual[e423] * self[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e4] * self[e15]) + (right_dual[e5] * self[e41]),
                (right_dual[e4] * self[e25]) + (right_dual[e5] * self[e42]),
                (right_dual[e4] * self[e35]) + (right_dual[e5] * self[e43]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1())
                - (right_dual.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * right_dual.group2().xyz())
                + (right_dual.group3().yzx() * self.group2().zxy())
                - (right_dual.group3().zxy() * self.group2().yzx()))
            .with_w(right_dual[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3().xyz().with_w(right_dual[e4]),
        );
    }
}
impl BulkExpansion<DualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       20        0
    fn bulk_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ other.group0() * Simd32x2::from(-1.0));
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_dual[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_dual[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], 1.0])
                * right_dual
                    .group0()
                    .yy()
                    .with_zw(right_dual[scalar], (right_dual[e3215] * self[scalar]) + (right_dual[scalar] * self[e3215])),
        );
    }
}
impl BulkExpansion<FlatPoint> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn bulk_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_dual[e321] * self[scalar]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual
                    .group0()
                    .xyz()
                    .with_w(-(right_dual[e235] * self[e41]) - (right_dual[e315] * self[e42]) - (right_dual[e125] * self[e43]) - (right_dual[e321] * self[e45])),
        );
    }
}
impl BulkExpansion<Flector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        3        4        0
    //    simd4        4        9        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       31       57        0
    fn bulk_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (right_dual.group1().zxyx() * self.group0().yzx().with_w(self[e4235]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234])
                        - (right_dual[e315] * self[e42])
                        - (right_dual[e125] * self[e43])
                        - (right_dual[e321] * self[e45]),
                )
                - (self.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e235])),
            // e415, e425, e435, e321
            (self.group0() * right_dual.group1().www().with_w(right_dual[e321])) + Simd32x3::from(0.0).with_w(-(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]))
                - (right_dual.group1().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz())
                + (right_dual.group1().yzx() * self.group2().zxy())
                - (right_dual.group1().zxy() * self.group2().yzx()))
            .with_w(right_dual[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x3::from(1.0).with_w(0.0) * self.group0().www().with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<Line> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       34        0
    fn bulk_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ other.group0(), /* e15, e25, e35 */ other.group1());
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[scalar], self[scalar], self[scalar], 1.0])
                * right_dual
                    .group1()
                    .with_w(-(right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e35] * self[e42]),
                (right_dual[e31] * self[e45]) + (right_dual[e15] * self[e43]),
                (right_dual[e12] * self[e45]) + (right_dual[e25] * self[e41]),
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) - (right_dual.group1().yzx() * self.group0().zxy()).with_w(right_dual[e23] * self[e15]),
        );
    }
}
impl BulkExpansion<Motor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       25       49        0
    fn bulk_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_dual[scalar]) * self.group0(),
            // e23, e31, e12, e45
            ((Simd32x3::from(right_dual[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz())).with_w(right_dual[scalar] * self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                right_dual[e15] * self[scalar],
                right_dual[e25] * self[scalar],
                right_dual[e35] * self[scalar],
                -(right_dual[e23] * self[e41]) - (right_dual[e31] * self[e42]) - (right_dual[e12] * self[e43]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                right_dual[e35] * self[e42],
                right_dual[e15] * self[e43],
                right_dual[e25] * self[e41],
                -(right_dual[e31] * self[e25]) - (right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (right_dual.group0() * self.group1().www().with_w(self[e3215]))
                + (right_dual.group0().www() * self.group3().xyz()).with_w(right_dual[e3215] * self[scalar])
                - (right_dual.group1().yzx() * self.group0().zxy()).with_w(right_dual[e23] * self[e15]),
        );
    }
}
impl BulkExpansion<MultiVector> for VersorOdd {
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
    fn bulk_expansion(self, other: MultiVector) -> Self::Output {
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
                right_dual[scalar] * self[scalar],
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e5
            right_dual[e5] * self[scalar],
            // e15, e25, e35, e45
            (Simd32x4::from(right_dual[scalar]) * self.group2().xyz().with_w(self[e45])) + (Simd32x4::from(self[scalar]) * right_dual.group3()),
            // e41, e42, e43
            (Simd32x3::from(right_dual[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group4()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e5] * self[e41]) + (right_dual[e415] * self[scalar]),
                (right_dual[e5] * self[e42]) + (right_dual[e425] * self[scalar]),
                (right_dual[e5] * self[e43]) + (right_dual[e435] * self[scalar]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (right_dual.group1().www() * self.group2().xyz()).with_w(right_dual[e321] * self[scalar])
                - (right_dual.group1().xyzx() * self.group1().wwwx()),
            // e423, e431, e412
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group7()) + (right_dual.group1().zxy() * self.group0().yzx())
                - (right_dual.group1().yzx() * self.group0().zxy()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group8()) + (right_dual.group1().yzx() * self.group2().zxy())
                - (right_dual.group1().zxy() * self.group2().yzx()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e35] * self[e42]) + (right_dual[e45] * self[e23]) + (right_dual[e23] * self[e45]) + (right_dual[e4235] * self[scalar]),
                (right_dual[e15] * self[e43]) + (right_dual[e45] * self[e31]) + (right_dual[e31] * self[e45]) + (right_dual[e4315] * self[scalar]),
                (right_dual[e25] * self[e41]) + (right_dual[e45] * self[e12]) + (right_dual[e12] * self[e45]) + (right_dual[e4125] * self[scalar]),
                -(right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]) - (right_dual[e12] * self[e35]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group3())
                + (right_dual.group4().yzx() * self.group2().zxy()).with_w(right_dual[e3215] * self[scalar])
                - (self.group2().yzxx() * right_dual.group4().zxy().with_w(right_dual[e23]))
                - (right_dual.group3().yzx() * self.group0().zxy()).with_w(right_dual[e31] * self[e25]),
            // e1234
            (right_dual[scalar] * self[e1234]) + (right_dual[e1234] * self[scalar])
                - (right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
    }
}
impl BulkExpansion<Plane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        2        3        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        7       24        0
    //  no simd       17       48        0
    fn bulk_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                right_dual[e2] * self[e43] * -1.0,
                right_dual[e3] * self[e41] * -1.0,
                right_dual[e1] * self[e42] * -1.0,
                (right_dual[e2] * self[e4315]) + (right_dual[e3] * self[e4125]) + (right_dual[e5] * self[e1234]),
            ]) + (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e4235])),
            // e415, e425, e435, e321
            Simd32x4::from([
                right_dual[e5] * self[e41],
                right_dual[e5] * self[e42],
                right_dual[e5] * self[e43],
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (right_dual.group0().yzx() * self.group2().zxy()) - (right_dual.group0().zxy() * self.group2().yzx()))
                .with_w(right_dual[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x3::from(1.0).with_w(0.0) * self.group0().www().with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl BulkExpansion<RoundPoint> for VersorOdd {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn bulk_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            other[e4] * -1.0,
        );
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e1234
            right_dual[e1234] * self[scalar],
        );
    }
}
impl BulkExpansion<Scalar> for VersorOdd {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bulk_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e12345 */ other[scalar]);
        return AntiScalar::from_groups(/* e12345 */ right_dual[e12345] * self[scalar]);
    }
}
impl BulkExpansion<Sphere> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        2        3        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       24       47        0
    fn bulk_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            other[e3215],
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                right_dual[e2] * self[e43] * -1.0,
                right_dual[e3] * self[e41] * -1.0,
                right_dual[e1] * self[e42] * -1.0,
                (right_dual[e3] * self[e4125]) + (right_dual[e4] * self[e3215]) + (right_dual[e5] * self[e1234]),
            ]) + (right_dual.group0().zxyx() * self.group0().yzx().with_w(self[e4235]))
                + (right_dual.group0().wwwy() * self.group1().xyz().with_w(self[e4315])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e4] * self[e15]) + (right_dual[e5] * self[e41]),
                (right_dual[e4] * self[e25]) + (right_dual[e5] * self[e42]),
                (right_dual[e4] * self[e35]) + (right_dual[e5] * self[e43]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group0().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz()) + (right_dual.group0().yzx() * self.group2().zxy()) - (right_dual.group0().zxy() * self.group2().yzx()))
                .with_w(right_dual[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
        );
    }
}
impl BulkExpansion<VersorEven> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd3        1        4        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       48       77        0
    fn bulk_expansion(self, other: VersorEven) -> Self::Output {
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
            ((Simd32x3::from(right_dual[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz())).with_w(right_dual[scalar] * self[scalar]),
            // e23, e31, e12, e45
            (Simd32x4::from(right_dual[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * right_dual.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(right_dual[scalar]) * self.group2())
                + (Simd32x4::from(self[scalar]) * right_dual.group2())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_dual[e23] * self[e45]) + (right_dual[e45] * self[e23]) + (right_dual[e35] * self[e42]) + (right_dual[e4235] * self[scalar]),
                (right_dual[e31] * self[e45]) + (right_dual[e45] * self[e31]) + (right_dual[e15] * self[e43]) + (right_dual[e4315] * self[scalar]),
                (right_dual[e12] * self[e45]) + (right_dual[e45] * self[e12]) + (right_dual[e25] * self[e41]) + (right_dual[e4125] * self[scalar]),
                -(right_dual[e12] * self[e35]) - (right_dual[e15] * self[e23]) - (right_dual[e25] * self[e31]) - (right_dual[e35] * self[e12]),
            ]) + (right_dual.group0().yzxw() * self.group2().zxy().with_w(self[e3215]))
                + (right_dual.group0().www() * self.group3().xyz()).with_w(right_dual[e3215] * self[scalar])
                - (self.group2().yzxx() * right_dual.group0().zxy().with_w(right_dual[e23]))
                - (right_dual.group2().yzx() * self.group0().zxy()).with_w(right_dual[e31] * self[e25]),
        );
    }
}
impl BulkExpansion<VersorOdd> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        6        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       77        0
    fn bulk_expansion(self, other: VersorOdd) -> Self::Output {
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
            (Simd32x4::from(self[scalar]) * right_dual.group0())
                + (right_dual.group3().wwwx() * self.group1().xyz().with_w(self[e4235]))
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
                + (right_dual.group3().zxy() * self.group0().yzx()).with_w(right_dual[e5] * self[e1234])
                - (right_dual.group3().yzx() * self.group0().zxy()).with_w(right_dual[e423] * self[e15]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_dual[e5] * self[e41]) + (right_dual[e4] * self[e15]),
                (right_dual[e5] * self[e42]) + (right_dual[e4] * self[e25]),
                (right_dual[e5] * self[e43]) + (right_dual[e4] * self[e35]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1())
                - (right_dual.group3().xyzx() * self.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_dual[e5]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * right_dual.group2().xyz())
                + (right_dual.group3().yzx() * self.group2().zxy())
                - (right_dual.group3().zxy() * self.group2().yzx()))
            .with_w(right_dual[e5] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group3(),
        );
    }
}
