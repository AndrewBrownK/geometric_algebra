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
//   Median:         7       9       0
//  Average:        23      29       0
//  Maximum:       188     225       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         7      12       0
//  Average:        39      46       0
//  Maximum:       372     404       0
impl std::ops::Div<anti_constraint_violation> for AntiCircleRotor {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       34        0
    //    simd3        0        4        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       31       41        0
    //  no simd       46       58        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[scalar] * self[scalar]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (self.group1().wwwz() * anti_reverse.group1().xyz().with_w(anti_reverse[e43]))
                + (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e41] * self[e23])
                + (self.group0().yzx() * anti_reverse.group2().zxy()).with_w(anti_reverse[e42] * self[e31])
                + (anti_reverse.group1().www() * self.group1().xyz()).with_w(anti_reverse[e23] * self[e41]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[scalar], 2)
                        - f32::powi(self[e45], 2),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDipoleInversion {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       58        0
    //    simd3        0        6        0
    //    simd4       11        7        0
    // Totals...
    // yes simd       60       71        0
    //  no simd       93      104        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e321] * self[e321]) + (anti_reverse[e4] * self[e5]) + (anti_reverse[e5] * self[e4])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125]) + (anti_reverse[e5] * self[e321])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e321] * self[e5])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e435])
                    - (anti_reverse[e125] * self[e3]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e4] * self[e235]) + (anti_reverse[e3] * self[e425]),
                (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e4] * self[e315]) + (anti_reverse[e1] * self[e435]),
                (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e4] * self[e125]) + (anti_reverse[e2] * self[e415]),
                -(anti_reverse[e435] * self[e412]) - (anti_reverse[e4] * self[e321]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]),
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * anti_reverse.group1().xxyw())
                + (Simd32x4::from([self[e315], self[e5], self[e5], self[e2]]) * anti_reverse.group0().zyz().with_w(anti_reverse[e431]))
                + (Simd32x4::from([self[e5], self[e125], self[e235], self[e1]]) * anti_reverse.group0().xxy().with_w(anti_reverse[e423]))
                + (self.group0().zxy() * anti_reverse.group2().yzx()).with_w(anti_reverse[e412] * self[e3])
                - (anti_reverse.group2().zx().with_zw(anti_reverse[e5], self[e435]) * self.group0().yzz().with_w(anti_reverse[e412]))
                - (anti_reverse.group3().ww().with_zw(anti_reverse[e315], self[e425]) * self.group0().xyx().with_w(anti_reverse[e431]))
                - (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e423] * self[e415])
                - (anti_reverse.group1().yzx() * self.group3().zxy()).with_w(anti_reverse[e415] * self[e423])
                - (anti_reverse.group3().yzx() * self.group1().zxy()).with_w(anti_reverse[e425] * self[e431])
                - (self.group2().www() * anti_reverse.group2().xyz()).with_w(anti_reverse[e1] * self[e423]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125])
                        - 2.0 * (self[e4] * self[e5]),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * self[scalar] * -2.0, 0.0]));
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlatPoint {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlector {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125]) + (anti_reverse[e5] * self[e321])
                - (anti_reverse[e235] * self[e1])
                - (anti_reverse[e315] * self[e2])
                - (anti_reverse[e125] * self[e3])
                - (anti_reverse[e321] * self[e5]),
            0.0,
        ]));
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiLine {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiLine {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse[e23] * self[e15])
                + (anti_reverse[e31] * self[e25])
                + (anti_reverse[e12] * self[e35])
                + (anti_reverse[e15] * self[e23])
                + (anti_reverse[e25] * self[e31])
                + (anti_reverse[e35] * self[e12]),
            0.0,
        ]));
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiMotor {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse[e23] * self[e15])
                + (anti_reverse[e31] * self[e25])
                + (anti_reverse[e12] * self[e35])
                + (anti_reverse[e15] * self[e23])
                + (anti_reverse[e25] * self[e31])
                + (anti_reverse[e35] * self[e12])
                - (anti_reverse[scalar] * self[e3215])
                - (anti_reverse[e3215] * self[scalar]),
            0.0,
        ]));
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiPlane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Circle {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       35       45        0
    //  no simd       44       56        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e321] * self[e321])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e423] * self[e415])
                - (anti_reverse.group2().zxy() * self.group0().yzx()).with_w(anti_reverse[e431] * self[e425]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e321], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125]),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleRotor {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       41        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       37       46        0
    //  no simd       46       58        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e321] * self[e321]) + (anti_reverse[e12345] * self[e12345])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e423] * self[e415])
                - (self.group0().yzx() * anti_reverse.group2().zxy()).with_w(anti_reverse[e431] * self[e425]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e321], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e12345], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125]),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for Dipole {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Dipole {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd3        0        5        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       44       56        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Dipole::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    - (anti_reverse[e45] * self[e45]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (self.group1().wwwz() * anti_reverse.group1().xyz().with_w(anti_reverse[e43]))
                + (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e41] * self[e23])
                + (anti_reverse.group2().zxy() * self.group0().yzx()).with_w(anti_reverse[e42] * self[e31])
                + (anti_reverse.group1().www() * self.group1().xyz()).with_w(anti_reverse[e23] * self[e41]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        - f32::powi(self[e45], 2),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleInversion {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       51        0
    //    simd3        0        7        0
    //    simd4       13        8        0
    // Totals...
    // yes simd       54       66        0
    //  no simd       93      104        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    + (anti_reverse[e4235] * self[e4235])
                    + (anti_reverse[e4315] * self[e4315])
                    + (anti_reverse[e4125] * self[e4125])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[e1234] * self[e3215])
                    - (anti_reverse[e3215] * self[e1234]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12])
                    + (anti_reverse[e4235] * self[e15])
                    + (anti_reverse[e4315] * self[e25])
                    + (anti_reverse[e4125] * self[e35])
                    + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[e45] * self[e3215])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse[e1234] * self[e15]) - (anti_reverse[e4315] * self[e12]),
                -(anti_reverse[e1234] * self[e25]) - (anti_reverse[e4125] * self[e23]),
                -(anti_reverse[e1234] * self[e35]) - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e12] * self[e43]) + (anti_reverse[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse.group1().xxy().with_w(anti_reverse[e42]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e12]]) * anti_reverse.group1().zyz().with_w(anti_reverse[e43]))
                + (anti_reverse.group2().zx().with_zw(anti_reverse[e3215], self[e31]) * self.group0().yzz().with_w(anti_reverse[e42]))
                + (anti_reverse.group3().ww().with_zw(anti_reverse[e25], self[e4235]) * self.group0().xyx().with_w(anti_reverse[e41]))
                + (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e41] * self[e23])
                + (anti_reverse.group1().www() * self.group1().xyz()).with_w(anti_reverse[e43] * self[e4125])
                + (anti_reverse.group3().zxy() * self.group1().yzx()).with_w(anti_reverse[e31] * self[e42])
                + (self.group2().www() * anti_reverse.group2().xyz()).with_w(anti_reverse[e23] * self[e41])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], anti_reverse[e4315]]) * anti_reverse.group0().zyz().with_w(self[e42]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], anti_reverse[e4235]]) * anti_reverse.group0().xxy().with_w(self[e41]))
                - (self.group0().zxy() * anti_reverse.group2().yzx()).with_w(anti_reverse[e4125] * self[e43])
                - (anti_reverse.group1().yzx() * self.group3().zxy()).with_w(anti_reverse[e1234] * self[e45]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + 2.0 * (self[e1234] * self[e3215])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        - f32::powi(self[e45], 2)
                        - f32::powi(self[e4235], 2)
                        - f32::powi(self[e4315], 2)
                        - f32::powi(self[e4125], 2),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for DualNum {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5] * self[e12345] * 2.0, 0.0]));
    }
}
impl std::ops::Div<anti_constraint_violation> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for FlatPoint {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Flector {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse[e4235] * self[e15]) + (anti_reverse[e4315] * self[e25]) + (anti_reverse[e4125] * self[e35]) + (anti_reverse[e3215] * self[e45])
                - (anti_reverse[e15] * self[e4235])
                - (anti_reverse[e25] * self[e4315])
                - (anti_reverse[e35] * self[e4125])
                - (anti_reverse[e45] * self[e3215]),
            0.0,
        ]));
    }
}
impl std::ops::Div<anti_constraint_violation> for Line {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(anti_reverse[e415] * self[e235])
                - (anti_reverse[e425] * self[e315])
                - (anti_reverse[e435] * self[e125])
                - (anti_reverse[e235] * self[e415])
                - (anti_reverse[e315] * self[e425])
                - (anti_reverse[e125] * self[e435]),
            0.0,
        ]));
    }
}
impl std::ops::Div<anti_constraint_violation> for Motor {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse[e12345] * self[e5]) + (anti_reverse[e5] * self[e12345])
                - (anti_reverse[e415] * self[e235])
                - (anti_reverse[e425] * self[e315])
                - (anti_reverse[e435] * self[e125])
                - (anti_reverse[e235] * self[e415])
                - (anti_reverse[e315] * self[e425])
                - (anti_reverse[e125] * self[e435]),
            0.0,
        ]));
    }
}
impl std::ops::Div<anti_constraint_violation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      116      147        0
    //    simd2       16       16        0
    //    simd3        0       23        0
    //    simd4       56       39        0
    // Totals...
    // yes simd      188      225        0
    //  no simd      372      404        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MultiVector::from_groups(
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
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (anti_reverse[e12345] * self[scalar])
                    + (anti_reverse[e5] * self[e1234])
                    + (anti_reverse[e4315] * self[e2])
                    + (anti_reverse[e4125] * self[e3])
                    + (anti_reverse[e3215] * self[e4])
                    + (anti_reverse[e1234] * self[e5])
                    - (anti_reverse[e15] * self[e423])
                    - (anti_reverse[e25] * self[e431])
                    - (anti_reverse[e35] * self[e412])
                    - (anti_reverse[e45] * self[e321])
                    - (anti_reverse[e415] * self[e23])
                    - (anti_reverse[e425] * self[e31])
                    - (anti_reverse[e435] * self[e12])
                    - (anti_reverse[e321] * self[e45])
                    - (anti_reverse[e431] * self[e25])
                    - (anti_reverse[e412] * self[e35]),
                (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    + (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e321] * self[e321])
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3])
                    - (anti_reverse[e3215] * self[e1234])
                    - (anti_reverse[e1234] * self[e3215]),
            ]) + (Simd32x2::from(self[e12345]) * anti_reverse.group0())
                + (Simd32x2::from([anti_reverse[e2], anti_reverse[e4235]]) * self.group9().yx())
                + (Simd32x2::from([anti_reverse[e3], anti_reverse[e4315]]) * self.group9().zy())
                + (Simd32x2::from([anti_reverse[e4], anti_reverse[e4125]]) * self.group9().wz())
                + (Simd32x2::from([anti_reverse[e4235], anti_reverse[e5]]) * self.group1().xw())
                + (Simd32x2::from([self[e4235], self[e5]]) * anti_reverse.group1().xw())
                - (Simd32x2::from(anti_reverse[e235]) * Simd32x2::from([self[e41], self[e423]]))
                - (Simd32x2::from(anti_reverse[e315]) * Simd32x2::from([self[e42], self[e431]]))
                - (Simd32x2::from(anti_reverse[e125]) * Simd32x2::from([self[e43], self[e412]]))
                - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e23], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e31], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e12], anti_reverse[e435]]))
                - (Simd32x2::from(self[e235]) * Simd32x2::from([anti_reverse[e41], anti_reverse[e423]]))
                - (Simd32x2::from(self[e315]) * Simd32x2::from([anti_reverse[e42], anti_reverse[e431]]))
                - (Simd32x2::from(self[e125]) * Simd32x2::from([anti_reverse[e43], anti_reverse[e412]]))
                - (Simd32x2::from([anti_reverse[e423], anti_reverse[e45]]) * self.group3().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e15] * self[e1234]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]),
                (anti_reverse[e25] * self[e1234]) + (anti_reverse[e415] * self[e3]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]),
                (anti_reverse[e35] * self[e1234]) + (anti_reverse[e425] * self[e1]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]),
                -(anti_reverse[e3] * self[e412]) - (anti_reverse[e4] * self[e321]) - (anti_reverse[e435] * self[e412]) - (anti_reverse[e1234] * self[e45]),
            ]) + (Simd32x4::from([self[e5], self[e125], self[e235], self[e4125]]) * anti_reverse.group7().xxy().with_w(anti_reverse[e43]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse.group5().xxy().with_w(anti_reverse[e42]))
                + (Simd32x4::from([self[e315], self[e5], self[e5], self[e41]]) * anti_reverse.group7().zyz().with_w(anti_reverse[e23]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e12]]) * anti_reverse.group5().zyz().with_w(anti_reverse[e43]))
                + (anti_reverse.group0().xx().with_zw(anti_reverse[scalar], anti_reverse[e12345]) * self.group9().xyz().with_w(self[e4]))
                + (anti_reverse.group0().yy().with_zw(anti_reverse[e12345], self[e12345]) * self.group1().xyz().with_w(anti_reverse[e4]))
                + (self.group0().xx().with_zw(self[scalar], anti_reverse[e41]) * anti_reverse.group9().xyz().with_w(self[e23]))
                + (self.group0().yy().with_zw(self[e12345], anti_reverse[e41]) * anti_reverse.group1().xyz().with_w(self[e4235]))
                + (anti_reverse.group3().zx().with_zw(anti_reverse[e3215], self[e1]) * self.group4().yzz().with_w(anti_reverse[e423]))
                + (anti_reverse.group3().ww().with_zw(anti_reverse[e4315], self[e2]) * self.group5().xyx().with_w(anti_reverse[e431]))
                + (anti_reverse.group9().zx().with_zw(anti_reverse[e45], self[e3]) * self.group5().yzz().with_w(anti_reverse[e412]))
                + (anti_reverse.group9().ww().with_zw(anti_reverse[e25], self[e43]) * self.group4().xyx().with_w(anti_reverse[e12]))
                + (self.group8() * anti_reverse.group1().www()).with_w(anti_reverse[e45] * self[e1234])
                + (anti_reverse.group4().yzx() * self.group3().zxy()).with_w(anti_reverse[e42] * self[e31])
                + (anti_reverse.group8().yzx() * self.group7().zxy()).with_w(anti_reverse[e31] * self[e42])
                + (anti_reverse.group1().zxy() * self.group6().yzx()).with_w(anti_reverse[e321] * self[e4])
                - (Simd32x4::from([anti_reverse[e5], anti_reverse[e5], anti_reverse[e5], anti_reverse[e1]]) * self.group7().with_w(self[e423]))
                - (Simd32x4::from([anti_reverse[e1234], anti_reverse[e1234], anti_reverse[e1234], anti_reverse[e425]]) * self.group3().xyz().with_w(self[e431]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e435]]) * anti_reverse.group8().xxy().with_w(anti_reverse[e412]))
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], anti_reverse[e1234]]) * anti_reverse.group4().zyz().with_w(self[scalar]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], anti_reverse[e4235]]) * anti_reverse.group8().zyz().with_w(self[e41]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * anti_reverse.group4().xxy().with_w(anti_reverse[scalar]))
                - (anti_reverse.group9().yzxz() * self.group5().zxy().with_w(self[e43]))
                - (anti_reverse.group5().yzx() * self.group9().zxy()).with_w(anti_reverse[e423] * self[e415])
                - (anti_reverse.group7().yzx() * self.group8().zxy()).with_w(anti_reverse[e431] * self[e425])
                - (self.group4().zxy() * anti_reverse.group3().yzx()).with_w(anti_reverse[e4315] * self[e42])
                - (anti_reverse.group1().yzx() * self.group6().zxy()).with_w(anti_reverse[e415] * self[e423])
                - (anti_reverse.group6().yzx() * self.group1().zxy()).with_w(anti_reverse[e2] * self[e431]),
            // e5
            (anti_reverse[e12345] * self[e5])
                + (anti_reverse[e1] * self[e235])
                + (anti_reverse[e2] * self[e315])
                + (anti_reverse[e3] * self[e125])
                + (anti_reverse[e5] * self[e12345])
                + (anti_reverse[e5] * self[e321])
                + (anti_reverse[e15] * self[e23])
                + (anti_reverse[e25] * self[e31])
                + (anti_reverse[e35] * self[e12])
                + (anti_reverse[e23] * self[e15])
                + (anti_reverse[e31] * self[e25])
                + (anti_reverse[e12] * self[e35])
                + (anti_reverse[e4235] * self[e15])
                + (anti_reverse[e4315] * self[e25])
                + (anti_reverse[e4125] * self[e35])
                + (anti_reverse[e3215] * self[e45])
                - (anti_reverse[scalar] * self[e3215])
                - (anti_reverse[e15] * self[e4235])
                - (anti_reverse[e25] * self[e4315])
                - (anti_reverse[e35] * self[e4125])
                - (anti_reverse[e45] * self[e3215])
                - (anti_reverse[e415] * self[e235])
                - (anti_reverse[e425] * self[e315])
                - (anti_reverse[e435] * self[e125])
                - (anti_reverse[e321] * self[e5])
                - (anti_reverse[e235] * self[e1])
                - (anti_reverse[e235] * self[e415])
                - (anti_reverse[e315] * self[e2])
                - (anti_reverse[e315] * self[e425])
                - (anti_reverse[e125] * self[e3])
                - (anti_reverse[e125] * self[e435])
                - (anti_reverse[e3215] * self[scalar]),
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
                (anti_reverse[e45] * self[e415]) + (anti_reverse[e415] * self[e45]) + (anti_reverse[e435] * self[e4315]) + (anti_reverse[e4125] * self[e425]),
                (anti_reverse[e45] * self[e425]) + (anti_reverse[e415] * self[e4125]) + (anti_reverse[e425] * self[e45]) + (anti_reverse[e4235] * self[e435]),
                (anti_reverse[e45] * self[e435]) + (anti_reverse[e425] * self[e4235]) + (anti_reverse[e435] * self[e45]) + (anti_reverse[e4315] * self[e415]),
                -(anti_reverse[e5] * self[e45]) - (anti_reverse[e425] * self[e25]) - (anti_reverse[e435] * self[e35]) - (anti_reverse[e321] * self[e3215]),
            ]) + (Simd32x4::from([anti_reverse[e5], anti_reverse[e5], anti_reverse[e5], self[e15]]) * self.group4().with_w(anti_reverse[e1]))
                + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e4125]]) * anti_reverse.group8().zyz().with_w(anti_reverse[e125]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4315]]) * anti_reverse.group8().xxy().with_w(anti_reverse[e315]))
                + (anti_reverse.group0().yy().with_zw(anti_reverse[e12345], anti_reverse[scalar]) * self.group9().xyz().with_w(self[e5]))
                + (self.group0().yy().with_zw(self[e12345], anti_reverse[e12345]) * anti_reverse.group9().xyz().with_w(self[e3215]))
                + (anti_reverse.group3().zx().with_zw(anti_reverse[e3215], self[e5]) * self.group7().yzz().with_w(anti_reverse[e45]))
                + (anti_reverse.group9().ww().with_zw(anti_reverse[e25], self[e35]) * self.group7().xyx().with_w(anti_reverse[e3]))
                + (anti_reverse.group4().yzx() * self.group8().zxy()).with_w(anti_reverse[e5] * self[scalar])
                + (anti_reverse.group5().yzx() * self.group1().zxy()).with_w(anti_reverse[e3215] * self[e12345])
                + (anti_reverse.group7().yzx() * self.group3().zxy()).with_w(anti_reverse[e235] * self[e4235])
                + (self.group5().zxy() * anti_reverse.group1().yzx()).with_w(anti_reverse[e2] * self[e25])
                + (self.group1().www() * anti_reverse.group3().xyz()).with_w(anti_reverse[e3215] * self[e321])
                - (Simd32x4::from([anti_reverse[e1234], anti_reverse[e1234], anti_reverse[e1234], self[e425]]) * self.group8().with_w(anti_reverse[e25]))
                - (Simd32x4::from([self[e2], self[e321], self[e321], self[e12]]) * anti_reverse.group5().zyz().with_w(anti_reverse[e125]))
                - (Simd32x4::from([self[e5], self[e125], self[e235], self[e125]]) * anti_reverse.group4().xxy().with_w(anti_reverse[e12]))
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], anti_reverse[e4315]]) * anti_reverse.group7().zyz().with_w(self[e315]))
                - (Simd32x4::from([self[e321], self[e3], self[e1], self[e31]]) * anti_reverse.group5().xxy().with_w(anti_reverse[e315]))
                - (Simd32x4::from([self[e315], self[e5], self[e5], self[e23]]) * anti_reverse.group4().zyz().with_w(anti_reverse[e235]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], anti_reverse[e4235]]) * anti_reverse.group7().xxy().with_w(self[e235]))
                - (anti_reverse.group0().xx().with_zw(anti_reverse[scalar], anti_reverse[e23]) * self.group1().xyz().with_w(self[e235]))
                - (self.group0().xx().with_zw(self[scalar], anti_reverse[e31]) * anti_reverse.group1().xyz().with_w(self[e315]))
                - (anti_reverse.group1().zx().with_zw(anti_reverse[e321], self[e415]) * self.group5().yzz().with_w(anti_reverse[e15]))
                - (anti_reverse.group6().ww().with_zw(anti_reverse[e2], self[e1]) * self.group5().xyx().with_w(anti_reverse[e15]))
                - (anti_reverse.group8().yzx() * self.group4().zxy()).with_w(anti_reverse[e4125] * self[e125])
                - (self.group7().zxy() * anti_reverse.group3().yzx()).with_w(anti_reverse[e25] * self[e2])
                - (anti_reverse.group1().www() * self.group3().xyz()).with_w(anti_reverse[e35] * self[e3])
                - (anti_reverse.group6().yzx() * self.group9().zxy()).with_w(anti_reverse[e35] * self[e435])
                - (anti_reverse.group9().yzx() * self.group6().zxy()).with_w(anti_reverse[e415] * self[e15]),
            // e1234
            (anti_reverse[scalar] * self[e4])
                + (anti_reverse[e12345] * self[e1234])
                + (anti_reverse[e4] * self[scalar])
                + (anti_reverse[e4] * self[e45])
                + (anti_reverse[e41] * self[e1])
                + (anti_reverse[e42] * self[e2])
                + (anti_reverse[e43] * self[e3])
                + (anti_reverse[e321] * self[e1234])
                + (anti_reverse[e4235] * self[e423])
                + (anti_reverse[e4315] * self[e431])
                + (anti_reverse[e4125] * self[e412])
                + (anti_reverse[e1234] * self[e12345])
                - (anti_reverse[e1] * self[e41])
                - (anti_reverse[e2] * self[e42])
                - (anti_reverse[e3] * self[e43])
                - (anti_reverse[e45] * self[e4])
                - (anti_reverse[e41] * self[e415])
                - (anti_reverse[e42] * self[e425])
                - (anti_reverse[e43] * self[e435])
                - (anti_reverse[e23] * self[e423])
                - (anti_reverse[e31] * self[e431])
                - (anti_reverse[e12] * self[e412])
                - (anti_reverse[e415] * self[e41])
                - (anti_reverse[e425] * self[e42])
                - (anti_reverse[e435] * self[e43])
                - (anti_reverse[e423] * self[e23])
                - (anti_reverse[e423] * self[e4235])
                - (anti_reverse[e431] * self[e31])
                - (anti_reverse[e431] * self[e4315])
                - (anti_reverse[e412] * self[e12])
                - (anti_reverse[e412] * self[e4125])
                - (anti_reverse[e1234] * self[e321]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_anti_product[scalar], 0.0]),
            // e1, e2, e3, e4
            geometric_anti_product.group1(),
            // e5
            geometric_anti_product[e5],
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
            geometric_anti_product.group9(),
            // e1234
            geometric_anti_product[e1234],
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Plane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for RoundPoint {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for Scalar {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Scalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Sphere {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for VersorEven {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       59        0
    //    simd3        0        3        0
    //    simd4       13       12        0
    // Totals...
    // yes simd       66       74        0
    //  no simd      105      116        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e12345] * self[e12345]) + (anti_reverse[e321] * self[e321]) + (anti_reverse[e5] * self[e4]) + (anti_reverse[e4] * self[e5])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e12345] * self[e5])
                    + (anti_reverse[e5] * self[e12345])
                    + (anti_reverse[e5] * self[e321])
                    + (anti_reverse[e1] * self[e235])
                    + (anti_reverse[e2] * self[e315])
                    + (anti_reverse[e3] * self[e125])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e321] * self[e5])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e435])
                    - (anti_reverse[e125] * self[e3]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e315] * self[e412]) + (anti_reverse[e1] * self[e12345]) + (anti_reverse[e3] * self[e425]) + (anti_reverse[e4] * self[e235]),
                (anti_reverse[e125] * self[e423]) + (anti_reverse[e1] * self[e435]) + (anti_reverse[e2] * self[e12345]) + (anti_reverse[e4] * self[e315]),
                (anti_reverse[e235] * self[e431]) + (anti_reverse[e2] * self[e415]) + (anti_reverse[e3] * self[e12345]) + (anti_reverse[e4] * self[e125]),
                -(anti_reverse[e1] * self[e423]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]) - (anti_reverse[e4] * self[e321]),
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * anti_reverse.group1().xxy().with_w(anti_reverse[e12345]))
                + (Simd32x4::from([self[e2], self[e321], self[e321], self[e4]]) * anti_reverse.group1().zyzw())
                + (anti_reverse.group0().xxyx() * self.group2().wzx().with_w(self[e1]))
                + (anti_reverse.group0().zyzy() * self.group2().yww().with_w(self[e2]))
                + (anti_reverse.group0().wwwz() * self.group3().xyzz())
                + (anti_reverse.group1().www() * self.group1().xyz()).with_w(anti_reverse[e4] * self[e12345])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * anti_reverse.group2().zyz().with_w(anti_reverse[e415]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e435]]) * anti_reverse.group2().xxy().with_w(anti_reverse[e412]))
                - (anti_reverse.group0().yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().xyzy() * anti_reverse.group2().www().with_w(anti_reverse[e425]))
                - (anti_reverse.group1().yzx() * self.group3().zxy()).with_w(anti_reverse[e431] * self[e425])
                - (anti_reverse.group3().yzx() * self.group1().zxy()).with_w(anti_reverse[e435] * self[e412]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                        - f32::powi(self[e12345], 2)
                        - f32::powi(self[e415], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e435], 2)
                        - 2.0 * (self[e423] * self[e235])
                        - 2.0 * (self[e431] * self[e315])
                        - 2.0 * (self[e412] * self[e125])
                        - 2.0 * (self[e5] * self[e4]),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       43        0
    //    simd3        0        3        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       57       62        0
    //  no simd      105      116        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    + (anti_reverse[e4235] * self[e4235])
                    + (anti_reverse[e4315] * self[e4315])
                    + (anti_reverse[e4125] * self[e4125])
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[e1234] * self[e3215])
                    - (anti_reverse[e3215] * self[e1234]),
            ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12])
                    + (anti_reverse[e4235] * self[e15])
                    + (anti_reverse[e4315] * self[e25])
                    + (anti_reverse[e4125] * self[e35])
                    + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[scalar] * self[e3215])
                    - (anti_reverse[e45] * self[e3215])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125])
                    - (anti_reverse[e3215] * self[scalar]),
            ),
            // e1, e2, e3, e4
            (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e41]]) * anti_reverse.group2().zyz().with_w(anti_reverse[e23]))
                + (Simd32x4::from([self[scalar], self[e12], self[e23], self[e42]]) * anti_reverse.group3().xxy().with_w(anti_reverse[e31]))
                + (Simd32x4::from([self[e31], self[scalar], self[scalar], self[e43]]) * anti_reverse.group3().zyz().with_w(anti_reverse[e12]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e31]]) * anti_reverse.group1().xxy().with_w(anti_reverse[e42]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4125]]) * anti_reverse.group2().xxy().with_w(anti_reverse[e43]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e4315]]) * anti_reverse.group1().zyz().with_w(anti_reverse[e42]))
                + (anti_reverse.group0().yzxx() * self.group2().zxy().with_w(self[e23]))
                + (anti_reverse.group0().wwwx() * self.group3().xyzx())
                + (self.group1().xyzz() * anti_reverse.group1().www().with_w(anti_reverse[e43]))
                + (anti_reverse.group3().www() * self.group0().xyz()).with_w(anti_reverse[e45] * self[e1234])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[scalar]]) * anti_reverse.group0().zyz().with_w(anti_reverse[e1234]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * anti_reverse.group0().xxyw())
                - (anti_reverse.group3().yzxz() * self.group1().zxy().with_w(self[e43]))
                - (self.group0().zxyx() * anti_reverse.group2().yzx().with_w(anti_reverse[e4235]))
                - (anti_reverse.group1().yzx() * self.group3().zxy()).with_w(anti_reverse[e1234] * self[e45])
                - (anti_reverse.group2().www() * self.group2().xyz()).with_w(anti_reverse[e4315] * self[e42]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + 2.0 * (self[e1234] * self[e3215])
                        + f32::powi(self[scalar], 2)
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2)
                        - f32::powi(self[e45], 2)
                        - f32::powi(self[e4235], 2)
                        - f32::powi(self[e4315], 2)
                        - f32::powi(self[e4125], 2),
                ),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
