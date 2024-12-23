// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        14      13       0
//  Average:        26      30       0
//  Maximum:       212     240       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        15      16       0
//  Average:        42      48       0
//  Maximum:       397     420       0
impl std::ops::Div<constraint_violation> for AntiCircleRotor {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       40        0
    //    simd3        0        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       45        0
    //  no simd       46       58        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[e45] * self[e45]) + (reverse[scalar] * self[scalar])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]) + (reverse[e35] * self[e42]),
                (reverse[e43] * self[e15]) + (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]) + (reverse[e15] * self[e43]),
                (reverse[e41] * self[e25]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e25] * self[e41]),
                -(reverse[e12] * self[e35]) - (reverse[e15] * self[e23]) - (reverse[e25] * self[e31]) - (reverse[e35] * self[e12]),
            ]) - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (self.group0().zxy() * reverse.group2().yzx()).with_w(reverse[e31] * self[e25]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        - f32::powi(self[e45], 2)
                        - f32::powi(self[scalar], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiDipoleInversion {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       46        0
    //    simd3        0        2        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       54       61        0
    //  no simd       93      104        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    + (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    - (reverse[e321] * self[e321])
                    - (reverse[e4] * self[e5])
                    - (reverse[e5] * self[e4]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e423] * self[e1])
                    + (reverse[e431] * self[e425])
                    + (reverse[e431] * self[e2])
                    + (reverse[e412] * self[e435])
                    + (reverse[e412] * self[e3])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412])
                    + (reverse[e321] * self[e4])
                    - (reverse[e4] * self[e321])
                    - (reverse[e1] * self[e423])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e4] * self[e235]) - (reverse[e3] * self[e425]),
                -(reverse[e4] * self[e315]) - (reverse[e1] * self[e435]),
                -(reverse[e4] * self[e125]) - (reverse[e2] * self[e415]),
                (reverse[e3] * self[e125]) + (reverse[e5] * self[e321]),
            ]) + (Simd32x4::from([reverse[e315], reverse[e5], reverse[e5], self[e125]]) * self.group0().zyz().with_w(reverse[e435]))
                + (Simd32x4::from([reverse[e5], reverse[e125], reverse[e235], self[e315]]) * self.group0().xxy().with_w(reverse[e425]))
                + (reverse.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group1().xyzz() * reverse.group1().www().with_w(reverse[e125]))
                + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group2().wwwx() * reverse.group2().xyz().with_w(reverse[e1]))
                + (self.group1().ww().with_zw(self[e2], self[e415]) * reverse.group1().xyx().with_w(reverse[e235]))
                + (self.group3().zx().with_zw(self[e321], self[e425]) * reverse.group1().yzz().with_w(reverse[e315]))
                - (self.group3().yzxz() * reverse.group1().zxy().with_w(reverse[e125]))
                - (self.group2().zx().with_zw(self[e5], self[e1]) * reverse.group0().yzz().with_w(reverse[e235]))
                - (self.group3().ww().with_zw(self[e315], self[e5]) * reverse.group0().xyx().with_w(reverse[e321]))
                - (self.group0().yzx() * reverse.group2().zxy()).with_w(reverse[e315] * self[e2]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e4] * self[e5]) + f32::powi(self[e321], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e1], 2)
                        - f32::powi(self[e2], 2)
                        - f32::powi(self[e3], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiDualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for AntiDualNum {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e3215] * self[scalar] * 2.0, 0.0]));
    }
}
impl std::ops::Div<constraint_violation> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiFlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            f32::powi(self[e321], 2) - (self[e321] * AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0))[e321]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiFlector {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiFlector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd2        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       15       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        return AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([0.0, f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2)])
                + Simd32x2::from([
                    (reverse[e1] * self[e235]) + (reverse[e2] * self[e315]) + (reverse[e3] * self[e125]) + (reverse[e5] * self[e321])
                        - (reverse[e235] * self[e1])
                        - (reverse[e315] * self[e2])
                        - (reverse[e125] * self[e3])
                        - (reverse[e321] * self[e5]),
                    (reverse[e1] * self[e1]) + (reverse[e2] * self[e2]) + (reverse[e3] * self[e3]) - (reverse[e321] * self[e321]),
                ]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiLine {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiLine {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd2        1        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       11       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        return AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([0.0, f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)])
                + Simd32x2::from([
                    -(reverse[e23] * self[e15])
                        - (reverse[e31] * self[e25])
                        - (reverse[e12] * self[e35])
                        - (reverse[e15] * self[e23])
                        - (reverse[e25] * self[e31])
                        - (reverse[e35] * self[e12]),
                    -(reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                ]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiMotor {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd2        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       15       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([0.0, f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2)])
                + Simd32x2::from([
                    (reverse[scalar] * self[e3215]) + (reverse[e3215] * self[scalar])
                        - (reverse[e23] * self[e15])
                        - (reverse[e31] * self[e25])
                        - (reverse[e12] * self[e35])
                        - (reverse[e15] * self[e23])
                        - (reverse[e25] * self[e31])
                        - (reverse[e35] * self[e12]),
                    (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                ]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiPlane {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for Circle {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd3        0        5        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       44       56        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e321] * self[e321]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                -(reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                -(reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e425] * self[e315]) + (reverse[e435] * self[e125]),
            ]) + (self.group1().wwwz() * reverse.group1().xyz().with_w(reverse[e125]))
                + (reverse.group0().zxy() * self.group2().yzx()).with_w(reverse[e235] * self[e415])
                + (reverse.group2().yzx() * self.group0().zxy()).with_w(reverse[e315] * self[e425])
                + (reverse.group1().www() * self.group1().xyz()).with_w(reverse[e415] * self[e235]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e321], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleRotor {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       32        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       46       58        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e321] * self[e321])
                    - (reverse[e12345] * self[e12345]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                -(reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                -(reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]),
            ]) + (reverse.group1().xyzz() * self.group1().www().with_w(self[e125]))
                + (self.group1().xyzx() * reverse.group1().www().with_w(reverse[e235]))
                + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group0().zxy() * reverse.group2().yzx()).with_w(reverse[e425] * self[e315]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e321], 2) + f32::powi(self[e12345], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for Dipole {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       35       45        0
    //  no simd       44       56        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[e45] * self[e45])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]) + (reverse[e35] * self[e42]),
                (reverse[e43] * self[e15]) + (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]) + (reverse[e15] * self[e43]),
                (reverse[e41] * self[e25]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e25] * self[e41]),
                -(reverse[e23] * self[e15]) - (reverse[e31] * self[e25]) - (reverse[e12] * self[e35]) - (reverse[e35] * self[e12]),
            ]) - (reverse.group0().zxy() * self.group2().yzx()).with_w(reverse[e15] * self[e23])
                - (reverse.group2().yzx() * self.group0().zxy()).with_w(reverse[e25] * self[e31]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        - f32::powi(self[e45], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleInversion {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       55        0
    //    simd3        0        3        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       60       68        0
    //  no simd       93      104        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[e45] * self[e45]) + (reverse[e1234] * self[e3215]) + (reverse[e3215] * self[e1234])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e41] * self[e4235]) + (reverse[e42] * self[e4315]) + (reverse[e43] * self[e4125]) + (reverse[e45] * self[e1234])
                    - (reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23]) + (reverse[e1234] * self[e15]) + (reverse[e4315] * self[e12]),
                (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31]) + (reverse[e1234] * self[e25]) + (reverse[e4125] * self[e23]),
                (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e1234] * self[e35]) + (reverse[e4235] * self[e31]),
                -(reverse[e25] * self[e31]) - (reverse[e25] * self[e4315]) - (reverse[e35] * self[e12]) - (reverse[e35] * self[e4125]),
            ]) + (self.group1().ww().with_zw(self[e4315], self[e45]) * reverse.group1().xyx().with_w(reverse[e3215]))
                + (self.group2().zx().with_zw(self[e3215], self[e25]) * reverse.group0().yzz().with_w(reverse[e4315]))
                + (self.group3().ww().with_zw(self[e25], self[e15]) * reverse.group0().xyx().with_w(reverse[e4235]))
                + (self.group0().yzx() * reverse.group2().zxy()).with_w(reverse[e4125] * self[e35])
                - (Simd32x4::from([reverse[e25], reverse[e3215], reverse[e3215], self[e35]]) * self.group0().zyz().with_w(reverse[e12]))
                - (Simd32x4::from([reverse[e3215], reverse[e35], reverse[e15], self[e25]]) * self.group0().xxy().with_w(reverse[e31]))
                - (reverse.group1().zxyw() * self.group3().yzxw())
                - (reverse.group2().xyzx() * self.group2().www().with_w(self[e23]))
                - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (reverse.group3().zxy() * self.group1().yzx()).with_w(reverse[e15] * self[e4235]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[e4235], 2)
                        + f32::powi(self[e4315], 2)
                        + f32::powi(self[e4125], 2)
                        - f32::powi(self[e45], 2)
                        - 2.0 * (self[e1234] * self[e3215]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e5] * self[e12345] * -2.0, 0.0]));
    }
}
impl std::ops::Div<constraint_violation> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0))[e45]) - f32::powi(self[e45], 2),
        );
    }
}
impl std::ops::Div<constraint_violation> for Flector {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Flector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd2        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       15       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        return AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([0.0, f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[e45], 2)])
                + Simd32x2::from([
                    (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35]) + (reverse[e3215] * self[e45])
                        - (reverse[e15] * self[e4235])
                        - (reverse[e25] * self[e4315])
                        - (reverse[e35] * self[e4125])
                        - (reverse[e45] * self[e3215]),
                    (reverse[e45] * self[e45]) - (reverse[e4235] * self[e4235]) - (reverse[e4315] * self[e4315]) - (reverse[e4125] * self[e4125]),
                ]),
        );
    }
}
impl std::ops::Div<constraint_violation> for Line {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Line {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd2        1        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       11       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([0.0, -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2)])
                + Simd32x2::from([
                    (reverse[e415] * self[e235])
                        + (reverse[e425] * self[e315])
                        + (reverse[e435] * self[e125])
                        + (reverse[e235] * self[e415])
                        + (reverse[e315] * self[e425])
                        + (reverse[e125] * self[e435]),
                    (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]),
                ]),
        );
    }
}
impl std::ops::Div<constraint_violation> for Motor {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd2        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       15       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([0.0, f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2)])
                + Simd32x2::from([
                    (reverse[e415] * self[e235])
                        + (reverse[e425] * self[e315])
                        + (reverse[e435] * self[e125])
                        + (reverse[e235] * self[e415])
                        + (reverse[e315] * self[e425])
                        + (reverse[e125] * self[e435])
                        - (reverse[e12345] * self[e5])
                        - (reverse[e5] * self[e12345]),
                    (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e12345] * self[e12345]),
                ]),
        );
    }
}
impl std::ops::Div<constraint_violation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for MultiVector {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      139      162        0
    //    simd2       17       16        0
    //    simd3        0       22        0
    //    simd4       56       40        0
    // Totals...
    // yes simd      212      240        0
    //  no simd      397      420        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            self.group3() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (reverse[e45] * self[e45])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e12345] * self[e12345])
                    - (reverse[e4] * self[e5])
                    - (reverse[e5] * self[e4])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
                (reverse[e12345] * self[scalar])
                    + (reverse[e5] * self[e1234])
                    + (reverse[e4315] * self[e2])
                    + (reverse[e4125] * self[e3])
                    + (reverse[e3215] * self[e4])
                    + (reverse[e1234] * self[e5])
                    - (reverse[e15] * self[e423])
                    - (reverse[e25] * self[e431])
                    - (reverse[e35] * self[e412])
                    - (reverse[e45] * self[e321])
                    - (reverse[e425] * self[e31])
                    - (reverse[e435] * self[e12])
                    - (reverse[e321] * self[e45])
                    - (reverse[e423] * self[e15])
                    - (reverse[e431] * self[e25])
                    - (reverse[e412] * self[e35]),
            ]) + (Simd32x2::from(reverse[scalar]) * self.group0())
                + (Simd32x2::from(reverse[e1]) * Simd32x2::from([self[e1], self[e4235]]))
                + (Simd32x2::from(reverse[e2]) * Simd32x2::from([self[e2], self[e4315]]))
                + (Simd32x2::from(reverse[e3]) * Simd32x2::from([self[e3], self[e4125]]))
                + (Simd32x2::from(self[e3215]) * Simd32x2::from([reverse[e1234], reverse[e4]]))
                + (Simd32x2::from([self[e1234], self[e1]]) * reverse.group9().wx())
                - (Simd32x2::from(reverse[e41]) * Simd32x2::from([self[e15], self[e235]]))
                - (Simd32x2::from(reverse[e42]) * Simd32x2::from([self[e25], self[e315]]))
                - (Simd32x2::from(reverse[e43]) * Simd32x2::from([self[e35], self[e125]]))
                - (Simd32x2::from(reverse[e23]) * Simd32x2::from([self[e23], self[e415]]))
                - (Simd32x2::from(reverse[e31]) * Simd32x2::from([self[e31], self[e425]]))
                - (Simd32x2::from(reverse[e12]) * Simd32x2::from([self[e12], self[e435]]))
                - (Simd32x2::from(self[e41]) * Simd32x2::from([reverse[e15], reverse[e235]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([reverse[e25], reverse[e315]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([reverse[e35], reverse[e125]]))
                - (Simd32x2::from([self[e321], self[e23]]) * reverse.group6().wx()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse[e15] * self[e4]) + (reverse[e25] * self[e412]) + (reverse[e435] * self[e4315]) + (reverse[e4125] * self[e425]),
                (reverse[e25] * self[e4]) + (reverse[e415] * self[e4125]) + (reverse[e4235] * self[e435]) + (reverse[e3215] * self[e431]),
                (reverse[e35] * self[e4]) + (reverse[e425] * self[e4235]) + (reverse[e4315] * self[e415]) + (reverse[e3215] * self[e412]),
                -(reverse[e4] * self[e45]) - (reverse[e321] * self[e1234]) - (reverse[e4315] * self[e431]) - (reverse[e4125] * self[e412]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group1())
                + (Simd32x4::from([reverse[e2], reverse[e321], reverse[e321], self[e4]]) * self.group5().zyz().with_w(reverse[e45]))
                + (Simd32x4::from([reverse[e5], reverse[e5], reverse[e5], reverse[e2]]) * self.group4().with_w(self[e42]))
                + (Simd32x4::from([reverse[e321], reverse[e3], reverse[e1], reverse[e3]]) * self.group5().xxy().with_w(self[e43]))
                + (Simd32x4::from([reverse[e3215], reverse[e35], reverse[e15], reverse[e1234]]) * self.group7().xxy().with_w(self[e321]))
                + (Simd32x4::from([self[e1234], self[e1234], self[e42], self[e4125]]) * reverse.group8().xyx().with_w(reverse[e412]))
                + (self.group0().xx().with_zw(self[scalar], reverse[e12345]) * reverse.group1().xyz().with_w(self[e1234]))
                + (self.group4().zx().with_zw(self[e1234], reverse[e1]) * reverse.group8().yzz().with_w(self[e41]))
                + (self.group1().zx().with_zw(self[e321], self[e4235]) * reverse.group5().yzz().with_w(reverse[e423]))
                + (self.group6().ww().with_zw(self[e2], reverse[e1234]) * reverse.group5().xyx().with_w(self[e12345]))
                + (reverse.group4().zxy() * self.group8().yzx()).with_w(reverse[e4] * self[scalar])
                + (reverse.group7().zxy() * self.group3().yzx()).with_w(reverse[e431] * self[e4315])
                - (Simd32x4::from([reverse[e1234], reverse[e1234], reverse[e1234], self[e31]]) * self.group8().with_w(reverse[e431]))
                - (Simd32x4::from([self[e5], self[e5], self[e315], self[e2]]) * reverse.group4().xyx().with_w(reverse[e42]))
                - (self.group1().yzxz() * reverse.group5().zxy().with_w(reverse[e43]))
                - (reverse.group0().yy().with_zw(reverse[e12345], reverse[e41]) * self.group9().xyz().with_w(self[e1]))
                - (self.group0().yy().with_zw(self[e12345], reverse[e41]) * reverse.group9().xyz().with_w(self[e415]))
                - (self.group8().zx().with_zw(self[e5], self[e425]) * reverse.group4().yzz().with_w(reverse[e42]))
                - (self.group3().zx().with_zw(self[e3215], self[e423]) * reverse.group7().yzz().with_w(reverse[e23]))
                - (self.group3().ww().with_zw(self[e4315], reverse[e425]) * reverse.group6().xyx().with_w(self[e42]))
                - (self.group9().zx().with_zw(self[e45], reverse[e435]) * reverse.group6().yzz().with_w(self[e43]))
                - (self.group9().ww().with_zw(self[e25], self[e435]) * reverse.group7().xyx().with_w(reverse[e43]))
                - (reverse.group8().zxy() * self.group4().yzx()).with_w(reverse[e31] * self[e431])
                - (self.group5().yzx() * reverse.group1().zxy()).with_w(reverse[e12] * self[e412])
                - (self.group7().yzx() * reverse.group3().zxy()).with_w(reverse[e423] * self[e23])
                - (reverse.group1().www() * self.group3().xyz()).with_w(reverse[e412] * self[e12])
                - (reverse.group3().www() * self.group6().xyz()).with_w(reverse[e415] * self[e41])
                - (reverse.group9().yzx() * self.group6().zxy()).with_w(reverse[e4235] * self[e423]),
            // e5
            (reverse[scalar] * self[e5])
                + (reverse[e12345] * self[e3215])
                + (reverse[e5] * self[scalar])
                + (reverse[e5] * self[e45])
                + (reverse[e15] * self[e1])
                + (reverse[e25] * self[e2])
                + (reverse[e35] * self[e3])
                + (reverse[e321] * self[e3215])
                + (reverse[e4235] * self[e235])
                + (reverse[e4315] * self[e315])
                + (reverse[e4125] * self[e125])
                + (reverse[e3215] * self[e12345])
                - (reverse[e1] * self[e15])
                - (reverse[e2] * self[e25])
                - (reverse[e3] * self[e35])
                - (reverse[e15] * self[e415])
                - (reverse[e25] * self[e425])
                - (reverse[e35] * self[e435])
                - (reverse[e45] * self[e5])
                - (reverse[e23] * self[e235])
                - (reverse[e31] * self[e315])
                - (reverse[e12] * self[e125])
                - (reverse[e415] * self[e15])
                - (reverse[e425] * self[e25])
                - (reverse[e435] * self[e35])
                - (reverse[e235] * self[e23])
                - (reverse[e235] * self[e4235])
                - (reverse[e315] * self[e31])
                - (reverse[e315] * self[e4315])
                - (reverse[e125] * self[e12])
                - (reverse[e125] * self[e4125])
                - (reverse[e3215] * self[e321]),
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
            Simd32x4::from([
                (reverse[e415] * self[e321]) + (reverse[e425] * self[e3]) + (reverse[e321] * self[e415]) + (reverse[e1234] * self[e15]),
                (reverse[e425] * self[e321]) + (reverse[e435] * self[e1]) + (reverse[e321] * self[e425]) + (reverse[e1234] * self[e25]),
                (reverse[e415] * self[e2]) + (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]) + (reverse[e1234] * self[e35]),
                -(reverse[e25] * self[e4315]) - (reverse[e35] * self[e4125]) - (reverse[e45] * self[e3215]) - (reverse[e321] * self[e5]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group9())
                + (Simd32x4::from([reverse[e5], reverse[e5], reverse[e5], self[e45]]) * self.group7().with_w(reverse[e3215]))
                + (Simd32x4::from([reverse[e45], reverse[e4125], reverse[e4235], self[e25]]) * self.group5().xxy().with_w(reverse[e4315]))
                + (Simd32x4::from([reverse[e4315], reverse[e45], reverse[e45], self[e35]]) * self.group5().zyz().with_w(reverse[e4125]))
                + (reverse.group0().yy().with_zw(reverse[e12345], self[scalar]) * self.group1().xyz().with_w(reverse[e3215]))
                + (self.group0().xx().with_zw(self[scalar], reverse[e235]) * reverse.group9().xyz().with_w(self[e415]))
                + (self.group0().yy().with_zw(self[e12345], reverse[e315]) * reverse.group1().xyz().with_w(self[e425]))
                + (self.group7().zx().with_zw(self[e4], reverse[e435]) * reverse.group8().yzz().with_w(self[e125]))
                + (self.group1().ww().with_zw(self[e431], reverse[e3]) * reverse.group8().xyx().with_w(self[e125]))
                + (self.group3().zx().with_zw(self[e3215], reverse[e1]) * reverse.group4().yzz().with_w(self[e235]))
                + (self.group3().ww().with_zw(self[e4315], reverse[e415]) * reverse.group5().xyx().with_w(self[e235]))
                + (self.group9().zx().with_zw(self[e45], reverse[e2]) * reverse.group5().yzz().with_w(self[e315]))
                + (self.group9().ww().with_zw(self[e25], self[e435]) * reverse.group4().xyx().with_w(reverse[e125]))
                + (reverse.group7().zxy() * self.group8().yzx()).with_w(reverse[e425] * self[e315])
                + (self.group4().yzx() * reverse.group3().zxy()).with_w(reverse[e4235] * self[e15])
                + (reverse.group1().yzx() * self.group6().zxy()).with_w(reverse[e5] * self[e321])
                - (Simd32x4::from([reverse[e25], reverse[e3215], reverse[e3215], self[e2]]) * self.group4().zyz().with_w(reverse[e315]))
                - (Simd32x4::from([reverse[e3215], reverse[e35], reverse[e15], self[e1]]) * self.group4().xxy().with_w(reverse[e235]))
                - (Simd32x4::from([self[e5], self[e5], self[e315], self[e15]]) * reverse.group7().xyx().with_w(reverse[e23]))
                - (Simd32x4::from([self[e1234], self[e1234], self[e1234], reverse[e35]]) * reverse.group3().xyz().with_w(self[e12]))
                - (self.group8().zx().with_zw(self[e5], self[e25]) * reverse.group7().yzz().with_w(reverse[e31]))
                - (self.group8() * reverse.group1().www()).with_w(reverse[e15] * self[e23])
                - (reverse.group4().zxy() * self.group3().yzx()).with_w(reverse[e12345] * self[e5])
                - (reverse.group5().zxy() * self.group9().yzx()).with_w(reverse[e5] * self[e12345])
                - (reverse.group8().zxy() * self.group7().yzx()).with_w(reverse[e12] * self[e35])
                - (self.group5().yzx() * reverse.group9().zxy()).with_w(reverse[e125] * self[e3])
                - (reverse.group1().zxy() * self.group6().yzx()).with_w(reverse[e25] * self[e31])
                - (reverse.group6().zxy() * self.group1().yzx()).with_w(reverse[e15] * self[e4235]),
            // e1234
            (reverse[scalar] * self[e1234])
                + (reverse[e45] * self[e1234])
                + (reverse[e41] * self[e4235])
                + (reverse[e42] * self[e4315])
                + (reverse[e43] * self[e4125])
                + (reverse[e415] * self[e423])
                + (reverse[e425] * self[e431])
                + (reverse[e435] * self[e412])
                + (reverse[e321] * self[e4])
                + (reverse[e423] * self[e1])
                + (reverse[e423] * self[e415])
                + (reverse[e431] * self[e2])
                + (reverse[e431] * self[e425])
                + (reverse[e412] * self[e3])
                + (reverse[e412] * self[e435])
                + (reverse[e1234] * self[scalar])
                - (reverse[e12345] * self[e4])
                - (reverse[e1] * self[e423])
                - (reverse[e2] * self[e431])
                - (reverse[e3] * self[e412])
                - (reverse[e4] * self[e12345])
                - (reverse[e4] * self[e321])
                - (reverse[e41] * self[e23])
                - (reverse[e42] * self[e31])
                - (reverse[e43] * self[e12])
                - (reverse[e23] * self[e41])
                - (reverse[e31] * self[e42])
                - (reverse[e12] * self[e43])
                - (reverse[e4235] * self[e41])
                - (reverse[e4315] * self[e42])
                - (reverse[e4125] * self[e43])
                - (reverse[e1234] * self[e45]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (self[e4] * self[e5])
                    + 2.0 * (self[e15] * self[e41])
                    + 2.0 * (self[e25] * self[e42])
                    + 2.0 * (self[e35] * self[e43])
                    + f32::powi(self[e12345], 2)
                    + f32::powi(self[e23], 2)
                    + f32::powi(self[e31], 2)
                    + f32::powi(self[e12], 2)
                    + f32::powi(self[e321], 2)
                    + f32::powi(self[e4235], 2)
                    + f32::powi(self[e4315], 2)
                    + f32::powi(self[e4125], 2)
                    - f32::powi(self[scalar], 2)
                    - f32::powi(self[e1], 2)
                    - f32::powi(self[e2], 2)
                    - f32::powi(self[e3], 2)
                    - f32::powi(self[e45], 2)
                    - f32::powi(self[e415], 2)
                    - f32::powi(self[e425], 2)
                    - f32::powi(self[e435], 2)
                    - 2.0 * (self[e423] * self[e235])
                    - 2.0 * (self[e431] * self[e315])
                    - 2.0 * (self[e412] * self[e125])
                    - 2.0 * (self[e3215] * self[e1234]),
                0.0,
            ]) + geometric_product.group0(),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e5
            geometric_product[e5],
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
            geometric_product.group9(),
            // e1234
            geometric_product[e1234],
        );
    }
}
impl std::ops::Div<constraint_violation> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Plane {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for RoundPoint {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for Scalar {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for Scalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Sphere {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       44        0
    //    simd3        0        4        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       57       63        0
    //  no simd      105      116        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    + (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    - (reverse[e12345] * self[e12345])
                    - (reverse[e321] * self[e321])
                    - (reverse[e5] * self[e4])
                    - (reverse[e4] * self[e5]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e423] * self[e1])
                    + (reverse[e431] * self[e425])
                    + (reverse[e431] * self[e2])
                    + (reverse[e412] * self[e435])
                    + (reverse[e412] * self[e3])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412])
                    + (reverse[e321] * self[e4])
                    - (reverse[e12345] * self[e4])
                    - (reverse[e1] * self[e423])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412])
                    - (reverse[e4] * self[e12345])
                    - (reverse[e4] * self[e321]),
            ),
            // e4235, e4315, e4125, e3215
            (reverse.group1().xyxz() * self.group1().ww().with_zw(self[e2], self[e125]))
                + (reverse.group2().xyxz() * self.group3().ww().with_zw(self[e431], self[e435]))
                + (reverse.group2().yzzw() * self.group0().zx().with_zw(self[e4], self[e321]))
                + (reverse.group3().xyxy() * self.group0().ww().with_zw(self[e425], self[e315]))
                + (reverse.group3().yzzz() * self.group1().zx().with_zw(self[e12345], self[e125]))
                + (self.group1().xyzy() * reverse.group1().www().with_w(reverse[e315]))
                + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group3().zx().with_zw(self[e321], self[e415]) * reverse.group1().yzz().with_w(reverse[e235]))
                + (reverse.group0().www() * self.group3().xyz()).with_w(reverse[e425] * self[e315])
                + (reverse.group2().www() * self.group0().xyz()).with_w(reverse[e1] * self[e235])
                - (reverse.group0().xyxw() * self.group2().wwyw())
                - (reverse.group2().zxyy() * self.group0().yzx().with_w(self[e2]))
                - (self.group2().zxww() * reverse.group0().yzz().with_w(reverse[e321]))
                - (self.group3().yzxx() * reverse.group1().zxy().with_w(reverse[e235]))
                - (reverse.group3().zxy() * self.group1().yzx()).with_w(reverse[e125] * self[e3])
                - (reverse.group3().www() * self.group2().xyz()).with_w(reverse[e5] * self[e12345]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e5] * self[e4]) + f32::powi(self[e12345], 2) + f32::powi(self[e321], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e1], 2)
                        - f32::powi(self[e2], 2)
                        - f32::powi(self[e3], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for VersorOdd {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       58        0
    //    simd3        0        2        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       66       73        0
    //  no simd      105      116        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(
                (reverse[scalar] * self[scalar]) + (reverse[e45] * self[e45]) + (reverse[e1234] * self[e3215]) + (reverse[e3215] * self[e1234])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e41] * self[e4235])
                    + (reverse[e42] * self[e4315])
                    + (reverse[e43] * self[e4125])
                    + (reverse[scalar] * self[e1234])
                    + (reverse[e45] * self[e1234])
                    + (reverse[e1234] * self[scalar])
                    - (reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15]) + (reverse[e4235] * self[scalar]) + (reverse[e4315] * self[e12]),
                (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25]) + (reverse[e4315] * self[scalar]) + (reverse[e4125] * self[e23]),
                (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35]) + (reverse[e4235] * self[e31]) + (reverse[e4125] * self[scalar]),
                -(reverse[e25] * self[e31]) - (reverse[e25] * self[e4315]) - (reverse[e35] * self[e12]) - (reverse[e35] * self[e4125]),
            ]) + (self.group1() * reverse.group1().www().with_w(reverse[e3215]))
                + (reverse.group0().xyxw() * self.group3().ww().with_zw(self[e25], self[e3215]))
                + (self.group1().ww().with_zw(self[e4315], self[e35]) * reverse.group1().xyx().with_w(reverse[e4125]))
                + (self.group2().zx().with_zw(self[e3215], self[e15]) * reverse.group0().yzz().with_w(reverse[e4235]))
                + (self.group3().zx().with_zw(self[e45], self[scalar]) * reverse.group1().yzz().with_w(reverse[e3215]))
                + (reverse.group0().www() * self.group3().xyz()).with_w(reverse[e4315] * self[e25])
                - (reverse.group1().zxyy() * self.group3().yzx().with_w(self[e25]))
                - (self.group1().yzxx() * reverse.group3().zxy().with_w(reverse[e15]))
                - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (self.group0().zx().with_zw(self[e1234], self[e3215]) * reverse.group2().yzz().with_w(reverse[e45]))
                - (self.group2().ww().with_zw(self[e42], self[e35]) * reverse.group2().xyx().with_w(reverse[e12]))
                - (reverse.group3().www() * self.group0().xyz()).with_w(reverse[e15] * self[e4235]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[e4235], 2)
                        + f32::powi(self[e4315], 2)
                        + f32::powi(self[e4125], 2)
                        - f32::powi(self[scalar], 2)
                        - f32::powi(self[e45], 2)
                        - 2.0 * (self[e1234] * self[e3215]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
