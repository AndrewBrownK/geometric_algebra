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
//   Median:        11      10       0
//  Average:        25      30       0
//  Maximum:       212     241       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        16      17       0
//  Average:        42      48       0
//  Maximum:       397     421       0
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
    //      f32       34       41        0
    //    simd3        0        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       46        0
    //  no simd       46       59        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            ]) - (crate::swizzle!(self.group2(), 1, 2, 0, 0) * crate::swizzle!(reverse.group0(), 2, 0, 1).extend_to_4(reverse[e23]))
                - (crate::swizzle!(self.group0(), 2, 0, 1) * crate::swizzle!(reverse.group2(), 1, 2, 0, _)).extend_to_4(reverse[e31] * self[e25]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) + f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
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
    //      f32       41       47        0
    //    simd3        0        2        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       54       62        0
    //  no simd       93      105        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            ]) + (Simd32x4::from([reverse[e315], reverse[e5], reverse[e5], self[e125]]) * crate::swizzle!(self.group0(), 2, 1, 2).extend_to_4(reverse[e435]))
                + (Simd32x4::from([reverse[e5], reverse[e125], reverse[e235], self[e315]]) * crate::swizzle!(self.group0(), 0, 0, 1).extend_to_4(reverse[e425]))
                + (crate::swizzle!(reverse.group3(), 1, 2, 0, 1) * crate::swizzle!(self.group1(), 2, 0, 1, _).extend_to_4(self[e315]))
                + (crate::swizzle!(self.group1(), 0, 1, 2, 2) * crate::swizzle!(reverse.group1(), 3, 3, 3, _).extend_to_4(reverse[e125]))
                + (crate::swizzle!(self.group2(), 1, 2, 0, 0) * crate::swizzle!(reverse.group0(), 2, 0, 1).extend_to_4(reverse[e415]))
                + (crate::swizzle!(self.group2(), 3, 3, 3, 0) * reverse.group2().truncate_to_3().extend_to_4(reverse[e1]))
                + (crate::swizzle!(self.group1(), 3, 3, _, _).extend_to_4(self[e2], self[e415]) * crate::swizzle!(reverse.group1(), 0, 1, 0, _).extend_to_4(reverse[e235]))
                + (crate::swizzle!(self.group3(), 2, 0, _, _).extend_to_4(self[e321], self[e425]) * crate::swizzle!(reverse.group1(), 1, 2, 2, _).extend_to_4(reverse[e315]))
                - (crate::swizzle!(self.group3(), 1, 2, 0, 2) * crate::swizzle!(reverse.group1(), 2, 0, 1, _).extend_to_4(reverse[e125]))
                - (crate::swizzle!(self.group2(), 2, 0, _, _).extend_to_4(self[e5], self[e1]) * crate::swizzle!(reverse.group0(), 1, 2, 2).extend_to_4(reverse[e235]))
                - (crate::swizzle!(self.group3(), 3, 3, _, _).extend_to_4(self[e315], self[e5]) * crate::swizzle!(reverse.group0(), 0, 1, 0).extend_to_4(reverse[e321]))
                - (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(reverse.group2(), 2, 0, 1, _)).extend_to_4(reverse[e315] * self[e2]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e4] * self[e5]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([self[e3215] * self[scalar], f32::powi(self[scalar], 2)]) * Simd32x2::from([2.0, 1.0]),
        );
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([geometric_product[e3215], 0.0]));
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
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       16       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([(reverse[e5] * self[e321]) - (reverse[e315] * self[e2]) - (reverse[e125] * self[e3]) - (reverse[e321] * self[e5]), 0.0])
                + (Simd32x2::from(reverse[e1]) * Simd32x2::from([self[e235], self[e1]]))
                + (Simd32x2::from(reverse[e2]) * Simd32x2::from([self[e315], self[e2]]))
                + (Simd32x2::from(reverse[e3]) * Simd32x2::from([self[e125], self[e3]]))
                - (Simd32x2::from([self[e1], self[e321]]) * crate::swizzle!(reverse.group0(), 0, 3, _, _)),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, dot_product[scalar] * -1.0]) + geometric_product.group0());
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
    //      f32        4        4        0
    //    simd2        4        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        9        0
    //  no simd       12       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([-(reverse[e15] * self[e23]) - (reverse[e25] * self[e31]) - (reverse[e35] * self[e12]), 0.0])
                - (Simd32x2::from(reverse[e23]) * Simd32x2::from([self[e15], self[e23]]))
                - (Simd32x2::from(reverse[e31]) * Simd32x2::from([self[e25], self[e31]]))
                - (Simd32x2::from(reverse[e12]) * Simd32x2::from([self[e35], self[e12]])),
        );
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, dot_product[scalar] * -1.0]) + geometric_product.group0());
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        0
    //  no simd       16       21        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([
                (reverse[e3215] * self[scalar]) - (reverse[e15] * self[e23]) - (reverse[e25] * self[e31]) - (reverse[e35] * self[e12]),
                0.0,
            ]) + (Simd32x2::from(reverse[scalar]) * Simd32x2::from([self[e3215], self[scalar]]))
                - (Simd32x2::from(reverse[e23]) * Simd32x2::from([self[e15], self[e23]]))
                - (Simd32x2::from(reverse[e31]) * Simd32x2::from([self[e25], self[e31]]))
                - (Simd32x2::from(reverse[e12]) * Simd32x2::from([self[e35], self[e12]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, dot_product[scalar] * -1.0]) + geometric_product.group0());
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
    //      f32       24       34        0
    //    simd3        0        5        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       44       57        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            ]) + (crate::swizzle!(self.group1(), 3, 3, 3, 2) * reverse.group1().truncate_to_3().extend_to_4(reverse[e125]))
                + (crate::swizzle!(reverse.group0(), 2, 0, 1) * crate::swizzle!(self.group2(), 1, 2, 0)).extend_to_4(reverse[e235] * self[e415])
                + (crate::swizzle!(reverse.group2(), 1, 2, 0) * crate::swizzle!(self.group0(), 2, 0, 1)).extend_to_4(reverse[e315] * self[e425])
                + (crate::swizzle!(reverse.group1(), 3, 3, 3, _) * self.group1().truncate_to_3()).extend_to_4(reverse[e415] * self[e235]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
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
    //      f32       26       33        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       31       40        0
    //  no simd       46       59        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            ]) + (crate::swizzle!(reverse.group1(), 0, 1, 2, 2) * crate::swizzle!(self.group1(), 3, 3, 3, _).extend_to_4(self[e125]))
                + (crate::swizzle!(self.group1(), 0, 1, 2, 0) * crate::swizzle!(reverse.group1(), 3, 3, 3, _).extend_to_4(reverse[e235]))
                + (crate::swizzle!(self.group2(), 1, 2, 0, 0) * crate::swizzle!(reverse.group0(), 2, 0, 1).extend_to_4(reverse[e415]))
                + (crate::swizzle!(self.group0(), 2, 0, 1) * crate::swizzle!(reverse.group2(), 1, 2, 0, _)).extend_to_4(reverse[e425] * self[e315]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
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
    //      f32       32       41        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       35       46        0
    //  no simd       44       57        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            ]) - (crate::swizzle!(reverse.group0(), 2, 0, 1) * crate::swizzle!(self.group2(), 1, 2, 0)).extend_to_4(reverse[e15] * self[e23])
                - (crate::swizzle!(reverse.group2(), 1, 2, 0) * crate::swizzle!(self.group0(), 2, 0, 1)).extend_to_4(reverse[e25] * self[e31]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
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
    //      f32       49       56        0
    //    simd3        0        3        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       60       69        0
    //  no simd       93      105        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            ]) + (crate::swizzle!(self.group1(), 3, 3, _, _).extend_to_4(self[e4315], self[e45]) * crate::swizzle!(reverse.group1(), 0, 1, 0, _).extend_to_4(reverse[e3215]))
                + (crate::swizzle!(self.group2(), 2, 0, _, _).extend_to_4(self[e3215], self[e25]) * crate::swizzle!(reverse.group0(), 1, 2, 2).extend_to_4(reverse[e4315]))
                + (crate::swizzle!(self.group3(), 3, 3, _, _).extend_to_4(self[e25], self[e15]) * crate::swizzle!(reverse.group0(), 0, 1, 0).extend_to_4(reverse[e4235]))
                + (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(reverse.group2(), 2, 0, 1, _)).extend_to_4(reverse[e4125] * self[e35])
                - (Simd32x4::from([reverse[e25], reverse[e3215], reverse[e3215], self[e35]]) * crate::swizzle!(self.group0(), 2, 1, 2).extend_to_4(reverse[e12]))
                - (Simd32x4::from([reverse[e3215], reverse[e35], reverse[e15], self[e25]]) * crate::swizzle!(self.group0(), 0, 0, 1).extend_to_4(reverse[e31]))
                - (crate::swizzle!(reverse.group1(), 2, 0, 1, 3) * crate::swizzle!(self.group3(), 1, 2, 0, 3))
                - (crate::swizzle!(reverse.group2(), 0, 1, 2, 0) * crate::swizzle!(self.group2(), 3, 3, 3, _).extend_to_4(self[e23]))
                - (crate::swizzle!(self.group2(), 1, 2, 0, 0) * crate::swizzle!(reverse.group0(), 2, 0, 1).extend_to_4(reverse[e23]))
                - (crate::swizzle!(reverse.group3(), 2, 0, 1, _) * crate::swizzle!(self.group1(), 1, 2, 0, _)).extend_to_4(reverse[e15] * self[e4235]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[e3215]) + f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([self[e5] * self[e12345], f32::powi(self[e12345], 2)]) * Simd32x2::from([-2.0, -1.0]),
        );
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([geometric_product[e3215], 0.0]));
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
        let reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e45] * self[e45]);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2));
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       16       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([
                (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35]) + (reverse[e3215] * self[e45]) - (reverse[e45] * self[e3215]),
                0.0,
            ]) + (Simd32x2::from([reverse[e4235], reverse[e45]]) * crate::swizzle!(self.group0(), 0, 3, _, _))
                - (Simd32x2::from(self[e4235]) * Simd32x2::from([reverse[e15], reverse[e4235]]))
                - (Simd32x2::from(self[e4315]) * Simd32x2::from([reverse[e25], reverse[e4315]]))
                - (Simd32x2::from(self[e4125]) * Simd32x2::from([reverse[e35], reverse[e4125]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, dot_product[scalar] * -1.0]) + geometric_product.group0());
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
    //      f32        4        4        0
    //    simd2        4        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        9        0
    //  no simd       12       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([(reverse[e235] * self[e415]) + (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]), 0.0])
                + (Simd32x2::from(reverse[e415]) * Simd32x2::from([self[e235], self[e415]]))
                + (Simd32x2::from(reverse[e425]) * Simd32x2::from([self[e315], self[e425]]))
                + (Simd32x2::from(reverse[e435]) * Simd32x2::from([self[e125], self[e435]])),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, dot_product[scalar] * -1.0]) + geometric_product.group0());
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
    //      f32        6        5        0
    //    simd2        5        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       11        0
    //  no simd       16       21        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([
                (reverse[e235] * self[e415]) + (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]) - (reverse[e5] * self[e12345]),
                0.0,
            ]) + (Simd32x2::from(reverse[e415]) * Simd32x2::from([self[e235], self[e415]]))
                + (Simd32x2::from(reverse[e425]) * Simd32x2::from([self[e315], self[e425]]))
                + (Simd32x2::from(reverse[e435]) * Simd32x2::from([self[e125], self[e435]]))
                - (Simd32x2::from(reverse[e12345]) * Simd32x2::from([self[e5], self[e12345]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, dot_product[scalar] * -1.0]) + geometric_product.group0());
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
    //      f32      139      163        0
    //    simd2       17       16        0
    //    simd3        0       22        0
    //    simd4       56       40        0
    // Totals...
    // yes simd      212      241        0
    //  no simd      397      421        0
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
                + (Simd32x2::from([self[e1234], self[e1]]) * crate::swizzle!(reverse.group9(), 3, 0, _, _))
                - (Simd32x2::from(reverse[e41]) * Simd32x2::from([self[e15], self[e235]]))
                - (Simd32x2::from(reverse[e42]) * Simd32x2::from([self[e25], self[e315]]))
                - (Simd32x2::from(reverse[e43]) * Simd32x2::from([self[e35], self[e125]]))
                - (Simd32x2::from(reverse[e23]) * Simd32x2::from([self[e23], self[e415]]))
                - (Simd32x2::from(reverse[e31]) * Simd32x2::from([self[e31], self[e425]]))
                - (Simd32x2::from(reverse[e12]) * Simd32x2::from([self[e12], self[e435]]))
                - (Simd32x2::from(self[e41]) * Simd32x2::from([reverse[e15], reverse[e235]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([reverse[e25], reverse[e315]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([reverse[e35], reverse[e125]]))
                - (Simd32x2::from([self[e321], self[e23]]) * crate::swizzle!(reverse.group6(), 3, 0, _, _)),
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse[e15] * self[e4]) + (reverse[e25] * self[e412]) + (reverse[e435] * self[e4315]) + (reverse[e4125] * self[e425]),
                (reverse[e25] * self[e4]) + (reverse[e415] * self[e4125]) + (reverse[e4235] * self[e435]) + (reverse[e3215] * self[e431]),
                (reverse[e35] * self[e4]) + (reverse[e425] * self[e4235]) + (reverse[e4315] * self[e415]) + (reverse[e3215] * self[e412]),
                -(reverse[e4] * self[e45]) - (reverse[e321] * self[e1234]) - (reverse[e4315] * self[e431]) - (reverse[e4125] * self[e412]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group1())
                + (Simd32x4::from([reverse[e2], reverse[e321], reverse[e321], self[e4]]) * crate::swizzle!(self.group5(), 2, 1, 2).extend_to_4(reverse[e45]))
                + (Simd32x4::from([reverse[e5], reverse[e5], reverse[e5], reverse[e2]]) * self.group4().extend_to_4(self[e42]))
                + (Simd32x4::from([reverse[e321], reverse[e3], reverse[e1], reverse[e3]]) * crate::swizzle!(self.group5(), 0, 0, 1).extend_to_4(self[e43]))
                + (Simd32x4::from([reverse[e3215], reverse[e35], reverse[e15], reverse[e1234]]) * crate::swizzle!(self.group7(), 0, 0, 1).extend_to_4(self[e321]))
                + (Simd32x4::from([self[e1234], self[e1234], self[e42], self[e4125]]) * crate::swizzle!(reverse.group8(), 0, 1, 0).extend_to_4(reverse[e412]))
                + (crate::swizzle!(self.group0(), 0, 0).extend_to_4(self[scalar], reverse[e12345]) * reverse.group1().truncate_to_3().extend_to_4(self[e1234]))
                + (crate::swizzle!(self.group4(), 2, 0, _).extend_to_4(self[e1234], reverse[e1]) * crate::swizzle!(reverse.group8(), 1, 2, 2).extend_to_4(self[e41]))
                + (crate::swizzle!(self.group1(), 2, 0, _, _).extend_to_4(self[e321], self[e4235]) * crate::swizzle!(reverse.group5(), 1, 2, 2).extend_to_4(reverse[e423]))
                + (crate::swizzle!(self.group6(), 3, 3, _, _).extend_to_4(self[e2], reverse[e1234]) * crate::swizzle!(reverse.group5(), 0, 1, 0).extend_to_4(self[e12345]))
                + (crate::swizzle!(reverse.group4(), 2, 0, 1) * crate::swizzle!(self.group8(), 1, 2, 0)).extend_to_4(reverse[e4] * self[scalar])
                + (crate::swizzle!(reverse.group7(), 2, 0, 1) * crate::swizzle!(self.group3(), 1, 2, 0, _)).extend_to_4(reverse[e431] * self[e4315])
                - (Simd32x4::from([reverse[e1234], reverse[e1234], reverse[e1234], self[e31]]) * self.group8().extend_to_4(reverse[e431]))
                - (Simd32x4::from([self[e5], self[e5], self[e315], self[e2]]) * crate::swizzle!(reverse.group4(), 0, 1, 0).extend_to_4(reverse[e42]))
                - (crate::swizzle!(self.group1(), 1, 2, 0, 2) * crate::swizzle!(reverse.group5(), 2, 0, 1).extend_to_4(reverse[e43]))
                - (crate::swizzle!(reverse.group0(), 1, 1).extend_to_4(reverse[e12345], reverse[e41]) * self.group9().truncate_to_3().extend_to_4(self[e1]))
                - (crate::swizzle!(self.group0(), 1, 1).extend_to_4(self[e12345], reverse[e41]) * reverse.group9().truncate_to_3().extend_to_4(self[e415]))
                - (crate::swizzle!(self.group8(), 2, 0, _).extend_to_4(self[e5], self[e425]) * crate::swizzle!(reverse.group4(), 1, 2, 2).extend_to_4(reverse[e42]))
                - (crate::swizzle!(self.group3(), 2, 0, _, _).extend_to_4(self[e3215], self[e423]) * crate::swizzle!(reverse.group7(), 1, 2, 2).extend_to_4(reverse[e23]))
                - (crate::swizzle!(self.group3(), 3, 3, _, _).extend_to_4(self[e4315], reverse[e425]) * crate::swizzle!(reverse.group6(), 0, 1, 0, _).extend_to_4(self[e42]))
                - (crate::swizzle!(self.group9(), 2, 0, _, _).extend_to_4(self[e45], reverse[e435]) * crate::swizzle!(reverse.group6(), 1, 2, 2, _).extend_to_4(self[e43]))
                - (crate::swizzle!(self.group9(), 3, 3, _, _).extend_to_4(self[e25], self[e435]) * crate::swizzle!(reverse.group7(), 0, 1, 0).extend_to_4(reverse[e43]))
                - (crate::swizzle!(reverse.group8(), 2, 0, 1) * crate::swizzle!(self.group4(), 1, 2, 0)).extend_to_4(reverse[e31] * self[e431])
                - (crate::swizzle!(self.group5(), 1, 2, 0) * crate::swizzle!(reverse.group1(), 2, 0, 1, _)).extend_to_4(reverse[e12] * self[e412])
                - (crate::swizzle!(self.group7(), 1, 2, 0) * crate::swizzle!(reverse.group3(), 2, 0, 1, _)).extend_to_4(reverse[e423] * self[e23])
                - (crate::swizzle!(reverse.group1(), 3, 3, 3, _) * self.group3().truncate_to_3()).extend_to_4(reverse[e412] * self[e12])
                - (crate::swizzle!(reverse.group3(), 3, 3, 3, _) * self.group6().truncate_to_3()).extend_to_4(reverse[e415] * self[e41])
                - (crate::swizzle!(reverse.group9(), 1, 2, 0, _) * crate::swizzle!(self.group6(), 2, 0, 1, _)).extend_to_4(reverse[e4235] * self[e423]),
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
                + (Simd32x4::from([reverse[e5], reverse[e5], reverse[e5], self[e45]]) * self.group7().extend_to_4(reverse[e3215]))
                + (Simd32x4::from([reverse[e45], reverse[e4125], reverse[e4235], self[e25]]) * crate::swizzle!(self.group5(), 0, 0, 1).extend_to_4(reverse[e4315]))
                + (Simd32x4::from([reverse[e4315], reverse[e45], reverse[e45], self[e35]]) * crate::swizzle!(self.group5(), 2, 1, 2).extend_to_4(reverse[e4125]))
                + (crate::swizzle!(reverse.group0(), 1, 1).extend_to_4(reverse[e12345], self[scalar]) * self.group1().truncate_to_3().extend_to_4(reverse[e3215]))
                + (crate::swizzle!(self.group0(), 0, 0).extend_to_4(self[scalar], reverse[e235]) * reverse.group9().truncate_to_3().extend_to_4(self[e415]))
                + (crate::swizzle!(self.group0(), 1, 1).extend_to_4(self[e12345], reverse[e315]) * reverse.group1().truncate_to_3().extend_to_4(self[e425]))
                + (crate::swizzle!(self.group7(), 2, 0, _).extend_to_4(self[e4], reverse[e435]) * crate::swizzle!(reverse.group8(), 1, 2, 2).extend_to_4(self[e125]))
                + (crate::swizzle!(self.group1(), 3, 3, _, _).extend_to_4(self[e431], reverse[e3]) * crate::swizzle!(reverse.group8(), 0, 1, 0).extend_to_4(self[e125]))
                + (crate::swizzle!(self.group3(), 2, 0, _, _).extend_to_4(self[e3215], reverse[e1]) * crate::swizzle!(reverse.group4(), 1, 2, 2).extend_to_4(self[e235]))
                + (crate::swizzle!(self.group3(), 3, 3, _, _).extend_to_4(self[e4315], reverse[e415]) * crate::swizzle!(reverse.group5(), 0, 1, 0).extend_to_4(self[e235]))
                + (crate::swizzle!(self.group9(), 2, 0, _, _).extend_to_4(self[e45], reverse[e2]) * crate::swizzle!(reverse.group5(), 1, 2, 2).extend_to_4(self[e315]))
                + (crate::swizzle!(self.group9(), 3, 3, _, _).extend_to_4(self[e25], self[e435]) * crate::swizzle!(reverse.group4(), 0, 1, 0).extend_to_4(reverse[e125]))
                + (crate::swizzle!(reverse.group7(), 2, 0, 1) * crate::swizzle!(self.group8(), 1, 2, 0)).extend_to_4(reverse[e425] * self[e315])
                + (crate::swizzle!(self.group4(), 1, 2, 0) * crate::swizzle!(reverse.group3(), 2, 0, 1, _)).extend_to_4(reverse[e4235] * self[e15])
                + (crate::swizzle!(reverse.group1(), 1, 2, 0, _) * crate::swizzle!(self.group6(), 2, 0, 1, _)).extend_to_4(reverse[e5] * self[e321])
                - (Simd32x4::from([reverse[e25], reverse[e3215], reverse[e3215], self[e2]]) * crate::swizzle!(self.group4(), 2, 1, 2).extend_to_4(reverse[e315]))
                - (Simd32x4::from([reverse[e3215], reverse[e35], reverse[e15], self[e1]]) * crate::swizzle!(self.group4(), 0, 0, 1).extend_to_4(reverse[e235]))
                - (Simd32x4::from([self[e5], self[e5], self[e315], self[e15]]) * crate::swizzle!(reverse.group7(), 0, 1, 0).extend_to_4(reverse[e23]))
                - (Simd32x4::from([self[e1234], self[e1234], self[e1234], reverse[e35]]) * reverse.group3().truncate_to_3().extend_to_4(self[e12]))
                - (crate::swizzle!(self.group8(), 2, 0, _).extend_to_4(self[e5], self[e25]) * crate::swizzle!(reverse.group7(), 1, 2, 2).extend_to_4(reverse[e31]))
                - (self.group8() * crate::swizzle!(reverse.group1(), 3, 3, 3, _)).extend_to_4(reverse[e15] * self[e23])
                - (crate::swizzle!(reverse.group4(), 2, 0, 1) * crate::swizzle!(self.group3(), 1, 2, 0, _)).extend_to_4(reverse[e12345] * self[e5])
                - (crate::swizzle!(reverse.group5(), 2, 0, 1) * crate::swizzle!(self.group9(), 1, 2, 0, _)).extend_to_4(reverse[e5] * self[e12345])
                - (crate::swizzle!(reverse.group8(), 2, 0, 1) * crate::swizzle!(self.group7(), 1, 2, 0)).extend_to_4(reverse[e12] * self[e35])
                - (crate::swizzle!(self.group5(), 1, 2, 0) * crate::swizzle!(reverse.group9(), 2, 0, 1, _)).extend_to_4(reverse[e125] * self[e3])
                - (crate::swizzle!(reverse.group1(), 2, 0, 1, _) * crate::swizzle!(self.group6(), 1, 2, 0, _)).extend_to_4(reverse[e25] * self[e31])
                - (crate::swizzle!(reverse.group6(), 2, 0, 1, _) * crate::swizzle!(self.group1(), 1, 2, 0, _)).extend_to_4(reverse[e15] * self[e4235]),
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
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e3215] * self[e1234])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                + f32::powi(self[e45], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e4] * self[e5])
                - 2.0 * (self[e15] * self[e41])
                - 2.0 * (self[e25] * self[e42])
                - 2.0 * (self[e35] * self[e43]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([dot_product[scalar] * -1.0, 0.0]) + geometric_product.group0(),
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
    //      f32       41       45        0
    //    simd3        0        4        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       57       64        0
    //  no simd      105      117        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            (crate::swizzle!(reverse.group1(), 0, 1, 0, 2) * crate::swizzle!(self.group1(), 3, 3, _, _).extend_to_4(self[e2], self[e125]))
                + (crate::swizzle!(reverse.group2(), 0, 1, 0, 2) * crate::swizzle!(self.group3(), 3, 3, _, _).extend_to_4(self[e431], self[e435]))
                + (crate::swizzle!(reverse.group2(), 1, 2, 2, 3) * crate::swizzle!(self.group0(), 2, 0, _, _).extend_to_4(self[e4], self[e321]))
                + (crate::swizzle!(reverse.group3(), 0, 1, 0, 1) * crate::swizzle!(self.group0(), 3, 3, _, _).extend_to_4(self[e425], self[e315]))
                + (crate::swizzle!(reverse.group3(), 1, 2, 2, 2) * crate::swizzle!(self.group1(), 2, 0, _, _).extend_to_4(self[e12345], self[e125]))
                + (crate::swizzle!(self.group1(), 0, 1, 2, 1) * crate::swizzle!(reverse.group1(), 3, 3, 3, _).extend_to_4(reverse[e315]))
                + (crate::swizzle!(self.group2(), 1, 2, 0, 0) * crate::swizzle!(reverse.group0(), 2, 0, 1, _).extend_to_4(reverse[e415]))
                + (crate::swizzle!(self.group3(), 2, 0, _, _).extend_to_4(self[e321], self[e415]) * crate::swizzle!(reverse.group1(), 1, 2, 2, _).extend_to_4(reverse[e235]))
                + (crate::swizzle!(reverse.group0(), 3, 3, 3, _) * self.group3().truncate_to_3()).extend_to_4(reverse[e425] * self[e315])
                + (crate::swizzle!(reverse.group2(), 3, 3, 3, _) * self.group0().truncate_to_3()).extend_to_4(reverse[e1] * self[e235])
                - (crate::swizzle!(reverse.group0(), 0, 1, 0, 3) * crate::swizzle!(self.group2(), 3, 3, 1, 3))
                - (crate::swizzle!(reverse.group2(), 2, 0, 1, 1) * crate::swizzle!(self.group0(), 1, 2, 0, _).extend_to_4(self[e2]))
                - (crate::swizzle!(self.group2(), 2, 0, 3, 3) * crate::swizzle!(reverse.group0(), 1, 2, 2, _).extend_to_4(reverse[e321]))
                - (crate::swizzle!(self.group3(), 1, 2, 0, 0) * crate::swizzle!(reverse.group1(), 2, 0, 1, _).extend_to_4(reverse[e235]))
                - (crate::swizzle!(reverse.group3(), 2, 0, 1, _) * crate::swizzle!(self.group1(), 1, 2, 0, _)).extend_to_4(reverse[e125] * self[e3])
                - (crate::swizzle!(reverse.group3(), 3, 3, 3, _) * self.group2().truncate_to_3()).extend_to_4(reverse[e5] * self[e12345]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e5] * self[e4]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
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
    //      f32       53       59        0
    //    simd3        0        2        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       66       74        0
    //  no simd      105      117        0
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
            Simd32x3::from(0.0).extend_to_4(
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
            Simd32x3::from(0.0).extend_to_4(
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
            ]) + (self.group1() * crate::swizzle!(reverse.group1(), 3, 3, 3, _).extend_to_4(reverse[e3215]))
                + (crate::swizzle!(reverse.group0(), 0, 1, 0, 3) * crate::swizzle!(self.group3(), 3, 3, _, _).extend_to_4(self[e25], self[e3215]))
                + (crate::swizzle!(self.group1(), 3, 3, _, _).extend_to_4(self[e4315], self[e35]) * crate::swizzle!(reverse.group1(), 0, 1, 0, _).extend_to_4(reverse[e4125]))
                + (crate::swizzle!(self.group2(), 2, 0, _, _).extend_to_4(self[e3215], self[e15]) * crate::swizzle!(reverse.group0(), 1, 2, 2, _).extend_to_4(reverse[e4235]))
                + (crate::swizzle!(self.group3(), 2, 0, _, _).extend_to_4(self[e45], self[scalar]) * crate::swizzle!(reverse.group1(), 1, 2, 2, _).extend_to_4(reverse[e3215]))
                + (crate::swizzle!(reverse.group0(), 3, 3, 3, _) * self.group3().truncate_to_3()).extend_to_4(reverse[e4315] * self[e25])
                - (crate::swizzle!(reverse.group1(), 2, 0, 1, 1) * crate::swizzle!(self.group3(), 1, 2, 0, _).extend_to_4(self[e25]))
                - (crate::swizzle!(self.group1(), 1, 2, 0, 0) * crate::swizzle!(reverse.group3(), 2, 0, 1, _).extend_to_4(reverse[e15]))
                - (crate::swizzle!(self.group2(), 1, 2, 0, 0) * crate::swizzle!(reverse.group0(), 2, 0, 1, _).extend_to_4(reverse[e23]))
                - (crate::swizzle!(self.group0(), 2, 0, _, _).extend_to_4(self[e1234], self[e3215]) * crate::swizzle!(reverse.group2(), 1, 2, 2, _).extend_to_4(reverse[e45]))
                - (crate::swizzle!(self.group2(), 3, 3, _, _).extend_to_4(self[e42], self[e35]) * crate::swizzle!(reverse.group2(), 0, 1, 0, _).extend_to_4(reverse[e12]))
                - (crate::swizzle!(reverse.group3(), 3, 3, 3, _) * self.group0().truncate_to_3()).extend_to_4(reverse[e15] * self[e4235]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[e3215]) + f32::powi(self[scalar], 2) + f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0() + Simd32x3::from(0.0).extend_to_4(dot_product[scalar] * -1.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
