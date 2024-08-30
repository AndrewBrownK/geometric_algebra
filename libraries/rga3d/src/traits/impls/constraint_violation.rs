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
//   Median:         8      12       0
//  Average:        21      26       0
//  Maximum:       111     121       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         8      13       0
//  Average:        34      41       0
//  Maximum:       185     202       0
impl ConstraintViolation for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            f32::powi(self.group0()[0], 2),
            ((self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
        ]));
        let subtraction = AntiScalar::from_groups(/* e1234 */ geometric_product.group0()[1]);
        return subtraction;
    }
}
impl ConstraintViolation for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       20       25        0
    //  no simd       44       52        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ (self.group1() * Simd32x4::from(-1.0)));
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + (Simd32x4::from(reverse.group1()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[2]]))
                - (swizzle!(reverse.group0(), 3, 3, 3, 1) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]))
                + (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(reverse.group1(), 2, 0, 1, 0))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group0()[1]) - (self.group0()[2] * reverse.group1()[1])),
                    ((self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])),
                    ((self.group1()[1] * reverse.group0()[0]) - (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group1()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group0()[2] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[3])),
                    (-(self.group0()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group0()[2] * reverse.group1()[3]) - (self.group0()[1] * reverse.group0()[0])),
                    ((self.group0()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group0()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_product.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group1()[0],
                geometric_product.group1()[1],
                geometric_product.group1()[2],
                (geometric_product.group1()[3] - scalar_product[scalar]),
            ]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Horizon::from_groups(/* e321 */ (self[e321] * -1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (self[e321] * reverse[e321] * -1.0));
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self[e321], 2) * -1.0));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       22       33        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Line::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((self.group1()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[0])),
                ((self.group1()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[1])
                    + (self.group0()[1] * reverse.group1()[0])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(self.group1()[1] * reverse.group1()[2]) + (self.group1()[2] * reverse.group1()[1])),
                ((self.group1()[0] * reverse.group1()[2]) - (self.group1()[2] * reverse.group1()[0])),
                (-(self.group1()[0] * reverse.group1()[1]) + (self.group1()[1] * reverse.group1()[0])),
                (-(self.group1()[2] * reverse.group1()[2]) - (self.group1()[0] * reverse.group1()[0]) - (self.group1()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2)));
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_product.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group1()[0],
                geometric_product.group1()[1],
                geometric_product.group1()[2],
                (geometric_product.group1()[3] - scalar_product[scalar]),
            ]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       30        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       36        0
    //  no simd       44       54        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(self.group1()[3]) * reverse.group0()) - (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + (Simd32x4::from(self.group0()[3]) * reverse.group1())
                - (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(reverse.group1(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group0()[3])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group1()[3])),
                    ((self.group1()[1] * reverse.group0()[3])
                        + (self.group1()[0] * reverse.group0()[2])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group1()[3])),
                    ((self.group1()[2] * reverse.group0()[3])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[3])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            ((Simd32x4::from(self.group1()[3]) * reverse.group1()) - (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group1(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group1()[1]) + (self.group1()[0] * reverse.group1()[3])),
                    ((self.group1()[0] * reverse.group1()[2]) + (self.group1()[1] * reverse.group1()[3])),
                    ((self.group1()[2] * reverse.group1()[3]) + (self.group1()[1] * reverse.group1()[0])),
                    (-(self.group1()[0] * reverse.group1()[0]) - (self.group1()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[3], 2) - f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2)),
        );
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_product.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group1()[0],
                geometric_product.group1()[1],
                geometric_product.group1()[2],
                (geometric_product.group1()[3] - scalar_product[scalar]),
            ]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MultiVector {
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
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            (self.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group3() * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            (self.group4() * Simd32x4::from(-1.0)),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (-(Simd32x2::from(self.group4()[3]) * Simd32x2::from([reverse.group4()[3], reverse.group1()[3]]))
                - (Simd32x2::from(self.group3()[2]) * Simd32x2::from([reverse.group3()[2], reverse.group2()[2]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([reverse.group3()[1], reverse.group2()[1]]))
                - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([reverse.group3()[0], reverse.group2()[0]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group1()[2], reverse.group4()[2]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group1()[1], reverse.group4()[1]]))
                + (Simd32x2::from(self.group0()[0]) * reverse.group0())
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group1()[0], reverse.group4()[0]]))
                + Simd32x2::from([
                    0.0,
                    (-(self.group4()[2] * reverse.group1()[2])
                        - (self.group4()[1] * reverse.group1()[1])
                        - (self.group4()[0] * reverse.group1()[0])
                        - (self.group2()[2] * reverse.group3()[2])
                        - (self.group2()[1] * reverse.group3()[1])
                        - (self.group2()[0] * reverse.group3()[0])
                        + (self.group1()[3] * reverse.group4()[3])
                        + (self.group0()[1] * reverse.group0()[0])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group4()[3]) * Simd32x4::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2], reverse.group0()[1]]))
                + (swizzle!(reverse.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[2]]))
                + (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group3()[1], reverse.group0()[0], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group2()[2]]))
                + (swizzle!(reverse.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group2()[1]]))
                + Simd32x4::from([
                    (-(self.group3()[1] * reverse.group1()[2]) + (self.group3()[0] * reverse.group4()[3]) + (self.group1()[0] * reverse.group0()[0])),
                    (-(self.group3()[2] * reverse.group1()[0]) + (self.group3()[1] * reverse.group4()[3]) + (self.group1()[0] * reverse.group3()[2])),
                    ((self.group3()[2] * reverse.group4()[3]) - (self.group3()[0] * reverse.group1()[1]) + (self.group1()[1] * reverse.group3()[0])),
                    (-(self.group4()[2] * reverse.group3()[2])
                        - (self.group4()[1] * reverse.group3()[1])
                        - (self.group4()[0] * reverse.group3()[0])
                        - (self.group3()[2] * reverse.group4()[2])
                        - (self.group3()[1] * reverse.group4()[1])
                        - (self.group3()[0] * reverse.group4()[0])
                        + (self.group2()[0] * reverse.group1()[0])
                        - (self.group1()[1] * reverse.group2()[1])
                        - (self.group1()[0] * reverse.group2()[0])
                        + (self.group0()[0] * reverse.group1()[3])
                        - (self.group0()[1] * reverse.group4()[3])),
                ])),
            // e41, e42, e43
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([reverse.group4()[0], reverse.group4()[1], reverse.group4()[2]]))
                + (Simd32x3::from(reverse.group4()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2) * Simd32x3::from([reverse.group2()[1], reverse.group0()[1], reverse.group0()[1]]))
                - (swizzle!(self.group3(), 1, 2, 0) * swizzle!(reverse.group2(), 2, 0, 1))
                + (swizzle!(self.group3(), 0, 0, 1) * Simd32x3::from([reverse.group0()[1], reverse.group2()[2], reverse.group2()[0]]))
                + (swizzle!(self.group2(), 2, 1, 2) * Simd32x3::from([reverse.group3()[1], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group2(), 1, 2, 0) * swizzle!(reverse.group3(), 2, 0, 1))
                + (swizzle!(self.group2(), 0, 0, 1) * Simd32x3::from([reverse.group0()[0], reverse.group3()[2], reverse.group3()[0]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                - (Simd32x3::from(reverse.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group2())
                + (Simd32x3::from(self.group0()[1]) * reverse.group3())
                + Simd32x3::from([
                    ((self.group4()[2] * reverse.group1()[1]) - (self.group4()[1] * reverse.group1()[2]) - (self.group1()[2] * reverse.group4()[1])
                        + (self.group1()[1] * reverse.group4()[2])),
                    (-(self.group4()[2] * reverse.group1()[0]) + (self.group4()[0] * reverse.group1()[2]) + (self.group1()[2] * reverse.group4()[0])
                        - (self.group1()[0] * reverse.group4()[2])),
                    ((self.group4()[1] * reverse.group1()[0]) - (self.group4()[0] * reverse.group1()[1]) - (self.group1()[1] * reverse.group4()[0])
                        + (self.group1()[0] * reverse.group4()[1])),
                ])),
            // e23, e31, e12
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2) * Simd32x3::from([reverse.group3()[1], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group3(), 1, 2, 0) * swizzle!(reverse.group3(), 2, 0, 1))
                + (swizzle!(self.group3(), 0, 0, 1) * Simd32x3::from([reverse.group0()[0], reverse.group3()[2], reverse.group3()[0]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group3())
                - (Simd32x3::from(reverse.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + Simd32x3::from([
                    (-(self.group1()[2] * reverse.group1()[1]) + (self.group1()[1] * reverse.group1()[2])),
                    ((self.group1()[2] * reverse.group1()[0]) - (self.group1()[0] * reverse.group1()[2])),
                    (-(self.group1()[1] * reverse.group1()[0]) + (self.group1()[0] * reverse.group1()[1])),
                ])),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group4()[3]) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group0()[0]]))
                + (swizzle!(reverse.group4(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]))
                - (swizzle!(reverse.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[2]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group3()[2]]))
                - (swizzle!(reverse.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[1]]))
                + Simd32x4::from([
                    ((self.group4()[2] * reverse.group3()[1]) - (self.group4()[1] * reverse.group3()[2]) + (self.group4()[0] * reverse.group0()[0])
                        - (self.group3()[1] * reverse.group4()[2])
                        + (self.group3()[0] * reverse.group1()[3])
                        + (self.group2()[1] * reverse.group1()[2])
                        - (self.group2()[0] * reverse.group4()[3])
                        + (self.group1()[3] * reverse.group3()[0])
                        + (self.group1()[2] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group0()[1])
                        + (self.group0()[0] * reverse.group4()[0])),
                    (-(self.group4()[2] * reverse.group3()[0]) + (self.group4()[1] * reverse.group0()[0]) + (self.group4()[0] * reverse.group3()[2])
                        - (self.group3()[2] * reverse.group4()[0])
                        + (self.group3()[1] * reverse.group1()[3])
                        + (self.group2()[2] * reverse.group1()[0])
                        - (self.group2()[1] * reverse.group4()[3])
                        + (self.group1()[3] * reverse.group3()[1])
                        + (self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group2()[2])
                        + (self.group0()[0] * reverse.group4()[1])),
                    ((self.group4()[2] * reverse.group0()[0]) + (self.group4()[1] * reverse.group3()[0]) - (self.group4()[0] * reverse.group3()[1])
                        + (self.group3()[2] * reverse.group1()[3])
                        - (self.group3()[0] * reverse.group4()[1])
                        - (self.group2()[2] * reverse.group4()[3])
                        + (self.group2()[0] * reverse.group1()[1])
                        + (self.group1()[3] * reverse.group3()[2])
                        + (self.group1()[2] * reverse.group0()[1])
                        + (self.group1()[1] * reverse.group2()[0])
                        + (self.group0()[0] * reverse.group4()[2])),
                    (-(self.group3()[0] * reverse.group1()[0]) - (self.group1()[1] * reverse.group3()[1]) - (self.group1()[0] * reverse.group3()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group4()[3], 2) - f32::powi(self.group3()[2], 2) - f32::powi(self.group3()[1], 2) - f32::powi(self.group3()[0], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group0()[0], 2)
                + f32::powi(self.group1()[0], 2)),
        );
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(geometric_product.group0()[0] - scalar_product[scalar]), geometric_product.group0()[1]]),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e41, e42, e43
            geometric_product.group2(),
            // e23, e31, e12
            geometric_product.group3(),
            // e423, e431, e412, e321
            geometric_product.group4(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Plane::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((self.group0()[0] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[0])),
                ((self.group0()[1] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[1])),
                ((self.group0()[2] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * reverse.group0()[3] * -1.0)]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2], 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group1()[3] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (-(self.group0()[0] * self.group0()[3]) + (self.group0()[3] * self.group0()[0])),
                (-(self.group0()[1] * self.group0()[3]) + (self.group0()[3] * self.group0()[1])),
                (-(self.group0()[2] * self.group0()[3]) + (self.group0()[3] * self.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self.group0()[1] * self.group0()[2]) - (self.group0()[2] * self.group0()[1])),
                (-(self.group0()[0] * self.group0()[2]) + (self.group0()[2] * self.group0()[0])),
                ((self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
                (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
            ]),
        );
        let subtraction = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Scalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
