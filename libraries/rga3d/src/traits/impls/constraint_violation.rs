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
//   Median:         1       4       0
//  Average:         7       8       0
//  Maximum:        38      38       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       7       0
//  Average:        13      15       0
//  Maximum:        71      75       0
impl std::ops::Div<constraint_violation> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([f32::powi(self[scalar], 2), self[scalar] * self[e1234]]) * Simd32x2::from([1.0, 2.0]),
        );
        return AntiScalar::from_groups(/* e1234 */ geometric_product[e1234]);
    }
}
impl std::ops::Div<constraint_violation> for Flector {
    type Output = DualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Flector {
    type Output = DualNum;
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
        let reverse = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (reverse[e321] * self[e4]) - (reverse[e2] * self[e431]) - (reverse[e3] * self[e412]) - (reverse[e4] * self[e321])])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([reverse[e1], reverse[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([reverse[e2], reverse[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([reverse[e3], reverse[e412]]))
                - (Simd32x2::from([reverse[e321], reverse[e1]]) * self.group1().wx()),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([dot_product[scalar] * -1.0, 0.0]) + geometric_product.group0());
    }
}
impl std::ops::Div<constraint_violation> for Horizon {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Horizon::from_groups(/* e321 */ self[e321] * -1.0);
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for Line {
    type Output = DualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Line {
    type Output = DualNum;
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
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43])])
                - (Simd32x2::from(self[e23]) * Simd32x2::from([reverse[e23], reverse[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([reverse[e31], reverse[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([reverse[e12], reverse[e43]])),
        );
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([dot_product[scalar] * -1.0, 0.0]) + geometric_product.group0());
    }
}
impl std::ops::Div<constraint_violation> for Motor {
    type Output = DualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = DualNum;
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
            // e41, e42, e43, e1234
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (reverse[scalar] * self[e1234]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43]),
            ]) + (Simd32x2::from(self[scalar]) * Simd32x2::from([reverse[scalar], reverse[e1234]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([reverse[e23], reverse[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([reverse[e31], reverse[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([reverse[e12], reverse[e43]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([dot_product[scalar] * -1.0, 0.0]) + geometric_product.group0());
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
    //      f32       21       19        0
    //    simd2        9        8        0
    //    simd3        0        4        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       38       38        0
    //  no simd       71       75        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4() * Simd32x4::from(-1.0),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (reverse[e1234] * self[scalar]) + (reverse[e321] * self[e4])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412])
                    - (reverse[e4] * self[e321])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ]) + (Simd32x2::from(reverse[scalar]) * self.group0())
                + (Simd32x2::from(self[e1]) * Simd32x2::from([reverse[e1], reverse[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([reverse[e2], reverse[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([reverse[e3], reverse[e412]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([reverse[e23], reverse[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([reverse[e31], reverse[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([reverse[e12], reverse[e43]]))
                - (Simd32x2::from([reverse[e321], reverse[e1]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(reverse[scalar]) * self.group1())
                + (Simd32x4::from([reverse[e2], reverse[e321], reverse[e321], reverse[e3]]) * self.group3().zyz().with_w(self[e43]))
                + (Simd32x4::from([reverse[e321], reverse[e3], reverse[e1], reverse[e2]]) * self.group3().xxy().with_w(self[e42]))
                + (self.group0().xx().with_zw(self[scalar], reverse[e1234]) * reverse.group1().xyz().with_w(self[e321]))
                + (self.group1().zx().with_zw(self[e321], reverse[e1]) * reverse.group3().yzz().with_w(self[e41]))
                + (self.group4().ww().with_zw(self[e2], reverse[e4]) * reverse.group3().xyx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(reverse[e42] * self[e2])
                        - (reverse[e43] * self[e3])
                        - (reverse[e23] * self[e423])
                        - (reverse[e31] * self[e431])
                        - (reverse[e12] * self[e412])
                        - (reverse[e423] * self[e23])
                        - (reverse[e431] * self[e31])
                        - (reverse[e412] * self[e12]),
                )
                - (reverse.group3().zxy() * self.group1().yzx()).with_w(reverse[e321] * self[e1234])
                - (self.group3().yzx() * reverse.group1().zxy()).with_w(reverse[e41] * self[e1]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([dot_product[scalar] * -1.0, 0.0]) + geometric_product.group0(),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
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
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Plane::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for Point {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Point {
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
