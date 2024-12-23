// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 9
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:         8       8       0
//  Maximum:        38      37       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       5       0
//  Average:        12      14       0
//  Maximum:        71      74       0
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
impl std::ops::Div<anti_constraint_violation> for DualNum {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * self[e1234] * 2.0);
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
    //      f32       13       12        0
    //    simd2        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       15       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0), /* e423, e431, e412, e321 */ self.group1());
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, f32::powi(self[e4], 2) - f32::powi(self[e423], 2) - f32::powi(self[e431], 2) - f32::powi(self[e412], 2)])
                + Simd32x2::from([
                    (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3]) + (anti_reverse[e321] * self[e4])
                        - (anti_reverse[e1] * self[e423])
                        - (anti_reverse[e2] * self[e431])
                        - (anti_reverse[e3] * self[e412])
                        - (anti_reverse[e4] * self[e321]),
                    (anti_reverse[e423] * self[e423]) + (anti_reverse[e431] * self[e431]) + (anti_reverse[e412] * self[e412]) - (anti_reverse[e4] * self[e4]),
                ]),
        );
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
    //      f32        9        9        0
    //    simd2        1        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       11       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Line::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, f32::powi(self[e41], 2) + f32::powi(self[e42], 2) + f32::powi(self[e43], 2)])
                + Simd32x2::from([
                    -(anti_reverse[e41] * self[e23])
                        - (anti_reverse[e42] * self[e31])
                        - (anti_reverse[e43] * self[e12])
                        - (anti_reverse[e23] * self[e41])
                        - (anti_reverse[e31] * self[e42])
                        - (anti_reverse[e12] * self[e43]),
                    -(anti_reverse[e41] * self[e41]) - (anti_reverse[e42] * self[e42]) - (anti_reverse[e43] * self[e43]),
                ]),
        );
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
    //      f32       13       12        0
    //    simd2        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       15       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, f32::powi(self[e41], 2) + f32::powi(self[e42], 2) + f32::powi(self[e43], 2) - f32::powi(self[e1234], 2)])
                + Simd32x2::from([
                    (anti_reverse[e1234] * self[scalar]) + (anti_reverse[scalar] * self[e1234])
                        - (anti_reverse[e41] * self[e23])
                        - (anti_reverse[e42] * self[e31])
                        - (anti_reverse[e43] * self[e12])
                        - (anti_reverse[e23] * self[e41])
                        - (anti_reverse[e31] * self[e42])
                        - (anti_reverse[e12] * self[e43]),
                    (anti_reverse[e1234] * self[e1234]) - (anti_reverse[e41] * self[e41]) - (anti_reverse[e42] * self[e42]) - (anti_reverse[e43] * self[e43]),
                ]),
        );
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
    //      f32       21       18        0
    //    simd2        9        8        0
    //    simd3        0        4        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       38       37        0
    //  no simd       71       74        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4(),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (anti_reverse[e1234] * self[scalar]) + (anti_reverse[e321] * self[e4])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412])
                    - (anti_reverse[e4] * self[e321])
                    - (anti_reverse[e23] * self[e41])
                    - (anti_reverse[e31] * self[e42])
                    - (anti_reverse[e12] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(anti_reverse[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(anti_reverse[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(anti_reverse[e412]) * Simd32x2::from([self[e3], self[e412]]))
                + (Simd32x2::from(self[e1234]) * anti_reverse.group0())
                - (Simd32x2::from(anti_reverse[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(anti_reverse[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(anti_reverse[e43]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from([self[e423], self[e4]]) * anti_reverse.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(anti_reverse[e1234]) * self.group4())
                + (Simd32x4::from([self[e4], self[e412], self[e423], anti_reverse[e321]]) * anti_reverse.group2().xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * anti_reverse.group2().zyz().with_w(anti_reverse[e23]))
                + (self.group0().yy().with_zw(self[e1234], self[scalar]) * anti_reverse.group4().xyz().with_w(anti_reverse[e4]))
                + (anti_reverse.group1().ww().with_zw(anti_reverse[e431], self[e431]) * self.group2().xyx().with_w(anti_reverse[e31]))
                + (anti_reverse.group4().zx().with_zw(anti_reverse[e4], self[e412]) * self.group2().yzz().with_w(anti_reverse[e12]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse[e1] * self[e41])
                        - (anti_reverse[e2] * self[e42])
                        - (anti_reverse[e3] * self[e43])
                        - (anti_reverse[e42] * self[e2])
                        - (anti_reverse[e43] * self[e3])
                        - (anti_reverse[e423] * self[e23])
                        - (anti_reverse[e431] * self[e31])
                        - (anti_reverse[e412] * self[e12]),
                )
                - (anti_reverse.group2().yzx() * self.group4().zxy()).with_w(anti_reverse[scalar] * self[e4])
                - (self.group2().zxy() * anti_reverse.group4().yzx()).with_w(anti_reverse[e41] * self[e1]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                f32::powi(self[e4], 2) + f32::powi(self[e41], 2) + f32::powi(self[e42], 2) + f32::powi(self[e43], 2)
                    - f32::powi(self[e1234], 2)
                    - f32::powi(self[e423], 2)
                    - f32::powi(self[e431], 2)
                    - f32::powi(self[e412], 2),
            ]) + geometric_anti_product.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            geometric_anti_product.group4(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for Origin {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(self[e4], 2) * 2.0);
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
impl std::ops::Div<anti_constraint_violation> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Point {
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
        return AntiScalar::from_groups(
            // e1234
            f32::powi(self[e4], 2) - (self[e4] * Point::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from(-1.0))[e4]),
        );
    }
}
