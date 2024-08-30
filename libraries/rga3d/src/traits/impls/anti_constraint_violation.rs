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
//   Median:         4      10       0
//  Average:        18      22       0
//  Maximum:       111     121       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4      13       0
//  Average:        30      36       0
//  Maximum:       185     202       0
impl AntiConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(
            // scalar, e1234
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ f32::powi(self.group0()[1], 2));
        let subtraction = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e1234])]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       20       25        0
    //  no simd       44       52        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Flector::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(-1.0)), /* e423, e431, e412, e321 */ self.group1());
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(anti_reverse.group1()[0] * self.group0()[3]) - (anti_reverse.group1()[2] * self.group1()[1])),
                (-(anti_reverse.group1()[0] * self.group1()[2]) - (anti_reverse.group1()[1] * self.group0()[3])),
                (-(anti_reverse.group1()[1] * self.group1()[0]) - (anti_reverse.group1()[2] * self.group0()[3])),
                ((anti_reverse.group1()[1] * self.group1()[1]) + (anti_reverse.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from(anti_reverse.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group1()[0] * self.group0()[2])),
                ((anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group1()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[3] * self.group1()[3]) + (anti_reverse.group1()[3] * self.group0()[3])),
            ]) + (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[2]]) * swizzle!(self.group1(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]) * swizzle!(anti_reverse.group1(), 3, 3, 3, 2))
                - (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group1(), 1, 2, 0, 1) * swizzle!(self.group0(), 2, 0, 1, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e1234
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e1234]),
            ]),
            // e23, e31, e12, scalar
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       22       33        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Line::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e1234]),
            ]),
            // e23, e31, e12, scalar
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        4        4        0
    // Totals...
    // yes simd       11       14        0
    //  no simd       15       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = DualNum::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                (-(anti_reverse.group1()[0] * self.group0()[0]) - (anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
                0.0,
            ]) - (Simd32x2::from(anti_reverse.group0()[0]) * Simd32x2::from([self.group1()[0], self.group0()[0]]))
                - (Simd32x2::from(anti_reverse.group0()[1]) * Simd32x2::from([self.group1()[1], self.group0()[1]]))
                - (Simd32x2::from(anti_reverse.group0()[2]) * Simd32x2::from([self.group1()[2], self.group0()[2]]))
                + (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e1234
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e1234])]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       75       82        0
    //    simd2        8        8        0
    //    simd3       18       20        0
    //    simd4       10       11        0
    // Totals...
    // yes simd      111      121        0
    //  no simd      185      202        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (self.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group3() * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            self.group4(),
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                ((anti_reverse.group0()[1] * self.group0()[0])
                    - (anti_reverse.group3()[0] * self.group2()[0])
                    - (anti_reverse.group3()[1] * self.group2()[1])
                    - (anti_reverse.group3()[2] * self.group2()[2])
                    - (anti_reverse.group1()[0] * self.group4()[0])
                    - (anti_reverse.group1()[1] * self.group4()[1])
                    - (anti_reverse.group1()[2] * self.group4()[2])
                    + (anti_reverse.group4()[3] * self.group1()[3])),
                0.0,
            ]) + (Simd32x2::from(self.group0()[1]) * anti_reverse.group0())
                - (Simd32x2::from(anti_reverse.group2()[0]) * Simd32x2::from([self.group3()[0], self.group2()[0]]))
                - (Simd32x2::from(anti_reverse.group2()[1]) * Simd32x2::from([self.group3()[1], self.group2()[1]]))
                - (Simd32x2::from(anti_reverse.group2()[2]) * Simd32x2::from([self.group3()[2], self.group2()[2]]))
                - (Simd32x2::from(anti_reverse.group1()[3]) * Simd32x2::from([self.group4()[3], self.group1()[3]]))
                + (Simd32x2::from(anti_reverse.group4()[0]) * Simd32x2::from([self.group1()[0], self.group4()[0]]))
                + (Simd32x2::from(anti_reverse.group4()[1]) * Simd32x2::from([self.group1()[1], self.group4()[1]]))
                + (Simd32x2::from(anti_reverse.group4()[2]) * Simd32x2::from([self.group1()[2], self.group4()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * anti_reverse.group4()[0]) + (anti_reverse.group2()[0] * self.group4()[3]) - (anti_reverse.group2()[1] * self.group1()[2])
                    + (anti_reverse.group2()[2] * self.group1()[1])
                    - (anti_reverse.group3()[0] * self.group1()[3])
                    + (anti_reverse.group3()[1] * self.group4()[2])
                    + (self.group2()[0] * anti_reverse.group4()[3])
                    + (self.group2()[1] * anti_reverse.group1()[2])
                    - (self.group2()[2] * anti_reverse.group1()[1])
                    + (self.group3()[0] * anti_reverse.group1()[3])
                    + (self.group3()[1] * anti_reverse.group4()[2])),
                ((self.group0()[0] * anti_reverse.group4()[1]) + (anti_reverse.group2()[0] * self.group1()[2]) + (anti_reverse.group2()[1] * self.group4()[3])
                    - (anti_reverse.group2()[2] * self.group1()[0])
                    - (anti_reverse.group3()[1] * self.group1()[3])
                    + (anti_reverse.group3()[2] * self.group4()[0])
                    - (self.group2()[0] * anti_reverse.group1()[2])
                    + (self.group2()[1] * anti_reverse.group4()[3])
                    + (self.group2()[2] * anti_reverse.group1()[0])
                    + (self.group3()[1] * anti_reverse.group1()[3])
                    + (self.group3()[2] * anti_reverse.group4()[0])),
                ((self.group0()[0] * anti_reverse.group4()[2]) - (anti_reverse.group2()[0] * self.group1()[1])
                    + (anti_reverse.group2()[1] * self.group1()[0])
                    + (anti_reverse.group2()[2] * self.group4()[3])
                    + (anti_reverse.group3()[0] * self.group4()[1])
                    - (anti_reverse.group3()[2] * self.group1()[3])
                    + (self.group2()[0] * anti_reverse.group1()[1])
                    - (self.group2()[1] * anti_reverse.group1()[0])
                    + (self.group2()[2] * anti_reverse.group4()[3])
                    + (self.group3()[0] * anti_reverse.group4()[1])
                    + (self.group3()[2] * anti_reverse.group1()[3])),
                (-(anti_reverse.group2()[2] * self.group4()[2]) - (self.group2()[1] * anti_reverse.group4()[1]) - (self.group2()[2] * anti_reverse.group4()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group1())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group1())
                - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group2()[0]]) * swizzle!(self.group4(), 0, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group2()[1]]) * swizzle!(self.group4(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(anti_reverse.group4(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((anti_reverse.group4()[1] * self.group4()[2]) - (anti_reverse.group4()[2] * self.group4()[1])),
                (-(anti_reverse.group4()[0] * self.group4()[2]) + (anti_reverse.group4()[2] * self.group4()[0])),
                ((anti_reverse.group4()[0] * self.group4()[1]) - (anti_reverse.group4()[1] * self.group4()[0])),
            ]) + (Simd32x3::from(anti_reverse.group0()[1]) * self.group2())
                + (Simd32x3::from(self.group0()[1]) * anti_reverse.group2())
                - (Simd32x3::from(anti_reverse.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse.group4()[2]]))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                + (swizzle!(anti_reverse.group2(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(anti_reverse.group1()[1] * self.group4()[2]) + (anti_reverse.group1()[2] * self.group4()[1]) + (anti_reverse.group4()[1] * self.group1()[2])
                    - (anti_reverse.group4()[2] * self.group1()[1])),
                ((anti_reverse.group1()[0] * self.group4()[2]) - (anti_reverse.group1()[2] * self.group4()[0]) - (anti_reverse.group4()[0] * self.group1()[2])
                    + (anti_reverse.group4()[2] * self.group1()[0])),
                (-(anti_reverse.group1()[0] * self.group4()[1]) + (anti_reverse.group1()[1] * self.group4()[0]) + (anti_reverse.group4()[0] * self.group1()[1])
                    - (anti_reverse.group4()[1] * self.group1()[0])),
            ]) + (Simd32x3::from(anti_reverse.group0()[0]) * self.group2())
                + (Simd32x3::from(anti_reverse.group0()[1]) * self.group3())
                + (Simd32x3::from(self.group0()[0]) * anti_reverse.group2())
                + (Simd32x3::from(self.group0()[1]) * anti_reverse.group3())
                - (Simd32x3::from(anti_reverse.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(anti_reverse.group4()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse.group4()[2]]))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                + (swizzle!(anti_reverse.group2(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))
                - (swizzle!(anti_reverse.group3(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                + (swizzle!(anti_reverse.group3(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((anti_reverse.group2()[0] * self.group1()[3]) - (anti_reverse.group2()[1] * self.group4()[2]) + (self.group2()[1] * anti_reverse.group4()[2])),
                ((anti_reverse.group2()[1] * self.group1()[3]) - (anti_reverse.group2()[2] * self.group4()[0]) + (self.group2()[2] * anti_reverse.group4()[0])),
                (-(anti_reverse.group2()[0] * self.group4()[1]) + (anti_reverse.group2()[2] * self.group1()[3]) + (self.group2()[0] * anti_reverse.group4()[1])),
                (-(anti_reverse.group0()[0] * self.group1()[3])
                    - (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    + (anti_reverse.group3()[1] * self.group4()[1])
                    + (anti_reverse.group3()[2] * self.group4()[2])
                    - (self.group2()[0] * anti_reverse.group1()[0])
                    - (self.group2()[1] * anti_reverse.group1()[1])
                    - (self.group2()[2] * anti_reverse.group1()[2])
                    - (self.group3()[1] * anti_reverse.group4()[1])
                    - (self.group3()[2] * anti_reverse.group4()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group4())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group4())
                + (Simd32x4::from(anti_reverse.group1()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]))
                + (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group3()[0]]) * swizzle!(self.group4(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * swizzle!(anti_reverse.group4(), 1, 2, 0, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e1234
            (f32::powi(self.group0()[1], 2) - f32::powi(self.group2()[0], 2) - f32::powi(self.group2()[1], 2) - f32::powi(self.group2()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group4()[0], 2)
                + f32::powi(self.group4()[1], 2)
                + f32::powi(self.group4()[2], 2)),
        );
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e1234])]),
            // e1, e2, e3, e4
            geometric_anti_product.group1(),
            // e41, e42, e43
            geometric_anti_product.group2(),
            // e23, e31, e12
            geometric_anti_product.group3(),
            // e423, e431, e412, e321
            geometric_anti_product.group4(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Origin::from_groups(/* e4 */ (self[e4] * -1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ (anti_reverse[e4] * self[e4] * -1.0));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (f32::powi(self[e4], 2) * -1.0));
        let subtraction = AntiScalar::from_groups(/* e1234 */ (-anti_scalar_product[e1234] + geometric_anti_product[e1234]));
        return subtraction;
    }
}
impl AntiConstraintViolation for Plane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Point::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (anti_reverse.group0()[3] * self.group0()[3] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((anti_reverse.group0()[0] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[0])),
                ((anti_reverse.group0()[1] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[1])),
                ((anti_reverse.group0()[2] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[2])),
                0.0,
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e1234])]),
            // e23, e31, e12, scalar
            Simd32x4::from([geometric_anti_product.group1()[0], geometric_anti_product.group1()[1], geometric_anti_product.group1()[2], 0.0]),
        );
        return subtraction;
    }
}
