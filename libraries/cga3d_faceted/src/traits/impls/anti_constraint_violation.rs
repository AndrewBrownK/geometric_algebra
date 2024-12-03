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
//   Median:        11      13       0
//  Average:        17      21       0
//  Maximum:       215     236       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        17      19       0
//  Average:        28      34       0
//  Maximum:       397     421       0
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
    //      f32        4        4        0
    //    simd2        4        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        9        0
    //  no simd       12       16        0
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
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32       26       35        0
    //    simd3        0        4        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       31       42        0
    //  no simd       46       59        0
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
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (crate::swizzle!(self.group1(), 3, 3, 3, 2) * anti_reverse.group1().truncate_to_3().extend_to_4(anti_reverse[e43]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e41] * self[e23])
                + (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(anti_reverse.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e42] * self[e31])
                + (crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _) * self.group1().truncate_to_3()).extend_to_4(anti_reverse[e23] * self[e41]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       26       35        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       38       51        0
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
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e43] * self[e12]) + (anti_reverse[e23] * self[e41]) + (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e41] * self[e23])
                + (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(anti_reverse.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e42] * self[e31]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       11       11        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       13        0
    //  no simd       15       18        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       11       10        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       23       26        0
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
                + (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * crate::swizzle!(self.group0(), 0, 3, 3, 3))
                + (crate::swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * crate::swizzle!(self.group0(), 1, 0, 1, 2)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        5        6        0
    //    simd2        4        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd       13       19        0
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
                (anti_reverse[e41] * self[e23]) + (anti_reverse[e42] * self[e31]) + (anti_reverse[e43] * self[e12]),
                anti_reverse[scalar] * self[scalar] * -1.0,
            ]) + (Simd32x2::from(anti_reverse[e23]) * Simd32x2::from([self[e41], self[e23]]))
                + (Simd32x2::from(anti_reverse[e31]) * Simd32x2::from([self[e42], self[e31]]))
                + (Simd32x2::from(anti_reverse[e12]) * Simd32x2::from([self[e43], self[e12]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32       49       59        0
    //    simd3        0        6        0
    //    simd4       11        7        0
    // Totals...
    // yes simd       60       72        0
    //  no simd       93      105        0
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
                (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e4] * self[e235]) + (anti_reverse[e3] * self[e425]),
                (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e4] * self[e315]) + (anti_reverse[e1] * self[e435]),
                (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e4] * self[e125]) + (anti_reverse[e2] * self[e415]),
                -(anti_reverse[e435] * self[e412]) - (anti_reverse[e4] * self[e321]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]),
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * crate::swizzle!(anti_reverse.group1(), 0, 0, 1, 3))
                + (Simd32x4::from([self[e315], self[e5], self[e5], self[e2]]) * crate::swizzle!(anti_reverse.group0(), 2, 1, 2).extend_to_4(anti_reverse[e431]))
                + (Simd32x4::from([self[e5], self[e125], self[e235], self[e1]]) * crate::swizzle!(anti_reverse.group0(), 0, 0, 1).extend_to_4(anti_reverse[e423]))
                + (crate::swizzle!(self.group0(), 2, 0, 1) * crate::swizzle!(anti_reverse.group2(), 1, 2, 0, _)).extend_to_4(anti_reverse[e412] * self[e3])
                - (crate::swizzle!(anti_reverse.group2(), 2, 0, _, _).extend_to_4(anti_reverse[e5], self[e435])
                    * crate::swizzle!(self.group0(), 1, 2, 2).extend_to_4(anti_reverse[e412]))
                - (crate::swizzle!(anti_reverse.group3(), 3, 3, _, _).extend_to_4(anti_reverse[e315], self[e425])
                    * crate::swizzle!(self.group0(), 0, 1, 0).extend_to_4(anti_reverse[e431]))
                - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e423] * self[e415])
                - (crate::swizzle!(anti_reverse.group1(), 1, 2, 0, _) * crate::swizzle!(self.group3(), 2, 0, 1, _)).extend_to_4(anti_reverse[e415] * self[e423])
                - (crate::swizzle!(anti_reverse.group3(), 1, 2, 0, _) * crate::swizzle!(self.group1(), 2, 0, 1, _)).extend_to_4(anti_reverse[e425] * self[e431])
                - (crate::swizzle!(self.group2(), 3, 3, 3, _) * anti_reverse.group2().truncate_to_3()).extend_to_4(anti_reverse[e1] * self[e423]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       28       28        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       32       33        0
    //  no simd       44       47        0
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
            ]) + (crate::swizzle!(anti_reverse.group0(), 3, 0, 0, 1) * crate::swizzle!(self.group0(), 3, 3, _, _).extend_to_4(self[e3], self[e1]))
                - (Simd32x4::from([anti_reverse[e425], anti_reverse[e2], self[e415], self[e425]])
                    * crate::swizzle!(self.group0(), 1, 2, _, _).extend_to_4(anti_reverse[e3], anti_reverse[e1]))
                - (Simd32x4::from([self[e415], self[e3], self[e1], self[e2]]) * crate::swizzle!(anti_reverse.group0(), 0, 1, 2, 0)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       16       17        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse =
            AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1());
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([
                (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3]) + (anti_reverse[e321] * self[e4]) - (anti_reverse[e3] * self[e412]),
                0.0,
            ]) + (Simd32x2::from([self[e1], self[e321]]) * crate::swizzle!(anti_reverse.group0(), 0, 3, _, _))
                - (Simd32x2::from([self[e423], self[e2]]) * crate::swizzle!(anti_reverse.group1(), 1, 2, _, _))
                - (Simd32x2::from([self[e431], self[e3]]) * crate::swizzle!(anti_reverse.group1(), 2, 3, _, _))
                - (Simd32x2::from([self[e321], self[e1]]) * anti_reverse.group1().truncate_to_2()),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32       31       41        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       51       67        0
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
                (anti_reverse[e423] * self[e5]) + (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]) + (anti_reverse[e4] * self[e235]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e431] * self[e5]) + (anti_reverse[e125] * self[e423]) + (anti_reverse[e4] * self[e315]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e412] * self[e5]) + (anti_reverse[e235] * self[e431]) + (anti_reverse[e4] * self[e125]),
                -(anti_reverse[e431] * self[e425]) - (anti_reverse[e412] * self[e435]),
            ]) - (Simd32x4::from([self[e431], self[e4], self[e4], anti_reverse[e423]]) * crate::swizzle!(anti_reverse.group2(), 2, 1, 2, _).extend_to_4(self[e415]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * crate::swizzle!(anti_reverse.group2(), 0, 0, 1, _).extend_to_4(anti_reverse[e435]))
                - (crate::swizzle!(self.group0(), 0, 1, 2, 1) * crate::swizzle!(anti_reverse.group0(), 3, 3, 3, _).extend_to_4(anti_reverse[e425]))
                - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, _) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e415] * self[e423]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       13       13        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       17       17        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       12        0
    //  no simd       13       16        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       13       13        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       15        0
    //  no simd       17       21        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32        6        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        9        7        0
    //  no simd       18       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar]);
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(anti_reverse[e12] * self[e12]) - (anti_reverse[e45] * self[e45]) - (anti_reverse[scalar] * self[scalar]), 0.0, 0.0, 0.0])
                + (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * crate::swizzle!(self.group0(), 0, 3, 3, 3))
                + (crate::swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * crate::swizzle!(self.group0(), 1, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2) - f32::powi(self[scalar], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32       15       14        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       19       18        0
    //  no simd       31       30        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1());
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]) - (anti_reverse[e3] * self[e3]),
                (anti_reverse[e415] * self[e321]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]),
                (anti_reverse[e415] * self[e3]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]),
                (anti_reverse[e425] * self[e1]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]),
            ]) + (Simd32x4::from([anti_reverse[e321], anti_reverse[e3], self[e435], self[e415]])
                * crate::swizzle!(self.group0(), 3, 1, _, _).extend_to_4(anti_reverse[e1], anti_reverse[e2]))
                - (Simd32x4::from([anti_reverse[e2], anti_reverse[e425], anti_reverse[e435], anti_reverse[e415]]) * crate::swizzle!(self.group1(), 1, 2, 0).extend_to_4(self[e2]))
                - (Simd32x4::from([self[e1], self[e435], self[e415], self[e425]]) * anti_reverse.group1().extend_to_4(anti_reverse[e1])),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        0
    //  no simd       16       21        0
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
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32       32       41        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       35       46        0
    //  no simd       44       57        0
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
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)).extend_to_4(anti_reverse[e423] * self[e415])
                - (crate::swizzle!(anti_reverse.group2(), 2, 0, 1) * crate::swizzle!(self.group0(), 1, 2, 0)).extend_to_4(anti_reverse[e431] * self[e425]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       24       34        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       49        0
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
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)).extend_to_4(anti_reverse[e423] * self[e415])
                - (crate::swizzle!(anti_reverse.group2(), 2, 0, 1) * crate::swizzle!(self.group0(), 1, 2, 0)).extend_to_4(anti_reverse[e431] * self[e425]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       10       13        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       16        0
    //  no simd       18       24        0
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        8       19        0
    //    simd3        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       20       31        0
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
                -(anti_reverse[e431] * self[e315]) - (anti_reverse[e412] * self[e125]) - (anti_reverse[e315] * self[e431]) - (anti_reverse[e125] * self[e412]),
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]),
            ]) - (anti_reverse.group0() * crate::swizzle!(self.group1(), 0, 2, 0)).extend_to_4(anti_reverse[e423] * self[e315])
                - (self.group0() * crate::swizzle!(anti_reverse.group1(), 0, 2, 0)).extend_to_4(anti_reverse[e315] * self[e423]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ -2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]));
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        4        4        0
    //    simd2        4        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        9        0
    //  no simd       12       16        0
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
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32        6       14        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       22       33        0
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
                -(anti_reverse[e431] * self[e315]) - (anti_reverse[e412] * self[e125]) - (anti_reverse[e315] * self[e431]) - (anti_reverse[e125] * self[e412]),
                anti_reverse[e412] * self[e315],
                anti_reverse[e423] * self[e125],
                anti_reverse[e431] * self[e235],
            ]) + (Simd32x4::from([anti_reverse[e321], anti_reverse[e315], self[e423], self[e431]])
                * crate::swizzle!(self.group0(), 3, 2, _, _).extend_to_4(anti_reverse[e125], anti_reverse[e235]))
                - (crate::swizzle!(anti_reverse.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 0, 2, 0).extend_to_4(self[e315]))
                - (crate::swizzle!(self.group0(), 0, 1, 2, 0) * crate::swizzle!(anti_reverse.group1(), 0, 2, 0).extend_to_4(anti_reverse[e315])),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32       34       42        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       37       47        0
    //  no simd       46       59        0
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
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e423] * self[e415])
                - (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(anti_reverse.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e431] * self[e425]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       26       35        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       38       51        0
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
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e412] * self[e435]) - (anti_reverse[e415] * self[e423]) - (anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e423] * self[e415])
                - (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(anti_reverse.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e431] * self[e425]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       11       11        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       13        0
    //  no simd       15       18        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       12       14        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       14       17        0
    //  no simd       20       26        0
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        5        5        0
    //    simd2        4        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       10        0
    //  no simd       13       18        0
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
                -(anti_reverse[e423] * self[e415]) - (anti_reverse[e431] * self[e425]) - (anti_reverse[e412] * self[e435]),
                anti_reverse[e12345] * self[e12345],
            ]) - (Simd32x2::from(anti_reverse[e415]) * Simd32x2::from([self[e423], self[e415]]))
                - (Simd32x2::from(anti_reverse[e425]) * Simd32x2::from([self[e431], self[e425]]))
                - (Simd32x2::from(anti_reverse[e435]) * Simd32x2::from([self[e412], self[e435]])),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32       24       34        0
    //    simd3        0        5        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       44       57        0
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
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (crate::swizzle!(self.group1(), 3, 3, 3, 2) * anti_reverse.group1().truncate_to_3().extend_to_4(anti_reverse[e43]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)).extend_to_4(anti_reverse[e41] * self[e23])
                + (crate::swizzle!(anti_reverse.group2(), 2, 0, 1) * crate::swizzle!(self.group0(), 1, 2, 0)).extend_to_4(anti_reverse[e42] * self[e31])
                + (crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _) * self.group1().truncate_to_3()).extend_to_4(anti_reverse[e23] * self[e41]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32        6       17        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       22       36        0
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
                (anti_reverse[e42] * self[e25]) + (anti_reverse[e43] * self[e35]) + (anti_reverse[e25] * self[e42]) + (anti_reverse[e35] * self[e43]),
                anti_reverse[e43] * self[e25] * -1.0,
                anti_reverse[e41] * self[e35] * -1.0,
                anti_reverse[e42] * self[e15] * -1.0,
            ]) + (crate::swizzle!(anti_reverse.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 0, 2, 0).extend_to_4(self[e25]))
                + (crate::swizzle!(self.group0(), 0, 1, 2, 0) * crate::swizzle!(anti_reverse.group1(), 0, 2, 0).extend_to_4(anti_reverse[e25]))
                - (Simd32x4::from([anti_reverse[e45], anti_reverse[e25], self[e41], self[e42]])
                    * crate::swizzle!(self.group0(), 3, 2, _, _).extend_to_4(anti_reverse[e35], anti_reverse[e15])),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - f32::powi(self[e45], 2),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        9        9        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       12       13        0
    //  no simd       21       24        0
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
                + (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * crate::swizzle!(self.group0(), 0, 3, 3, 3))
                + (crate::swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * crate::swizzle!(self.group0(), 1, 0, 1, 2)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        8       19        0
    //    simd3        0        4        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       20       31        0
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
                (anti_reverse[e42] * self[e25]) + (anti_reverse[e43] * self[e35]) + (anti_reverse[e25] * self[e42]) + (anti_reverse[e35] * self[e43]),
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
            ]) + (anti_reverse.group0() * crate::swizzle!(self.group1(), 0, 2, 0)).extend_to_4(anti_reverse[e41] * self[e25])
                + (self.group0() * crate::swizzle!(anti_reverse.group1(), 0, 2, 0)).extend_to_4(anti_reverse[e25] * self[e41]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ 2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]));
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32       41       52        0
    //    simd3        0        7        0
    //    simd4       13        8        0
    // Totals...
    // yes simd       54       67        0
    //  no simd       93      105        0
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
                -(anti_reverse[e1234] * self[e15]) - (anti_reverse[e4315] * self[e12]),
                -(anti_reverse[e1234] * self[e25]) - (anti_reverse[e4125] * self[e23]),
                -(anti_reverse[e1234] * self[e35]) - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e12] * self[e43]) + (anti_reverse[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * crate::swizzle!(anti_reverse.group1(), 0, 0, 1, _).extend_to_4(anti_reverse[e42]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e12]]) * crate::swizzle!(anti_reverse.group1(), 2, 1, 2, _).extend_to_4(anti_reverse[e43]))
                + (crate::swizzle!(anti_reverse.group2(), 2, 0, _, _).extend_to_4(anti_reverse[e3215], self[e31])
                    * crate::swizzle!(self.group0(), 1, 2, 2).extend_to_4(anti_reverse[e42]))
                + (crate::swizzle!(anti_reverse.group3(), 3, 3, _, _).extend_to_4(anti_reverse[e25], self[e4235])
                    * crate::swizzle!(self.group0(), 0, 1, 0).extend_to_4(anti_reverse[e41]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e41] * self[e23])
                + (crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _) * self.group1().truncate_to_3()).extend_to_4(anti_reverse[e43] * self[e4125])
                + (crate::swizzle!(anti_reverse.group3(), 2, 0, 1, _) * crate::swizzle!(self.group1(), 1, 2, 0, _)).extend_to_4(anti_reverse[e31] * self[e42])
                + (crate::swizzle!(self.group2(), 3, 3, 3, _) * anti_reverse.group2().truncate_to_3()).extend_to_4(anti_reverse[e23] * self[e41])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], anti_reverse[e4315]]) * crate::swizzle!(anti_reverse.group0(), 2, 1, 2).extend_to_4(self[e42]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], anti_reverse[e4235]]) * crate::swizzle!(anti_reverse.group0(), 0, 0, 1).extend_to_4(self[e41]))
                - (crate::swizzle!(self.group0(), 2, 0, 1) * crate::swizzle!(anti_reverse.group2(), 1, 2, 0, _)).extend_to_4(anti_reverse[e4125] * self[e43])
                - (crate::swizzle!(anti_reverse.group1(), 1, 2, 0, _) * crate::swizzle!(self.group3(), 2, 0, 1, _)).extend_to_4(anti_reverse[e1234] * self[e45]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       25       31        0
    //    simd3        0        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       57       69        0
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
            (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e4125]]) * crate::swizzle!(anti_reverse.group1(), 2, 1, 2, _).extend_to_4(anti_reverse[e43]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4315]]) * crate::swizzle!(anti_reverse.group1(), 0, 0, 1, _).extend_to_4(anti_reverse[e42]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group1(), 2, 0, 1, _).extend_to_4(self[e4235]))
                + (crate::swizzle!(anti_reverse.group2(), 3, 3, 3, _) * self.group0().truncate_to_3()).extend_to_4(anti_reverse[e45] * self[e1234])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[e41]]) * crate::swizzle!(anti_reverse.group0(), 2, 1, 2, _).extend_to_4(anti_reverse[e4235]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e45]]) * crate::swizzle!(anti_reverse.group0(), 0, 0, 1, _).extend_to_4(anti_reverse[e1234]))
                - (crate::swizzle!(self.group0(), 2, 0, 1, 1) * crate::swizzle!(anti_reverse.group1(), 1, 2, 0, _).extend_to_4(anti_reverse[e4315]))
                - (crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _) * self.group1().truncate_to_3()).extend_to_4(anti_reverse[e4125] * self[e43]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       20       23        0
    //    simd3        0        1        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       30        0
    //  no simd       44       50        0
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
            ]) + (Simd32x4::from([self[e31], self[e4315], self[e45], self[e45]]) * crate::swizzle!(anti_reverse.group0(), 1, 2, 1, 2))
                + (Simd32x4::from([self[e4235], self[e31], self[e12], self[e23]]) * crate::swizzle!(anti_reverse.group2(), 0, 2, 0, 1))
                + (crate::swizzle!(anti_reverse.group0(), 0, 0, 0, 1) * crate::swizzle!(self.group0(), 0, 3, _, _).extend_to_4(self[e4125], self[e4235]))
                + (crate::swizzle!(anti_reverse.group0(), 2, 3, 3, 3) * crate::swizzle!(self.group0(), 2, 0, 1, 2))
                - (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * crate::swizzle!(anti_reverse.group0(), 3, 1, 2, 0)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        7       17        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       35       49        0
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
                (anti_reverse[e25] * self[e42]) + (anti_reverse[e35] * self[e43]),
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e1234] * self[e15]),
                -(anti_reverse[e35] * self[e41]) - (anti_reverse[e1234] * self[e25]),
                -(anti_reverse[e15] * self[e42]) - (anti_reverse[e1234] * self[e35]),
            ]) + (Simd32x4::from([anti_reverse[e43], anti_reverse[e15], self[e43], self[e41]])
                * crate::swizzle!(self.group1(), 2, 3, _, _).extend_to_4(anti_reverse[e15], anti_reverse[e25]))
                + (Simd32x4::from([self[e25], self[e41], self[e42], self[e43]]) * crate::swizzle!(anti_reverse.group0(), 1, 3, 3, 3))
                + (crate::swizzle!(anti_reverse.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 0, 2, 0, 1))
                + (crate::swizzle!(anti_reverse.group1(), 0, 2, 1, 2) * self.group0().truncate_to_2().extend_to_4(self[e1234], self[e1234]))
                - (Simd32x4::from([self[e1234], self[e3215], self[e35], self[e15]]) * crate::swizzle!(anti_reverse.group0(), 3, 0, 0, 1))
                - (crate::swizzle!(self.group0(), 3, 2, 3, 3) * crate::swizzle!(anti_reverse.group1(), 3, 1, _, _).extend_to_4(anti_reverse[e42], anti_reverse[e43])),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15]) + 2.0 * (self[e42] * self[e25]) + 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       16       17        0
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
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32       31       41        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       51       67        0
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
                -(anti_reverse[e41] * self[e3215]) - (anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]) - (anti_reverse[e1234] * self[e15]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e42] * self[e3215]) - (anti_reverse[e35] * self[e41]) - (anti_reverse[e1234] * self[e25]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e43] * self[e3215]) - (anti_reverse[e15] * self[e42]) - (anti_reverse[e1234] * self[e35]),
                (anti_reverse[e42] * self[e31]) + (anti_reverse[e43] * self[e12]),
            ]) + (Simd32x4::from([self[e42], self[e1234], self[e1234], anti_reverse[e41]]) * crate::swizzle!(anti_reverse.group2(), 2, 1, 2, _).extend_to_4(self[e23]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e43]]) * crate::swizzle!(anti_reverse.group2(), 0, 0, 1, _).extend_to_4(anti_reverse[e12]))
                + (crate::swizzle!(self.group0(), 0, 1, 2, 1) * crate::swizzle!(anti_reverse.group0(), 3, 3, 3, _).extend_to_4(anti_reverse[e31]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, _) * crate::swizzle!(self.group2(), 2, 0, 1, _)).extend_to_4(anti_reverse[e23] * self[e41]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       24       34        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       49        0
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
                -(anti_reverse[e43] * self[e25]) - (anti_reverse[e25] * self[e43]),
                -(anti_reverse[e41] * self[e35]) - (anti_reverse[e35] * self[e41]),
                -(anti_reverse[e42] * self[e15]) - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e43] * self[e12]) + (anti_reverse[e23] * self[e41]) + (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)).extend_to_4(anti_reverse[e41] * self[e23])
                + (crate::swizzle!(anti_reverse.group2(), 2, 0, 1) * crate::swizzle!(self.group0(), 1, 2, 0)).extend_to_4(anti_reverse[e42] * self[e31]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       13       13        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       17       17        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32        9       10        0
    //    simd3        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       12        0
    //  no simd       13       16        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       13       13        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       15        0
    //  no simd       17       21        0
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32      143      159        0
    //    simd2       17       16        0
    //    simd3        0       14        0
    //    simd4       55       47        0
    // Totals...
    // yes simd      215      236        0
    //  no simd      397      421        0
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
                + (Simd32x2::from([anti_reverse[e2], anti_reverse[e4235]]) * crate::swizzle!(self.group9(), 2, 1, _, _))
                + (Simd32x2::from([anti_reverse[e3], anti_reverse[e4315]]) * crate::swizzle!(self.group9(), 3, 2, _, _))
                + (Simd32x2::from([anti_reverse[e4235], anti_reverse[e5]]) * crate::swizzle!(self.group1(), 0, 3, _, _))
                + (Simd32x2::from([self[e5], self[e4125]]) * crate::swizzle!(anti_reverse.group9(), 0, 3, _, _))
                + (Simd32x2::from([self[e4235], self[e5]]) * crate::swizzle!(anti_reverse.group1(), 0, 3, _, _))
                - (Simd32x2::from(anti_reverse[e423]) * Simd32x2::from([self[e15], self[e235]]))
                - (Simd32x2::from(anti_reverse[e431]) * Simd32x2::from([self[e25], self[e315]]))
                - (Simd32x2::from(anti_reverse[e412]) * Simd32x2::from([self[e35], self[e125]]))
                - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e23], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e31], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e12], anti_reverse[e435]]))
                - (Simd32x2::from(self[e423]) * Simd32x2::from([anti_reverse[e15], anti_reverse[e235]]))
                - (Simd32x2::from(self[e431]) * Simd32x2::from([anti_reverse[e25], anti_reverse[e315]]))
                - (Simd32x2::from(self[e412]) * Simd32x2::from([anti_reverse[e35], anti_reverse[e125]]))
                - (Simd32x2::from([anti_reverse[e235], anti_reverse[e45]]) * crate::swizzle!(self.group3(), 0, 3, _, _)),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e415] * self[e321]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e3215] * self[e41]),
                (anti_reverse[e415] * self[e3]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e3215] * self[e42]),
                (anti_reverse[e425] * self[e1]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e3215] * self[e43]),
                -(anti_reverse[e1234] * self[e45]) - (anti_reverse[e4235] * self[e41]) - (anti_reverse[e4315] * self[e42]) - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from([self[e5], self[e125], self[e235], anti_reverse[e41]]) * crate::swizzle!(anti_reverse.group7(), 0, 0, 1).extend_to_4(self[e23]))
                + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e1]]) * crate::swizzle!(anti_reverse.group4(), 2, 1, 2).extend_to_4(anti_reverse[e423]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e2]]) * crate::swizzle!(anti_reverse.group5(), 0, 0, 1).extend_to_4(anti_reverse[e431]))
                + (Simd32x4::from([self[e315], self[e5], self[e5], anti_reverse[e42]]) * crate::swizzle!(anti_reverse.group7(), 2, 1, 2).extend_to_4(self[e31]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e43]]) * crate::swizzle!(anti_reverse.group4(), 0, 0, 1).extend_to_4(anti_reverse[e12]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e3]]) * crate::swizzle!(anti_reverse.group5(), 2, 1, 2).extend_to_4(anti_reverse[e412]))
                + (crate::swizzle!(anti_reverse.group0(), 0, 0).extend_to_4(anti_reverse[scalar], anti_reverse[e12345])
                    * crate::swizzle!(self.group9(), 1, 2, 3, _).extend_to_4(self[e4]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 1).extend_to_4(anti_reverse[e12345], self[e12345]) * self.group1().truncate_to_3().extend_to_4(anti_reverse[e4]))
                + (crate::swizzle!(self.group0(), 0, 0).extend_to_4(self[scalar], anti_reverse[e23]) * crate::swizzle!(anti_reverse.group9(), 1, 2, 3, _).extend_to_4(self[e41]))
                + (crate::swizzle!(self.group0(), 1, 1).extend_to_4(self[e12345], anti_reverse[e31]) * anti_reverse.group1().truncate_to_3().extend_to_4(self[e42]))
                + (crate::swizzle!(anti_reverse.group3(), 3, 3, _, _).extend_to_4(anti_reverse[e4315], self[e4315])
                    * crate::swizzle!(self.group5(), 0, 1, 0).extend_to_4(anti_reverse[e42]))
                + (crate::swizzle!(anti_reverse.group9(), 3, 1, _, _).extend_to_4(anti_reverse[e45], self[e4125])
                    * crate::swizzle!(self.group5(), 1, 2, 2).extend_to_4(anti_reverse[e43]))
                + (self.group8() * crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _)).extend_to_4(anti_reverse[e45] * self[e1234])
                + (crate::swizzle!(anti_reverse.group8(), 1, 2, 0) * crate::swizzle!(self.group7(), 2, 0, 1)).extend_to_4(anti_reverse[e43] * self[e12])
                + (crate::swizzle!(self.group4(), 2, 0, 1) * crate::swizzle!(anti_reverse.group3(), 1, 2, 0, _)).extend_to_4(anti_reverse[e41] * self[e4235])
                + (crate::swizzle!(anti_reverse.group1(), 2, 0, 1, _) * crate::swizzle!(self.group6(), 1, 2, 0, _)).extend_to_4(anti_reverse[e321] * self[e4])
                - (Simd32x4::from([anti_reverse[e5], anti_reverse[e5], anti_reverse[e5], anti_reverse[e425]]) * self.group7().extend_to_4(self[e431]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e425]]) * crate::swizzle!(anti_reverse.group8(), 0, 0, 1).extend_to_4(anti_reverse[e431]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e435]]) * crate::swizzle!(anti_reverse.group8(), 2, 1, 2).extend_to_4(anti_reverse[e412]))
                - (Simd32x4::from([self[e3215], self[e3215], self[e3215], anti_reverse[e435]]) * anti_reverse.group3().truncate_to_3().extend_to_4(self[e412]))
                - (crate::swizzle!(anti_reverse.group3(), 2, 0, _, _).extend_to_4(anti_reverse[e1234], anti_reverse[e415])
                    * crate::swizzle!(self.group4(), 1, 2, 2).extend_to_4(self[e423]))
                - (crate::swizzle!(anti_reverse.group9(), 0, 0, _, _).extend_to_4(anti_reverse[e42], anti_reverse[e1])
                    * crate::swizzle!(self.group4(), 0, 1, 0).extend_to_4(self[e423]))
                - (crate::swizzle!(anti_reverse.group4(), 1, 2, 0) * crate::swizzle!(self.group3(), 2, 0, 1, _)).extend_to_4(anti_reverse[scalar] * self[e1234])
                - (crate::swizzle!(anti_reverse.group5(), 1, 2, 0) * crate::swizzle!(self.group9(), 3, 1, 2, _)).extend_to_4(anti_reverse[e1234] * self[scalar])
                - (crate::swizzle!(anti_reverse.group7(), 1, 2, 0) * crate::swizzle!(self.group8(), 2, 0, 1)).extend_to_4(anti_reverse[e423] * self[e415])
                - (crate::swizzle!(self.group5(), 2, 0, 1) * crate::swizzle!(anti_reverse.group9(), 2, 3, 1, _)).extend_to_4(anti_reverse[e2] * self[e431])
                - (crate::swizzle!(anti_reverse.group1(), 1, 2, 0, _) * crate::swizzle!(self.group6(), 2, 0, 1, _)).extend_to_4(anti_reverse[e3] * self[e412])
                - (crate::swizzle!(anti_reverse.group6(), 1, 2, 0, _) * crate::swizzle!(self.group1(), 2, 0, 1, _)).extend_to_4(anti_reverse[e4] * self[e321]),
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
            ]) + (Simd32x4::from([anti_reverse[e12345], anti_reverse[e235], anti_reverse[e4315], anti_reverse[e4125]])
                * crate::swizzle!(self.group9(), 0, 0, _, _).extend_to_4(self[e12345], self[e12345]))
                + (Simd32x4::from([anti_reverse[e4], anti_reverse[e125], self[e43], self[e41]])
                    * crate::swizzle!(self.group3(), 3, 1, _, _).extend_to_4(anti_reverse[e235], anti_reverse[e315]))
                + (Simd32x4::from([anti_reverse[e4], anti_reverse[e4235], self[e412], self[e423]]) * self.group0().extend_to_4(anti_reverse[e15], anti_reverse[e25]))
                + (Simd32x4::from([anti_reverse[e41], anti_reverse[e15], self[e1234], self[e1234]])
                    * crate::swizzle!(self.group1(), 0, 3, _, _).extend_to_4(anti_reverse[e315], anti_reverse[e125]))
                + (Simd32x4::from([anti_reverse[e42], anti_reverse[e31], anti_reverse[e3], anti_reverse[e1]])
                    * crate::swizzle!(self.group1(), 1, 2, _, _).extend_to_4(self[e23], self[e31]))
                + (Simd32x4::from([anti_reverse[e4235], anti_reverse[e35], self[e1], self[e2]]) * self.group7().truncate_to_2().extend_to_4(anti_reverse[e12], anti_reverse[e23]))
                + (Simd32x4::from([anti_reverse[e4315], anti_reverse[e3215], self[e15], self[e25]])
                    * crate::swizzle!(self.group7(), 1, 0, _).extend_to_4(anti_reverse[e412], anti_reverse[e423]))
                + (Simd32x4::from([self[e12345], self[e425], self[e4], self[e4]])
                    * crate::swizzle!(anti_reverse.group9(), 0, 3, _, _).extend_to_4(anti_reverse[e25], anti_reverse[e35]))
                + (Simd32x4::from([self[e3], self[e125], anti_reverse[e3215], anti_reverse[e3215]])
                    * crate::swizzle!(anti_reverse.group3(), 2, 1, _, _).extend_to_4(self[e431], self[e412]))
                + (Simd32x4::from([self[e4], self[e4235], self[e4315], self[e4125]]) * anti_reverse.group0().extend_to_4(anti_reverse[e12345], anti_reverse[e12345]))
                + (Simd32x4::from([self[e1234], self[e45], anti_reverse[e43], anti_reverse[e41]])
                    * crate::swizzle!(anti_reverse.group6(), 3, 0, _, _).extend_to_4(self[e235], self[e315]))
                - (Simd32x4::from([anti_reverse[e2], anti_reverse[e315], anti_reverse[e321], anti_reverse[e2]])
                    * crate::swizzle!(self.group3(), 1, 2, _, _).extend_to_4(self[e31], self[e23]))
                - (Simd32x4::from([anti_reverse[e43], anti_reverse[e4315], self[e5], self[e5]])
                    * crate::swizzle!(self.group6(), 2, 2, _, _).extend_to_4(anti_reverse[e42], anti_reverse[e43]))
                - (Simd32x4::from([anti_reverse[e45], anti_reverse[scalar], self[e4235], self[e4315]])
                    * crate::swizzle!(self.group1(), 3, 0, _, _).extend_to_4(anti_reverse[e435], anti_reverse[e415]))
                - (Simd32x4::from([anti_reverse[e23], anti_reverse[e25], self[e2], self[e3]])
                    * crate::swizzle!(self.group7(), 0, 2, _).extend_to_4(anti_reverse[scalar], anti_reverse[scalar]))
                - (Simd32x4::from([anti_reverse[e431], anti_reverse[e425], self[e3215], self[e3215]])
                    * crate::swizzle!(self.group9(), 2, 3, _, _).extend_to_4(anti_reverse[e431], anti_reverse[e412]))
                - (Simd32x4::from([anti_reverse[e431], anti_reverse[e321], self[e35], self[e15]])
                    * crate::swizzle!(self.group5(), 1, 0, _).extend_to_4(anti_reverse[e423], anti_reverse[e431]))
                - (Simd32x4::from([anti_reverse[e412], anti_reverse[e3], self[e41], self[e42]])
                    * crate::swizzle!(self.group5(), 2, 1, _).extend_to_4(anti_reverse[e125], anti_reverse[e235]))
                - (Simd32x4::from([self[e41], self[scalar], anti_reverse[e4], anti_reverse[e4]])
                    * crate::swizzle!(anti_reverse.group1(), 0, 0, _, _).extend_to_4(self[e25], self[e35]))
                - (Simd32x4::from([self[e43], self[e15], anti_reverse[e1], anti_reverse[e321]])
                    * crate::swizzle!(anti_reverse.group1(), 2, 3, _, _).extend_to_4(self[e12], self[e12]))
                - (Simd32x4::from([self[e23], self[e3215], self[e3], self[e1]])
                    * crate::swizzle!(anti_reverse.group7(), 0, 0, _).extend_to_4(anti_reverse[e23], anti_reverse[e31]))
                - (Simd32x4::from([self[e415], self[e315], anti_reverse[e1234], anti_reverse[e42]])
                    * crate::swizzle!(anti_reverse.group3(), 0, 2, _, _).extend_to_4(self[e315], self[e235]))
                - (Simd32x4::from([self[e425], self[e5], anti_reverse[e41], anti_reverse[e1234]])
                    * crate::swizzle!(anti_reverse.group3(), 1, 0, _, _).extend_to_4(self[e125], self[e125]))
                - (Simd32x4::from([self[e321], self[e235], self[e415], self[e425]]) * crate::swizzle!(anti_reverse.group9(), 0, 0, 3, 1))
                - (Simd32x4::from([self[e431], self[e321], anti_reverse[e2], anti_reverse[e3]])
                    * crate::swizzle!(anti_reverse.group5(), 1, 0, _).extend_to_4(self[scalar], self[scalar]))
                - (Simd32x4::from([self[e412], self[e2], self[e423], self[e431]])
                    * crate::swizzle!(anti_reverse.group5(), 2, 2, _).extend_to_4(anti_reverse[e35], anti_reverse[e15]))
                - (Simd32x4::from([self[e4235], self[e25], self[e321], self[e321]])
                    * crate::swizzle!(anti_reverse.group7(), 0, 2, _).extend_to_4(anti_reverse[e31], anti_reverse[e12])),
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
            Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0(),
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
    //      f32        5        7        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7        9        0
    //  no simd       13       15        0
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        7        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       10        0
    //  no simd       15       16        0
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        4        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        7        6        0
    //  no simd       16       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(anti_reverse[e12] * self[e12]) - (anti_reverse[e45] * self[e45]), 0.0, 0.0, 0.0])
                + (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * crate::swizzle!(self.group0(), 0, 3, 3, 3))
                + (crate::swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * crate::swizzle!(self.group0(), 1, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2));
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        7        9        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       13       15        0
    //  no simd       31       33        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1());
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (anti_reverse[e12] * self[e12]) + (anti_reverse[e4125] * self[e4125]),
                anti_reverse[e31] * self[e4125] * -1.0,
                anti_reverse[e12] * self[e4235] * -1.0,
                anti_reverse[e23] * self[e4315] * -1.0,
            ]) + (Simd32x4::from([anti_reverse[e4315], anti_reverse[e12], anti_reverse[e23], anti_reverse[e31]])
                * crate::swizzle!(self.group1(), 1, 1, 2).extend_to_4(self[e4235]))
                + (Simd32x4::from([self[e4235], self[e31], self[e12], self[e23]]) * crate::swizzle!(anti_reverse.group1(), 0, 2, 0).extend_to_4(anti_reverse[e4315]))
                + (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * crate::swizzle!(self.group0(), 0, 3, 3, 3))
                + (crate::swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * crate::swizzle!(self.group0(), 1, 0, 1, 2))
                - (Simd32x4::from([anti_reverse[e45], anti_reverse[e4315], self[e23], self[e31]])
                    * crate::swizzle!(self.group0(), 3, 2, _, _).extend_to_4(anti_reverse[e4125], anti_reverse[e4235])),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32       19       17        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       24       22        0
    //  no simd       39       37        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                -(anti_reverse[e3] * self[e3]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
                (anti_reverse[e1] * self[e12345]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e435] * self[e2]) + (anti_reverse[e321] * self[e415]),
                (anti_reverse[e2] * self[e12345]) + (anti_reverse[e415] * self[e3]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]),
                (anti_reverse[e3] * self[e12345]) + (anti_reverse[e425] * self[e1]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * self.group0())
                + (Simd32x4::from([anti_reverse[e321], anti_reverse[e3], self[e435], self[e415]])
                    * crate::swizzle!(self.group1(), 3, 1, _, _).extend_to_4(anti_reverse[e1], anti_reverse[e2]))
                - (Simd32x4::from([anti_reverse[e2], anti_reverse[e425], self[e1], self[e2]])
                    * crate::swizzle!(self.group0(), 2, 3, _, _).extend_to_4(anti_reverse[e435], anti_reverse[e415]))
                - (Simd32x4::from([self[e1], self[e435], self[e415], self[e425]]) * crate::swizzle!(anti_reverse.group0(), 1, 2, 3, 1)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        7        1        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       15       10        0
    //  no simd       39       37        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([self[e4125], self[e31], self[scalar], self[scalar]]) * crate::swizzle!(anti_reverse.group0(), 3, 3, 2, 3))
                + (Simd32x4::from([self[e31], self[e4315], self[e45], self[e45]]) * crate::swizzle!(anti_reverse.group1(), 1, 2, 1, 2))
                + (crate::swizzle!(anti_reverse.group0(), 1, 0, 0, 0) * crate::swizzle!(self.group0(), 1, 1, 2, 3))
                + (crate::swizzle!(anti_reverse.group0(), 2, 1, 1, 2) * crate::swizzle!(self.group0(), 2, 0, _, _).extend_to_4(self[e12], self[e23]))
                + (crate::swizzle!(anti_reverse.group1(), 0, 0, 0, 1) * crate::swizzle!(self.group1(), 0, 3, _, _).extend_to_4(self[e4125], self[e4235]))
                + (crate::swizzle!(anti_reverse.group1(), 2, 3, 3, 3) * crate::swizzle!(self.group1(), 2, 0, 1, 2))
                - (Simd32x4::from([self[scalar], self[e12], self[e23], self[e31]]) * crate::swizzle!(anti_reverse.group0(), 0, 2, 3, 1))
                - (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * crate::swizzle!(anti_reverse.group1(), 3, 1, 2, 0)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32       53       60        0
    //    simd3        0        3        0
    //    simd4       13       12        0
    // Totals...
    // yes simd       66       75        0
    //  no simd      105      117        0
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
                (anti_reverse[e315] * self[e412]) + (anti_reverse[e1] * self[e12345]) + (anti_reverse[e3] * self[e425]) + (anti_reverse[e4] * self[e235]),
                (anti_reverse[e125] * self[e423]) + (anti_reverse[e1] * self[e435]) + (anti_reverse[e2] * self[e12345]) + (anti_reverse[e4] * self[e315]),
                (anti_reverse[e235] * self[e431]) + (anti_reverse[e2] * self[e415]) + (anti_reverse[e3] * self[e12345]) + (anti_reverse[e4] * self[e125]),
                -(anti_reverse[e1] * self[e423]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]) - (anti_reverse[e4] * self[e321]),
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * crate::swizzle!(anti_reverse.group1(), 0, 0, 1, _).extend_to_4(anti_reverse[e12345]))
                + (Simd32x4::from([self[e2], self[e321], self[e321], self[e4]]) * crate::swizzle!(anti_reverse.group1(), 2, 1, 2, 3))
                + (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 0) * crate::swizzle!(self.group2(), 3, 2, 0, _).extend_to_4(self[e1]))
                + (crate::swizzle!(anti_reverse.group0(), 2, 1, 2, 1) * crate::swizzle!(self.group2(), 1, 3, 3, _).extend_to_4(self[e2]))
                + (crate::swizzle!(anti_reverse.group0(), 3, 3, 3, 2) * crate::swizzle!(self.group3(), 0, 1, 2, 2))
                + (crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _) * self.group1().truncate_to_3()).extend_to_4(anti_reverse[e4] * self[e12345])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * crate::swizzle!(anti_reverse.group2(), 2, 1, 2, _).extend_to_4(anti_reverse[e415]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e435]]) * crate::swizzle!(anti_reverse.group2(), 0, 0, 1, _).extend_to_4(anti_reverse[e412]))
                - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _).extend_to_4(self[e415]))
                - (crate::swizzle!(self.group0(), 0, 1, 2, 1) * crate::swizzle!(anti_reverse.group2(), 3, 3, 3, _).extend_to_4(anti_reverse[e425]))
                - (crate::swizzle!(anti_reverse.group1(), 1, 2, 0, _) * crate::swizzle!(self.group3(), 2, 0, 1, _)).extend_to_4(anti_reverse[e431] * self[e425])
                - (crate::swizzle!(anti_reverse.group3(), 1, 2, 0, _) * crate::swizzle!(self.group1(), 2, 0, 1, _)).extend_to_4(anti_reverse[e435] * self[e412]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       29       38        0
    //    simd3        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       36       47        0
    //  no simd       57       73        0
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
                (anti_reverse[e4] * self[e235]) + (anti_reverse[e315] * self[e412]),
                (anti_reverse[e4] * self[e315]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e4] * self[e125]) + (anti_reverse[e235] * self[e431]),
                -(anti_reverse[e425] * self[e431]) - (anti_reverse[e435] * self[e412]),
            ]) + (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 3) * crate::swizzle!(self.group2(), 3, 2, 0, _).extend_to_4(self[e4]))
                + (crate::swizzle!(anti_reverse.group0(), 2, 1, 2, _) * crate::swizzle!(self.group2(), 1, 3, 3, _)).extend_to_4(anti_reverse[e4] * self[e12345])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e435]]) * crate::swizzle!(anti_reverse.group2(), 2, 1, 2, _).extend_to_4(anti_reverse[e412]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e425]]) * crate::swizzle!(anti_reverse.group2(), 0, 0, 1, _).extend_to_4(anti_reverse[e431]))
                - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _).extend_to_4(self[e415]))
                - (crate::swizzle!(self.group0(), 0, 1, 2, 0) * crate::swizzle!(anti_reverse.group2(), 3, 3, 3, _).extend_to_4(anti_reverse[e415])),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       34       33        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       39       39        0
    //  no simd       54       57        0
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
                + (Simd32x4::from([anti_reverse[e321], anti_reverse[e3], self[e435], self[e415]])
                    * crate::swizzle!(self.group1(), 3, 1, _, _).extend_to_4(anti_reverse[e1], anti_reverse[e2]))
                - (Simd32x4::from([anti_reverse[e2], anti_reverse[e425], self[e1], self[e2]])
                    * crate::swizzle!(self.group0(), 2, 3, _, _).extend_to_4(anti_reverse[e435], anti_reverse[e415]))
                - (Simd32x4::from([self[e1], self[e435], self[e415], self[e425]]) * crate::swizzle!(anti_reverse.group0(), 1, 2, 3, 1)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        7       19        0
    //    simd2        0        1        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       35       49        0
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
                -(anti_reverse[e431] * self[e315]) - (anti_reverse[e412] * self[e125]),
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e4] * self[e235]),
                (anti_reverse[e4] * self[e315]) + (anti_reverse[e125] * self[e423]),
                (anti_reverse[e4] * self[e125]) + (anti_reverse[e235] * self[e431]),
            ]) + (crate::swizzle!(anti_reverse.group0(), 3, 0, 0, 1) * crate::swizzle!(self.group1(), 3, 3, 2, 0))
                + (crate::swizzle!(anti_reverse.group1(), 3, 1, _, _) * crate::swizzle!(self.group0(), 3, 2, _, _))
                    .extend_to_4(anti_reverse[e431] * self[e5], anti_reverse[e412] * self[e5])
                - (crate::swizzle!(anti_reverse.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 0, 2, 0, 1))
                - (crate::swizzle!(anti_reverse.group1(), 0, 0, 0, 1) * crate::swizzle!(self.group0(), 0, 3, 2, 0))
                - (crate::swizzle!(anti_reverse.group1(), 1, 2, 1, 2) * crate::swizzle!(self.group0(), 1, 1, 3, 3))
                - (crate::swizzle!(anti_reverse.group1(), 2, 3, 3, 3) * crate::swizzle!(self.group0(), 2, 0, 1, 2)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_dot_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5]) - 2.0 * (self[e423] * self[e235]) - 2.0 * (self[e431] * self[e315]) - 2.0 * (self[e412] * self[e125]),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        0
    //  no simd       16       21        0
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
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, anti_dot_product[e12345] * -1.0]) + geometric_anti_product.group0());
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
    //      f32       25       32        0
    //    simd3        0        3        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       57       69        0
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
            (crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 0) * crate::swizzle!(self.group1(), 3, 2, 0, _).extend_to_4(self[e1]))
                + (crate::swizzle!(anti_reverse.group0(), 2, 1, 2, 1) * crate::swizzle!(self.group1(), 1, 3, 3, _).extend_to_4(self[e2]))
                + (crate::swizzle!(anti_reverse.group1(), 1, 2, 0, _) * crate::swizzle!(self.group0(), 2, 0, 1, _)).extend_to_4(anti_reverse[e412] * self[e3])
                + (crate::swizzle!(anti_reverse.group2(), 3, 3, 3, _) * self.group1().truncate_to_3()).extend_to_4(anti_reverse[e321] * self[e4])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * crate::swizzle!(anti_reverse.group1(), 2, 1, 2, _).extend_to_4(anti_reverse[e3]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * crate::swizzle!(anti_reverse.group1(), 0, 0, 1, _).extend_to_4(anti_reverse[e2]))
                - (self.group0() * crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _).extend_to_4(anti_reverse[e4]))
                - (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, _) * crate::swizzle!(self.group1(), 2, 0, 1, _)).extend_to_4(anti_reverse[e1] * self[e423]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       41       44        0
    //    simd3        0        3        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       57       63        0
    //  no simd      105      117        0
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
            (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e41]]) * crate::swizzle!(anti_reverse.group2(), 2, 1, 2, _).extend_to_4(anti_reverse[e23]))
                + (Simd32x4::from([self[scalar], self[e12], self[e23], self[e42]]) * crate::swizzle!(anti_reverse.group3(), 0, 0, 1, _).extend_to_4(anti_reverse[e31]))
                + (Simd32x4::from([self[e31], self[scalar], self[scalar], self[e43]]) * crate::swizzle!(anti_reverse.group3(), 2, 1, 2, _).extend_to_4(anti_reverse[e12]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e31]]) * crate::swizzle!(anti_reverse.group1(), 0, 0, 1, _).extend_to_4(anti_reverse[e42]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4125]]) * crate::swizzle!(anti_reverse.group2(), 0, 0, 1, _).extend_to_4(anti_reverse[e43]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e4315]]) * crate::swizzle!(anti_reverse.group1(), 2, 1, 2, _).extend_to_4(anti_reverse[e42]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _).extend_to_4(self[e23]))
                + (crate::swizzle!(anti_reverse.group0(), 3, 3, 3, 0) * crate::swizzle!(self.group3(), 0, 1, 2, 0))
                + (crate::swizzle!(self.group1(), 0, 1, 2, 2) * crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _).extend_to_4(anti_reverse[e43]))
                + (crate::swizzle!(anti_reverse.group3(), 3, 3, 3, _) * self.group0().truncate_to_3()).extend_to_4(anti_reverse[e45] * self[e1234])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[scalar]]) * crate::swizzle!(anti_reverse.group0(), 2, 1, 2, _).extend_to_4(anti_reverse[e1234]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                - (crate::swizzle!(anti_reverse.group3(), 1, 2, 0, 2) * crate::swizzle!(self.group1(), 2, 0, 1, _).extend_to_4(self[e43]))
                - (crate::swizzle!(self.group0(), 2, 0, 1, 0) * crate::swizzle!(anti_reverse.group2(), 1, 2, 0, _).extend_to_4(anti_reverse[e4235]))
                - (crate::swizzle!(anti_reverse.group1(), 1, 2, 0, _) * crate::swizzle!(self.group3(), 2, 0, 1, _)).extend_to_4(anti_reverse[e1234] * self[e45])
                - (crate::swizzle!(anti_reverse.group2(), 3, 3, 3, _) * self.group2().truncate_to_3()).extend_to_4(anti_reverse[e4315] * self[e42]),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
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
    //      f32       22       27        0
    //    simd2        0        1        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       30       36        0
    //  no simd       54       61        0
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
            ]) + (Simd32x4::from([anti_reverse[e4235], anti_reverse[scalar], self[e31], self[e12]])
                * crate::swizzle!(self.group2(), 0, 0, _, _).extend_to_4(anti_reverse[e45], anti_reverse[e45]))
                + (Simd32x4::from([self[e31], self[e4315], self[e4125], self[e4235]]) * crate::swizzle!(anti_reverse.group1(), 1, 2, 0, 1))
                + (Simd32x4::from([self[e4315], self[scalar], self[e12], self[e23]]) * crate::swizzle!(anti_reverse.group2(), 1, 0, 0, 1))
                + (Simd32x4::from([self[e4125], self[e31], self[scalar], self[scalar]]) * crate::swizzle!(anti_reverse.group2(), 2, 2, 1, 2))
                + (crate::swizzle!(anti_reverse.group1(), 2, 3, 1, 2) * crate::swizzle!(self.group1(), 2, 0, 3, 3))
                + (crate::swizzle!(anti_reverse.group1(), 0, 0, _, _) * crate::swizzle!(self.group1(), 0, 3, _, _))
                    .extend_to_4(anti_reverse[scalar] * self[e4315], anti_reverse[scalar] * self[e4125])
                - (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * crate::swizzle!(anti_reverse.group1(), 3, 1, 2, 0)),
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
            Simd32x4::from([anti_dot_product[e12345] * -1.0, 0.0, 0.0, 0.0]) + geometric_anti_product.group0(),
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
    //      f32       29       38        0
    //    simd3        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       36       47        0
    //  no simd       57       73        0
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
                -(anti_reverse[e25] * self[e43]) - (anti_reverse[e1234] * self[e15]),
                -(anti_reverse[e35] * self[e41]) - (anti_reverse[e1234] * self[e25]),
                -(anti_reverse[e15] * self[e42]) - (anti_reverse[e1234] * self[e35]),
                (anti_reverse[e31] * self[e42]) + (anti_reverse[e12] * self[e43]),
            ]) + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e41]]) * crate::swizzle!(anti_reverse.group2(), 2, 1, 2, _).extend_to_4(anti_reverse[e23]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e12]]) * crate::swizzle!(anti_reverse.group2(), 0, 0, 1, _).extend_to_4(anti_reverse[e43]))
                + (crate::swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group2(), 2, 0, 1, _).extend_to_4(self[e23]))
                + (crate::swizzle!(anti_reverse.group1(), 3, 3, 3, _) * self.group0().truncate_to_3()).extend_to_4(anti_reverse[e42] * self[e31])
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * crate::swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[scalar]]) * crate::swizzle!(anti_reverse.group0(), 2, 1, 2, _).extend_to_4(anti_reverse[e1234])),
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
            geometric_anti_product.group0() + Simd32x3::from(0.0).extend_to_4(anti_dot_product[e12345] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
