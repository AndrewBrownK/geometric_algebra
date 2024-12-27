// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 59
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        15      21       0
//  Average:        23      29       0
//  Maximum:       191     221       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        17      26       0
//  Average:        34      44       0
//  Maximum:       372     404       0
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        return Origin::from_groups(
            // e4
            (anti_reverse[e41] * self[e23])
                + (anti_reverse[e42] * self[e31])
                + (anti_reverse[e43] * self[e12])
                + (anti_reverse[e23] * self[e41])
                + (anti_reverse[e31] * self[e42])
                + (anti_reverse[e12] * self[e43]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotor {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       34        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       38       50        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
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
                (anti_reverse[e43] * self[e12]) + (anti_reverse[e23] * self[e41]) + (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e41] * self[e23])
                + (self.group0().yzx() * anti_reverse.group2().zxy()).with_w(anti_reverse[e42] * self[e31]),
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
                        + f32::powi(self[scalar], 2),
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       15       17        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w((anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[scalar] * self[scalar])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       15       25        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(anti_reverse[e12] * self[e12]) - (anti_reverse[e45] * self[e45]) - (anti_reverse[scalar] * self[scalar]), 0.0, 0.0, 0.0])
                + (anti_reverse.group0().xxyz() * self.group0().xwww())
                + (anti_reverse.group0().ywww() * self.group0().yxyz()),
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
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        return Origin::from_groups(
            // e4
            (anti_reverse[e41] * self[e23])
                + (anti_reverse[e42] * self[e31])
                + (anti_reverse[e43] * self[e12])
                + (anti_reverse[e23] * self[e41])
                + (anti_reverse[e31] * self[e42])
                + (anti_reverse[e12] * self[e43]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversion {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       25       32        0
    //  no simd       34       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2(),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e435] * self[e435]) - (anti_reverse[e1] * self[e1]) - (anti_reverse[e2] * self[e2]) - (anti_reverse[e3] * self[e3]),
                (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e3] * self[e425]),
                (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e1] * self[e435]),
                (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e2] * self[e415]),
            ]) + (anti_reverse.group0().wxxy() * self.group0().ww().with_zw(self[e3], self[e1]))
                - (Simd32x4::from([anti_reverse[e425], anti_reverse[e2], self[e415], self[e425]]) * self.group0().yz().with_zw(anti_reverse[e3], anti_reverse[e1]))
                - (Simd32x4::from([self[e415], self[e3], self[e1], self[e2]]) * anti_reverse.group0().xyzx()),
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
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse =
            AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1());
        return Origin::from_groups(
            // e4
            (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3]) + (anti_reverse[e321] * self[e4])
                - (anti_reverse[e4] * self[e321])
                - (anti_reverse[e1] * self[e423])
                - (anti_reverse[e2] * self[e431])
                - (anti_reverse[e3] * self[e412]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       40        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       36       47        0
    //  no simd       51       66        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e5] * self[e4]) + (anti_reverse[e4] * self[e5])
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
                (anti_reverse[e423] * self[e5]) + (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) + (anti_reverse[e4] * self[e235]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e431] * self[e5]) + (anti_reverse[e125] * self[e423]) + (anti_reverse[e4] * self[e315]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e412] * self[e5]) + (anti_reverse[e235] * self[e431]) + (anti_reverse[e4] * self[e125]),
                -(anti_reverse[e431] * self[e425]) - (anti_reverse[e412] * self[e435]),
            ]) - (Simd32x4::from([self[e431], self[e4], self[e4], anti_reverse[e423]]) * anti_reverse.group2().zyz().with_w(self[e415]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * anti_reverse.group2().xxy().with_w(anti_reverse[e435]))
                - (self.group0().xyzy() * anti_reverse.group0().www().with_w(anti_reverse[e425]))
                - (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e415] * self[e423]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[e415], 2)
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDualNum {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * self[scalar] * -2.0);
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiFlector {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       17       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w((anti_reverse[e321] * self[e321]) - (anti_reverse[e1] * self[e1]) - (anti_reverse[e2] * self[e2]) - (anti_reverse[e3] * self[e3])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125]) + (anti_reverse[e5] * self[e321])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e3])
                    - (anti_reverse[e321] * self[e5]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiLine {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd3        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       13       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w((anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       17       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w((anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[scalar] * self[scalar])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12])
                    - (anti_reverse[scalar] * self[e3215])
                    - (anti_reverse[e3215] * self[scalar]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[scalar], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar]);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[scalar] * self[scalar]),
                (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]),
                (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]),
                (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       26        0
    //  no simd       21       29        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1());
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e321] * self[e321])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3]),
                (anti_reverse[e415] * self[e321]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e3] * self[e425])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e2] * self[e435]),
                (anti_reverse[e415] * self[e3]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e1] * self[e435])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e3] * self[e415]),
                (anti_reverse[e425] * self[e1]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e2] * self[e415])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e1] * self[e425]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Origin::from_groups(
            // e4
            (anti_reverse[e41] * self[e23])
                + (anti_reverse[e42] * self[e31])
                + (anti_reverse[e43] * self[e12])
                + (anti_reverse[e23] * self[e41])
                + (anti_reverse[e31] * self[e42])
                + (anti_reverse[e12] * self[e43])
                - (anti_reverse[scalar] * self[e1234])
                - (anti_reverse[e1234] * self[scalar]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Circle {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       27       38        0
    //  no simd       36       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                -(anti_reverse[e423] * self[e235])
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
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e423] * self[e415])
                - (anti_reverse.group2().zxy() * self.group0().yzx()).with_w(anti_reverse[e431] * self[e425]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[e415], 2)
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       23        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
                anti_reverse[e321] * self[e415],
                anti_reverse[e321] * self[e425],
                anti_reverse[e321] * self[e435],
            ]) + (Simd32x4::from(self[e321]) * anti_reverse.group0().wxyz()),
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
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       14       24        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) - (anti_reverse[e431] * self[e125]) - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]) - (anti_reverse[e412] * self[e235]) - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]) - (anti_reverse[e423] * self[e315]) - (anti_reverse[e315] * self[e423]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        return Origin::from_groups(
            // e4
            -(anti_reverse[e423] * self[e415])
                - (anti_reverse[e431] * self[e425])
                - (anti_reverse[e412] * self[e435])
                - (anti_reverse[e415] * self[e423])
                - (anti_reverse[e425] * self[e431])
                - (anti_reverse[e435] * self[e412]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleOrthogonalOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       19        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       26        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e321] * self[e321])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) - (anti_reverse[e431] * self[e125]) - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]) - (anti_reverse[e412] * self[e235]) - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]) - (anti_reverse[e423] * self[e315]) - (anti_reverse[e315] * self[e423]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotor {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       34        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       38       50        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e12345] * self[e12345])
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
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e423] * self[e415])
                - (self.group0().yzx() * anti_reverse.group2().zxy()).with_w(anti_reverse[e431] * self[e425]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[e415], 2)
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       15       17        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0)
                .with_w((anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       12       25        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
                anti_reverse[e321] * self[e415],
                anti_reverse[e321] * self[e425],
                anti_reverse[e321] * self[e435],
            ]) + (Simd32x4::from(self[e321]) * anti_reverse.group0().wxyz()),
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
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        return Origin::from_groups(
            // e4
            -(anti_reverse[e423] * self[e415])
                - (anti_reverse[e431] * self[e425])
                - (anti_reverse[e412] * self[e435])
                - (anti_reverse[e415] * self[e423])
                - (anti_reverse[e425] * self[e431])
                - (anti_reverse[e435] * self[e412]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Dipole {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       19        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       26        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    - (anti_reverse[e45] * self[e45]),
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e35] * self[e42]) - (anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e15] * self[e43]) - (anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e25] * self[e41]) - (anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       14       23        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(anti_reverse[e12] * self[e12]) - (anti_reverse[e45] * self[e45]), 0.0, 0.0, 0.0])
                + (anti_reverse.group0().xxyz() * self.group0().xwww())
                + (anti_reverse.group0().ywww() * self.group0().yxyz()),
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
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       14       24        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43]),
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e35] * self[e42]) - (anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e15] * self[e43]) - (anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e25] * self[e41]) - (anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversion {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       30        0
    //    simd3        0        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       33       40        0
    //  no simd       57       68        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
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
                (anti_reverse[e4235] * self[e15]) + (anti_reverse[e4315] * self[e25]) + (anti_reverse[e4125] * self[e35]) + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[e45] * self[e3215])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125]),
            ),
            // e1, e2, e3, e4
            (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e4125]]) * anti_reverse.group1().zyz().with_w(anti_reverse[e43]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4315]]) * anti_reverse.group1().xxy().with_w(anti_reverse[e42]))
                + (anti_reverse.group0().yzxx() * self.group1().zxy().with_w(self[e4235]))
                + (anti_reverse.group2().www() * self.group0().xyz()).with_w(anti_reverse[e45] * self[e1234])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[e41]]) * anti_reverse.group0().zyz().with_w(anti_reverse[e4235]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e45]]) * anti_reverse.group0().xxy().with_w(anti_reverse[e1234]))
                - (self.group0().zxyy() * anti_reverse.group1().yzx().with_w(anti_reverse[e4315]))
                - (anti_reverse.group1().www() * self.group1().xyz()).with_w(anti_reverse[e4125] * self[e43]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) + 2.0 * (self[e1234] * self[e3215])
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        0        1        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       34       49        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e4315] * self[e4315]) + (anti_reverse[e4125] * self[e4125]),
                anti_reverse[e4315] * self[e12] * -1.0,
                anti_reverse[e4125] * self[e23] * -1.0,
                anti_reverse[e4235] * self[e31] * -1.0,
            ]) + (Simd32x4::from([self[e31], self[e4315], self[e45], self[e45]]) * anti_reverse.group0().yzyz())
                + (Simd32x4::from([self[e4235], self[e31], self[e12], self[e23]]) * anti_reverse.group2().xzxy())
                + (anti_reverse.group0().xxxy() * self.group0().xw().with_zw(self[e4125], self[e4235]))
                + (anti_reverse.group0().zwww() * self.group0().zxyz())
                - (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse.group0().wyzx()),
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
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       28       34        0
    //  no simd       28       40        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    - (anti_reverse[e3215] * self[e1234])
                    - (anti_reverse[e1234] * self[e3215]),
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e3215] * self[e41]) + (anti_reverse[e15] * self[e1234]) + (anti_reverse[e35] * self[e42])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e1234] * self[e15]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e3215] * self[e42]) + (anti_reverse[e15] * self[e43]) + (anti_reverse[e25] * self[e1234])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e1234] * self[e25]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e3215] * self[e43]) + (anti_reverse[e25] * self[e41]) + (anti_reverse[e35] * self[e1234])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e1234] * self[e35]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse =
            DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1());
        return Origin::from_groups(
            // e4
            (anti_reverse[e41] * self[e4235]) + (anti_reverse[e42] * self[e4315]) + (anti_reverse[e43] * self[e4125]) + (anti_reverse[e45] * self[e1234])
                - (anti_reverse[e1234] * self[e45])
                - (anti_reverse[e4235] * self[e41])
                - (anti_reverse[e4315] * self[e42])
                - (anti_reverse[e4125] * self[e43]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       40        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       36       47        0
    //  no simd       51       66        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
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
                    - (anti_reverse[e3215] * self[e1234])
                    - (anti_reverse[e1234] * self[e3215]),
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
                -(anti_reverse[e41] * self[e3215]) - (anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]) - (anti_reverse[e1234] * self[e15]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e42] * self[e3215]) - (anti_reverse[e35] * self[e41]) - (anti_reverse[e1234] * self[e25]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e43] * self[e3215]) - (anti_reverse[e15] * self[e42]) - (anti_reverse[e1234] * self[e35]),
                (anti_reverse[e42] * self[e31]) + (anti_reverse[e43] * self[e12]),
            ]) + (Simd32x4::from([self[e42], self[e1234], self[e1234], anti_reverse[e41]]) * anti_reverse.group2().zyz().with_w(self[e23]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e43]]) * anti_reverse.group2().xxy().with_w(anti_reverse[e12]))
                + (self.group0().xyzy() * anti_reverse.group0().www().with_w(anti_reverse[e31]))
                + (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e23] * self[e41]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + 2.0 * (self[e3215] * self[e1234])
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2),
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       27       38        0
    //  no simd       36       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
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
                    + (anti_reverse[e35] * self[e43]),
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
                (anti_reverse[e43] * self[e12]) + (anti_reverse[e23] * self[e41]) + (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (anti_reverse.group0().yzx() * self.group2().zxy()).with_w(anti_reverse[e41] * self[e23])
                + (anti_reverse.group2().zxy() * self.group0().yzx()).with_w(anti_reverse[e42] * self[e31]),
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
                        + f32::powi(self[e12], 2),
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * self[e12345] * 2.0);
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Flector {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       17       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0)
                .with_w((anti_reverse[e4235] * self[e4235]) + (anti_reverse[e4315] * self[e4315]) + (anti_reverse[e4125] * self[e4125]) - (anti_reverse[e45] * self[e45])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e4235] * self[e15]) + (anti_reverse[e4315] * self[e25]) + (anti_reverse[e4125] * self[e35]) + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125])
                    - (anti_reverse[e45] * self[e3215]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(-f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Line {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd3        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       13       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(-(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0() + Simd32x3::from(0.0).with_w(-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Motor {
    type Output = Motor;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<AntiConstraintViolationPrefixOrPostfix> for Motor {
    fn div_assign(&mut self, _rhs: AntiConstraintViolationPrefixOrPostfix) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       17       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0)
                .with_w((anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e12345] * self[e5]) + (anti_reverse[e5] * self[e12345])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) - f32::powi(self[e12345], 2)),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<AntiConstraintViolationPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiConstraintViolationPrefixOrPostfix) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      120      144        0
    //    simd2       16       16        0
    //    simd3        0       16        0
    //    simd4       55       45        0
    // Totals...
    // yes simd      191      221        0
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
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (anti_reverse[e12345] * self[scalar])
                    + (anti_reverse[e4] * self[e3215])
                    + (anti_reverse[e5] * self[e1234])
                    + (anti_reverse[e4315] * self[e2])
                    + (anti_reverse[e4125] * self[e3])
                    + (anti_reverse[e3215] * self[e4])
                    - (anti_reverse[e41] * self[e235])
                    - (anti_reverse[e42] * self[e315])
                    - (anti_reverse[e43] * self[e125])
                    - (anti_reverse[e45] * self[e321])
                    - (anti_reverse[e415] * self[e23])
                    - (anti_reverse[e425] * self[e31])
                    - (anti_reverse[e435] * self[e12])
                    - (anti_reverse[e321] * self[e45])
                    - (anti_reverse[e315] * self[e42])
                    - (anti_reverse[e125] * self[e43]),
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e321] * self[e321])
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3])
                    - (anti_reverse[e1234] * self[e3215])
                    - (anti_reverse[e3215] * self[e1234]),
            ]) + (Simd32x2::from(self[e12345]) * anti_reverse.group0())
                + (Simd32x2::from([anti_reverse[e2], anti_reverse[e4235]]) * self.group9().zy())
                + (Simd32x2::from([anti_reverse[e3], anti_reverse[e4315]]) * self.group9().wz())
                + (Simd32x2::from([anti_reverse[e4235], anti_reverse[e5]]) * self.group1().xw())
                + (Simd32x2::from([self[e5], self[e4125]]) * anti_reverse.group9().xw())
                + (Simd32x2::from([self[e4235], self[e5]]) * anti_reverse.group1().xw())
                - (Simd32x2::from(anti_reverse[e423]) * Simd32x2::from([self[e15], self[e235]]))
                - (Simd32x2::from(anti_reverse[e431]) * Simd32x2::from([self[e25], self[e315]]))
                - (Simd32x2::from(anti_reverse[e412]) * Simd32x2::from([self[e35], self[e125]]))
                - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e23], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e31], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e12], anti_reverse[e435]]))
                - (Simd32x2::from(self[e423]) * Simd32x2::from([anti_reverse[e15], anti_reverse[e235]]))
                - (Simd32x2::from(self[e431]) * Simd32x2::from([anti_reverse[e25], anti_reverse[e315]]))
                - (Simd32x2::from(self[e412]) * Simd32x2::from([anti_reverse[e35], anti_reverse[e125]]))
                - (Simd32x2::from([anti_reverse[e235], anti_reverse[e45]]) * self.group3().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e415] * self[e321]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e3215] * self[e41]),
                (anti_reverse[e415] * self[e3]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e3215] * self[e42]),
                (anti_reverse[e425] * self[e1]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e3215] * self[e43]),
                -(anti_reverse[e1234] * self[e45]) - (anti_reverse[e4235] * self[e41]) - (anti_reverse[e4315] * self[e42]) - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from([self[e5], self[e125], self[e235], anti_reverse[e41]]) * anti_reverse.group7().xxy().with_w(self[e23]))
                + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e1]]) * anti_reverse.group4().zyz().with_w(anti_reverse[e423]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e2]]) * anti_reverse.group5().xxy().with_w(anti_reverse[e431]))
                + (Simd32x4::from([self[e315], self[e5], self[e5], anti_reverse[e42]]) * anti_reverse.group7().zyz().with_w(self[e31]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e43]]) * anti_reverse.group4().xxy().with_w(anti_reverse[e12]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e3]]) * anti_reverse.group5().zyz().with_w(anti_reverse[e412]))
                + (anti_reverse.group0().xx().with_zw(anti_reverse[scalar], anti_reverse[e12345]) * self.group9().yzw().with_w(self[e4]))
                + (anti_reverse.group0().yy().with_zw(anti_reverse[e12345], self[e12345]) * self.group1().xyz().with_w(anti_reverse[e4]))
                + (self.group0().xx().with_zw(self[scalar], anti_reverse[e23]) * anti_reverse.group9().yzw().with_w(self[e41]))
                + (self.group0().yy().with_zw(self[e12345], anti_reverse[e31]) * anti_reverse.group1().xyz().with_w(self[e42]))
                + (anti_reverse.group3().ww().with_zw(anti_reverse[e4315], self[e4315]) * self.group5().xyx().with_w(anti_reverse[e42]))
                + (anti_reverse.group9().wy().with_zw(anti_reverse[e45], self[e4125]) * self.group5().yzz().with_w(anti_reverse[e43]))
                + (self.group8() * anti_reverse.group1().www()).with_w(anti_reverse[e45] * self[e1234])
                + (anti_reverse.group8().yzx() * self.group7().zxy()).with_w(anti_reverse[e43] * self[e12])
                + (self.group4().zxy() * anti_reverse.group3().yzx()).with_w(anti_reverse[e41] * self[e4235])
                + (anti_reverse.group1().zxy() * self.group6().yzx()).with_w(anti_reverse[e321] * self[e4])
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e425]]) * anti_reverse.group8().xxy().with_w(anti_reverse[e431]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e435]]) * anti_reverse.group8().zyz().with_w(anti_reverse[e412]))
                - (anti_reverse.group3().zx().with_zw(anti_reverse[e1234], anti_reverse[e415]) * self.group4().yzz().with_w(self[e423]))
                - (anti_reverse.group9().xx().with_zw(anti_reverse[e42], anti_reverse[e1]) * self.group4().xyx().with_w(self[e423]))
                - (Simd32x3::from(anti_reverse[e5]) * self.group7()).with_w(anti_reverse[e425] * self[e431])
                - (Simd32x3::from(self[e3215]) * anti_reverse.group3().xyz()).with_w(anti_reverse[e435] * self[e412])
                - (anti_reverse.group4().yzx() * self.group3().zxy()).with_w(anti_reverse[scalar] * self[e1234])
                - (anti_reverse.group5().yzx() * self.group9().wyz()).with_w(anti_reverse[e1234] * self[scalar])
                - (anti_reverse.group7().yzx() * self.group8().zxy()).with_w(anti_reverse[e423] * self[e415])
                - (self.group5().zxy() * anti_reverse.group9().zwy()).with_w(anti_reverse[e2] * self[e431])
                - (anti_reverse.group1().yzx() * self.group6().zxy()).with_w(anti_reverse[e3] * self[e412])
                - (anti_reverse.group6().yzx() * self.group1().zxy()).with_w(anti_reverse[e4] * self[e321]),
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
                - (anti_reverse[e45] * self[e3215])
                - (anti_reverse[e15] * self[e4235])
                - (anti_reverse[e25] * self[e4315])
                - (anti_reverse[e35] * self[e4125])
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
            Simd32x4::from([
                (anti_reverse[e4125] * self[e412])
                    - (anti_reverse[e415] * self[e41])
                    - (anti_reverse[e425] * self[e42])
                    - (anti_reverse[e435] * self[e43])
                    - (anti_reverse[e412] * self[e4125]),
                (anti_reverse[e2] * self[e12])
                    + (anti_reverse[e5] * self[e41])
                    + (anti_reverse[e45] * self[e415])
                    + (anti_reverse[e435] * self[e4315])
                    + (anti_reverse[e431] * self[e35]),
                (anti_reverse[e5] * self[e42])
                    + (anti_reverse[e45] * self[e425])
                    + (anti_reverse[e415] * self[e4125])
                    + (anti_reverse[e425] * self[e45])
                    + (anti_reverse[e4235] * self[e435]),
                (anti_reverse[e5] * self[e43])
                    + (anti_reverse[e45] * self[e435])
                    + (anti_reverse[e425] * self[e4235])
                    + (anti_reverse[e435] * self[e45])
                    + (anti_reverse[e4315] * self[e415]),
            ]) + (Simd32x4::from([anti_reverse[e12345], anti_reverse[e235], anti_reverse[e4315], anti_reverse[e4125]]) * self.group9().xx().with_zw(self[e12345], self[e12345]))
                + (Simd32x4::from([anti_reverse[e4], anti_reverse[e125], self[e43], self[e41]]) * self.group3().wy().with_zw(anti_reverse[e235], anti_reverse[e315]))
                + (Simd32x4::from([anti_reverse[e4], anti_reverse[e4235], self[e412], self[e423]]) * self.group0().with_zw(anti_reverse[e15], anti_reverse[e25]))
                + (Simd32x4::from([anti_reverse[e41], anti_reverse[e15], self[e1234], self[e1234]]) * self.group1().xw().with_zw(anti_reverse[e315], anti_reverse[e125]))
                + (Simd32x4::from([anti_reverse[e42], anti_reverse[e31], anti_reverse[e3], anti_reverse[e1]]) * self.group1().yz().with_zw(self[e23], self[e31]))
                + (Simd32x4::from([anti_reverse[e4235], anti_reverse[e35], self[e1], self[e2]]) * self.group7().xy().with_zw(anti_reverse[e12], anti_reverse[e23]))
                + (Simd32x4::from([anti_reverse[e4315], anti_reverse[e3215], self[e15], self[e25]]) * self.group7().yx().with_zw(anti_reverse[e412], anti_reverse[e423]))
                + (Simd32x4::from([self[e12345], self[e425], self[e4], self[e4]]) * anti_reverse.group9().xw().with_zw(anti_reverse[e25], anti_reverse[e35]))
                + (Simd32x4::from([self[e3], self[e125], anti_reverse[e3215], anti_reverse[e3215]]) * anti_reverse.group3().zy().with_zw(self[e431], self[e412]))
                + (Simd32x4::from([self[e4], self[e4235], self[e4315], self[e4125]]) * anti_reverse.group0().with_zw(anti_reverse[e12345], anti_reverse[e12345]))
                + (Simd32x4::from([self[e1234], self[e45], anti_reverse[e43], anti_reverse[e41]]) * anti_reverse.group6().wx().with_zw(self[e235], self[e315]))
                - (Simd32x4::from([anti_reverse[e2], anti_reverse[e315], anti_reverse[e321], anti_reverse[e2]]) * self.group3().yz().with_zw(self[e31], self[e23]))
                - (Simd32x4::from([anti_reverse[e43], anti_reverse[e4315], self[e5], self[e5]]) * self.group6().zz().with_zw(anti_reverse[e42], anti_reverse[e43]))
                - (Simd32x4::from([anti_reverse[e45], anti_reverse[scalar], self[e4235], self[e4315]]) * self.group1().wx().with_zw(anti_reverse[e435], anti_reverse[e415]))
                - (Simd32x4::from([anti_reverse[e23], anti_reverse[e25], self[e2], self[e3]]) * self.group7().xz().with_zw(anti_reverse[scalar], anti_reverse[scalar]))
                - (Simd32x4::from([anti_reverse[e431], anti_reverse[e425], self[e3215], self[e3215]]) * self.group9().zw().with_zw(anti_reverse[e431], anti_reverse[e412]))
                - (Simd32x4::from([anti_reverse[e431], anti_reverse[e321], self[e35], self[e15]]) * self.group5().yx().with_zw(anti_reverse[e423], anti_reverse[e431]))
                - (Simd32x4::from([anti_reverse[e412], anti_reverse[e3], self[e41], self[e42]]) * self.group5().zy().with_zw(anti_reverse[e125], anti_reverse[e235]))
                - (Simd32x4::from([self[e41], self[scalar], anti_reverse[e4], anti_reverse[e4]]) * anti_reverse.group1().xx().with_zw(self[e25], self[e35]))
                - (Simd32x4::from([self[e43], self[e15], anti_reverse[e1], anti_reverse[e321]]) * anti_reverse.group1().zw().with_zw(self[e12], self[e12]))
                - (Simd32x4::from([self[e23], self[e3215], self[e3], self[e1]]) * anti_reverse.group7().xx().with_zw(anti_reverse[e23], anti_reverse[e31]))
                - (Simd32x4::from([self[e415], self[e315], anti_reverse[e1234], anti_reverse[e42]]) * anti_reverse.group3().xz().with_zw(self[e315], self[e235]))
                - (Simd32x4::from([self[e425], self[e5], anti_reverse[e41], anti_reverse[e1234]]) * anti_reverse.group3().yx().with_zw(self[e125], self[e125]))
                - (Simd32x4::from([self[e321], self[e235], self[e415], self[e425]]) * anti_reverse.group9().xxwy())
                - (Simd32x4::from([self[e431], self[e321], anti_reverse[e2], anti_reverse[e3]]) * anti_reverse.group5().yx().with_zw(self[scalar], self[scalar]))
                - (Simd32x4::from([self[e412], self[e2], self[e423], self[e431]]) * anti_reverse.group5().zz().with_zw(anti_reverse[e35], anti_reverse[e15]))
                - (Simd32x4::from([self[e4235], self[e25], self[e321], self[e321]]) * anti_reverse.group7().xz().with_zw(anti_reverse[e31], anti_reverse[e12])),
            // e3215
            (anti_reverse[scalar] * self[e5])
                + (anti_reverse[e12345] * self[e3215])
                + (anti_reverse[e1] * self[e15])
                + (anti_reverse[e2] * self[e25])
                + (anti_reverse[e3] * self[e35])
                + (anti_reverse[e5] * self[scalar])
                + (anti_reverse[e45] * self[e5])
                + (anti_reverse[e235] * self[e4235])
                + (anti_reverse[e315] * self[e4315])
                + (anti_reverse[e125] * self[e4125])
                + (anti_reverse[e3215] * self[e12345])
                + (anti_reverse[e3215] * self[e321])
                - (anti_reverse[e5] * self[e45])
                - (anti_reverse[e15] * self[e1])
                - (anti_reverse[e15] * self[e415])
                - (anti_reverse[e25] * self[e2])
                - (anti_reverse[e25] * self[e425])
                - (anti_reverse[e35] * self[e3])
                - (anti_reverse[e35] * self[e435])
                - (anti_reverse[e23] * self[e235])
                - (anti_reverse[e31] * self[e315])
                - (anti_reverse[e12] * self[e125])
                - (anti_reverse[e415] * self[e15])
                - (anti_reverse[e425] * self[e25])
                - (anti_reverse[e435] * self[e35])
                - (anti_reverse[e321] * self[e3215])
                - (anti_reverse[e235] * self[e23])
                - (anti_reverse[e315] * self[e31])
                - (anti_reverse[e125] * self[e12])
                - (anti_reverse[e4235] * self[e235])
                - (anti_reverse[e4315] * self[e315])
                - (anti_reverse[e4125] * self[e125]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_anti_product[scalar], 0.0]),
            // e1, e2, e3, e4
            geometric_anti_product.group1(),
            // e5
            geometric_anti_product[e5],
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
            geometric_anti_product.group9(),
            // e3215
            geometric_anti_product[e3215],
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryCircle {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryCircle {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       14        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0));
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e321] * self[e321]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
                (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]),
                (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]),
                (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345]);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e321] * self[e321]) + (anti_reverse[e12345] * self[e12345])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435]),
                (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]),
                (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]),
                (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryDipole {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryDipole {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       14        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0));
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[e45] * self[e45]),
                (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]),
                (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]),
                (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       26        0
    //  no simd       21       29        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1());
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e4235] * self[e4235])
                    + (anti_reverse[e4315] * self[e4315])
                    + (anti_reverse[e4125] * self[e4125])
                    - (anti_reverse[e45] * self[e45]),
                (anti_reverse[e23] * self[e45]) + (anti_reverse[e12] * self[e4315]) + (anti_reverse[e45] * self[e23]) + (anti_reverse[e4125] * self[e31])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[e23] * self[e4125]) + (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]) + (anti_reverse[e4235] * self[e12])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[e31] * self[e4235]) + (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]) + (anti_reverse[e4315] * self[e23])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e4235] * self[e31]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryVersorEven {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryVersorEven {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       33        0
    //  no simd       28       36        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0));
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e12345] * self[e12345]) + (anti_reverse[e321] * self[e321])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435]),
                (anti_reverse[e12345] * self[e1])
                    + (anti_reverse[e1] * self[e12345])
                    + (anti_reverse[e3] * self[e425])
                    + (anti_reverse[e415] * self[e321])
                    + (anti_reverse[e435] * self[e2])
                    + (anti_reverse[e321] * self[e415])
                    - (anti_reverse[e2] * self[e435])
                    - (anti_reverse[e425] * self[e3]),
                (anti_reverse[e12345] * self[e2])
                    + (anti_reverse[e1] * self[e435])
                    + (anti_reverse[e2] * self[e12345])
                    + (anti_reverse[e415] * self[e3])
                    + (anti_reverse[e425] * self[e321])
                    + (anti_reverse[e321] * self[e425])
                    - (anti_reverse[e3] * self[e415])
                    - (anti_reverse[e435] * self[e1]),
                (anti_reverse[e12345] * self[e3])
                    + (anti_reverse[e2] * self[e415])
                    + (anti_reverse[e3] * self[e12345])
                    + (anti_reverse[e425] * self[e1])
                    + (anti_reverse[e435] * self[e321])
                    + (anti_reverse[e321] * self[e435])
                    - (anti_reverse[e1] * self[e425])
                    - (anti_reverse[e415] * self[e2]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryVersorOdd {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryVersorOdd {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       33        0
    //  no simd       28       36        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0));
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e4235] * self[e4235])
                    + (anti_reverse[e4315] * self[e4315])
                    + (anti_reverse[e4125] * self[e4125])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e45] * self[e45]),
                (anti_reverse[scalar] * self[e4235])
                    + (anti_reverse[e4235] * self[scalar])
                    + (anti_reverse[e4125] * self[e31])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e45] * self[e23])
                    - (anti_reverse[e4315] * self[e12])
                    - (anti_reverse[e31] * self[e4125]),
                (anti_reverse[scalar] * self[e4315])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e4315] * self[scalar])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e45] * self[e31])
                    - (anti_reverse[e4125] * self[e23])
                    - (anti_reverse[e12] * self[e4235]),
                (anti_reverse[scalar] * self[e4125])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e4125] * self[scalar])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e45] * self[e12])
                    - (anti_reverse[e4235] * self[e31])
                    - (anti_reverse[e23] * self[e4315]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<AntiConstraintViolationPrefixOrPostfix> for VersorEven {
    fn div_assign(&mut self, _rhs: AntiConstraintViolationPrefixOrPostfix) {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       37        0
    //    simd3        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       36       46        0
    //  no simd       57       72        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e12345] * self[e12345]) + (anti_reverse[e4] * self[e5]) + (anti_reverse[e5] * self[e4])
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
                (anti_reverse[e12345] * self[e5]) + (anti_reverse[e5] * self[e12345])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e4] * self[e235]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e4] * self[e315]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e4] * self[e125]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) + (anti_reverse.group0().xxyw() * self.group2().wzx().with_w(self[e4]))
                + (anti_reverse.group0().zyz() * self.group2().yww()).with_w(anti_reverse[e4] * self[e12345])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e435]]) * anti_reverse.group2().zyz().with_w(anti_reverse[e412]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e425]]) * anti_reverse.group2().xxy().with_w(anti_reverse[e431]))
                - (anti_reverse.group0().yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().xyzx() * anti_reverse.group2().www().with_w(anti_reverse[e415])),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[e12345], 2)
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       32        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       31       38        0
    //  no simd       43       56        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e3] * self[e3]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
                (anti_reverse[e1] * self[e12345]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]),
                (anti_reverse[e2] * self[e12345]) + (anti_reverse[e415] * self[e3]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]),
                (anti_reverse[e3] * self[e12345]) + (anti_reverse[e425] * self[e1]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * self.group0())
                + (Simd32x4::from([anti_reverse[e321], anti_reverse[e3], self[e435], self[e415]]) * self.group1().wy().with_zw(anti_reverse[e1], anti_reverse[e2]))
                - (Simd32x4::from([anti_reverse[e2], anti_reverse[e425], self[e1], self[e2]]) * self.group0().zw().with_zw(anti_reverse[e435], anti_reverse[e415]))
                - (Simd32x4::from([self[e1], self[e435], self[e415], self[e425]]) * anti_reverse.group0().yzwy()),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e12345] * self[e5])
                    + (anti_reverse[e1] * self[e235])
                    + (anti_reverse[e2] * self[e315])
                    + (anti_reverse[e3] * self[e125])
                    + (anti_reverse[e5] * self[e12345])
                    + (anti_reverse[e5] * self[e321])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e321] * self[e5])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e3])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       28       34        0
    //  no simd       28       40        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e4] * self[e5]) + (anti_reverse[e5] * self[e4])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
                (anti_reverse[e423] * self[e5]) + (anti_reverse[e412] * self[e315]) + (anti_reverse[e4] * self[e235]) + (anti_reverse[e315] * self[e412])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e5] * self[e423]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e431] * self[e5]) + (anti_reverse[e4] * self[e315]) + (anti_reverse[e125] * self[e423])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e5] * self[e431]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e412] * self[e5]) + (anti_reverse[e4] * self[e125]) + (anti_reverse[e235] * self[e431])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e5] * self[e412]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Origin::from_groups(
            // e4
            (anti_reverse[e12345] * self[e4]) + (anti_reverse[e4] * self[e12345])
                - (anti_reverse[e423] * self[e415])
                - (anti_reverse[e431] * self[e425])
                - (anti_reverse[e412] * self[e435])
                - (anti_reverse[e415] * self[e423])
                - (anti_reverse[e425] * self[e431])
                - (anti_reverse[e435] * self[e412]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       31        0
    //    simd3        0        3        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       57       68        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e321] * self[e321]) + (anti_reverse[e5] * self[e4]) + (anti_reverse[e4] * self[e5])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
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
                (anti_reverse[e5] * self[e321]) + (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125])
                    - (anti_reverse[e321] * self[e5])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e3]),
            ),
            // e1, e2, e3, e4
            (anti_reverse.group0().xxyx() * self.group1().wzx().with_w(self[e1]))
                + (anti_reverse.group0().zyzy() * self.group1().yww().with_w(self[e2]))
                + (anti_reverse.group1().yzx() * self.group0().zxy()).with_w(anti_reverse[e412] * self[e3])
                + (anti_reverse.group2().www() * self.group1().xyz()).with_w(anti_reverse[e321] * self[e4])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * anti_reverse.group1().zyz().with_w(anti_reverse[e3]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * anti_reverse.group1().xxy().with_w(anti_reverse[e2]))
                - (self.group0() * anti_reverse.group1().www().with_w(anti_reverse[e4]))
                - (anti_reverse.group0().yzx() * self.group1().zxy()).with_w(anti_reverse[e1] * self[e423]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e321], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
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
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = AntiFlector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOddAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd2        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       43       60        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                anti_reverse[scalar] * self[scalar] * -1.0,
                anti_reverse[e4315] * self[e12] * -1.0,
                anti_reverse[e4125] * self[e23] * -1.0,
                anti_reverse[e4235] * self[e31] * -1.0,
            ]) + (Simd32x4::from([anti_reverse[e4235], anti_reverse[scalar], self[e31], self[e12]]) * self.group2().xx().with_zw(anti_reverse[e45], anti_reverse[e45]))
                + (Simd32x4::from([self[e31], self[e4315], self[e4125], self[e4235]]) * anti_reverse.group1().yzxy())
                + (Simd32x4::from([self[e4315], self[scalar], self[e12], self[e23]]) * anti_reverse.group2().yxxy())
                + (Simd32x4::from([self[e4125], self[e31], self[scalar], self[scalar]]) * anti_reverse.group2().zzyz())
                + (anti_reverse.group1().zwyz() * self.group1().zxww())
                + (anti_reverse.group1().xx() * self.group1().xw()).with_zw(anti_reverse[scalar] * self[e4315], anti_reverse[scalar] * self[e4125])
                - (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse.group1().wyzx()),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(
                (anti_reverse[e15] * self[e23])
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
                    - (anti_reverse[e3215] * self[scalar]),
            ),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            geometric_anti_product.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([geometric_anti_product[e1], geometric_anti_product[e2], geometric_anti_product[e3], geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       37        0
    //    simd3        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       36       46        0
    //  no simd       57       72        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
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
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e3215] * self[e1234])
                    - (anti_reverse[e1234] * self[e3215]),
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
                    - (anti_reverse[scalar] * self[e3215])
                    - (anti_reverse[e3215] * self[scalar]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse[e25] * self[e43]) - (anti_reverse[e1234] * self[e15]),
                -(anti_reverse[e35] * self[e41]) - (anti_reverse[e1234] * self[e25]),
                -(anti_reverse[e15] * self[e42]) - (anti_reverse[e1234] * self[e35]),
                (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e41]]) * anti_reverse.group2().zyz().with_w(anti_reverse[e23]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e12]]) * anti_reverse.group2().xxy().with_w(anti_reverse[e43]))
                + (anti_reverse.group0().yzxx() * self.group2().zxy().with_w(self[e23]))
                + (anti_reverse.group1().www() * self.group0().xyz()).with_w(anti_reverse[e42] * self[e31])
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * anti_reverse.group0().xxyw())
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[scalar]]) * anti_reverse.group0().zyz().with_w(anti_reverse[e1234])),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            geometric_anti_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e41] * self[e15])
                        + 2.0 * (self[e42] * self[e25])
                        + 2.0 * (self[e43] * self[e35])
                        + 2.0 * (self[e3215] * self[e1234])
                        + f32::powi(self[scalar], 2)
                        + f32::powi(self[e23], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e12], 2),
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
