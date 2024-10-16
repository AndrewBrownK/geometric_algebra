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
//  Average:         8       9       0
//  Maximum:        44      44       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       7       0
//  Average:        12      15       0
//  Maximum:        67      74       0
impl AntiConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = DualNum::from_groups(
            // scalar, e1234
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
        let subtraction = Scalar::from_groups(/* scalar */ geometric_anti_product.group0()[0]);
        return subtraction;
    }
}
impl AntiConstraintViolation for Flector {
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
        let anti_reverse = Flector::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(-1.0)), /* e423, e431, e412, e321 */ self.group1());
        let geometric_anti_product = DualNum::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                (-(anti_reverse.group0()[0] * self.group1()[0]) - (anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
                0.0,
            ]) - (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                + (Simd32x2::from(anti_reverse.group1()[0]) * Simd32x2::from([self.group0()[0], self.group1()[0]]))
                + (Simd32x2::from(anti_reverse.group1()[1]) * Simd32x2::from([self.group0()[1], self.group1()[1]]))
                + (Simd32x2::from(anti_reverse.group1()[2]) * Simd32x2::from([self.group0()[2], self.group1()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e1234
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        let subtraction = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e1234])]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Line {
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
        let anti_reverse = Line::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = DualNum::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                (-(anti_reverse.group1()[0] * self.group0()[0]) - (anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2])),
                0.0,
            ]) - (Simd32x2::from(anti_reverse.group0()[0]) * Simd32x2::from([self.group1()[0], self.group0()[0]]))
                - (Simd32x2::from(anti_reverse.group0()[1]) * Simd32x2::from([self.group1()[1], self.group0()[1]]))
                - (Simd32x2::from(anti_reverse.group0()[2]) * Simd32x2::from([self.group1()[2], self.group0()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e1234])]),
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
    //      f32       31       28        0
    //    simd2        8        8        0
    //    simd3        0        2        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       44       44        0
    //  no simd       67       74        0
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
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
        let anti_reverse = Point::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ (anti_reverse.group0()[3] * self.group0()[3] * -1.0));
        let anti_scalar_product = AntiScalar::from_groups(/* e1234 */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = AntiScalar::from_groups(/* e1234 */ (-anti_scalar_product[e1234] + geometric_anti_product[e1234]));
        return subtraction;
    }
}
