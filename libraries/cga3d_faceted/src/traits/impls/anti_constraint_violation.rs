// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 83
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        12      12       0
//  Average:        24      27       0
//  Maximum:       347     357       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        14      18       0
//  Average:        26      33       0
//  Maximum:       396     420       0
impl std::ops::Div<anti_constraint_violation> for AntiCircleOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd2        3        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        8        0
    //  no simd       11       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([(anti_reverse[e23] * self[e41]) + (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]), 0.0])
                + (Simd32x2::from(self[e23]) * Simd32x2::from([anti_reverse[e41], anti_reverse[e23]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([anti_reverse[e42], anti_reverse[e31]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([anti_reverse[e43], anti_reverse[e12]])),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
        );
    }
}
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
    //      f32       43       47        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       43       50        0
    //  no simd       43       58        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]) + (anti_reverse[e35] * self[e42])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]) + (anti_reverse[e15] * self[e43])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]) + (anti_reverse[e25] * self[e41])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[scalar], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       40        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       35       43        0
    //  no simd       35       50        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e35] * self[e42]) - (anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e15] * self[e43]) - (anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e25] * self[e41]) - (anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[scalar], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       12       17        0
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
            Simd32x3::from(0.0)
                .extend_to_4((anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[scalar] * self[scalar])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiCircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       17       19        0
    //  no simd       17       25        0
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
            Simd32x4::from([
                (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[scalar] * self[scalar]),
                (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]),
                (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]),
                (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2) - f32::powi(self[scalar], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiCircleRotorOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        5        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       10        0
    //  no simd       12       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([
                (anti_reverse[e23] * self[e41]) + (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
                (anti_reverse[scalar] * self[scalar]) * -1.0,
            ]) + (Simd32x2::from(self[e23]) * Simd32x2::from([anti_reverse[e41], anti_reverse[e23]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([anti_reverse[e42], anti_reverse[e31]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([anti_reverse[e43], anti_reverse[e12]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
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
    //      f32       86       89        0
    //    simd3        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       87       93        0
    //  no simd       90      104        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
                (anti_reverse[e423] * self[e5])
                    + (anti_reverse[e412] * self[e315])
                    + (anti_reverse[e415] * self[e321])
                    + (anti_reverse[e435] * self[e2])
                    + (anti_reverse[e315] * self[e412])
                    + (anti_reverse[e4] * self[e235])
                    + (anti_reverse[e3] * self[e425])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e2] * self[e435])
                    - (anti_reverse[e5] * self[e423]),
                (anti_reverse[e423] * self[e125])
                    + (anti_reverse[e431] * self[e5])
                    + (anti_reverse[e415] * self[e3])
                    + (anti_reverse[e425] * self[e321])
                    + (anti_reverse[e125] * self[e423])
                    + (anti_reverse[e4] * self[e315])
                    + (anti_reverse[e1] * self[e435])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e3] * self[e415])
                    - (anti_reverse[e5] * self[e431]),
                (anti_reverse[e431] * self[e235])
                    + (anti_reverse[e412] * self[e5])
                    + (anti_reverse[e425] * self[e1])
                    + (anti_reverse[e435] * self[e321])
                    + (anti_reverse[e235] * self[e431])
                    + (anti_reverse[e4] * self[e125])
                    + (anti_reverse[e2] * self[e415])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e1] * self[e425])
                    - (anti_reverse[e5] * self[e412]),
                (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3])
                    - (anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412])
                    - (anti_reverse[e4] * self[e321])
                    - (anti_reverse[e1] * self[e423])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412]),
            ]) + (Simd32x4::from(anti_reverse[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
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
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       35        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       38       38        0
    //  no simd       41       46        0
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
                -(anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3]),
                (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e3] * self[e425])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e2] * self[e435]),
                (anti_reverse[e415] * self[e3]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e1] * self[e435])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e3] * self[e415]),
                (anti_reverse[e425] * self[e1]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e2] * self[e415])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e1] * self[e425]),
            ]) + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
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
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDipoleInversionOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        0
    //  no simd       15       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse =
            AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1());
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([
                (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3]) - (anti_reverse[e4] * self[e321]),
                0.0,
            ]) + (Simd32x2::from(anti_reverse[e321]) * Simd32x2::from([self[e4], self[e321]]))
                - (Simd32x2::from(anti_reverse[e1]) * Simd32x2::from([self[e423], self[e1]]))
                - (Simd32x2::from(anti_reverse[e2]) * Simd32x2::from([self[e431], self[e2]]))
                - (Simd32x2::from(anti_reverse[e3]) * Simd32x2::from([self[e412], self[e3]])),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       55        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       48       58        0
    //  no simd       48       66        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e423] * self[e5]) + (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) + (anti_reverse[e4] * self[e235])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e5] * self[e423])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e431] * self[e5]) + (anti_reverse[e125] * self[e423]) + (anti_reverse[e4] * self[e315])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e5] * self[e431])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e412] * self[e5]) + (anti_reverse[e235] * self[e431]) + (anti_reverse[e4] * self[e125])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e5] * self[e412])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4]),
                -(anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e5] * self[e4])
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        5        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDualNum {
    type Output = Origin;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([self[e1234] * self[scalar], f32::powi(self[scalar], 2)]) * Simd32x2::from([-2.0, -1.0]),
        );
        return Origin::from_groups(/* e4 */ geometric_anti_product[e4]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlatOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlatOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0);
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        5        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlector {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       14       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).extend_to_4((anti_reverse[e321] * self[e321]) - (anti_reverse[e1] * self[e1]) - (anti_reverse[e2] * self[e2]) - (anti_reverse[e3] * self[e3])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125]) + (anti_reverse[e5] * self[e321])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e3])
                    - (anti_reverse[e321] * self[e5]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlectorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlectorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e321] * self[e321]) - (anti_reverse[e1] * self[e1]) - (anti_reverse[e2] * self[e2]) - (anti_reverse[e3] * self[e3]),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiLine {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       10       15        0
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
            Simd32x3::from(0.0).extend_to_4((anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiLineOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        5        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]));
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       14       20        0
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
            Simd32x3::from(0.0)
                .extend_to_4((anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[scalar] * self[scalar])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
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
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiMotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[scalar] * self[scalar]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiMysteryCircleRotor {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMysteryCircleRotor {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       12       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar]);
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[scalar] * self[scalar]),
                (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]),
                (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]),
                (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2) - f32::powi(self[scalar], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiMysteryDipoleInversion {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMysteryDipoleInversion {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       21        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       25       23        0
    //  no simd       28       29        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1());
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3]),
                (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e3] * self[e425])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e2] * self[e435]),
                (anti_reverse[e415] * self[e3]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e1] * self[e435])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e3] * self[e415]),
                (anti_reverse[e425] * self[e1]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e2] * self[e415])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e1] * self[e425]),
            ]) + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
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
impl std::ops::Div<anti_constraint_violation> for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiPlaneOnOrigin {
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
impl std::ops::Div<anti_constraint_violation> for AntiSphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiSphereOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiVersorEvenOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       15       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([
                (anti_reverse[e23] * self[e41]) + (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]) - (anti_reverse[e1234] * self[scalar]),
                0.0,
            ]) + (Simd32x2::from(self[e23]) * Simd32x2::from([anti_reverse[e41], anti_reverse[e23]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([anti_reverse[e42], anti_reverse[e31]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([anti_reverse[e43], anti_reverse[e12]]))
                - (Simd32x2::from(anti_reverse[scalar]) * Simd32x2::from([self[e1234], self[scalar]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
        );
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
    //      f32       41       46        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       41       49        0
    //  no simd       41       56        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423]),
                -(anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       39        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       33       48        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) - (anti_reverse[e431] * self[e125]) - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]) - (anti_reverse[e412] * self[e235]) - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]) - (anti_reverse[e423] * self[e315]) - (anti_reverse[e315] * self[e423]),
                -(anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            -f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       12        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       15        0
    //  no simd       15       23        0
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
            ]) + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleAtOrigin {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       17       30        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
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
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]));
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd2        3        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        8        0
    //  no simd       11       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([-(anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]), 0.0])
                - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e423], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e431], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e412], anti_reverse[e435]])),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleOrthogonalOrigin {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleOrthogonalOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       19       32        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
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
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
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
    //      f32       43       47        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       43       50        0
    //  no simd       43       58        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423]),
                -(anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) + f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       40        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       35       43        0
    //  no simd       35       50        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) - (anti_reverse[e431] * self[e125]) - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]) - (anti_reverse[e412] * self[e235]) - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]) - (anti_reverse[e423] * self[e315]) - (anti_reverse[e315] * self[e423]),
                -(anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       12       17        0
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
                .extend_to_4((anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       13        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       14       16        0
    //  no simd       17       25        0
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
            ]) + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) + f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleRotorOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        4        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9        9        0
    //  no simd       12       17        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([
                -(anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
                anti_reverse[e12345] * self[e12345],
            ]) - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e423], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e431], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e412], anti_reverse[e435]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
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
    //      f32       41       46        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       41       49        0
    //  no simd       41       56        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]) + (anti_reverse[e35] * self[e42])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]) + (anti_reverse[e15] * self[e43])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]) + (anti_reverse[e25] * self[e41])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleAligningOrigin {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAligningOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       19       32        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
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
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - f32::powi(self[e45], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       16        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       18        0
    //  no simd       15       23        0
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
            Simd32x4::from([
                (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[e45] * self[e45]),
                (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]),
                (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]),
                (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2));
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleAtOrigin {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       17       30        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
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
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]));
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
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
    //      f32       82       85        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       84       90        0
    //  no simd       90      104        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
                (anti_reverse[e42] * self[e35])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e15] * self[e1234])
                    + (anti_reverse[e35] * self[e42])
                    + (anti_reverse[e4125] * self[e31])
                    + (anti_reverse[e3215] * self[e41])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[e43] * self[e15])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e15] * self[e43])
                    + (anti_reverse[e25] * self[e1234])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e3215] * self[e42])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[e41] * self[e25])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e25] * self[e41])
                    + (anti_reverse[e35] * self[e1234])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e3215] * self[e43])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e41] * self[e4235])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e42] * self[e4315])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e43] * self[e4125])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43])
                    - (anti_reverse[e4235] * self[e41])
                    - (anti_reverse[e4315] * self[e42])
                    - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from(anti_reverse[e45]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]))
                - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
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
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleInversionAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       52        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       48       56        0
    //  no simd       54       68        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e4235] * self[e15]) + (anti_reverse[e4315] * self[e25]) + (anti_reverse[e4125] * self[e35]) + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[e45] * self[e3215])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e35] * self[e42]) + (anti_reverse[e3215] * self[e41])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e15] * self[e43]) + (anti_reverse[e3215] * self[e42])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e25] * self[e41]) + (anti_reverse[e3215] * self[e43])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e4235]) + (anti_reverse[e42] * self[e4315]) + (anti_reverse[e43] * self[e4125])
                    - (anti_reverse[e4235] * self[e41])
                    - (anti_reverse[e4315] * self[e42])
                    - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * Simd32x4::from([anti_reverse[e15], anti_reverse[e25], anti_reverse[e35], anti_reverse[e45]]))
                - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2)
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       39        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       41       41        0
    //  no simd       41       46        0
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
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
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
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleInversionAtOrigin {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       32       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    - (anti_reverse[e3215] * self[e1234]),
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e3215] * self[e41]) + (anti_reverse[e15] * self[e1234]) + (anti_reverse[e35] * self[e42])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e1234] * self[e15]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e3215] * self[e42]) + (anti_reverse[e15] * self[e43]) + (anti_reverse[e25] * self[e1234])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e1234] * self[e25]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e3215] * self[e43]) + (anti_reverse[e25] * self[e41]) + (anti_reverse[e35] * self[e1234])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e1234] * self[e35]),
            ]) - (Simd32x4::from(self[e3215]) * Simd32x4::from([anti_reverse[e1234], anti_reverse[e41], anti_reverse[e42], anti_reverse[e43]])),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleInversionOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        0
    //  no simd       15       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse =
            DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1());
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([
                (anti_reverse[e45] * self[e1234]) - (anti_reverse[e4235] * self[e41]) - (anti_reverse[e4315] * self[e42]) - (anti_reverse[e4125] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(self[e4235]) * Simd32x2::from([anti_reverse[e41], anti_reverse[e4235]]))
                + (Simd32x2::from(self[e4315]) * Simd32x2::from([anti_reverse[e42], anti_reverse[e4315]]))
                + (Simd32x2::from(self[e4125]) * Simd32x2::from([anti_reverse[e43], anti_reverse[e4125]]))
                - (Simd32x2::from(self[e45]) * Simd32x2::from([anti_reverse[e1234], anti_reverse[e45]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[e45], 2),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       55        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       48       58        0
    //  no simd       48       66        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
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
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - 2.0 * (self[e3215] * self[e1234]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2) * -1.0);
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       39        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       33       48        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e35] * self[e42]) - (anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e15] * self[e43]) - (anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e25] * self[e41]) - (anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
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
    type Output = Origin;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([self[e4] * self[e12345], f32::powi(self[e12345], 2)]) * Simd32x2::from([2.0, 1.0]));
        return Origin::from_groups(/* e4 */ geometric_anti_product[e4]);
    }
}
impl std::ops::Div<anti_constraint_violation> for FlatOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for FlatOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0);
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2) * -1.0);
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2) * -1.0);
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for Flector {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       14       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0)
                .extend_to_4((anti_reverse[e4235] * self[e4235]) + (anti_reverse[e4315] * self[e4315]) + (anti_reverse[e4125] * self[e4125]) - (anti_reverse[e45] * self[e45])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e4235] * self[e15]) + (anti_reverse[e4315] * self[e25]) + (anti_reverse[e4125] * self[e35]) + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125])
                    - (anti_reverse[e45] * self[e3215]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[e45], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for FlectorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for FlectorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e4235] * self[e4235]) + (anti_reverse[e4315] * self[e4315]) + (anti_reverse[e4125] * self[e4125]) - (anti_reverse[e45] * self[e45]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[e45], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for Line {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       10       15        0
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
            Simd32x3::from(0.0).extend_to_4(-(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for LineOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for LineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        5        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            -(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for Motor {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for Motor {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       14       20        0
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
                .extend_to_4((anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435])),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e12345] * self[e5]) + (anti_reverse[e5] * self[e12345])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product[e415],
                geometric_anti_product[e425],
                geometric_anti_product[e435],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for MotorOnOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_dot_product[e12345]);
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
    //      f32      320      324        0
    //    simd2       16       16        0
    //    simd3        0        4        0
    //    simd4       11       13        0
    // Totals...
    // yes simd      347      357        0
    //  no simd      396      420        0
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
                    + (anti_reverse[e1234] * self[e5])
                    + (anti_reverse[e4235] * self[e1])
                    + (anti_reverse[e4315] * self[e2])
                    + (anti_reverse[e4125] * self[e3])
                    + (anti_reverse[e3215] * self[e4])
                    - (anti_reverse[e415] * self[e23])
                    - (anti_reverse[e425] * self[e31])
                    - (anti_reverse[e435] * self[e12])
                    - (anti_reverse[e321] * self[e45])
                    - (anti_reverse[e423] * self[e15])
                    - (anti_reverse[e431] * self[e25])
                    - (anti_reverse[e412] * self[e35])
                    - (anti_reverse[e235] * self[e41])
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
            ]) + (Simd32x2::from(anti_reverse[e4]) * Simd32x2::from([self[e3215], self[e5]]))
                + (Simd32x2::from(anti_reverse[e5]) * Simd32x2::from([self[e1234], self[e4]]))
                + (Simd32x2::from(self[e12345]) * anti_reverse.group0())
                + (Simd32x2::from(self[e4235]) * Simd32x2::from([anti_reverse[e1], anti_reverse[e4235]]))
                + (Simd32x2::from(self[e4315]) * Simd32x2::from([anti_reverse[e2], anti_reverse[e4315]]))
                + (Simd32x2::from(self[e4125]) * Simd32x2::from([anti_reverse[e3], anti_reverse[e4125]]))
                - (Simd32x2::from(anti_reverse[e45]) * Simd32x2::from([self[e321], self[e45]]))
                - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e23], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e31], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e12], anti_reverse[e435]]))
                - (Simd32x2::from(self[e423]) * Simd32x2::from([anti_reverse[e15], anti_reverse[e235]]))
                - (Simd32x2::from(self[e431]) * Simd32x2::from([anti_reverse[e25], anti_reverse[e315]]))
                - (Simd32x2::from(self[e412]) * Simd32x2::from([anti_reverse[e35], anti_reverse[e125]]))
                - (Simd32x2::from(self[e235]) * Simd32x2::from([anti_reverse[e41], anti_reverse[e423]]))
                - (Simd32x2::from(self[e315]) * Simd32x2::from([anti_reverse[e42], anti_reverse[e431]]))
                - (Simd32x2::from(self[e125]) * Simd32x2::from([anti_reverse[e43], anti_reverse[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[scalar] * self[e4235])
                    + (anti_reverse[e3] * self[e425])
                    + (anti_reverse[e4] * self[e235])
                    + (anti_reverse[e42] * self[e35])
                    + (anti_reverse[e15] * self[e1234])
                    + (anti_reverse[e35] * self[e42])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e415] * self[e321])
                    + (anti_reverse[e435] * self[e2])
                    + (anti_reverse[e423] * self[e5])
                    + (anti_reverse[e412] * self[e315])
                    + (anti_reverse[e315] * self[e412])
                    + (anti_reverse[e4235] * self[scalar])
                    + (anti_reverse[e4125] * self[e31])
                    + (anti_reverse[e3215] * self[e41])
                    - (anti_reverse[e2] * self[e435])
                    - (anti_reverse[e5] * self[e423])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[scalar] * self[e4315])
                    + (anti_reverse[e1] * self[e435])
                    + (anti_reverse[e4] * self[e315])
                    + (anti_reverse[e43] * self[e15])
                    + (anti_reverse[e15] * self[e43])
                    + (anti_reverse[e25] * self[e1234])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e415] * self[e3])
                    + (anti_reverse[e425] * self[e321])
                    + (anti_reverse[e423] * self[e125])
                    + (anti_reverse[e431] * self[e5])
                    + (anti_reverse[e125] * self[e423])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e4315] * self[scalar])
                    + (anti_reverse[e3215] * self[e42])
                    - (anti_reverse[e3] * self[e415])
                    - (anti_reverse[e5] * self[e431])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[scalar] * self[e4125])
                    + (anti_reverse[e2] * self[e415])
                    + (anti_reverse[e4] * self[e125])
                    + (anti_reverse[e41] * self[e25])
                    + (anti_reverse[e25] * self[e41])
                    + (anti_reverse[e35] * self[e1234])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e425] * self[e1])
                    + (anti_reverse[e435] * self[e321])
                    + (anti_reverse[e431] * self[e235])
                    + (anti_reverse[e412] * self[e5])
                    + (anti_reverse[e235] * self[e431])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e4125] * self[scalar])
                    + (anti_reverse[e3215] * self[e43])
                    - (anti_reverse[e1] * self[e425])
                    - (anti_reverse[e5] * self[e412])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e41] * self[e4235])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e42] * self[e4315])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e43] * self[e4125])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43])
                    + (anti_reverse[e423] * self[e1])
                    + (anti_reverse[e431] * self[e2])
                    + (anti_reverse[e412] * self[e3])
                    - (anti_reverse[scalar] * self[e1234])
                    - (anti_reverse[e1] * self[e423])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412])
                    - (anti_reverse[e4] * self[e321])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412])
                    - (anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e1234] * self[e45])
                    - (anti_reverse[e4235] * self[e41])
                    - (anti_reverse[e4315] * self[e42])
                    - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * self.group1())
                + (Simd32x4::from(anti_reverse[e45]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]))
                + (Simd32x4::from(anti_reverse[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]]))
                + (Simd32x4::from(self[e12345]) * anti_reverse.group1())
                - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]])),
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
                (anti_reverse[e4] * self[scalar])
                    + (anti_reverse[e41] * self[e1])
                    + (anti_reverse[e42] * self[e2])
                    + (anti_reverse[e43] * self[e3])
                    + (anti_reverse[e4235] * self[e423])
                    + (anti_reverse[e4315] * self[e431])
                    + (anti_reverse[e4125] * self[e412])
                    - (anti_reverse[e1] * self[e41])
                    - (anti_reverse[e2] * self[e42])
                    - (anti_reverse[e3] * self[e43])
                    - (anti_reverse[e41] * self[e415])
                    - (anti_reverse[e42] * self[e425])
                    - (anti_reverse[e43] * self[e435])
                    - (anti_reverse[e45] * self[e4])
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
                    - (anti_reverse[e412] * self[e4125]),
                (anti_reverse[e2] * self[e12])
                    + (anti_reverse[e5] * self[e41])
                    + (anti_reverse[e42] * self[e125])
                    + (anti_reverse[e45] * self[e415])
                    + (anti_reverse[e35] * self[e431])
                    + (anti_reverse[e31] * self[e3])
                    + (anti_reverse[e435] * self[e4315])
                    + (anti_reverse[e431] * self[e35])
                    + (anti_reverse[e125] * self[e42])
                    + (anti_reverse[e4125] * self[e425])
                    + (anti_reverse[e3215] * self[e423])
                    - (anti_reverse[scalar] * self[e1])
                    - (anti_reverse[e1] * self[scalar])
                    - (anti_reverse[e3] * self[e31])
                    - (anti_reverse[e4] * self[e15])
                    - (anti_reverse[e41] * self[e5])
                    - (anti_reverse[e43] * self[e315])
                    - (anti_reverse[e25] * self[e412])
                    - (anti_reverse[e12] * self[e2])
                    - (anti_reverse[e425] * self[e4125])
                    - (anti_reverse[e321] * self[e23])
                    - (anti_reverse[e423] * self[e3215])
                    - (anti_reverse[e412] * self[e25])
                    - (anti_reverse[e315] * self[e43])
                    - (anti_reverse[e1234] * self[e235])
                    - (anti_reverse[e4315] * self[e435]),
                (anti_reverse[e3] * self[e23])
                    + (anti_reverse[e5] * self[e42])
                    + (anti_reverse[e43] * self[e235])
                    + (anti_reverse[e45] * self[e425])
                    + (anti_reverse[e15] * self[e412])
                    + (anti_reverse[e12] * self[e1])
                    + (anti_reverse[e415] * self[e4125])
                    + (anti_reverse[e412] * self[e15])
                    + (anti_reverse[e235] * self[e43])
                    + (anti_reverse[e4235] * self[e435])
                    + (anti_reverse[e3215] * self[e431])
                    - (anti_reverse[scalar] * self[e2])
                    - (anti_reverse[e1] * self[e12])
                    - (anti_reverse[e2] * self[scalar])
                    - (anti_reverse[e4] * self[e25])
                    - (anti_reverse[e41] * self[e125])
                    - (anti_reverse[e42] * self[e5])
                    - (anti_reverse[e35] * self[e423])
                    - (anti_reverse[e23] * self[e3])
                    - (anti_reverse[e435] * self[e4235])
                    - (anti_reverse[e321] * self[e31])
                    - (anti_reverse[e423] * self[e35])
                    - (anti_reverse[e431] * self[e3215])
                    - (anti_reverse[e125] * self[e41])
                    - (anti_reverse[e1234] * self[e315])
                    - (anti_reverse[e4125] * self[e415]),
                (anti_reverse[e1] * self[e31])
                    + (anti_reverse[e5] * self[e43])
                    + (anti_reverse[e41] * self[e315])
                    + (anti_reverse[e45] * self[e435])
                    + (anti_reverse[e25] * self[e423])
                    + (anti_reverse[e23] * self[e2])
                    + (anti_reverse[e425] * self[e4235])
                    + (anti_reverse[e423] * self[e25])
                    + (anti_reverse[e315] * self[e41])
                    + (anti_reverse[e4315] * self[e415])
                    + (anti_reverse[e3215] * self[e412])
                    - (anti_reverse[scalar] * self[e3])
                    - (anti_reverse[e2] * self[e23])
                    - (anti_reverse[e3] * self[scalar])
                    - (anti_reverse[e4] * self[e35])
                    - (anti_reverse[e42] * self[e235])
                    - (anti_reverse[e43] * self[e5])
                    - (anti_reverse[e15] * self[e431])
                    - (anti_reverse[e31] * self[e1])
                    - (anti_reverse[e415] * self[e4315])
                    - (anti_reverse[e321] * self[e12])
                    - (anti_reverse[e431] * self[e15])
                    - (anti_reverse[e412] * self[e3215])
                    - (anti_reverse[e235] * self[e42])
                    - (anti_reverse[e1234] * self[e125])
                    - (anti_reverse[e4235] * self[e425]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * self.group9())
                + (Simd32x4::from(self[e12345]) * anti_reverse.group9())
                + (Simd32x4::from(self[e4]) * Simd32x4::from([anti_reverse[scalar], anti_reverse[e15], anti_reverse[e25], anti_reverse[e35]]))
                + (Simd32x4::from(self[e45]) * Simd32x4::from([anti_reverse[e4], anti_reverse[e415], anti_reverse[e425], anti_reverse[e435]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([anti_reverse[e321], anti_reverse[e235], anti_reverse[e315], anti_reverse[e125]]))
                - (Simd32x4::from(self[e321]) * Simd32x4::from([anti_reverse[e1234], anti_reverse[e23], anti_reverse[e31], anti_reverse[e12]])),
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
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5])
                + 2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
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
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_anti_product[scalar], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
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
impl std::ops::Div<anti_constraint_violation> for MysteryCircle {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryCircle {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       10       14        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
                anti_reverse[e321] * self[e415],
                anti_reverse[e321] * self[e425],
                anti_reverse[e321] * self[e435],
            ]) + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for MysteryCircleRotor {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryCircleRotor {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9        9        0
    //  no simd       12       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345]);
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
                anti_reverse[e321] * self[e415],
                anti_reverse[e321] * self[e425],
                anti_reverse[e321] * self[e435],
            ]) + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) + f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for MysteryDipole {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryDipole {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       10       14        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[e45] * self[e45]),
                (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]),
                (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]),
                (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2));
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for MysteryDipoleInversion {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryDipoleInversion {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       26        0
    //  no simd       28       29        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1());
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
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
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for MysteryVersorEven {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for MysteryVersorEven {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryVersorEven {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       24        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       30       27        0
    //  no simd       36       36        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435]),
                (anti_reverse[e1] * self[e12345]) + (anti_reverse[e3] * self[e425]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415])
                    - (anti_reverse[e2] * self[e435])
                    - (anti_reverse[e425] * self[e3]),
                (anti_reverse[e1] * self[e435]) + (anti_reverse[e2] * self[e12345]) + (anti_reverse[e415] * self[e3]) + (anti_reverse[e321] * self[e425])
                    - (anti_reverse[e3] * self[e415])
                    - (anti_reverse[e435] * self[e1]),
                (anti_reverse[e2] * self[e415]) + (anti_reverse[e3] * self[e12345]) + (anti_reverse[e425] * self[e1]) + (anti_reverse[e321] * self[e435])
                    - (anti_reverse[e1] * self[e425])
                    - (anti_reverse[e415] * self[e2]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * self.group0())
                + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group1(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       36       33        0
    //  no simd       36       36        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
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
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e45], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
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
impl std::ops::Div<anti_constraint_violation> for PlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for PlaneOnOrigin {
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
impl std::ops::Div<anti_constraint_violation> for RoundPointAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for RoundPointAtOrigin {
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
impl std::ops::Div<anti_constraint_violation> for SphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for SphereAtOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for SphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for SphereOnOrigin {
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
    //      f32       90       92        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       93       98        0
    //  no simd      102      116        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
                (anti_reverse[e423] * self[e5])
                    + (anti_reverse[e412] * self[e315])
                    + (anti_reverse[e415] * self[e321])
                    + (anti_reverse[e435] * self[e2])
                    + (anti_reverse[e315] * self[e412])
                    + (anti_reverse[e3] * self[e425])
                    + (anti_reverse[e4] * self[e235])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e5] * self[e423])
                    - (anti_reverse[e2] * self[e435]),
                (anti_reverse[e423] * self[e125])
                    + (anti_reverse[e431] * self[e5])
                    + (anti_reverse[e415] * self[e3])
                    + (anti_reverse[e425] * self[e321])
                    + (anti_reverse[e125] * self[e423])
                    + (anti_reverse[e1] * self[e435])
                    + (anti_reverse[e4] * self[e315])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e5] * self[e431])
                    - (anti_reverse[e3] * self[e415]),
                (anti_reverse[e431] * self[e235])
                    + (anti_reverse[e412] * self[e5])
                    + (anti_reverse[e425] * self[e1])
                    + (anti_reverse[e435] * self[e321])
                    + (anti_reverse[e235] * self[e431])
                    + (anti_reverse[e2] * self[e415])
                    + (anti_reverse[e4] * self[e125])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e5] * self[e412])
                    - (anti_reverse[e1] * self[e425]),
                (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3])
                    - (anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412])
                    - (anti_reverse[e1] * self[e423])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412])
                    - (anti_reverse[e4] * self[e321]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * self.group3())
                + (Simd32x4::from(anti_reverse[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]]))
                + (Simd32x4::from(self[e12345]) * anti_reverse.group3()),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
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
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorEvenAligningOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       56        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       51       60        0
    //  no simd       54       72        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
                (anti_reverse[e423] * self[e5]) + (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e5] * self[e423]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e431] * self[e5]) + (anti_reverse[e125] * self[e423])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e5] * self[e431]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e412] * self[e5]) + (anti_reverse[e235] * self[e431])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e5] * self[e412]),
                (anti_reverse[e12345] * self[e4])
                    - (anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]) + (Simd32x4::from(anti_reverse[e4]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5]) + f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for VersorEvenAtInfinity {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       40        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       45       44        0
    //  no simd       51       56        0
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
                -(anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435]),
                (anti_reverse[e1] * self[e12345]) + (anti_reverse[e3] * self[e425]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415])
                    - (anti_reverse[e2] * self[e435])
                    - (anti_reverse[e425] * self[e3]),
                (anti_reverse[e1] * self[e435]) + (anti_reverse[e2] * self[e12345]) + (anti_reverse[e415] * self[e3]) + (anti_reverse[e321] * self[e425])
                    - (anti_reverse[e3] * self[e415])
                    - (anti_reverse[e435] * self[e1]),
                (anti_reverse[e2] * self[e415]) + (anti_reverse[e3] * self[e12345]) + (anti_reverse[e425] * self[e1]) + (anti_reverse[e321] * self[e435])
                    - (anti_reverse[e1] * self[e425])
                    - (anti_reverse[e415] * self[e2]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * self.group0())
                + (Simd32x4::from(self[e321]) * crate::swizzle!(anti_reverse.group1(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
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
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorEvenAtOrigin {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       32       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e5] * self[e4])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e4] * self[e235]) + (anti_reverse[e315] * self[e412])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e5] * self[e423]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e4] * self[e315]) + (anti_reverse[e125] * self[e423])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e5] * self[e431]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e4] * self[e125]) + (anti_reverse[e235] * self[e431])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e5] * self[e412]),
            ]) + (Simd32x4::from(self[e5]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5]) - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorEvenOnOrigin {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       15       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([
                (anti_reverse[e4] * self[e12345]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
                0.0,
            ]) + (Simd32x2::from(anti_reverse[e12345]) * Simd32x2::from([self[e4], self[e12345]]))
                - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e423], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e431], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e412], anti_reverse[e435]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product[e4], geometric_anti_product[e12345] - anti_dot_product[e12345]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       60        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       54       62        0
    //  no simd       54       68        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
                (anti_reverse[e5] * self[e321]) + (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125])
                    - (anti_reverse[e321] * self[e5])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e3]),
            ),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e423] * self[e5]) + (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) + (anti_reverse[e4] * self[e235])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e5] * self[e423]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e431] * self[e5]) + (anti_reverse[e125] * self[e423]) + (anti_reverse[e4] * self[e315])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e5] * self[e431]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e412] * self[e5]) + (anti_reverse[e235] * self[e431]) + (anti_reverse[e4] * self[e125])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e5] * self[e412]),
                (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3]) + (anti_reverse[e321] * self[e4])
                    - (anti_reverse[e1] * self[e423])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412])
                    - (anti_reverse[e4] * self[e321]),
            ]),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e5] * self[e4]) + f32::powi(self[e321], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
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
    //      f32       94       96        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       96      101        0
    //  no simd      102      116        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x4::from([
                (anti_reverse[e42] * self[e35])
                    + (anti_reverse[scalar] * self[e4235])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e15] * self[e1234])
                    + (anti_reverse[e35] * self[e42])
                    + (anti_reverse[e4235] * self[scalar])
                    + (anti_reverse[e4125] * self[e31])
                    + (anti_reverse[e3215] * self[e41])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[e43] * self[e15])
                    + (anti_reverse[scalar] * self[e4315])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e15] * self[e43])
                    + (anti_reverse[e25] * self[e1234])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e4315] * self[scalar])
                    + (anti_reverse[e3215] * self[e42])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[e41] * self[e25])
                    + (anti_reverse[scalar] * self[e4125])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e25] * self[e41])
                    + (anti_reverse[e35] * self[e1234])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e4125] * self[scalar])
                    + (anti_reverse[e3215] * self[e43])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e41] * self[e4235])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e42] * self[e4315])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e43] * self[e4125])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43])
                    - (anti_reverse[scalar] * self[e1234])
                    - (anti_reverse[e1234] * self[e45])
                    - (anti_reverse[e4235] * self[e41])
                    - (anti_reverse[e4315] * self[e42])
                    - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from(anti_reverse[e45]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]))
                - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
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
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       51       48        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       51       50        0
    //  no simd       51       56        0
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
                (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e4235] * self[e4235])
                    + (anti_reverse[e4315] * self[e4315])
                    + (anti_reverse[e4125] * self[e4125])
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e45] * self[e45]),
                (anti_reverse[scalar] * self[e4235])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e45] * self[e23])
                    + (anti_reverse[e4235] * self[scalar])
                    + (anti_reverse[e4125] * self[e31])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[scalar] * self[e4315])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e45] * self[e31])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e4315] * self[scalar])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[scalar] * self[e4125])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e45] * self[e12])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e4125] * self[scalar])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e4235] * self[e31]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).extend_to_4(
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
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e45], 2),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                geometric_anti_product[e12345] - anti_dot_product[e12345],
                geometric_anti_product[e1],
                geometric_anti_product[e2],
                geometric_anti_product[e3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       56        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       51       60        0
    //  no simd       54       72        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e3215] * self[e41]) + (anti_reverse[e15] * self[e1234]) + (anti_reverse[e35] * self[e42])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e3215] * self[e42]) + (anti_reverse[e15] * self[e43]) + (anti_reverse[e25] * self[e1234])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e3215] * self[e43]) + (anti_reverse[e25] * self[e41]) + (anti_reverse[e35] * self[e1234])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43])
                    - (anti_reverse[scalar] * self[e1234]),
            ]) - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[scalar], 2)
                - 2.0 * (self[e3215] * self[e1234]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product[e423],
                geometric_anti_product[e431],
                geometric_anti_product[e412],
                geometric_anti_product[e12345] - anti_dot_product[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
