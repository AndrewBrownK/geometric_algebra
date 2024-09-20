// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 101
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        14      17       0
//  Average:        30      37       0
//  Maximum:       548     574       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        22      29       0
//  Average:        49      59       0
//  Maximum:      1016    1060       0
impl AntiConstraintViolation for AntiCircleOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       22       33        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[1]) - (anti_reverse.group0()[1] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[0])),
                ((anti_reverse.group1()[0] * self.group1()[0]) + (anti_reverse.group1()[1] * self.group1()[1]) + (anti_reverse.group1()[2] * self.group1()[2])),
            ]),
            // e415, e425, e435, e4
            Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group1()[2]) - (anti_reverse.group1()[2] * self.group1()[1])),
                (-(anti_reverse.group1()[0] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group1()[0])),
                ((anti_reverse.group1()[0] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group1()[0])),
                ((anti_reverse.group0()[0] * self.group1()[0])
                    + (anti_reverse.group0()[1] * self.group1()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)));
        let subtraction = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e4
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       70        0
    //    simd3        0        1        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       68       87        0
    //  no simd      113      137        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1]) - (self.group0()[0] * anti_reverse.group2()[3]) - (self.group0()[1] * anti_reverse.group1()[2])),
                (-(anti_reverse.group0()[1] * self.group1()[3]) - (self.group0()[1] * anti_reverse.group2()[3]) - (self.group0()[2] * anti_reverse.group1()[0])),
                (-(anti_reverse.group0()[2] * self.group1()[3]) - (self.group0()[0] * anti_reverse.group1()[1]) - (self.group0()[2] * anti_reverse.group2()[3])),
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (self.group0()[0] * anti_reverse.group2()[0])
                    + (self.group0()[1] * anti_reverse.group2()[1])
                    + (self.group0()[2] * anti_reverse.group2()[2])),
            ]) - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group2()[3]]))
                - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group1(), 3, 2, 0, 3))
                + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group1()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group1()[1]]) * swizzle!(anti_reverse.group1(), 3, 2, 0, 1))
                + (Simd32x4::from([self.group0()[2], self.group0()[1], self.group0()[2], self.group1()[2]]) * swizzle!(anti_reverse.group1(), 1, 3, 3, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group1()[0] * self.group2()[3]) - (anti_reverse.group1()[2] * self.group1()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group1()[0] * self.group1()[2]) - (anti_reverse.group1()[1] * self.group2()[3])),
                (-(anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group1()[1] * self.group1()[0]) - (anti_reverse.group1()[2] * self.group2()[3])),
                ((anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2]) - (self.group0()[2] * anti_reverse.group2()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[3]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[3]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 3))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[2] * self.group2()[1])
                    - (anti_reverse.group1()[3] * self.group2()[0])
                    - (anti_reverse.group2()[0] * self.group2()[3])
                    - (anti_reverse.group2()[2] * self.group1()[1])
                    - (anti_reverse.group2()[3] * self.group2()[0])),
                (-(anti_reverse.group1()[0] * self.group2()[2])
                    - (anti_reverse.group1()[3] * self.group2()[1])
                    - (anti_reverse.group2()[0] * self.group1()[2])
                    - (anti_reverse.group2()[1] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[1])),
                (-(anti_reverse.group1()[1] * self.group2()[0])
                    - (anti_reverse.group1()[3] * self.group2()[2])
                    - (anti_reverse.group2()[1] * self.group1()[0])
                    - (anti_reverse.group2()[2] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[2])),
                ((anti_reverse.group1()[1] * self.group2()[1]) + (anti_reverse.group1()[2] * self.group2()[2]) + (anti_reverse.group2()[2] * self.group1()[2])),
            ]) + (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group2(), 0, 1, 0, 0) * swizzle!(self.group1(), 3, 3, 1, 0))
                + (swizzle!(anti_reverse.group2(), 1, 2, 2, 1) * swizzle!(self.group1(), 2, 0, 3, 1))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[1]) + (self.group0()[1] * anti_reverse.group2()[2])
                    - (self.group0()[2] * anti_reverse.group2()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[0]) - (self.group0()[0] * anti_reverse.group2()[2])
                    + (self.group0()[2] * anti_reverse.group2()[0])),
                ((anti_reverse.group0()[0] * self.group2()[1]) - (anti_reverse.group0()[1] * self.group2()[0]) + (self.group0()[0] * anti_reverse.group2()[1])
                    - (self.group0()[1] * anti_reverse.group2()[0])),
                ((anti_reverse.group0()[2] * self.group1()[2])
                    + (self.group0()[0] * anti_reverse.group1()[0])
                    + (self.group0()[1] * anti_reverse.group1()[1])
                    + (self.group0()[2] * anti_reverse.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 3, 3, 3, 0))
                + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group0()[1]]) * swizzle!(self.group1(), 0, 1, 2, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2) - f32::powi(self.group2()[3], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       85        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       73       93        0
    //  no simd       91      115        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[1])
                    - (self.group0()[0] * anti_reverse.group2()[3])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[0])
                    - (self.group0()[1] * anti_reverse.group2()[3])),
                ((anti_reverse.group0()[0] * self.group1()[1]) - (anti_reverse.group0()[1] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[0])
                    - (self.group0()[2] * anti_reverse.group2()[3])),
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    + (self.group0()[0] * anti_reverse.group2()[0])
                    + (self.group0()[1] * anti_reverse.group2()[1])
                    + (self.group0()[2] * anti_reverse.group2()[2])),
            ]) - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group2()[3]]))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group1()[0] * self.group2()[3]) + (anti_reverse.group1()[1] * self.group1()[2])
                    - (anti_reverse.group1()[2] * self.group1()[1])
                    + (self.group0()[2] * anti_reverse.group2()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group1()[0] * self.group1()[2]) - (anti_reverse.group1()[1] * self.group2()[3])
                    + (anti_reverse.group1()[2] * self.group1()[0])
                    + (self.group0()[0] * anti_reverse.group2()[2])),
                (-(anti_reverse.group0()[1] * self.group2()[0]) + (anti_reverse.group1()[0] * self.group1()[1])
                    - (anti_reverse.group1()[1] * self.group1()[0])
                    - (anti_reverse.group1()[2] * self.group2()[3])
                    + (self.group0()[1] * anti_reverse.group2()[0])),
                ((anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2]) - (self.group0()[2] * anti_reverse.group2()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 1))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[2] * self.group2()[1])
                    - (self.group1()[1] * anti_reverse.group2()[2])
                    - (anti_reverse.group2()[0] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[0])),
                (-(anti_reverse.group1()[0] * self.group2()[2])
                    - (self.group1()[2] * anti_reverse.group2()[0])
                    - (anti_reverse.group2()[1] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[1])),
                (-(anti_reverse.group1()[1] * self.group2()[0])
                    - (self.group1()[0] * anti_reverse.group2()[1])
                    - (anti_reverse.group2()[2] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[2])),
                ((anti_reverse.group1()[1] * self.group2()[1])
                    + (anti_reverse.group1()[2] * self.group2()[2])
                    + (self.group1()[1] * anti_reverse.group2()[1])
                    + (self.group1()[2] * anti_reverse.group2()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 0))),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[1]) + (self.group0()[1] * anti_reverse.group2()[2])
                    - (self.group0()[2] * anti_reverse.group2()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[0]) - (self.group0()[0] * anti_reverse.group2()[2])
                    + (self.group0()[2] * anti_reverse.group2()[0])),
                ((anti_reverse.group0()[0] * self.group2()[1]) - (anti_reverse.group0()[1] * self.group2()[0]) + (self.group0()[0] * anti_reverse.group2()[1])
                    - (self.group0()[1] * anti_reverse.group2()[0])),
                ((anti_reverse.group0()[0] * self.group1()[0])
                    + (anti_reverse.group0()[1] * self.group1()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group2()[3], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       31        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       35        0
    //  no simd       36       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[1]) - (self.group0()[0] * anti_reverse.group1()[3])),
                (-(anti_reverse.group0()[0] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[0]) - (self.group0()[1] * anti_reverse.group1()[3])),
                ((anti_reverse.group0()[0] * self.group0()[1]) - (anti_reverse.group0()[1] * self.group0()[0]) - (self.group0()[2] * anti_reverse.group1()[3])),
                ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[3]]))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1])
                    - (self.group0()[1] * anti_reverse.group1()[2])
                    - (anti_reverse.group1()[0] * self.group1()[3])
                    - (anti_reverse.group1()[3] * self.group1()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[2])
                    - (self.group0()[2] * anti_reverse.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[3])
                    - (anti_reverse.group1()[3] * self.group1()[1])),
                (-(anti_reverse.group0()[1] * self.group1()[0])
                    - (self.group0()[0] * anti_reverse.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[3])
                    - (anti_reverse.group1()[3] * self.group1()[2])),
                ((anti_reverse.group0()[1] * self.group1()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (self.group0()[1] * anti_reverse.group1()[1])
                    + (self.group0()[2] * anti_reverse.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       34        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       51       62        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group0()[2]) - (anti_reverse.group0()[3] * self.group0()[3]) - (anti_reverse.group1()[3] * self.group1()[3])),
                0.0,
                0.0,
                0.0,
            ]) + (swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * swizzle!(self.group0(), 0, 3, 3, 3))
                + (swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * swizzle!(self.group0(), 1, 0, 1, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group1()[3]) - (anti_reverse.group0()[2] * self.group0()[1]) - (anti_reverse.group1()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[1] * self.group1()[3]) - (anti_reverse.group1()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[1] * self.group0()[0]) - (anti_reverse.group0()[2] * self.group1()[3]) - (anti_reverse.group1()[3] * self.group0()[2])),
                (anti_reverse.group1()[3] * self.group0()[3]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[3]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 3))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1])
                    - (anti_reverse.group0()[3] * self.group1()[0])
                    - (anti_reverse.group1()[0] * self.group1()[3])
                    - (anti_reverse.group1()[2] * self.group0()[1])
                    - (anti_reverse.group1()[3] * self.group1()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[2])
                    - (anti_reverse.group0()[3] * self.group1()[1])
                    - (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[1] * self.group1()[3])
                    - (anti_reverse.group1()[3] * self.group1()[1])),
                (-(anti_reverse.group0()[1] * self.group1()[0])
                    - (anti_reverse.group0()[3] * self.group1()[2])
                    - (anti_reverse.group1()[1] * self.group0()[0])
                    - (anti_reverse.group1()[2] * self.group1()[3])
                    - (anti_reverse.group1()[3] * self.group1()[2])),
                ((anti_reverse.group0()[1] * self.group1()[1]) + (anti_reverse.group0()[2] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group0()[2])),
            ]) + (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group1(), 0, 1, 0, 0) * swizzle!(self.group0(), 3, 3, 1, 0))
                + (swizzle!(anti_reverse.group1(), 1, 2, 2, 1) * swizzle!(self.group0(), 2, 0, 3, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       39        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       36       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group0()[2]) - (self.group1()[1] * anti_reverse.group0()[2]) + (self.group1()[2] * anti_reverse.group0()[1])
                    - (anti_reverse.group0()[0] * self.group0()[3])
                    - (anti_reverse.group0()[3] * self.group0()[0])),
                ((anti_reverse.group1()[2] * self.group0()[0]) + (self.group1()[0] * anti_reverse.group0()[2])
                    - (self.group1()[2] * anti_reverse.group0()[0])
                    - (anti_reverse.group0()[1] * self.group0()[3])
                    - (anti_reverse.group0()[3] * self.group0()[1])),
                ((anti_reverse.group1()[0] * self.group0()[1]) - (self.group1()[0] * anti_reverse.group0()[1]) + (self.group1()[1] * anti_reverse.group0()[0])
                    - (anti_reverse.group0()[2] * self.group0()[3])
                    - (anti_reverse.group0()[3] * self.group0()[2])),
                ((anti_reverse.group1()[0] * self.group1()[0]) + (anti_reverse.group1()[1] * self.group1()[1]) + (anti_reverse.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group0()[3]]) * swizzle!(self.group0(), 1, 2, 0, 3))),
            // e415, e425, e435, e4
            Simd32x4::from([
                (-(anti_reverse.group1()[0] * self.group0()[3]) + (anti_reverse.group1()[1] * self.group1()[2])
                    - (anti_reverse.group1()[2] * self.group1()[1])
                    - (self.group1()[0] * anti_reverse.group0()[3])),
                (-(anti_reverse.group1()[0] * self.group1()[2]) - (anti_reverse.group1()[1] * self.group0()[3]) + (anti_reverse.group1()[2] * self.group1()[0])
                    - (self.group1()[1] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[0] * self.group1()[1])
                    - (anti_reverse.group1()[1] * self.group1()[0])
                    - (anti_reverse.group1()[2] * self.group0()[3])
                    - (self.group1()[2] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (self.group1()[0] * anti_reverse.group0()[0])
                    + (self.group1()[1] * anti_reverse.group0()[1])
                    + (self.group1()[2] * anti_reverse.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e4
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      104        0
    //    simd3        0        1        0
    //    simd4       33       34        0
    // Totals...
    // yes simd      121      139        0
    //  no simd      220      243        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((self.group0()[0] * anti_reverse.group1()[3])
                    + (self.group0()[1] * anti_reverse.group1()[2])
                    + (anti_reverse.group1()[0] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group1()[0])
                    + (anti_reverse.group3()[0] * self.group2()[3])),
                ((self.group0()[1] * anti_reverse.group1()[3])
                    + (self.group0()[2] * anti_reverse.group1()[0])
                    + (anti_reverse.group1()[1] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group1()[1])
                    + (anti_reverse.group3()[1] * self.group2()[3])),
                ((self.group0()[0] * anti_reverse.group1()[1])
                    + (self.group0()[2] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[2] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group1()[2])
                    + (anti_reverse.group3()[2] * self.group2()[3])),
                (-(self.group0()[1] * anti_reverse.group2()[1])
                    - (self.group0()[2] * anti_reverse.group2()[2])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group3()[1] * self.group3()[1])
                    - (anti_reverse.group3()[2] * self.group3()[2])),
            ]) - (Simd32x4::from(anti_reverse.group0()[0]) * Simd32x4::from([self.group1()[3], self.group3()[2], self.group1()[1], self.group2()[0]]))
                - (Simd32x4::from(anti_reverse.group0()[1]) * Simd32x4::from([self.group1()[2], self.group1()[3], self.group3()[0], self.group2()[1]]))
                - (Simd32x4::from(anti_reverse.group0()[2]) * Simd32x4::from([self.group3()[1], self.group1()[0], self.group1()[3], self.group2()[2]]))
                + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group2()[3]]) * swizzle!(self.group3(), 2, 0, 1, 3))
                + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group1(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group2()[3]]) * swizzle!(anti_reverse.group3(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[0]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group2()[1]) + (self.group0()[0] * anti_reverse.group3()[3]) - (self.group0()[2] * anti_reverse.group2()[1])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    + (anti_reverse.group1()[3] * self.group3()[0])
                    + (anti_reverse.group3()[0] * self.group1()[3])
                    + (anti_reverse.group3()[2] * self.group3()[1])),
                ((anti_reverse.group0()[0] * self.group2()[2]) - (self.group0()[0] * anti_reverse.group2()[2])
                    + (self.group0()[1] * anti_reverse.group3()[3])
                    + (anti_reverse.group1()[0] * self.group1()[2])
                    + (anti_reverse.group1()[3] * self.group3()[1])
                    + (anti_reverse.group3()[0] * self.group3()[2])
                    + (anti_reverse.group3()[1] * self.group1()[3])),
                ((anti_reverse.group0()[1] * self.group2()[0]) - (self.group0()[1] * anti_reverse.group2()[0])
                    + (self.group0()[2] * anti_reverse.group3()[3])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[3] * self.group3()[2])
                    + (anti_reverse.group3()[1] * self.group3()[0])
                    + (anti_reverse.group3()[2] * self.group1()[3])),
                (-(anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group1()[2] * self.group3()[2])
                    - (anti_reverse.group3()[0] * self.group1()[0])
                    - (anti_reverse.group3()[1] * self.group1()[1])
                    - (anti_reverse.group3()[2] * self.group1()[2])
                    - (anti_reverse.group3()[3] * self.group2()[3])),
            ]) + (Simd32x4::from(self.group3()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group2()[3]]))
                - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group1()[1]]) * swizzle!(self.group3(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group3()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[2]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 2))
                + (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 0, 1, 2, 1))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group3()[3])
                    + (anti_reverse.group2()[2] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group3()[1])
                    + (anti_reverse.group3()[3] * self.group3()[0])),
                ((anti_reverse.group1()[1] * self.group3()[3])
                    + (anti_reverse.group2()[0] * self.group3()[2])
                    + (anti_reverse.group2()[1] * self.group1()[3])
                    + (anti_reverse.group3()[3] * self.group3()[1])),
                ((anti_reverse.group1()[2] * self.group3()[3])
                    + (anti_reverse.group2()[1] * self.group3()[0])
                    + (anti_reverse.group2()[2] * self.group1()[3])
                    + (anti_reverse.group3()[3] * self.group3()[2])),
                (-(anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group3()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group3()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group3()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group3()[3]]) * swizzle!(self.group1(), 3, 2, 0, 3))
                - (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group1()[3]]) * swizzle!(self.group3(), 2, 0, 1, 3))
                - (Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group2()[0]]) * swizzle!(self.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group1()[2]]) * swizzle!(self.group2(), 1, 2, 0, 2))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[2]]) * swizzle!(anti_reverse.group3(), 3, 3, 3, 2))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (swizzle!(anti_reverse.group1(), 3, 3, 3, 1) * swizzle!(self.group2(), 0, 1, 2, 1))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group3(), 1, 2, 0, 1) * swizzle!(self.group2(), 2, 0, 1, 1))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group2()[2])
                    + (self.group0()[2] * anti_reverse.group2()[1])
                    + (anti_reverse.group1()[0] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[0])
                    - (anti_reverse.group2()[0] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[0])
                    + (anti_reverse.group3()[2] * self.group1()[1])),
                (-(anti_reverse.group0()[2] * self.group2()[0])
                    + (self.group0()[0] * anti_reverse.group2()[2])
                    + (anti_reverse.group1()[1] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[1])
                    + (anti_reverse.group3()[0] * self.group1()[2])),
                (-(anti_reverse.group0()[0] * self.group2()[1])
                    + (self.group0()[1] * anti_reverse.group2()[0])
                    + (anti_reverse.group1()[2] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[2])
                    + (anti_reverse.group3()[1] * self.group1()[0])),
                (-(anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group3()[2])
                    - (self.group0()[1] * anti_reverse.group1()[1])
                    - (self.group0()[1] * anti_reverse.group3()[1])
                    - (self.group0()[2] * anti_reverse.group1()[2])
                    - (self.group0()[2] * anti_reverse.group3()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0]]) * swizzle!(self.group3(), 3, 3, 3, 0))
                + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group2(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(anti_reverse.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[3]]) * swizzle!(anti_reverse.group2(), 2, 0, 1, 3))
                + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group0()[1]]) * swizzle!(self.group3(), 1, 2, 0, 1))
                - (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group3()[0], 2)
                - f32::powi(self.group3()[1], 2)
                - f32::powi(self.group3()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       45        0
    //    simd3        0        1        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       55       62        0
    //  no simd      100      112        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
            // e1, e2, e3, e5
            self.group2(),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group0()[2])
                    - (anti_reverse.group2()[0] * self.group2()[0])
                    - (anti_reverse.group2()[1] * self.group2()[1])
                    - (anti_reverse.group2()[2] * self.group2()[2])),
                ((anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group0()[3] * self.group0()[0]) + (anti_reverse.group2()[2] * self.group0()[1])),
                ((anti_reverse.group0()[1] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[1]) + (anti_reverse.group2()[0] * self.group0()[2])),
                ((anti_reverse.group0()[2] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[2]) + (anti_reverse.group2()[1] * self.group0()[0])),
            ]) - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0]]) * swizzle!(self.group0(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group0()[0], self.group2()[2], self.group2()[0], self.group2()[1]]) * swizzle!(anti_reverse.group0(), 0, 1, 2, 0))
                + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group2()[2], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 3, 0, 0, 1))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group0()[1])
                    + (anti_reverse.group0()[3] * self.group2()[0])
                    + (anti_reverse.group2()[0] * self.group0()[3])
                    + (anti_reverse.group2()[2] * self.group2()[1])),
                ((anti_reverse.group0()[0] * self.group0()[2])
                    + (anti_reverse.group0()[3] * self.group2()[1])
                    + (anti_reverse.group2()[0] * self.group2()[2])
                    + (anti_reverse.group2()[1] * self.group0()[3])),
                ((anti_reverse.group0()[1] * self.group0()[0])
                    + (anti_reverse.group0()[3] * self.group2()[2])
                    + (anti_reverse.group2()[1] * self.group2()[0])
                    + (anti_reverse.group2()[2] * self.group0()[3])),
                (-(anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group0()[1]]) * swizzle!(self.group2(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[2] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group2()[1])
                    + (self.group1()[1] * anti_reverse.group0()[2])
                    + (anti_reverse.group0()[0] * self.group2()[3])),
                ((anti_reverse.group1()[0] * self.group2()[2])
                    + (anti_reverse.group1()[1] * self.group0()[3])
                    + (self.group1()[2] * anti_reverse.group0()[0])
                    + (anti_reverse.group0()[1] * self.group2()[3])),
                ((anti_reverse.group1()[1] * self.group2()[0])
                    + (anti_reverse.group1()[2] * self.group0()[3])
                    + (self.group1()[0] * anti_reverse.group0()[1])
                    + (anti_reverse.group0()[2] * self.group2()[3])),
                (-(anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group0()[3] * self.group2()[3])),
            ]) - (Simd32x4::from(self.group1()[0]) * Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[2], anti_reverse.group2()[1], anti_reverse.group0()[0]]))
                - (Simd32x4::from(self.group1()[1]) * Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group0()[3], anti_reverse.group0()[0], anti_reverse.group0()[1]]))
                - (Simd32x4::from(self.group1()[2]) * Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group2()[0], anti_reverse.group0()[3], anti_reverse.group0()[2]]))
                + (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group2()[3]]) * swizzle!(self.group0(), 3, 2, 0, 3))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group1()[1]]) * swizzle!(self.group2(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[2]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 2))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)
                - f32::powi(self.group2()[0], 2)
                - f32::powi(self.group2()[1], 2)
                - f32::powi(self.group2()[2], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       20        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       24       28        0
    //  no simd       45       52        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse =
            AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e4, e1, e2, e3 */ self.group1());
        let geometric_anti_product = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[1] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group1()[0])),
                (-(anti_reverse.group0()[2] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[0])
                    + (anti_reverse.group1()[3] * self.group1()[0])),
                0.0,
            ]) - (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[1]]) * swizzle!(self.group1(), 2, 3, 1, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[3]]) * swizzle!(anti_reverse.group1(), 2, 3, 1, 3))
                + (Simd32x4::from([self.group1()[3], self.group1()[1], self.group1()[2], self.group0()[3]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 3))
                - (swizzle!(anti_reverse.group1(), 0, 0, 0, 2) * swizzle!(self.group1(), 1, 2, 3, 2))),
            // e415, e425, e435, e4
            (Simd32x4::from([
                (anti_reverse.group1()[1] * self.group0()[3]),
                (anti_reverse.group1()[2] * self.group0()[3]),
                (anti_reverse.group1()[3] * self.group0()[3]),
                ((anti_reverse.group0()[2] * self.group1()[3]) + (anti_reverse.group0()[3] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group0()[0])
                    - (anti_reverse.group1()[2] * self.group0()[1])
                    - (anti_reverse.group1()[3] * self.group0()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[1]]) * swizzle!(self.group1(), 2, 3, 1, 2))
                - (Simd32x4::from([self.group1()[3], self.group1()[1], self.group1()[2], self.group0()[3]]) * swizzle!(anti_reverse.group1(), 2, 3, 1, 0))
                + (swizzle!(anti_reverse.group0(), 3, 3, 3, 0) * swizzle!(self.group1(), 1, 2, 3, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e4
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       69       91        0
    //    simd3        0        1        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       78      101        0
    //  no simd      105      130        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((self.group1()[0] * anti_reverse.group2()[3]) + (self.group1()[1] * anti_reverse.group0()[2])),
                ((self.group1()[1] * anti_reverse.group2()[3]) + (self.group1()[2] * anti_reverse.group0()[0])),
                ((self.group1()[0] * anti_reverse.group0()[1]) + (self.group1()[2] * anti_reverse.group2()[3])),
                (-(anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[3]]))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group2()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group2()[3]]) * swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(anti_reverse.group1()[1] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group1()[1]) + (anti_reverse.group0()[0] * self.group0()[3])
                    - (anti_reverse.group0()[1] * self.group2()[2])
                    + (anti_reverse.group0()[2] * self.group2()[1])
                    + (anti_reverse.group0()[3] * self.group0()[0])
                    + (anti_reverse.group2()[0] * self.group2()[3])
                    - (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[1])
                    + (anti_reverse.group2()[3] * self.group2()[0])),
                ((anti_reverse.group1()[0] * self.group1()[2]) - (anti_reverse.group1()[2] * self.group1()[0])
                    + (anti_reverse.group0()[0] * self.group2()[2])
                    + (anti_reverse.group0()[1] * self.group0()[3])
                    - (anti_reverse.group0()[2] * self.group2()[0])
                    + (anti_reverse.group0()[3] * self.group0()[1])
                    + (anti_reverse.group2()[0] * self.group0()[2])
                    + (anti_reverse.group2()[1] * self.group2()[3])
                    - (anti_reverse.group2()[2] * self.group0()[0])
                    + (anti_reverse.group2()[3] * self.group2()[1])),
                (-(anti_reverse.group1()[0] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group1()[0]) - (anti_reverse.group0()[0] * self.group2()[1])
                    + (anti_reverse.group0()[1] * self.group2()[0])
                    + (anti_reverse.group0()[2] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[2])
                    - (anti_reverse.group2()[0] * self.group0()[1])
                    + (anti_reverse.group2()[1] * self.group0()[0])
                    + (anti_reverse.group2()[2] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[2])),
                0.0,
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group0()[3])
                    + (anti_reverse.group1()[2] * self.group2()[1])
                    + (self.group1()[0] * anti_reverse.group0()[3])
                    + (self.group1()[1] * anti_reverse.group2()[2])),
                ((anti_reverse.group1()[0] * self.group2()[2])
                    + (anti_reverse.group1()[1] * self.group0()[3])
                    + (self.group1()[1] * anti_reverse.group0()[3])
                    + (self.group1()[2] * anti_reverse.group2()[0])),
                ((anti_reverse.group1()[1] * self.group2()[0])
                    + (anti_reverse.group1()[2] * self.group0()[3])
                    + (self.group1()[0] * anti_reverse.group2()[1])
                    + (self.group1()[2] * anti_reverse.group0()[3])),
                (-(anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (self.group1()[1] * anti_reverse.group2()[1])
                    - (self.group1()[2] * anti_reverse.group2()[2])),
            ]) - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 0))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((anti_reverse.group0()[0] * self.group0()[3]) + (anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group2()[0] * self.group2()[3])
                    + (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[3] * self.group2()[0])),
                ((anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group0()[1] * self.group0()[3]) - (anti_reverse.group2()[1] * self.group2()[3])
                    + (anti_reverse.group2()[2] * self.group0()[0])
                    + (anti_reverse.group2()[3] * self.group2()[1])),
                ((anti_reverse.group0()[1] * self.group2()[0]) + (anti_reverse.group0()[2] * self.group0()[3]) + (anti_reverse.group2()[0] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[2])),
                (-(anti_reverse.group1()[2] * self.group0()[2]) - (self.group1()[1] * anti_reverse.group0()[1]) - (self.group1()[2] * anti_reverse.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group0()[3] * self.group2()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([geometric_anti_product.group1()[0], geometric_anti_product.group1()[1], geometric_anti_product.group1()[2], 0.0]),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiDipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       11        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[1] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[2] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[2])),
                (anti_reverse.group0()[3] * self.group0()[3]),
            ]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self.group0()[3], 2));
        let subtraction = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiFlatOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * -1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse[e321] * self[e321]));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiFlatPoint {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       11        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([
                ((anti_reverse.group0()[0] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[0])),
                ((anti_reverse.group0()[1] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[1])),
                ((anti_reverse.group0()[2] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[2])),
                (anti_reverse.group0()[3] * self.group0()[3]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self.group0()[3], 2));
        let subtraction = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([
                geometric_anti_product.group1()[0],
                geometric_anti_product.group1()[1],
                geometric_anti_product.group1()[2],
                (geometric_anti_product.group1()[3] - anti_scalar_product[e12345]),
            ]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiFlector {
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
        let anti_reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3, e5 */ self.group1());
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group0()[3]) + (anti_reverse.group1()[2] * self.group1()[1])),
                ((anti_reverse.group1()[0] * self.group1()[2]) + (anti_reverse.group1()[1] * self.group0()[3])),
                ((anti_reverse.group1()[1] * self.group1()[0]) + (anti_reverse.group1()[2] * self.group0()[3])),
                (-(anti_reverse.group1()[1] * self.group1()[1]) - (anti_reverse.group1()[2] * self.group1()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e235, e315, e125, e5
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
            // e12345
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiFlectorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        5        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            ((anti_reverse.group0()[0] * self.group0()[0])
                - (anti_reverse.group0()[1] * self.group0()[1])
                - (anti_reverse.group0()[2] * self.group0()[2])
                - (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiLine {
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
        let anti_reverse = AntiLine::from_groups(
            // e23, e31, e12
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group0()[1]) - (anti_reverse.group0()[1] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[1]) - (anti_reverse.group0()[1] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[0])
                    + (anti_reverse.group0()[1] * self.group1()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiLineOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       10        0
    //  no simd        8       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_anti_product = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            ((anti_reverse.group0()[1] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[1])),
            (-(anti_reverse.group0()[0] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[0])),
            ((anti_reverse.group0()[0] * self.group0()[1]) - (anti_reverse.group0()[1] * self.group0()[0])),
            ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])),
        ]));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let subtraction = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
            (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = VersorRoundPointAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        4        4        0
    // Totals...
    // yes simd       11       14        0
    //  no simd       15       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = VersorRoundPointAligningOriginAtInfinity::from_groups(
            // e5, e12345
            (Simd32x2::from([
                ((anti_reverse.group1()[0] * self.group0()[0]) + (anti_reverse.group1()[1] * self.group0()[1]) + (anti_reverse.group1()[2] * self.group0()[2])
                    - (anti_reverse.group1()[3] * self.group0()[3])),
                0.0,
            ]) + (Simd32x2::from(anti_reverse.group0()[0]) * Simd32x2::from([self.group1()[0], self.group0()[0]]))
                + (Simd32x2::from(anti_reverse.group0()[1]) * Simd32x2::from([self.group1()[1], self.group0()[1]]))
                + (Simd32x2::from(anti_reverse.group0()[2]) * Simd32x2::from([self.group1()[2], self.group0()[2]]))
                - (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorRoundPointAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            geometric_anti_product.group0()[0],
            (geometric_anti_product.group0()[1] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiMotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])
                - (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiMysteryCircleRotor {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       16       17        0
    //  no simd       25       29        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)), /* scalar */ self[e31]);
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group0()[2]) - (anti_reverse.group0()[3] * self.group0()[3]) - (anti_reverse[e31] * self[e31])),
                0.0,
                0.0,
                0.0,
            ]) + (swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * swizzle!(self.group0(), 0, 3, 3, 3))
                + (swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * swizzle!(self.group0(), 1, 0, 1, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self[e31]) - (anti_reverse.group0()[2] * self.group0()[1]) - (self.group0()[0] * anti_reverse[e31])),
                (-(anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[1] * self[e31]) - (self.group0()[1] * anti_reverse[e31])),
                (-(anti_reverse.group0()[1] * self.group0()[0]) - (anti_reverse.group0()[2] * self[e31]) - (self.group0()[2] * anti_reverse[e31])),
                (self.group0()[3] * anti_reverse[e31]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e31]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 3))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2) - f32::powi(self[e31], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiMysteryDipoleInversion {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       33        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       36       38        0
    //  no simd       48       53        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3 */ self.group1());
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group0()[2] * self.group0()[2])),
                ((self.group1()[1] * anti_reverse.group0()[2]) + (anti_reverse.group0()[0] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[0])),
                ((self.group1()[2] * anti_reverse.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[1])),
                ((self.group1()[0] * anti_reverse.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 0, 2, 0, 1))
                + (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 3, 1, 2, 0))
                - (Simd32x4::from([self.group0()[1], self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 1, 1, 2, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group0()[3]) - (anti_reverse.group1()[1] * self.group1()[2])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    + (self.group1()[0] * anti_reverse.group0()[3])
                    + (anti_reverse.group0()[2] * self.group0()[1])),
                ((anti_reverse.group1()[0] * self.group1()[2]) + (anti_reverse.group1()[1] * self.group0()[3]) - (anti_reverse.group1()[2] * self.group1()[0])
                    + (self.group1()[1] * anti_reverse.group0()[3])
                    + (anti_reverse.group0()[0] * self.group0()[2])),
                (-(anti_reverse.group1()[0] * self.group1()[1])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[2] * self.group0()[3])
                    + (self.group1()[2] * anti_reverse.group0()[3])
                    + (anti_reverse.group0()[1] * self.group0()[0])),
                (-(anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[0] * anti_reverse.group0()[0])
                    - (self.group1()[1] * anti_reverse.group0()[1])
                    - (self.group1()[2] * anti_reverse.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)
                - f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiMysteryQuadNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMysteryQuadNum::from_groups(/* e45, scalar */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1])));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)));
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiPlane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiQuadNum {
    type Output = VersorRoundPointAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       10       13        0
    //  no simd       12       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_anti_product = VersorRoundPointAligningOrigin::from_groups(
            // e4, e5, e12345
            (Simd32x3::from([
                (-(anti_reverse.group0()[0] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[0]) - (anti_reverse.group0()[3] * self.group0()[0])),
                ((anti_reverse.group0()[1] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[1]) - (anti_reverse.group0()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) - (anti_reverse.group0()[1] * self.group0()[0]) - (anti_reverse.group0()[2] * self.group0()[2])),
            ]) - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[3]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2) - 2.0 * (self.group0()[0] * self.group0()[1])),
        );
        let subtraction = VersorRoundPointAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            (geometric_anti_product.group0()[2] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiQuadNumAtInfinity {
    type Output = VersorRoundPointAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd2        2        2        0
    // Totals...
    // yes simd        5        5        0
    //  no simd        7        7        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiQuadNumAtInfinity::from_groups(/* e3215, e45, scalar */ Simd32x3::from([self.group0()[0], (self.group0()[1] * -1.0), self.group0()[2]]));
        let geometric_anti_product = VersorRoundPointAligningOriginAtInfinity::from_groups(
            // e5, e12345
            (Simd32x2::from([((anti_reverse.group0()[0] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[0])), 0.0])
                - (Simd32x2::from(anti_reverse.group0()[1]) * Simd32x2::from([self.group0()[0], self.group0()[1]]))
                - (Simd32x2::from(self.group0()[2]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group0()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = VersorRoundPointAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            geometric_anti_product.group0()[0],
            (geometric_anti_product.group0()[1] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiQuadNumOrthogonalOrigin {
    type Output = VersorRoundPointAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        1        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       10        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiQuadNumOrthogonalOrigin::from_groups(/* e1234, e3215, e45 */ Simd32x3::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0)]));
        let geometric_anti_product = VersorRoundPointAligningOrigin::from_groups(
            // e4, e5, e12345
            (Simd32x3::from([
                (anti_reverse.group0()[2] * self.group0()[0]),
                (anti_reverse.group0()[1] * self.group0()[2]),
                (-(anti_reverse.group0()[1] * self.group0()[0]) - (anti_reverse.group0()[2] * self.group0()[2])),
            ]) - (swizzle!(anti_reverse.group0(), 0, 2, 0) * swizzle!(self.group0(), 2, 1, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[2], 2) - 2.0 * (self.group0()[0] * self.group0()[1])));
        let subtraction = VersorRoundPointAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            (geometric_anti_product.group0()[2] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiSphereOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = VersorRoundPointOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        4        4        0
    // Totals...
    // yes simd       11       14        0
    //  no simd       15       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = VersorRoundPointOnOrigin::from_groups(
            // e4, e12345
            (Simd32x2::from([
                ((anti_reverse.group1()[0] * self.group0()[0]) + (anti_reverse.group1()[1] * self.group0()[1]) + (anti_reverse.group1()[2] * self.group0()[2])
                    - (anti_reverse.group1()[3] * self.group0()[3])),
                0.0,
            ]) - (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([anti_reverse.group0()[1], anti_reverse.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([anti_reverse.group0()[2], anti_reverse.group1()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        let subtraction =
            VersorRoundPointOnOrigin::from_groups(
                // e4, e12345
                Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
            );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiVersorRoundPointAligningOriginAtInfinity {
    type Output = VersorRoundPointAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        1        4        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = VersorRoundPointAligningOriginAtInfinity::from_groups(
            // e5, e12345
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([-2.0, -1.0])),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[1], 2) * -1.0));
        let subtraction = VersorRoundPointAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            geometric_anti_product.group0()[0],
            (geometric_anti_product.group0()[1] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiVersorRoundPointOnOrigin {
    type Output = VersorRoundPointOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        1        4        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = VersorRoundPointOnOrigin::from_groups(
            // e4, e12345
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([-2.0, -1.0])),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[1], 2) * -1.0));
        let subtraction =
            VersorRoundPointOnOrigin::from_groups(
                // e4, e12345
                Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
            );
        return subtraction;
    }
}
impl AntiConstraintViolation for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       63       78        0
    //    simd3        0        2        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       70       88        0
    //  no simd       91      116        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Circle::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((self.group0()[0] * anti_reverse.group1()[3]) + (self.group0()[1] * anti_reverse.group1()[2])),
                ((self.group0()[1] * anti_reverse.group1()[3]) + (self.group0()[2] * anti_reverse.group1()[0])),
                ((self.group0()[0] * anti_reverse.group1()[1]) + (self.group0()[2] * anti_reverse.group1()[3])),
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[0], anti_reverse.group1()[0]]) * swizzle!(self.group1(), 3, 3, 1, 0))
                - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[2], anti_reverse.group1()[1]]) * swizzle!(self.group1(), 2, 0, 3, 1))
                + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group1(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 2))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group1()[2])
                    + (anti_reverse.group1()[2] * self.group1()[1])),
                ((anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[0])
                    + (anti_reverse.group1()[0] * self.group1()[2])
                    - (anti_reverse.group1()[2] * self.group1()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[1]) + (anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group0()[1])
                    + (anti_reverse.group2()[1] * self.group0()[0])
                    - (anti_reverse.group1()[0] * self.group1()[1])
                    + (anti_reverse.group1()[1] * self.group1()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[0]) - (anti_reverse.group0()[1] * self.group2()[1]) - (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group2()[0] * self.group1()[3]) + (anti_reverse.group2()[2] * self.group1()[1]) + (self.group2()[1] * anti_reverse.group1()[2])),
                ((anti_reverse.group2()[0] * self.group1()[2]) + (anti_reverse.group2()[1] * self.group1()[3]) + (self.group2()[2] * anti_reverse.group1()[0])),
                ((anti_reverse.group2()[1] * self.group1()[0]) + (anti_reverse.group2()[2] * self.group1()[3]) + (self.group2()[0] * anti_reverse.group1()[1])),
                (-(anti_reverse.group2()[1] * self.group1()[1]) - (anti_reverse.group2()[2] * self.group1()[2]) - (self.group2()[2] * anti_reverse.group1()[2])),
            ]) - (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group2()[0], self.group2()[0], self.group2()[1], self.group2()[0]]) * swizzle!(anti_reverse.group1(), 3, 2, 0, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[1], self.group2()[2], self.group2()[1]]) * swizzle!(anti_reverse.group1(), 1, 3, 3, 1))),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[1])
                    + (anti_reverse.group1()[0] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[0])),
                ((anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[1]) + (anti_reverse.group0()[1] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group0()[1])
                    - (anti_reverse.group2()[1] * self.group0()[0])
                    + (anti_reverse.group1()[2] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[2])),
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * anti_reverse.group1()[0])
                    - (self.group0()[1] * anti_reverse.group1()[1])
                    - (self.group0()[2] * anti_reverse.group1()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       87        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       71       90        0
    //  no simd       71       96        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleAligningOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group1()[1] * self.group1()[2])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[0]) + (anti_reverse.group1()[0] * self.group1()[2])
                    - (anti_reverse.group1()[2] * self.group1()[0])
                    + (anti_reverse.group2()[0] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[1]) + (anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group1()[0] * self.group1()[1])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    - (anti_reverse.group2()[0] * self.group0()[1])
                    + (anti_reverse.group2()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[0]) - (anti_reverse.group0()[1] * self.group2()[1]) - (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(anti_reverse.group1()[1] * self.group2()[2]) + (anti_reverse.group1()[2] * self.group2()[1]) - (anti_reverse.group2()[1] * self.group1()[2])
                    + (anti_reverse.group2()[2] * self.group1()[1])),
                ((anti_reverse.group1()[0] * self.group2()[2]) - (anti_reverse.group1()[2] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group1()[0])),
                (-(anti_reverse.group1()[0] * self.group2()[1]) + (anti_reverse.group1()[1] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group1()[1])
                    + (anti_reverse.group2()[1] * self.group1()[0])),
                (-(anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[1]) + (anti_reverse.group0()[1] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group0()[1])
                    - (anti_reverse.group2()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       33       47        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
                (anti_reverse.group0()[3] * self.group0()[0]),
                (anti_reverse.group0()[3] * self.group0()[1]),
                (anti_reverse.group0()[3] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * swizzle!(anti_reverse.group0(), 3, 0, 1, 2))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[1] * self.group0()[0])),
                0.0,
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group0()[3]) + (anti_reverse.group1()[2] * self.group0()[1]) + (self.group1()[1] * anti_reverse.group0()[2])),
                ((anti_reverse.group1()[0] * self.group0()[2]) + (anti_reverse.group1()[1] * self.group0()[3]) + (self.group1()[2] * anti_reverse.group0()[0])),
                ((anti_reverse.group1()[1] * self.group0()[0]) + (anti_reverse.group1()[2] * self.group0()[3]) + (self.group1()[0] * anti_reverse.group0()[1])),
                (-(anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2]) - (self.group1()[2] * anti_reverse.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[0], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 3, 2, 0, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[1], self.group1()[2], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 1, 3, 3, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([geometric_anti_product.group1()[0], geometric_anti_product.group1()[1], geometric_anti_product.group1()[2], 0.0]),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       42        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       31       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleAtOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])),
                (-(anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[0])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[0]) - (anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-2.0 * (self.group0()[0] * self.group1()[0]) - 2.0 * (self.group0()[1] * self.group1()[1]) - 2.0 * (self.group0()[2] * self.group1()[2])),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       22       33        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[0])),
                (-(anti_reverse.group1()[0] * self.group1()[0]) - (anti_reverse.group1()[1] * self.group1()[1]) - (anti_reverse.group1()[2] * self.group1()[2])),
            ]),
            // e415, e425, e435, e4
            Simd32x4::from([
                (-(anti_reverse.group1()[1] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group1()[1])),
                ((anti_reverse.group1()[0] * self.group1()[2]) - (anti_reverse.group1()[2] * self.group1()[0])),
                (-(anti_reverse.group1()[0] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group1()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)));
        let subtraction = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e4
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       39        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       45        0
    //  no simd       42       62        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[0] * anti_reverse.group0()[0])
                    - (self.group1()[1] * anti_reverse.group0()[1])
                    - (self.group1()[2] * anti_reverse.group0()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[3]) * self.group0())
                - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group1()[1] * self.group0()[2]) + (self.group1()[1] * anti_reverse.group0()[2])),
                (-(anti_reverse.group1()[2] * self.group0()[0]) + (self.group1()[2] * anti_reverse.group0()[0])),
                (-(anti_reverse.group1()[0] * self.group0()[1]) + (self.group1()[0] * anti_reverse.group0()[1])),
                ((anti_reverse.group1()[1] * self.group0()[1]) + (anti_reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[1] * anti_reverse.group0()[1])
                    - (self.group1()[2] * anti_reverse.group0()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group0()[3]) - (self.group1()[0] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[1] * self.group0()[3]) - (self.group1()[1] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[2] * self.group0()[3]) - (self.group1()[2] * anti_reverse.group0()[3])),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group0()[2]) - (anti_reverse.group1()[2] * self.group0()[1]) + (self.group1()[1] * anti_reverse.group0()[2])
                    - (self.group1()[2] * anti_reverse.group0()[1])),
                (-(anti_reverse.group1()[0] * self.group0()[2]) + (anti_reverse.group1()[2] * self.group0()[0]) - (self.group1()[0] * anti_reverse.group0()[2])
                    + (self.group1()[2] * anti_reverse.group0()[0])),
                ((anti_reverse.group1()[0] * self.group0()[1]) - (anti_reverse.group1()[1] * self.group0()[0]) + (self.group1()[0] * anti_reverse.group0()[1])
                    - (self.group1()[1] * anti_reverse.group0()[0])),
                0.0,
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2)
                - 2.0 * (self.group1()[0] * self.group0()[0])
                - 2.0 * (self.group1()[1] * self.group0()[1])
                - 2.0 * (self.group1()[2] * self.group0()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([geometric_anti_product.group2()[0], geometric_anti_product.group2()[1], geometric_anti_product.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([geometric_anti_product.group3()[0], geometric_anti_product.group3()[1], geometric_anti_product.group3()[2], 0.0]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       78        0
    //    simd3        0        1        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       74       93        0
    //  no simd      113      137        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((self.group0()[0] * anti_reverse.group1()[3]) + (self.group0()[0] * anti_reverse.group2()[3]) + (self.group0()[1] * anti_reverse.group1()[2])),
                ((self.group0()[1] * anti_reverse.group1()[3]) + (self.group0()[1] * anti_reverse.group2()[3]) + (self.group0()[2] * anti_reverse.group1()[0])),
                ((self.group0()[0] * anti_reverse.group1()[1]) + (self.group0()[2] * anti_reverse.group1()[3]) + (self.group0()[2] * anti_reverse.group2()[3])),
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * anti_reverse.group2()[0])
                    - (self.group0()[1] * anti_reverse.group2()[1])
                    - (self.group0()[2] * anti_reverse.group2()[2])),
            ]) + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group2()[3]]))
                - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[0], anti_reverse.group1()[0]]) * swizzle!(self.group1(), 3, 3, 1, 0))
                - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[2], anti_reverse.group1()[1]]) * swizzle!(self.group1(), 2, 0, 3, 1))
                + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group1(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(self.group0()[2] * anti_reverse.group2()[1]) + (anti_reverse.group1()[0] * self.group2()[3]) - (anti_reverse.group1()[1] * self.group1()[2])),
                (-(self.group0()[0] * anti_reverse.group2()[2]) + (anti_reverse.group1()[1] * self.group2()[3]) - (anti_reverse.group1()[2] * self.group1()[0])),
                (-(self.group0()[1] * anti_reverse.group2()[0]) - (anti_reverse.group1()[0] * self.group1()[1]) + (anti_reverse.group1()[2] * self.group2()[3])),
                (-(anti_reverse.group0()[1] * self.group2()[1]) - (anti_reverse.group0()[2] * self.group2()[2]) + (self.group0()[2] * anti_reverse.group2()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group2(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group2()[3]]) * swizzle!(self.group1(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 1))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[2] * self.group2()[1])
                    + (anti_reverse.group2()[0] * self.group1()[3])
                    + (anti_reverse.group2()[0] * self.group2()[3])
                    + (anti_reverse.group2()[2] * self.group1()[1])
                    + (anti_reverse.group2()[3] * self.group2()[0])),
                ((anti_reverse.group1()[0] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group1()[2])
                    + (anti_reverse.group2()[1] * self.group1()[3])
                    + (anti_reverse.group2()[1] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[1])),
                ((anti_reverse.group1()[1] * self.group2()[0])
                    + (anti_reverse.group2()[1] * self.group1()[0])
                    + (anti_reverse.group2()[2] * self.group1()[3])
                    + (anti_reverse.group2()[2] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[2])),
                (-(anti_reverse.group1()[2] * self.group2()[2]) - (anti_reverse.group2()[1] * self.group1()[1]) - (anti_reverse.group2()[2] * self.group1()[2])),
            ]) - (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (swizzle!(anti_reverse.group1(), 3, 3, 3, 1) * swizzle!(self.group2(), 0, 1, 2, 1))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[1]) - (self.group0()[1] * anti_reverse.group2()[2])
                    + (self.group0()[2] * anti_reverse.group2()[1])
                    + (anti_reverse.group1()[0] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[0])),
                ((anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[0]) + (self.group0()[0] * anti_reverse.group2()[2])
                    - (self.group0()[2] * anti_reverse.group2()[0])
                    + (anti_reverse.group1()[1] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[1]) + (anti_reverse.group0()[1] * self.group2()[0]) - (self.group0()[0] * anti_reverse.group2()[1])
                    + (self.group0()[1] * anti_reverse.group2()[0])
                    + (anti_reverse.group1()[2] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[2])),
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * anti_reverse.group1()[0])
                    - (self.group0()[1] * anti_reverse.group1()[1])
                    - (self.group0()[2] * anti_reverse.group1()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2) + f32::powi(self.group2()[3], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       85        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       73       93        0
    //  no simd       91      115        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[1])
                    + (self.group0()[0] * anti_reverse.group2()[3])),
                ((anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[0])
                    + (self.group0()[1] * anti_reverse.group2()[3])),
                (-(anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[0])
                    + (self.group0()[2] * anti_reverse.group2()[3])),
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (self.group0()[0] * anti_reverse.group2()[0])
                    - (self.group0()[1] * anti_reverse.group2()[1])
                    - (self.group0()[2] * anti_reverse.group2()[2])),
            ]) + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group2()[3]]))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group1()[0] * self.group2()[3]) - (anti_reverse.group1()[1] * self.group1()[2])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    - (self.group0()[2] * anti_reverse.group2()[1])),
                ((anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group1()[0] * self.group1()[2]) + (anti_reverse.group1()[1] * self.group2()[3])
                    - (anti_reverse.group1()[2] * self.group1()[0])
                    - (self.group0()[0] * anti_reverse.group2()[2])),
                ((anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group1()[0] * self.group1()[1])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[2] * self.group2()[3])
                    - (self.group0()[1] * anti_reverse.group2()[0])),
                (-(anti_reverse.group0()[1] * self.group2()[1]) - (anti_reverse.group0()[2] * self.group2()[2]) + (self.group0()[2] * anti_reverse.group2()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 1))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[2] * self.group2()[1])
                    + (self.group1()[1] * anti_reverse.group2()[2])
                    + (anti_reverse.group2()[0] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[0])),
                ((anti_reverse.group1()[0] * self.group2()[2])
                    + (self.group1()[2] * anti_reverse.group2()[0])
                    + (anti_reverse.group2()[1] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[1])),
                ((anti_reverse.group1()[1] * self.group2()[0])
                    + (self.group1()[0] * anti_reverse.group2()[1])
                    + (anti_reverse.group2()[2] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group2()[2])),
                (-(anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (self.group1()[1] * anti_reverse.group2()[1])
                    - (self.group1()[2] * anti_reverse.group2()[2])),
            ]) - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 0))),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[1]) - (self.group0()[1] * anti_reverse.group2()[2])
                    + (self.group0()[2] * anti_reverse.group2()[1])),
                ((anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[0]) + (self.group0()[0] * anti_reverse.group2()[2])
                    - (self.group0()[2] * anti_reverse.group2()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[1]) + (anti_reverse.group0()[1] * self.group2()[0]) - (self.group0()[0] * anti_reverse.group2()[1])
                    + (self.group0()[1] * anti_reverse.group2()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group2()[3], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       31        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       35        0
    //  no simd       36       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[1]) + (self.group0()[0] * anti_reverse.group1()[3])),
                ((anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[0]) + (self.group0()[1] * anti_reverse.group1()[3])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[1] * self.group0()[0]) + (self.group0()[2] * anti_reverse.group1()[3])),
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[3]]))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group1()[1])
                    + (self.group0()[1] * anti_reverse.group1()[2])
                    + (anti_reverse.group1()[0] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[0])),
                ((anti_reverse.group0()[0] * self.group1()[2])
                    + (self.group0()[2] * anti_reverse.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[1])),
                ((anti_reverse.group0()[1] * self.group1()[0])
                    + (self.group0()[0] * anti_reverse.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[2])),
                (-(anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (self.group0()[1] * anti_reverse.group1()[1])
                    - (self.group0()[2] * anti_reverse.group1()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       34        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       31       41        0
    //  no simd       49       62        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group1()[3])),
                (anti_reverse.group0()[3] * self.group0()[0]),
                (anti_reverse.group0()[3] * self.group0()[1]),
                (anti_reverse.group0()[3] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * swizzle!(anti_reverse.group0(), 3, 0, 1, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group1()[3] * self.group0()[0])),
                ((anti_reverse.group0()[1] * self.group1()[3]) - (anti_reverse.group0()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group1()[3])),
                0.0,
            ]) + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3]]) * swizzle!(self.group0(), 1, 1, 2, 3))
                + (Simd32x4::from([self.group1()[3], self.group0()[2], self.group0()[0], self.group1()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group1()[1])
                    + (anti_reverse.group1()[0] * self.group0()[3])
                    + (anti_reverse.group1()[0] * self.group1()[3])
                    + (anti_reverse.group1()[2] * self.group0()[1])
                    + (anti_reverse.group1()[3] * self.group1()[0])),
                ((anti_reverse.group0()[0] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[1] * self.group0()[3])
                    + (anti_reverse.group1()[1] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[1])),
                ((anti_reverse.group0()[1] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group0()[0])
                    + (anti_reverse.group1()[2] * self.group0()[3])
                    + (anti_reverse.group1()[2] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[2])),
                (-(anti_reverse.group0()[2] * self.group1()[2]) - (anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2])),
            ]) - (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                - (swizzle!(anti_reverse.group0(), 3, 3, 3, 1) * swizzle!(self.group1(), 0, 1, 2, 1))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group0(), 2, 0, 1, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       39        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       36       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group1()[1] * self.group0()[2]) + (self.group1()[1] * anti_reverse.group0()[2]) - (self.group1()[2] * anti_reverse.group0()[1])
                    + (anti_reverse.group0()[0] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[0])),
                (-(anti_reverse.group1()[2] * self.group0()[0]) - (self.group1()[0] * anti_reverse.group0()[2])
                    + (self.group1()[2] * anti_reverse.group0()[0])
                    + (anti_reverse.group0()[1] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[1])),
                (-(anti_reverse.group1()[0] * self.group0()[1]) + (self.group1()[0] * anti_reverse.group0()[1]) - (self.group1()[1] * anti_reverse.group0()[0])
                    + (anti_reverse.group0()[2] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[2])),
                (-(anti_reverse.group1()[0] * self.group1()[0]) - (anti_reverse.group1()[1] * self.group1()[1]) - (anti_reverse.group1()[2] * self.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group0()[3]]) * swizzle!(self.group0(), 1, 2, 0, 3))),
            // e415, e425, e435, e4
            Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group0()[3]) - (anti_reverse.group1()[1] * self.group1()[2])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    + (self.group1()[0] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[0] * self.group1()[2]) + (anti_reverse.group1()[1] * self.group0()[3]) - (anti_reverse.group1()[2] * self.group1()[0])
                    + (self.group1()[1] * anti_reverse.group0()[3])),
                (-(anti_reverse.group1()[0] * self.group1()[1])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[2] * self.group0()[3])
                    + (self.group1()[2] * anti_reverse.group0()[3])),
                (-(anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[0] * anti_reverse.group0()[0])
                    - (self.group1()[1] * anti_reverse.group0()[1])
                    - (self.group1()[2] * anti_reverse.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e4
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Dipole {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       55       70        0
    //    simd3        0        2        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       64       82        0
    //  no simd       91      116        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Dipole::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1]) - (self.group0()[1] * anti_reverse.group1()[2])),
                (-(anti_reverse.group0()[1] * self.group1()[3]) - (self.group0()[2] * anti_reverse.group1()[0])),
                (-(anti_reverse.group0()[2] * self.group1()[3]) - (self.group0()[0] * anti_reverse.group1()[1])),
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group1(), 3, 2, 0, 3))
                + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group1()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group1()[1]]) * swizzle!(anti_reverse.group1(), 3, 2, 0, 1))
                + (Simd32x4::from([self.group0()[2], self.group0()[1], self.group0()[2], self.group1()[2]]) * swizzle!(anti_reverse.group1(), 1, 3, 3, 2))),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group1()[2])
                    - (anti_reverse.group1()[2] * self.group1()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[0])
                    - (anti_reverse.group1()[0] * self.group1()[2])
                    + (anti_reverse.group1()[2] * self.group1()[0])),
                ((anti_reverse.group0()[0] * self.group2()[1]) - (anti_reverse.group0()[1] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group0()[1])
                    - (anti_reverse.group2()[1] * self.group0()[0])
                    + (anti_reverse.group1()[0] * self.group1()[1])
                    - (anti_reverse.group1()[1] * self.group1()[0])),
                ((anti_reverse.group0()[0] * self.group2()[0]) + (anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group2()[2] * self.group1()[1]) - (self.group2()[0] * anti_reverse.group1()[3]) - (self.group2()[1] * anti_reverse.group1()[2])),
                (-(anti_reverse.group2()[0] * self.group1()[2]) - (self.group2()[1] * anti_reverse.group1()[3]) - (self.group2()[2] * anti_reverse.group1()[0])),
                (-(anti_reverse.group2()[1] * self.group1()[0]) - (self.group2()[0] * anti_reverse.group1()[1]) - (self.group2()[2] * anti_reverse.group1()[3])),
                ((anti_reverse.group2()[2] * self.group1()[2]) + (self.group2()[1] * anti_reverse.group1()[1]) + (self.group2()[2] * anti_reverse.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group2()[0], anti_reverse.group2()[0]]) * swizzle!(self.group1(), 3, 3, 1, 0))
                + (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[2], anti_reverse.group2()[1]]) * swizzle!(self.group1(), 2, 0, 3, 1))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group2()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group2()[1]) - (anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group0()[1])
                    + (anti_reverse.group2()[1] * self.group0()[0])),
                ((anti_reverse.group0()[2] * self.group1()[2])
                    + (self.group0()[0] * anti_reverse.group1()[0])
                    + (self.group0()[1] * anti_reverse.group1()[1])
                    + (self.group0()[2] * anti_reverse.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 3, 3, 3, 0))
                + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group0()[1]]) * swizzle!(self.group1(), 0, 1, 2, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       39        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       45        0
    //  no simd       42       62        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (self.group1()[0] * anti_reverse.group0()[0])
                    + (self.group1()[1] * anti_reverse.group0()[1])
                    + (self.group1()[2] * anti_reverse.group0()[2])),
            ]) - (Simd32x4::from(self.group0()[3]) * anti_reverse.group0())
                + (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 0, 1, 2, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group0()[2]) - (self.group1()[1] * anti_reverse.group0()[2])),
                ((anti_reverse.group1()[2] * self.group0()[0]) - (self.group1()[2] * anti_reverse.group0()[0])),
                ((anti_reverse.group1()[0] * self.group0()[1]) - (self.group1()[0] * anti_reverse.group0()[1])),
                (-(anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2])
                    + (self.group1()[1] * anti_reverse.group0()[1])
                    + (self.group1()[2] * anti_reverse.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group0()[3]) - (self.group1()[0] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[1] * self.group0()[3]) - (self.group1()[1] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[2] * self.group0()[3]) - (self.group1()[2] * anti_reverse.group0()[3])),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(anti_reverse.group1()[1] * self.group0()[2]) + (anti_reverse.group1()[2] * self.group0()[1]) - (self.group1()[1] * anti_reverse.group0()[2])
                    + (self.group1()[2] * anti_reverse.group0()[1])),
                ((anti_reverse.group1()[0] * self.group0()[2]) - (anti_reverse.group1()[2] * self.group0()[0]) + (self.group1()[0] * anti_reverse.group0()[2])
                    - (self.group1()[2] * anti_reverse.group0()[0])),
                (-(anti_reverse.group1()[0] * self.group0()[1]) + (anti_reverse.group1()[1] * self.group0()[0]) - (self.group1()[0] * anti_reverse.group0()[1])
                    + (self.group1()[1] * anti_reverse.group0()[0])),
                0.0,
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2)
                + 2.0 * (self.group1()[0] * self.group0()[0])
                + 2.0 * (self.group1()[1] * self.group0()[1])
                + 2.0 * (self.group1()[2] * self.group0()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([geometric_anti_product.group2()[0], geometric_anti_product.group2()[1], geometric_anti_product.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([geometric_anti_product.group3()[0], geometric_anti_product.group3()[1], geometric_anti_product.group3()[2], 0.0]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       20        0
    //    simd3        0        1        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       21       27        0
    //  no simd       36       47        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([((anti_reverse.group0()[2] * self.group0()[2]) - (anti_reverse.group0()[3] * self.group0()[3])), 0.0, 0.0, 0.0])
                + (swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * swizzle!(self.group0(), 0, 3, 3, 3))
                + (swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * swizzle!(self.group0(), 1, 0, 1, 2))),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group0()[1]) - (anti_reverse.group0()[1] * self.group0()[0])),
                0.0,
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[2] * self.group0()[1]) - (self.group1()[0] * anti_reverse.group0()[3]) - (self.group1()[1] * anti_reverse.group0()[2])),
                (-(anti_reverse.group1()[0] * self.group0()[2]) - (self.group1()[1] * anti_reverse.group0()[3]) - (self.group1()[2] * anti_reverse.group0()[0])),
                (-(anti_reverse.group1()[1] * self.group0()[0]) - (self.group1()[0] * anti_reverse.group0()[1]) - (self.group1()[2] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[2] * self.group0()[2]) + (self.group1()[1] * anti_reverse.group0()[1]) + (self.group1()[2] * anti_reverse.group0()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 3, 3, 1, 0))
                + (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[2], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 2, 0, 3, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([geometric_anti_product.group1()[0], geometric_anti_product.group1()[1], geometric_anti_product.group1()[2], 0.0]),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       42        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       31       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAtOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                ((anti_reverse.group0()[0] * self.group1()[0])
                    + (anti_reverse.group0()[1] * self.group1()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])),
                ((anti_reverse.group0()[1] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[1]) - (anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[0])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[1]) - (anti_reverse.group0()[1] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[0]) + (anti_reverse.group0()[1] * self.group1()[1]) + (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (2.0 * (self.group0()[0] * self.group1()[0]) + 2.0 * (self.group0()[1] * self.group1()[1]) + 2.0 * (self.group0()[2] * self.group1()[2])),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd3        0        1        0
    //    simd4       37       38        0
    // Totals...
    // yes simd      109      127        0
    //  no simd      220      243        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1])
                    - (self.group0()[1] * anti_reverse.group1()[2])
                    - (anti_reverse.group1()[0] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group1()[0])
                    - (anti_reverse.group2()[3] * self.group3()[0])),
                (-(anti_reverse.group0()[1] * self.group1()[3])
                    - (self.group0()[2] * anti_reverse.group1()[0])
                    - (anti_reverse.group1()[1] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group1()[1])
                    - (anti_reverse.group2()[3] * self.group3()[1])),
                (-(anti_reverse.group0()[2] * self.group1()[3])
                    - (self.group0()[0] * anti_reverse.group1()[1])
                    - (anti_reverse.group1()[2] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group1()[2])
                    - (anti_reverse.group2()[3] * self.group3()[2])),
                ((anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group3()[1] * self.group3()[1])
                    + (anti_reverse.group3()[2] * self.group3()[2])),
            ]) + (Simd32x4::from(self.group0()[0]) * Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[2], anti_reverse.group3()[1], anti_reverse.group2()[0]]))
                + (Simd32x4::from(self.group0()[1]) * Simd32x4::from([anti_reverse.group3()[2], anti_reverse.group1()[3], anti_reverse.group1()[0], anti_reverse.group2()[1]]))
                + (Simd32x4::from(self.group0()[2]) * Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group3()[0], anti_reverse.group1()[3], anti_reverse.group2()[2]]))
                - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group1(), 3, 2, 0, 3))
                + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group1()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group3()[0]]) * swizzle!(self.group3(), 2, 0, 1, 0))
                - (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group2()[3]]) * swizzle!(self.group3(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group2()[3]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 3))
                + (Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 3, 3, 3, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group2()[1]) + (self.group0()[2] * anti_reverse.group2()[1]) + (anti_reverse.group1()[1] * self.group1()[2])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) + (self.group0()[0] * anti_reverse.group2()[2]) + (anti_reverse.group1()[2] * self.group1()[0])),
                (-(anti_reverse.group0()[1] * self.group2()[0]) + (self.group0()[1] * anti_reverse.group2()[0]) + (anti_reverse.group1()[0] * self.group1()[1])),
                ((anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2]) - (anti_reverse.group2()[3] * self.group3()[3])),
            ]) - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[0]]) * swizzle!(self.group3(), 3, 3, 3, 0))
                + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * swizzle!(anti_reverse.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group3()[1]]) * swizzle!(anti_reverse.group1(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group1()[3], self.group3()[2], self.group3()[0], self.group1()[1]]) * swizzle!(anti_reverse.group3(), 0, 0, 1, 1))
                - (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[2]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 0, 1, 2, 1))
                - (Simd32x4::from([self.group3()[1], self.group1()[3], self.group1()[3], self.group1()[2]]) * swizzle!(anti_reverse.group3(), 2, 1, 2, 2))
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 3))
                - (swizzle!(anti_reverse.group1(), 3, 3, 3, 2) * swizzle!(self.group3(), 0, 1, 2, 2))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[2] * self.group2()[1])
                    - (anti_reverse.group1()[3] * self.group2()[0])
                    - (anti_reverse.group3()[2] * self.group2()[1])
                    - (anti_reverse.group3()[3] * self.group1()[0])),
                (-(anti_reverse.group1()[3] * self.group2()[1])
                    - (anti_reverse.group3()[0] * self.group2()[2])
                    - (anti_reverse.group3()[1] * self.group3()[3])
                    - (anti_reverse.group3()[3] * self.group1()[1])),
                (-(anti_reverse.group1()[3] * self.group2()[2])
                    - (anti_reverse.group2()[1] * self.group1()[0])
                    - (anti_reverse.group3()[1] * self.group2()[0])
                    - (anti_reverse.group3()[3] * self.group1()[2])),
                ((anti_reverse.group1()[2] * self.group2()[2])
                    + (anti_reverse.group3()[1] * self.group2()[1])
                    + (anti_reverse.group3()[2] * self.group2()[2])
                    + (anti_reverse.group3()[3] * self.group1()[3])),
            ]) + (Simd32x4::from(anti_reverse.group2()[0]) * Simd32x4::from([self.group1()[3], self.group3()[2], self.group1()[1], self.group1()[0]]))
                + (Simd32x4::from(anti_reverse.group2()[1]) * Simd32x4::from([self.group1()[2], self.group1()[3], self.group3()[0], self.group1()[1]]))
                + (Simd32x4::from(anti_reverse.group2()[2]) * Simd32x4::from([self.group3()[1], self.group1()[0], self.group1()[3], self.group1()[2]]))
                - (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group2()[0]]) * swizzle!(self.group3(), 2, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group2()[2], anti_reverse.group3()[2], anti_reverse.group2()[2]]) * swizzle!(self.group3(), 3, 0, 3, 2))
                + (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group1()[1]]) * swizzle!(self.group2(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group3()[1], self.group3()[1]]) * swizzle!(anti_reverse.group2(), 2, 0, 0, 1))
                + (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[0]]) * swizzle!(anti_reverse.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group3()[3], self.group2()[2], self.group2()[0], self.group3()[3]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 3))
                + (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group3()[3]) - (anti_reverse.group0()[2] * self.group2()[1])
                    + (self.group0()[0] * anti_reverse.group3()[3])
                    + (self.group0()[1] * anti_reverse.group2()[2])
                    - (anti_reverse.group1()[1] * self.group3()[2])
                    + (anti_reverse.group2()[0] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[1] * self.group3()[3])
                    + (self.group0()[1] * anti_reverse.group3()[3])
                    + (self.group0()[2] * anti_reverse.group2()[0])
                    - (anti_reverse.group1()[2] * self.group3()[0])
                    + (anti_reverse.group2()[1] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[1])),
                (-(anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group0()[2] * self.group3()[3])
                    + (self.group0()[0] * anti_reverse.group2()[1])
                    + (self.group0()[2] * anti_reverse.group3()[3])
                    - (anti_reverse.group1()[0] * self.group3()[1])
                    + (anti_reverse.group2()[2] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[2])),
                ((anti_reverse.group0()[0] * self.group3()[0])
                    + (anti_reverse.group0()[1] * self.group3()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group0()[2] * self.group3()[2])
                    - (self.group0()[1] * anti_reverse.group3()[1])
                    + (self.group0()[2] * anti_reverse.group1()[2])
                    - (self.group0()[2] * anti_reverse.group3()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group1()[3]]) * swizzle!(self.group2(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[3]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 3))
                + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 0, 1, 2, 0))
                + (Simd32x4::from([anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group0()[1]]) * swizzle!(self.group1(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group1()[3], self.group3()[2], self.group3()[0], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 0))
                + (Simd32x4::from([self.group3()[1], self.group1()[3], self.group1()[3], self.group0()[1]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group3()[0], 2)
                + f32::powi(self.group3()[1], 2)
                + f32::powi(self.group3()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleInversionAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       70        0
    //    simd4       22       23        0
    // Totals...
    // yes simd       70       93        0
    //  no simd      136      162        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((anti_reverse.group1()[3] * self.group2()[0]) * -1.0),
                ((anti_reverse.group0()[1] * self.group0()[3]) * -1.0),
                ((anti_reverse.group0()[2] * self.group0()[3]) * -1.0),
                ((anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group2()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3]]) * swizzle!(self.group2(), 1, 1, 2, 3))
                + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group0()[1], self.group1()[3], self.group1()[3], self.group2()[1]]) * swizzle!(anti_reverse.group2(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[3]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[3], self.group2()[2], self.group2()[0], self.group0()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group1()[3], self.group0()[2], self.group0()[0], self.group2()[0]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 0))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group0()[3] * self.group2()[0]) + (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group2()[0] * self.group0()[3])
                    - (anti_reverse.group2()[2] * self.group2()[1])
                    - (anti_reverse.group2()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[3] * self.group2()[1]) + (anti_reverse.group1()[2] * self.group0()[0])
                    - (anti_reverse.group2()[0] * self.group2()[2])
                    - (anti_reverse.group2()[1] * self.group0()[3])
                    - (anti_reverse.group2()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group0()[3] * self.group2()[2]) + (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group2()[1] * self.group2()[0])
                    - (anti_reverse.group2()[2] * self.group0()[3])
                    - (anti_reverse.group2()[3] * self.group0()[2])),
                ((anti_reverse.group0()[1] * self.group1()[1]) + (anti_reverse.group0()[2] * self.group1()[2])),
            ]) - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[3]]))
                - (Simd32x4::from([self.group0()[1], self.group1()[3], self.group1()[3], self.group0()[1]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]) * swizzle!(anti_reverse.group1(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group1()[3], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 0))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[3]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 3))
                + (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[2] * self.group2()[1]) - (anti_reverse.group2()[2] * self.group1()[1])),
                ((anti_reverse.group1()[0] * self.group2()[2]) - (anti_reverse.group2()[0] * self.group1()[2])),
                ((anti_reverse.group1()[1] * self.group2()[0]) - (anti_reverse.group2()[1] * self.group1()[0])),
                (-(anti_reverse.group1()[2] * self.group2()[2]) + (anti_reverse.group2()[2] * self.group1()[2])),
            ]) - (Simd32x4::from(anti_reverse.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                + (Simd32x4::from(self.group0()[3]) * Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group2()[3]]))
                - (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group1()[1]]) * swizzle!(self.group2(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 1))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group2()[3]) - (anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group1()[2] * self.group0()[1])
                    - (anti_reverse.group1()[3] * self.group1()[0])
                    + (anti_reverse.group2()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) - (anti_reverse.group0()[1] * self.group2()[3]) + (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group1()[3] * self.group1()[1])
                    + (anti_reverse.group2()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group0()[2] * self.group2()[3]) + (anti_reverse.group1()[1] * self.group0()[0])
                    - (anti_reverse.group1()[3] * self.group1()[2])
                    + (anti_reverse.group2()[3] * self.group0()[2])),
                ((anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 3) * swizzle!(self.group0(), 2, 0, 1, 3))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2)
                + f32::powi(self.group2()[0], 2)
                + f32::powi(self.group2()[1], 2)
                + f32::powi(self.group2()[2], 2)
                + 2.0 * (self.group0()[0] * self.group1()[0])
                + 2.0 * (self.group0()[1] * self.group1()[1])
                + 2.0 * (self.group0()[2] * self.group1()[2])
                - 2.0 * (self.group1()[3] * self.group2()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       32        0
    //    simd3        0        1        0
    //    simd4       19       20        0
    // Totals...
    // yes simd       43       53        0
    //  no simd      100      115        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                ((anti_reverse.group2()[1] * self.group2()[1]) + (anti_reverse.group2()[2] * self.group2()[2])),
                ((anti_reverse.group2()[1] * self.group0()[2]) * -1.0),
                ((anti_reverse.group2()[2] * self.group0()[0]) * -1.0),
                ((anti_reverse.group2()[0] * self.group0()[1]) * -1.0),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[3], self.group2()[2], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 0, 0, 0, 1))
                + (Simd32x4::from([self.group0()[1], self.group2()[1], self.group0()[3], self.group0()[3]]) * swizzle!(anti_reverse.group0(), 1, 2, 1, 2))
                - (Simd32x4::from([self.group0()[3], self.group2()[2], self.group2()[0], self.group2()[1]]) * swizzle!(anti_reverse.group0(), 3, 1, 2, 0))
                + (Simd32x4::from([self.group2()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 0, 2, 0, 1))
                + (swizzle!(anti_reverse.group0(), 2, 3, 3, 3) * swizzle!(self.group0(), 2, 0, 1, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group2()[1] * self.group2()[2])),
                ((anti_reverse.group0()[2] * self.group0()[0]) + (anti_reverse.group2()[2] * self.group2()[0])),
                ((anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group2()[0] * self.group2()[1])),
                (-(anti_reverse.group0()[2] * self.group2()[2]) - (anti_reverse.group2()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group0()[3], self.group2()[2], self.group2()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 0))
                - (Simd32x4::from([self.group2()[1], self.group0()[3], self.group0()[3], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 2, 1, 2, 1))
                - (swizzle!(anti_reverse.group0(), 3, 3, 3, 1) * swizzle!(self.group2(), 0, 1, 2, 1))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[2] * self.group0()[1])
                    - (self.group1()[1] * anti_reverse.group0()[2])
                    - (self.group1()[1] * anti_reverse.group2()[2])
                    - (anti_reverse.group2()[3] * self.group0()[0])),
                (-(anti_reverse.group1()[0] * self.group0()[2])
                    - (self.group1()[2] * anti_reverse.group0()[0])
                    - (self.group1()[2] * anti_reverse.group2()[0])
                    - (anti_reverse.group2()[3] * self.group0()[1])),
                (-(anti_reverse.group1()[1] * self.group0()[0])
                    - (self.group1()[0] * anti_reverse.group2()[1])
                    - (self.group1()[2] * anti_reverse.group0()[3])
                    - (anti_reverse.group2()[3] * self.group0()[2])),
                ((self.group1()[1] * anti_reverse.group0()[1])
                    + (self.group1()[2] * anti_reverse.group0()[2])
                    + (self.group1()[2] * anti_reverse.group2()[2])
                    + (anti_reverse.group2()[3] * self.group0()[3])),
            ]) + (Simd32x4::from(anti_reverse.group1()[0]) * Simd32x4::from([self.group0()[3], self.group2()[2], self.group0()[1], self.group0()[0]]))
                + (Simd32x4::from(anti_reverse.group1()[1]) * Simd32x4::from([self.group0()[2], self.group0()[3], self.group2()[0], self.group0()[1]]))
                + (Simd32x4::from(anti_reverse.group1()[2]) * Simd32x4::from([self.group2()[1], self.group0()[0], self.group0()[3], self.group0()[2]]))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[0], self.group2()[3]]) * swizzle!(anti_reverse.group0(), 3, 3, 1, 3))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[1]]) * swizzle!(self.group2(), 3, 3, 3, 1))
                - (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group1()[2]]) * swizzle!(self.group2(), 3, 3, 3, 2))
                + (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)
                + f32::powi(self.group2()[0], 2)
                + f32::powi(self.group2()[1], 2)
                + f32::powi(self.group2()[2], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleInversionAtOrigin {
    type Output = MysteryVersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       22        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       32       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = MysteryVersorRoundPoint::from_groups(
            // e1, e2, e3, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[3] * self.group1()[0])),
                (-(anti_reverse.group1()[2] * self.group0()[0]) - (anti_reverse.group1()[3] * self.group1()[1])),
                (-(anti_reverse.group1()[0] * self.group0()[1]) - (anti_reverse.group1()[3] * self.group1()[2])),
                ((anti_reverse.group0()[2] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[3]]) * swizzle!(self.group0(), 2, 3, 3, 3))
                + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group0()[1], self.group1()[3], self.group1()[3], self.group0()[1]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group1()[3], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 0))
                + (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (2.0 * (self.group0()[0] * self.group1()[0]) + 2.0 * (self.group0()[1] * self.group1()[1]) + 2.0 * (self.group0()[2] * self.group1()[2])
                - 2.0 * (self.group0()[3] * self.group1()[3])),
        );
        let subtraction = MysteryVersorRoundPoint::from_groups(/* e1, e2, e3, e12345 */ Simd32x4::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
            (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleInversionOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       24       31        0
    //  no simd       45       55        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e1234, e4235, e4315, e4125
            self.group1(),
        );
        let geometric_anti_product = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[2]) + (anti_reverse.group0()[3] * self.group0()[0])
                    - (anti_reverse.group1()[0] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])),
                (-(anti_reverse.group0()[1] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[1])
                    - (anti_reverse.group1()[0] * self.group1()[2])
                    - (anti_reverse.group1()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[2] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[2])
                    - (anti_reverse.group1()[0] * self.group1()[3])
                    - (anti_reverse.group1()[1] * self.group0()[1])),
                0.0,
            ]) + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group1()[1]]) * swizzle!(self.group1(), 3, 1, 2, 1))
                + (Simd32x4::from([self.group0()[1], self.group1()[0], self.group1()[0], self.group1()[3]]) * swizzle!(anti_reverse.group1(), 3, 2, 3, 3))
                - (Simd32x4::from([self.group0()[3], self.group1()[3], self.group1()[1], self.group0()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group1()[0], self.group0()[2], self.group0()[0], self.group1()[2]]) * swizzle!(anti_reverse.group1(), 1, 1, 2, 2))),
            // e415, e425, e435, e4
            (Simd32x4::from([
                ((anti_reverse.group0()[3] * self.group1()[1]) * -1.0),
                ((anti_reverse.group0()[3] * self.group1()[2]) * -1.0),
                ((anti_reverse.group0()[3] * self.group1()[3]) * -1.0),
                ((anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[3]) + (anti_reverse.group0()[3] * self.group1()[0])
                    - (anti_reverse.group1()[2] * self.group0()[1])
                    - (anti_reverse.group1()[3] * self.group0()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[3], anti_reverse.group1()[1], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 3, 1, 2, 1))
                - (Simd32x4::from([self.group0()[3], self.group1()[3], self.group1()[1], self.group0()[3]]) * swizzle!(anti_reverse.group1(), 1, 1, 2, 0))
                - (Simd32x4::from([self.group1()[2], self.group0()[3], self.group0()[3], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 3, 2, 3, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e4
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       69       91        0
    //    simd3        0        1        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       78      101        0
    //  no simd      105      130        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(self.group1()[0] * anti_reverse.group2()[3]) - (self.group1()[1] * anti_reverse.group0()[2])),
                (-(self.group1()[1] * anti_reverse.group2()[3]) - (self.group1()[2] * anti_reverse.group0()[0])),
                (-(self.group1()[0] * anti_reverse.group0()[1]) - (self.group1()[2] * anti_reverse.group2()[3])),
                ((anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])),
            ]) - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[3]]))
                + (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group2()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group2()[3]]) * swizzle!(self.group0(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group1()[2]) - (anti_reverse.group1()[2] * self.group1()[1]) - (anti_reverse.group0()[0] * self.group0()[3])
                    + (anti_reverse.group0()[1] * self.group2()[2])
                    - (anti_reverse.group0()[2] * self.group2()[1])
                    - (anti_reverse.group0()[3] * self.group0()[0])
                    - (anti_reverse.group2()[0] * self.group2()[3])
                    + (anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[1])
                    - (anti_reverse.group2()[3] * self.group2()[0])),
                (-(anti_reverse.group1()[0] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group1()[0])
                    - (anti_reverse.group0()[0] * self.group2()[2])
                    - (anti_reverse.group0()[1] * self.group0()[3])
                    + (anti_reverse.group0()[2] * self.group2()[0])
                    - (anti_reverse.group0()[3] * self.group0()[1])
                    - (anti_reverse.group2()[0] * self.group0()[2])
                    - (anti_reverse.group2()[1] * self.group2()[3])
                    + (anti_reverse.group2()[2] * self.group0()[0])
                    - (anti_reverse.group2()[3] * self.group2()[1])),
                ((anti_reverse.group1()[0] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group1()[0]) + (anti_reverse.group0()[0] * self.group2()[1])
                    - (anti_reverse.group0()[1] * self.group2()[0])
                    - (anti_reverse.group0()[2] * self.group0()[3])
                    - (anti_reverse.group0()[3] * self.group0()[2])
                    + (anti_reverse.group2()[0] * self.group0()[1])
                    - (anti_reverse.group2()[1] * self.group0()[0])
                    - (anti_reverse.group2()[2] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[2])),
                0.0,
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[0] * self.group0()[3])
                    - (anti_reverse.group1()[2] * self.group2()[1])
                    - (self.group1()[0] * anti_reverse.group0()[3])
                    - (self.group1()[1] * anti_reverse.group2()[2])),
                (-(anti_reverse.group1()[0] * self.group2()[2])
                    - (anti_reverse.group1()[1] * self.group0()[3])
                    - (self.group1()[1] * anti_reverse.group0()[3])
                    - (self.group1()[2] * anti_reverse.group2()[0])),
                (-(anti_reverse.group1()[1] * self.group2()[0])
                    - (anti_reverse.group1()[2] * self.group0()[3])
                    - (self.group1()[0] * anti_reverse.group2()[1])
                    - (self.group1()[2] * anti_reverse.group0()[3])),
                ((anti_reverse.group1()[1] * self.group2()[1])
                    + (anti_reverse.group1()[2] * self.group2()[2])
                    + (self.group1()[1] * anti_reverse.group2()[1])
                    + (self.group1()[2] * anti_reverse.group2()[2])),
            ]) + (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 0))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[3]) - (anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group2()[0] * self.group2()[3])
                    - (anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group2()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group0()[1] * self.group0()[3]) + (anti_reverse.group2()[1] * self.group2()[3])
                    - (anti_reverse.group2()[2] * self.group0()[0])
                    - (anti_reverse.group2()[3] * self.group2()[1])),
                (-(anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group0()[2] * self.group0()[3]) - (anti_reverse.group2()[0] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group2()[2])),
                ((anti_reverse.group1()[2] * self.group0()[2]) + (self.group1()[1] * anti_reverse.group0()[1]) + (self.group1()[2] * anti_reverse.group0()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                + (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 1, 2, 0, 1))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group0()[3] * self.group2()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([geometric_anti_product.group1()[0], geometric_anti_product.group1()[1], geometric_anti_product.group1()[2], 0.0]),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[1] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[2] * self.group0()[3]) + (anti_reverse.group0()[3] * self.group0()[2])),
                (anti_reverse.group0()[3] * self.group0()[3] * -1.0),
            ]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       87        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       71       90        0
    //  no simd       71       96        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group1()[2]) - (anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[0]) - (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[1]) - (anti_reverse.group0()[1] * self.group1()[0]) + (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group1()[1] * self.group1()[2])
                    - (anti_reverse.group1()[2] * self.group1()[1])
                    + (anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[0]) - (anti_reverse.group1()[0] * self.group1()[2])
                    + (anti_reverse.group1()[2] * self.group1()[0])
                    - (anti_reverse.group2()[0] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group2()[1]) - (anti_reverse.group0()[1] * self.group2()[0]) + (anti_reverse.group1()[0] * self.group1()[1])
                    - (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group2()[0] * self.group0()[1])
                    - (anti_reverse.group2()[1] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group2()[0]) + (anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group2()[2]) - (anti_reverse.group1()[2] * self.group2()[1]) + (anti_reverse.group2()[1] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group1()[1])),
                (-(anti_reverse.group1()[0] * self.group2()[2]) + (anti_reverse.group1()[2] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group1()[2])
                    + (anti_reverse.group2()[2] * self.group1()[0])),
                ((anti_reverse.group1()[0] * self.group2()[1]) - (anti_reverse.group1()[1] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group1()[0])),
                ((anti_reverse.group1()[0] * self.group2()[0])
                    + (anti_reverse.group1()[1] * self.group2()[1])
                    + (anti_reverse.group1()[2] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group1()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group2()[2]) - (anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) + (anti_reverse.group0()[2] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group0()[2])
                    - (anti_reverse.group2()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group2()[1]) - (anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group2()[0] * self.group0()[1])
                    + (anti_reverse.group2()[1] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group1()[0])
                    + (anti_reverse.group0()[1] * self.group1()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for FlatOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = FlatOrigin::from_groups(/* e45 */ (self[e45] * -1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse[e45] * self[e45] * -1.0));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self[e45], 2) * -1.0));
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
    }
}
impl AntiConstraintViolation for FlatPoint {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([
                ((anti_reverse.group0()[0] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[0])),
                ((anti_reverse.group0()[1] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[1])),
                ((anti_reverse.group0()[2] * self.group0()[3]) - (anti_reverse.group0()[3] * self.group0()[2])),
                (anti_reverse.group0()[3] * self.group0()[3] * -1.0),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([
                geometric_anti_product.group1()[0],
                geometric_anti_product.group1()[1],
                geometric_anti_product.group1()[2],
                (geometric_anti_product.group1()[3] - anti_scalar_product[e12345]),
            ]),
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
        let anti_reverse = Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from([
                (-(anti_reverse.group1()[0] * self.group0()[3]) - (anti_reverse.group1()[2] * self.group1()[1])),
                (-(anti_reverse.group1()[0] * self.group1()[2]) - (anti_reverse.group1()[1] * self.group0()[3])),
                (-(anti_reverse.group1()[1] * self.group1()[0]) - (anti_reverse.group1()[2] * self.group0()[3])),
                ((anti_reverse.group1()[1] * self.group1()[1]) + (anti_reverse.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from(anti_reverse.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e235, e315, e125, e5
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
            // e12345
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for FlectorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        5        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (-(anti_reverse.group0()[0] * self.group0()[0])
                + (anti_reverse.group0()[1] * self.group0()[1])
                + (anti_reverse.group0()[2] * self.group0()[2])
                + (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[1] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
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
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e235, e315, e125, e5
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for LineOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       10        0
    //  no simd        8       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_anti_product = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            (-(anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[1])),
            ((anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[0])),
            (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[1] * self.group0()[0])),
            (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
        ]));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
            (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for Motor {
    type Output = VersorRoundPointAligningOriginAtInfinity;
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
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = VersorRoundPointAligningOriginAtInfinity::from_groups(
            // e5, e12345
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
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorRoundPointAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            geometric_anti_product.group0()[0],
            (geometric_anti_product.group0()[1] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for MotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])
                + (anti_reverse.group0()[3] * self.group0()[3])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      342      360        0
    //    simd2       16       16        0
    //    simd3      118      124        0
    //    simd4       72       74        0
    // Totals...
    // yes simd      548      574        0
    //  no simd     1016     1060        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (self.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e12345
            (Simd32x2::from([
                ((anti_reverse.group0()[1] * self.group0()[0])
                    - (anti_reverse.group8()[0] * self.group3()[0])
                    - (anti_reverse.group8()[1] * self.group3()[1])
                    - (anti_reverse.group8()[2] * self.group3()[2])
                    - (self.group5()[0] * anti_reverse.group6()[0])
                    - (self.group5()[1] * anti_reverse.group6()[1])
                    - (self.group5()[2] * anti_reverse.group6()[2])
                    - (self.group8()[0] * anti_reverse.group3()[0])
                    - (self.group8()[1] * anti_reverse.group3()[1])
                    - (self.group8()[2] * anti_reverse.group3()[2])
                    - (anti_reverse.group6()[3] * self.group3()[3])
                    + (anti_reverse.group9()[0] * self[e1])
                    + (anti_reverse.group9()[1] * self.group1()[0])
                    + (anti_reverse.group9()[2] * self.group1()[1])
                    + (anti_reverse.group9()[3] * self.group1()[2])
                    + (self.group9()[0] * anti_reverse[e1])),
                (-(anti_reverse.group0()[0] * self.group0()[0])
                    + (anti_reverse.group4()[0] * self.group3()[0])
                    + (anti_reverse.group4()[1] * self.group3()[1])
                    + (anti_reverse.group4()[2] * self.group3()[2])
                    + (anti_reverse.group5()[0] * self.group5()[0])
                    + (anti_reverse.group5()[1] * self.group5()[1])
                    + (anti_reverse.group5()[2] * self.group5()[2])
                    + (self.group4()[0] * anti_reverse.group3()[0])
                    + (self.group4()[1] * anti_reverse.group3()[1])
                    + (self.group4()[2] * anti_reverse.group3()[2])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group6()[3] * self.group6()[3])
                    - (anti_reverse.group9()[0] * self[e45])
                    - (self.group9()[0] * anti_reverse[e45])),
            ]) + (Simd32x2::from(self.group0()[1]) * anti_reverse.group0())
                - (Simd32x2::from(anti_reverse.group7()[0]) * Simd32x2::from([self.group4()[0], self.group8()[0]]))
                - (Simd32x2::from(anti_reverse.group7()[1]) * Simd32x2::from([self.group4()[1], self.group8()[1]]))
                - (Simd32x2::from(anti_reverse.group7()[2]) * Simd32x2::from([self.group4()[2], self.group8()[2]]))
                - (Simd32x2::from(self.group7()[0]) * Simd32x2::from([anti_reverse.group4()[0], anti_reverse.group8()[0]]))
                - (Simd32x2::from(self.group7()[1]) * Simd32x2::from([anti_reverse.group4()[1], anti_reverse.group8()[1]]))
                - (Simd32x2::from(self.group7()[2]) * Simd32x2::from([anti_reverse.group4()[2], anti_reverse.group8()[2]]))
                + (Simd32x2::from(anti_reverse.group1()[3]) * Simd32x2::from([self[e45], self[e1]]))
                - (Simd32x2::from(anti_reverse.group3()[3]) * Simd32x2::from([self.group6()[3], self.group3()[3]]))
                + (Simd32x2::from(self.group1()[3]) * Simd32x2::from([anti_reverse[e45], anti_reverse[e1]]))
                - (Simd32x2::from(self.group6()[0]) * Simd32x2::from([anti_reverse.group5()[0], anti_reverse.group6()[0]]))
                - (Simd32x2::from(self.group6()[1]) * Simd32x2::from([anti_reverse.group5()[1], anti_reverse.group6()[1]]))
                - (Simd32x2::from(self.group6()[2]) * Simd32x2::from([anti_reverse.group5()[2], anti_reverse.group6()[2]]))
                + (Simd32x2::from(self.group9()[1]) * Simd32x2::from([anti_reverse.group1()[0], anti_reverse.group9()[1]]))
                + (Simd32x2::from(self.group9()[2]) * Simd32x2::from([anti_reverse.group1()[1], anti_reverse.group9()[2]]))
                + (Simd32x2::from(self.group9()[3]) * Simd32x2::from([anti_reverse.group1()[2], anti_reverse.group9()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * anti_reverse.group9()[1]) + (anti_reverse.group7()[0] * self[e1]) - (anti_reverse.group7()[1] * self.group8()[2])
                    + (anti_reverse.group7()[2] * self.group8()[1])
                    - (anti_reverse.group8()[0] * self.group1()[3])
                    + (anti_reverse.group8()[1] * self.group7()[2])
                    - (anti_reverse.group8()[2] * self.group7()[1])
                    - (self.group4()[1] * anti_reverse.group3()[2])
                    + (self.group5()[1] * anti_reverse.group9()[3])
                    - (self.group7()[0] * anti_reverse[e1])
                    + (self.group8()[0] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[2] * self.group6()[1])
                    - (anti_reverse.group3()[0] * self[e45])
                    + (anti_reverse.group6()[2] * self.group1()[1])
                    + (anti_reverse.group6()[3] * self.group6()[0])),
                ((self.group0()[0] * anti_reverse.group9()[2]) + (anti_reverse.group7()[0] * self.group8()[2]) + (anti_reverse.group7()[1] * self[e1])
                    - (anti_reverse.group7()[2] * self.group8()[0])
                    - (anti_reverse.group8()[0] * self.group7()[2])
                    - (anti_reverse.group8()[1] * self.group1()[3])
                    + (anti_reverse.group8()[2] * self.group7()[0])
                    - (self.group4()[2] * anti_reverse.group3()[0])
                    + (self.group5()[2] * anti_reverse.group9()[1])
                    - (self.group7()[1] * anti_reverse[e1])
                    + (self.group8()[1] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[0] * self.group6()[2])
                    - (anti_reverse.group3()[1] * self[e45])
                    + (anti_reverse.group6()[1] * self.group6()[3])
                    + (anti_reverse.group6()[3] * self.group6()[1])),
                ((self.group0()[0] * anti_reverse.group9()[3]) - (anti_reverse.group7()[0] * self.group8()[1])
                    + (anti_reverse.group7()[1] * self.group8()[0])
                    + (anti_reverse.group7()[2] * self[e1])
                    + (anti_reverse.group8()[0] * self.group7()[1])
                    - (anti_reverse.group8()[1] * self.group7()[0])
                    - (anti_reverse.group8()[2] * self.group1()[3])
                    - (self.group4()[0] * anti_reverse.group3()[1])
                    + (self.group5()[0] * anti_reverse.group9()[2])
                    - (self.group7()[2] * anti_reverse[e1])
                    + (self.group8()[2] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[1] * self.group6()[0])
                    - (anti_reverse.group3()[2] * self[e45])
                    + (anti_reverse.group6()[2] * self.group6()[3])
                    + (anti_reverse.group6()[3] * self.group6()[2])),
                ((anti_reverse.group7()[0] * self.group1()[0]) + (anti_reverse.group7()[1] * self.group1()[1]) - (anti_reverse.group7()[1] * self.group6()[1])
                    + (anti_reverse.group7()[2] * self.group1()[2])
                    - (anti_reverse.group7()[2] * self.group6()[2])
                    + (self.group5()[2] * anti_reverse.group3()[2])
                    - (self.group7()[0] * anti_reverse.group1()[0])
                    - (self.group7()[1] * anti_reverse.group1()[1])
                    - (self.group7()[1] * anti_reverse.group6()[1])
                    - (self.group7()[2] * anti_reverse.group1()[2])
                    - (self.group7()[2] * anti_reverse.group6()[2])
                    - (anti_reverse.group1()[3] * self.group6()[3])
                    + (anti_reverse.group3()[3] * self.group9()[0])
                    - (anti_reverse.group9()[2] * self.group3()[1])
                    - (anti_reverse.group9()[3] * self.group3()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group1())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group1())
                - (Simd32x4::from(anti_reverse.group9()[0]) * Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group0()[0]]))
                + (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group3()[0]]) * swizzle!(self.group9(), 1, 2, 3, 1))
                + (Simd32x4::from([anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse.group4()[2], anti_reverse.group3()[1]]) * swizzle!(self.group9(), 0, 0, 0, 2))
                - (Simd32x4::from([anti_reverse.group4()[1], anti_reverse.group4()[2], anti_reverse.group4()[0], anti_reverse.group9()[0]]) * swizzle!(self.group3(), 2, 0, 1, 3))
                + (Simd32x4::from([anti_reverse.group4()[2], anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse.group5()[0]]) * swizzle!(self.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([anti_reverse.group5()[0], anti_reverse.group5()[1], anti_reverse.group5()[2], anti_reverse.group5()[1]]) * swizzle!(self.group3(), 3, 3, 3, 1))
                - (Simd32x4::from([anti_reverse.group5()[1], anti_reverse.group5()[2], anti_reverse.group5()[0], anti_reverse.group0()[0]]) * swizzle!(self.group9(), 3, 1, 2, 0))
                + (Simd32x4::from([anti_reverse.group5()[2], anti_reverse.group5()[0], anti_reverse.group5()[1], anti_reverse.group3()[2]]) * swizzle!(self.group9(), 2, 3, 1, 3))
                + (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group5()[0], self.group5()[1], self.group5()[2], self.group5()[1]]) * swizzle!(anti_reverse.group3(), 3, 3, 3, 1))
                - (Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group3()[0]]) * swizzle!(anti_reverse.group9(), 2, 3, 1, 1))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group7()[0]]) * swizzle!(self.group6(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group7()[0]]) * swizzle!(anti_reverse.group6(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group6()[3], self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(anti_reverse.group6(), 0, 0, 1, 3))
                + (Simd32x4::from([anti_reverse[e45], anti_reverse[e45], anti_reverse[e45], anti_reverse.group5()[2]]) * swizzle!(self.group3(), 0, 1, 2, 2))),
            // e5
            (-(anti_reverse.group0()[0] * self[e45]) + (anti_reverse.group0()[1] * self[e1]) - (self.group0()[0] * anti_reverse[e45])
                + (self.group0()[1] * anti_reverse[e1])
                + (anti_reverse.group4()[0] * self.group5()[0])
                - (anti_reverse.group4()[0] * self.group9()[1])
                + (anti_reverse.group4()[1] * self.group5()[1])
                - (anti_reverse.group4()[1] * self.group9()[2])
                + (anti_reverse.group4()[2] * self.group5()[2])
                - (anti_reverse.group4()[2] * self.group9()[3])
                + (anti_reverse.group5()[0] * self.group4()[0])
                + (anti_reverse.group5()[1] * self.group4()[1])
                + (anti_reverse.group5()[2] * self.group4()[2])
                - (anti_reverse.group8()[0] * self.group1()[0])
                - (anti_reverse.group8()[0] * self.group6()[0])
                - (anti_reverse.group8()[1] * self.group1()[1])
                - (anti_reverse.group8()[1] * self.group6()[1])
                - (anti_reverse.group8()[2] * self.group1()[2])
                - (anti_reverse.group8()[2] * self.group6()[2])
                + (self.group4()[0] * anti_reverse.group9()[1])
                + (self.group4()[1] * anti_reverse.group9()[2])
                + (self.group4()[2] * anti_reverse.group9()[3])
                + (self.group8()[0] * anti_reverse.group1()[0])
                - (self.group8()[0] * anti_reverse.group6()[0])
                + (self.group8()[1] * anti_reverse.group1()[1])
                - (self.group8()[1] * anti_reverse.group6()[1])
                + (self.group8()[2] * anti_reverse.group1()[2])
                - (self.group8()[2] * anti_reverse.group6()[2])
                - (anti_reverse.group3()[3] * self[e45])
                - (anti_reverse.group6()[3] * self[e1])
                + (self.group3()[3] * anti_reverse[e45])
                + (self.group6()[3] * anti_reverse[e1])),
            // e41, e42, e43, e45
            (Simd32x4::from([
                ((anti_reverse.group0()[0] * self.group7()[0])
                    + (self.group0()[0] * anti_reverse.group7()[0])
                    + (anti_reverse.group5()[2] * self.group7()[1])
                    + (anti_reverse.group7()[0] * self.group3()[3])
                    - (anti_reverse.group7()[1] * self.group5()[2])
                    + (anti_reverse.group7()[2] * self.group5()[1])
                    + (self.group7()[2] * anti_reverse.group9()[2])
                    + (anti_reverse.group1()[3] * self.group9()[1])
                    - (anti_reverse.group3()[2] * self.group1()[1])
                    + (anti_reverse.group6()[0] * self.group9()[0])
                    + (anti_reverse.group6()[2] * self.group3()[1])
                    + (anti_reverse.group6()[3] * self.group3()[0])
                    + (anti_reverse.group9()[0] * self.group6()[0])),
                ((anti_reverse.group0()[0] * self.group7()[1])
                    + (self.group0()[0] * anti_reverse.group7()[1])
                    + (anti_reverse.group5()[0] * self.group7()[2])
                    + (anti_reverse.group7()[0] * self.group5()[2])
                    + (anti_reverse.group7()[1] * self.group3()[3])
                    - (anti_reverse.group7()[2] * self.group5()[0])
                    + (self.group7()[0] * anti_reverse.group9()[3])
                    + (anti_reverse.group1()[3] * self.group9()[2])
                    - (anti_reverse.group3()[0] * self.group1()[2])
                    + (anti_reverse.group6()[0] * self.group3()[2])
                    + (anti_reverse.group6()[1] * self.group9()[0])
                    + (anti_reverse.group6()[3] * self.group3()[1])
                    + (anti_reverse.group9()[0] * self.group6()[1])),
                ((anti_reverse.group0()[0] * self.group7()[2]) + (self.group0()[0] * anti_reverse.group7()[2]) + (anti_reverse.group5()[1] * self.group7()[0])
                    - (anti_reverse.group7()[0] * self.group5()[1])
                    + (anti_reverse.group7()[1] * self.group5()[0])
                    + (anti_reverse.group7()[2] * self.group3()[3])
                    + (self.group7()[1] * anti_reverse.group9()[1])
                    + (anti_reverse.group1()[3] * self.group9()[3])
                    - (anti_reverse.group3()[1] * self.group1()[0])
                    + (anti_reverse.group6()[1] * self.group3()[0])
                    + (anti_reverse.group6()[2] * self.group9()[0])
                    + (anti_reverse.group6()[3] * self.group3()[2])
                    + (anti_reverse.group9()[0] * self.group6()[2])),
                ((anti_reverse.group5()[1] * self.group1()[1])
                    + (anti_reverse.group5()[2] * self.group1()[2])
                    + (anti_reverse.group7()[0] * self.group4()[0])
                    + (anti_reverse.group7()[1] * self.group4()[1])
                    + (anti_reverse.group7()[2] * self.group4()[2])
                    - (anti_reverse.group8()[1] * self.group3()[1])
                    - (anti_reverse.group8()[2] * self.group3()[2])
                    + (self.group8()[2] * anti_reverse.group3()[2])
                    - (anti_reverse.group1()[3] * self[e45])
                    - (anti_reverse.group6()[1] * self.group9()[2])
                    - (anti_reverse.group6()[2] * self.group9()[3])
                    - (anti_reverse.group9()[3] * self.group6()[2])
                    + (self.group1()[3] * anti_reverse[e45])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group3())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group3())
                - (Simd32x4::from(self.group7()[0]) * Simd32x4::from([anti_reverse.group3()[3], anti_reverse.group5()[2], anti_reverse.group9()[2], anti_reverse.group4()[0]]))
                - (Simd32x4::from(self.group7()[1]) * Simd32x4::from([anti_reverse.group9()[3], anti_reverse.group3()[3], anti_reverse.group5()[0], anti_reverse.group4()[1]]))
                - (Simd32x4::from(self.group7()[2]) * Simd32x4::from([anti_reverse.group5()[1], anti_reverse.group9()[1], anti_reverse.group3()[3], anti_reverse.group4()[2]]))
                - (Simd32x4::from(anti_reverse.group9()[0]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self[e1]]))
                + (Simd32x4::from([anti_reverse.group5()[0], anti_reverse.group5()[1], anti_reverse.group5()[2], anti_reverse.group5()[0]]) * swizzle!(self.group1(), 3, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group7()[1], anti_reverse.group7()[2], anti_reverse.group7()[0], anti_reverse.group6()[0]]) * swizzle!(self.group9(), 3, 1, 2, 1))
                + (Simd32x4::from([anti_reverse.group7()[2], anti_reverse.group7()[0], anti_reverse.group7()[1], anti_reverse[e1]]) * swizzle!(self.group9(), 2, 3, 1, 0))
                + (Simd32x4::from([self.group5()[0], self.group5()[1], self.group5()[2], self.group5()[0]]) * swizzle!(anti_reverse.group1(), 3, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group8()[0]]) * swizzle!(self.group3(), 2, 0, 1, 0))
                - (Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group3()[0], anti_reverse.group0()[0]]) * swizzle!(self.group6(), 3, 3, 1, 3))
                - (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[2], anti_reverse.group9()[1]]) * swizzle!(self.group6(), 2, 0, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group6()[2], self.group1()[1], self.group8()[0]]) * swizzle!(anti_reverse.group3(), 1, 0, 0, 0))
                - (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group6()[1]]) * swizzle!(anti_reverse.group9(), 1, 2, 3, 2))
                + (Simd32x4::from([self.group3()[1], self.group9()[0], self.group9()[0], self.group5()[2]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 2))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]) * swizzle!(anti_reverse.group6(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group6()[1], self.group1()[0], self.group6()[0], self.group8()[1]]) * swizzle!(anti_reverse.group3(), 2, 2, 1, 1))
                + (Simd32x4::from([self.group9()[0], self.group3()[2], self.group3()[0], self.group5()[1]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 1))),
            // e15, e25, e35
            ((Simd32x3::from(anti_reverse.group0()[0]) * self.group8())
                + (Simd32x3::from(anti_reverse.group0()[1]) * self.group4())
                + (Simd32x3::from(self.group0()[0]) * anti_reverse.group8())
                + (Simd32x3::from(self.group0()[1]) * anti_reverse.group4())
                - (Simd32x3::from(anti_reverse.group8()[0]) * Simd32x3::from([self.group3()[3], self.group9()[3], self.group5()[1]]))
                - (Simd32x3::from(anti_reverse.group8()[1]) * Simd32x3::from([self.group5()[2], self.group3()[3], self.group9()[1]]))
                - (Simd32x3::from(anti_reverse.group8()[2]) * Simd32x3::from([self.group9()[2], self.group5()[0], self.group3()[3]]))
                - (Simd32x3::from(self.group4()[0]) * Simd32x3::from([anti_reverse.group6()[3], anti_reverse.group6()[2], anti_reverse.group1()[1]]))
                - (Simd32x3::from(self.group4()[1]) * Simd32x3::from([anti_reverse.group1()[2], anti_reverse.group6()[3], anti_reverse.group6()[0]]))
                - (Simd32x3::from(self.group4()[2]) * Simd32x3::from([anti_reverse.group6()[1], anti_reverse.group1()[0], anti_reverse.group6()[3]]))
                - (Simd32x3::from(anti_reverse[e1]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                + (Simd32x3::from(anti_reverse[e1]) * self.group5())
                + (Simd32x3::from(anti_reverse[e45]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(anti_reverse[e45]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (Simd32x3::from(self[e1]) * Simd32x3::from([anti_reverse.group9()[1], anti_reverse.group9()[2], anti_reverse.group9()[3]]))
                - (Simd32x3::from(self[e45]) * Simd32x3::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2]]))
                + (Simd32x3::from(self[e45]) * Simd32x3::from([anti_reverse.group6()[0], anti_reverse.group6()[1], anti_reverse.group6()[2]]))
                + (Simd32x3::from([self.group5()[1], self.group9()[1], self.group5()[0]]) * swizzle!(anti_reverse.group8(), 2, 2, 1))
                + (Simd32x3::from([self.group8()[1], self[e1], self[e1]]) * swizzle!(anti_reverse.group5(), 2, 1, 2))
                + (Simd32x3::from([anti_reverse.group1()[1], anti_reverse.group6()[0], anti_reverse.group1()[0]]) * swizzle!(self.group4(), 2, 2, 1))
                + (Simd32x3::from([anti_reverse.group3()[3], anti_reverse.group3()[3], anti_reverse.group9()[2]]) * swizzle!(self.group8(), 0, 1, 0))
                + (Simd32x3::from([anti_reverse.group6()[2], anti_reverse.group1()[2], anti_reverse.group6()[1]]) * swizzle!(self.group4(), 1, 0, 0))
                - (Simd32x3::from([anti_reverse.group9()[2], anti_reverse.group9()[3], anti_reverse.group9()[1]]) * swizzle!(self.group8(), 2, 0, 1))
                + (Simd32x3::from([anti_reverse.group9()[3], anti_reverse.group9()[1], anti_reverse.group3()[3]]) * swizzle!(self.group8(), 1, 2, 2))
                + (Simd32x3::from([self.group1()[1], self.group6()[2], self.group6()[0]]) * swizzle!(anti_reverse.group4(), 2, 0, 1))
                - (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group4(), 1, 2, 0))
                + (Simd32x3::from([self.group6()[1], self.group6()[3], self.group6()[3]]) * swizzle!(anti_reverse.group4(), 2, 1, 2))
                - (Simd32x3::from([self.group6()[2], self.group6()[0], self.group6()[1]]) * swizzle!(anti_reverse.group4(), 1, 2, 0))
                + (Simd32x3::from([self.group6()[3], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group4(), 0, 0, 1))
                + (Simd32x3::from([self.group9()[3], self.group5()[2], self.group9()[2]]) * swizzle!(anti_reverse.group8(), 1, 0, 0))
                + (Simd32x3::from([self[e1], self.group8()[2], self.group8()[0]]) * swizzle!(anti_reverse.group5(), 0, 0, 1))
                - (swizzle!(anti_reverse.group5(), 1, 2, 0) * swizzle!(self.group8(), 2, 0, 1))),
            // e23, e31, e12
            (Simd32x3::from([
                ((anti_reverse.group1()[1] * self.group9()[3]) - (anti_reverse.group1()[2] * self.group9()[2]) + (anti_reverse.group9()[2] * self.group1()[2])
                    - (anti_reverse.group9()[3] * self.group1()[1])),
                (-(anti_reverse.group1()[0] * self.group9()[3]) + (anti_reverse.group1()[2] * self.group9()[1]) - (anti_reverse.group9()[1] * self.group1()[2])
                    + (anti_reverse.group9()[3] * self.group1()[0])),
                ((anti_reverse.group1()[0] * self.group9()[2]) - (anti_reverse.group1()[1] * self.group9()[1]) + (anti_reverse.group9()[1] * self.group1()[1])
                    - (anti_reverse.group9()[2] * self.group1()[0])),
            ]) + (Simd32x3::from(anti_reverse.group0()[0]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (Simd32x3::from(anti_reverse.group0()[1]) * self.group5())
                + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([anti_reverse.group6()[0], anti_reverse.group6()[1], anti_reverse.group6()[2]]))
                + (Simd32x3::from(self.group0()[1]) * anti_reverse.group5())
                + (Simd32x3::from(anti_reverse.group1()[3]) * self.group4())
                - (Simd32x3::from(anti_reverse.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(anti_reverse.group6()[3]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                - (Simd32x3::from(self.group3()[3]) * Simd32x3::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2]]))
                - (Simd32x3::from(self.group6()[3]) * Simd32x3::from([anti_reverse.group9()[1], anti_reverse.group9()[2], anti_reverse.group9()[3]]))
                + (Simd32x3::from(anti_reverse[e1]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                + (Simd32x3::from(anti_reverse[e45]) * self.group7())
                + (Simd32x3::from(self[e1]) * Simd32x3::from([anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group3()[2]]))
                + (Simd32x3::from([self.group4()[1], self[e45], self[e45]]) * swizzle!(anti_reverse.group7(), 2, 1, 2))
                + (Simd32x3::from([self.group7()[1], self.group1()[3], self.group1()[3]]) * swizzle!(anti_reverse.group4(), 2, 1, 2))
                - (Simd32x3::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0]]) * swizzle!(self.group8(), 2, 0, 1))
                + (Simd32x3::from([anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group9()[0]]) * swizzle!(self.group8(), 1, 2, 2))
                - (Simd32x3::from([anti_reverse.group6()[1], anti_reverse.group6()[2], anti_reverse.group6()[0]]) * swizzle!(self.group5(), 2, 0, 1))
                + (Simd32x3::from([anti_reverse.group6()[2], anti_reverse.group6()[0], anti_reverse.group6()[1]]) * swizzle!(self.group5(), 1, 2, 0))
                + (Simd32x3::from([anti_reverse.group9()[0], anti_reverse.group9()[0], anti_reverse.group3()[1]]) * swizzle!(self.group8(), 0, 1, 0))
                + (Simd32x3::from([self.group1()[3], self.group7()[2], self.group7()[0]]) * swizzle!(anti_reverse.group4(), 0, 0, 1))
                + (Simd32x3::from([self.group3()[1], self.group9()[0], self.group9()[0]]) * swizzle!(anti_reverse.group8(), 2, 1, 2))
                - (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * swizzle!(anti_reverse.group8(), 1, 2, 0))
                + (Simd32x3::from([self.group6()[1], self.group6()[2], self.group6()[0]]) * swizzle!(anti_reverse.group5(), 2, 0, 1))
                - (Simd32x3::from([self.group6()[2], self.group6()[0], self.group6()[1]]) * swizzle!(anti_reverse.group5(), 1, 2, 0))
                + (Simd32x3::from([self.group9()[0], self.group3()[2], self.group3()[0]]) * swizzle!(anti_reverse.group8(), 0, 0, 1))
                + (Simd32x3::from([self[e45], self.group4()[2], self.group4()[0]]) * swizzle!(anti_reverse.group7(), 0, 0, 1))
                - (swizzle!(anti_reverse.group4(), 1, 2, 0) * swizzle!(self.group7(), 2, 0, 1))
                - (swizzle!(anti_reverse.group7(), 1, 2, 0) * swizzle!(self.group4(), 2, 0, 1))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group5()[0]) - (self.group0()[0] * anti_reverse.group5()[0]) + (anti_reverse.group5()[1] * self.group5()[2])
                    - (anti_reverse.group5()[2] * self.group5()[1])
                    - (anti_reverse.group7()[1] * self.group8()[2])
                    + (anti_reverse.group7()[2] * self.group8()[1])
                    + (anti_reverse.group8()[0] * self.group1()[3])
                    - (anti_reverse.group8()[1] * self.group7()[2])
                    + (anti_reverse.group8()[2] * self.group7()[1])
                    - (self.group4()[1] * anti_reverse.group3()[2])
                    + (self.group7()[0] * anti_reverse[e1])
                    + (self.group8()[0] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[0] * self.group6()[3])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    + (anti_reverse.group6()[2] * self.group6()[1])
                    + (anti_reverse.group6()[3] * self.group1()[0])),
                (-(anti_reverse.group0()[0] * self.group5()[1]) - (self.group0()[0] * anti_reverse.group5()[1]) - (anti_reverse.group5()[0] * self.group5()[2])
                    + (anti_reverse.group5()[2] * self.group5()[0])
                    + (anti_reverse.group7()[0] * self.group8()[2])
                    - (anti_reverse.group7()[2] * self.group8()[0])
                    + (anti_reverse.group8()[0] * self.group7()[2])
                    + (anti_reverse.group8()[1] * self.group1()[3])
                    - (anti_reverse.group8()[2] * self.group7()[0])
                    - (self.group4()[2] * anti_reverse.group3()[0])
                    + (self.group7()[1] * anti_reverse[e1])
                    + (self.group8()[1] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[0] * self.group1()[2])
                    + (anti_reverse.group1()[1] * self.group6()[3])
                    + (anti_reverse.group6()[0] * self.group6()[2])
                    + (anti_reverse.group6()[3] * self.group1()[1])),
                (-(anti_reverse.group0()[0] * self.group5()[2]) - (self.group0()[0] * anti_reverse.group5()[2]) + (anti_reverse.group5()[0] * self.group5()[1])
                    - (anti_reverse.group5()[1] * self.group5()[0])
                    - (anti_reverse.group7()[0] * self.group8()[1])
                    + (anti_reverse.group7()[1] * self.group8()[0])
                    - (anti_reverse.group8()[0] * self.group7()[1])
                    + (anti_reverse.group8()[1] * self.group7()[0])
                    + (anti_reverse.group8()[2] * self.group1()[3])
                    - (self.group4()[0] * anti_reverse.group3()[1])
                    + (self.group7()[2] * anti_reverse[e1])
                    + (self.group8()[2] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[2] * self.group6()[3])
                    + (anti_reverse.group6()[1] * self.group6()[0])
                    + (anti_reverse.group6()[3] * self.group1()[2])),
                (-(anti_reverse.group4()[2] * self.group3()[2])
                    - (anti_reverse.group5()[2] * self.group9()[3])
                    - (anti_reverse.group7()[0] * self.group8()[0])
                    - (anti_reverse.group7()[1] * self.group8()[1])
                    - (anti_reverse.group7()[2] * self.group8()[2])
                    + (anti_reverse.group8()[0] * self.group7()[0])
                    + (anti_reverse.group8()[1] * self.group7()[1])
                    + (anti_reverse.group8()[2] * self.group7()[2])
                    + (self.group4()[0] * anti_reverse.group3()[0])
                    + (self.group4()[1] * anti_reverse.group3()[1])
                    + (self.group4()[2] * anti_reverse.group3()[2])
                    - (anti_reverse.group1()[2] * self.group6()[2])
                    - (anti_reverse.group6()[0] * self.group1()[0])
                    - (anti_reverse.group6()[1] * self.group1()[1])
                    - (anti_reverse.group6()[2] * self.group1()[2])
                    - (self.group1()[3] * anti_reverse[e1])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group6())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group6())
                + (Simd32x4::from(self[e1]) * Simd32x4::from([anti_reverse.group7()[0], anti_reverse.group7()[1], anti_reverse.group7()[2], anti_reverse.group1()[3]]))
                - (Simd32x4::from(self[e45]) * Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group9()[0]]))
                - (Simd32x4::from([anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse.group4()[2], anti_reverse.group5()[0]]) * swizzle!(self.group9(), 0, 0, 0, 1))
                + (Simd32x4::from([anti_reverse.group4()[1], anti_reverse.group4()[2], anti_reverse.group4()[0], anti_reverse.group0()[0]]) * swizzle!(self.group3(), 2, 0, 1, 3))
                - (Simd32x4::from([anti_reverse.group4()[2], anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse.group4()[0]]) * swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group5()[0]]) * swizzle!(anti_reverse.group9(), 0, 0, 0, 1))
                + (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group0()[0]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 3))
                - (Simd32x4::from([anti_reverse.group3()[3], anti_reverse.group3()[3], anti_reverse.group3()[3], anti_reverse.group5()[1]]) * swizzle!(self.group9(), 1, 2, 3, 2))
                - (Simd32x4::from([anti_reverse.group6()[1], anti_reverse.group6()[2], anti_reverse.group6()[0], anti_reverse.group1()[1]]) * swizzle!(self.group6(), 2, 0, 1, 1))
                + (Simd32x4::from([anti_reverse.group9()[2], anti_reverse.group9()[3], anti_reverse.group9()[1], anti_reverse[e45]]) * swizzle!(self.group9(), 3, 1, 2, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group6()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group3()[3], self.group9()[3], self.group9()[1], self.group5()[1]]) * swizzle!(anti_reverse.group9(), 1, 1, 2, 2))
                - (Simd32x4::from([self.group9()[2], self.group3()[3], self.group3()[3], self.group5()[2]]) * swizzle!(anti_reverse.group9(), 3, 2, 3, 3))
                - (Simd32x4::from([anti_reverse[e45], anti_reverse[e45], anti_reverse[e45], anti_reverse.group4()[1]]) * swizzle!(self.group3(), 0, 1, 2, 1))),
            // e423, e431, e412
            (Simd32x3::from([
                ((anti_reverse.group3()[1] * self.group9()[3]) - (anti_reverse.group3()[2] * self.group9()[2]) - (anti_reverse.group9()[2] * self.group3()[2])
                    + (anti_reverse.group9()[3] * self.group3()[1])),
                (-(anti_reverse.group3()[0] * self.group9()[3]) + (anti_reverse.group3()[2] * self.group9()[1]) + (anti_reverse.group9()[1] * self.group3()[2])
                    - (anti_reverse.group9()[3] * self.group3()[0])),
                ((anti_reverse.group3()[0] * self.group9()[2]) - (anti_reverse.group3()[1] * self.group9()[1]) - (anti_reverse.group9()[1] * self.group3()[1])
                    + (anti_reverse.group9()[2] * self.group3()[0])),
            ]) - (Simd32x3::from(anti_reverse.group0()[0]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                + (Simd32x3::from(anti_reverse.group0()[1]) * self.group7())
                - (Simd32x3::from(self.group0()[0]) * Simd32x3::from([anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group3()[2]]))
                + (Simd32x3::from(self.group0()[1]) * anti_reverse.group7())
                - (Simd32x3::from(anti_reverse.group7()[0]) * Simd32x3::from([self.group6()[3], self.group1()[2], self.group6()[1]]))
                - (Simd32x3::from(anti_reverse.group7()[1]) * Simd32x3::from([self.group6()[2], self.group6()[3], self.group1()[0]]))
                - (Simd32x3::from(anti_reverse.group7()[2]) * Simd32x3::from([self.group1()[1], self.group6()[0], self.group6()[3]]))
                - (Simd32x3::from(anti_reverse.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(anti_reverse.group1()[3]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (Simd32x3::from(anti_reverse.group3()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                - (Simd32x3::from(anti_reverse.group9()[0]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([anti_reverse.group6()[0], anti_reverse.group6()[1], anti_reverse.group6()[2]]))
                - (Simd32x3::from(self.group3()[3]) * Simd32x3::from([anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group3()[2]]))
                + (Simd32x3::from(self.group9()[0]) * Simd32x3::from([anti_reverse.group9()[1], anti_reverse.group9()[2], anti_reverse.group9()[3]]))
                - (Simd32x3::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group7(), 2, 0, 1))
                + (Simd32x3::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group6()[1]]) * swizzle!(self.group7(), 1, 2, 0))
                + (Simd32x3::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0]]) * swizzle!(self.group5(), 2, 0, 1))
                - (Simd32x3::from([anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group9()[0]]) * swizzle!(self.group5(), 1, 2, 2))
                - (Simd32x3::from([anti_reverse.group6()[1], anti_reverse.group6()[2], anti_reverse.group6()[0]]) * swizzle!(self.group7(), 2, 0, 1))
                + (Simd32x3::from([anti_reverse.group6()[2], anti_reverse.group6()[0], anti_reverse.group6()[3]]) * swizzle!(self.group7(), 1, 2, 2))
                + (Simd32x3::from([anti_reverse.group6()[3], anti_reverse.group6()[3], anti_reverse.group1()[1]]) * swizzle!(self.group7(), 0, 1, 0))
                - (Simd32x3::from([anti_reverse.group9()[0], anti_reverse.group9()[0], anti_reverse.group3()[1]]) * swizzle!(self.group5(), 0, 1, 0))
                + (Simd32x3::from([self.group1()[2], self.group6()[2], self.group1()[1]]) * swizzle!(anti_reverse.group7(), 1, 0, 0))
                - (Simd32x3::from([self.group3()[1], self.group9()[0], self.group9()[0]]) * swizzle!(anti_reverse.group5(), 2, 1, 2))
                + (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * swizzle!(anti_reverse.group5(), 1, 2, 0))
                + (Simd32x3::from([self.group6()[1], self.group1()[0], self.group6()[0]]) * swizzle!(anti_reverse.group7(), 2, 2, 1))
                - (Simd32x3::from([self.group9()[0], self.group3()[2], self.group3()[0]]) * swizzle!(anti_reverse.group5(), 0, 0, 1))),
            // e235, e315, e125
            (-(Simd32x3::from(anti_reverse.group0()[0]) * self.group4()) + (Simd32x3::from(anti_reverse.group0()[1]) * self.group8())
                - (Simd32x3::from(self.group0()[0]) * anti_reverse.group4())
                + (Simd32x3::from(self.group0()[1]) * anti_reverse.group8())
                + (Simd32x3::from(anti_reverse.group4()[0]) * Simd32x3::from([self.group3()[3], self.group9()[3], self.group5()[1]]))
                + (Simd32x3::from(anti_reverse.group4()[1]) * Simd32x3::from([self.group5()[2], self.group3()[3], self.group9()[1]]))
                + (Simd32x3::from(anti_reverse.group4()[2]) * Simd32x3::from([self.group9()[2], self.group5()[0], self.group3()[3]]))
                - (Simd32x3::from(self.group8()[0]) * Simd32x3::from([anti_reverse.group6()[3], anti_reverse.group6()[2], anti_reverse.group1()[1]]))
                - (Simd32x3::from(self.group8()[1]) * Simd32x3::from([anti_reverse.group1()[2], anti_reverse.group6()[3], anti_reverse.group6()[0]]))
                - (Simd32x3::from(self.group8()[2]) * Simd32x3::from([anti_reverse.group6()[1], anti_reverse.group1()[0], anti_reverse.group6()[3]]))
                + (Simd32x3::from(anti_reverse[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(anti_reverse[e1]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (Simd32x3::from(anti_reverse[e45]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                - (Simd32x3::from(anti_reverse[e45]) * self.group5())
                - (Simd32x3::from(self[e1]) * Simd32x3::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2]]))
                + (Simd32x3::from(self[e1]) * Simd32x3::from([anti_reverse.group6()[0], anti_reverse.group6()[1], anti_reverse.group6()[2]]))
                - (Simd32x3::from(self[e45]) * Simd32x3::from([anti_reverse.group9()[1], anti_reverse.group9()[2], anti_reverse.group9()[3]]))
                - (Simd32x3::from([self.group4()[1], self[e45], self[e45]]) * swizzle!(anti_reverse.group5(), 2, 1, 2))
                - (Simd32x3::from([self.group5()[1], self.group9()[1], self.group5()[0]]) * swizzle!(anti_reverse.group4(), 2, 2, 1))
                + (Simd32x3::from([anti_reverse.group1()[1], anti_reverse.group6()[0], anti_reverse.group1()[0]]) * swizzle!(self.group8(), 2, 2, 1))
                - (Simd32x3::from([anti_reverse.group3()[3], anti_reverse.group3()[3], anti_reverse.group9()[2]]) * swizzle!(self.group4(), 0, 1, 0))
                + (Simd32x3::from([anti_reverse.group6()[2], anti_reverse.group1()[2], anti_reverse.group6()[1]]) * swizzle!(self.group8(), 1, 0, 0))
                + (Simd32x3::from([anti_reverse.group9()[2], anti_reverse.group9()[3], anti_reverse.group9()[1]]) * swizzle!(self.group4(), 2, 0, 1))
                - (Simd32x3::from([anti_reverse.group9()[3], anti_reverse.group9()[1], anti_reverse.group3()[3]]) * swizzle!(self.group4(), 1, 2, 2))
                + (Simd32x3::from([self.group1()[1], self.group6()[2], self.group6()[0]]) * swizzle!(anti_reverse.group8(), 2, 0, 1))
                - (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group8(), 1, 2, 0))
                + (Simd32x3::from([self.group6()[1], self.group6()[3], self.group6()[3]]) * swizzle!(anti_reverse.group8(), 2, 1, 2))
                - (Simd32x3::from([self.group6()[2], self.group6()[0], self.group6()[1]]) * swizzle!(anti_reverse.group8(), 1, 2, 0))
                + (Simd32x3::from([self.group6()[3], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group8(), 0, 0, 1))
                - (Simd32x3::from([self.group9()[3], self.group5()[2], self.group9()[2]]) * swizzle!(anti_reverse.group4(), 1, 0, 0))
                - (Simd32x3::from([self[e45], self.group4()[2], self.group4()[0]]) * swizzle!(anti_reverse.group5(), 0, 0, 1))
                + (swizzle!(anti_reverse.group5(), 1, 2, 0) * swizzle!(self.group4(), 2, 0, 1))),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([
                (-(anti_reverse.group5()[0] * self.group7()[0])
                    - (anti_reverse.group5()[1] * self.group7()[1])
                    - (anti_reverse.group5()[2] * self.group7()[2])
                    - (anti_reverse.group7()[0] * self.group5()[0])
                    - (anti_reverse.group7()[1] * self.group5()[1])
                    - (anti_reverse.group7()[1] * self.group9()[2])
                    - (anti_reverse.group7()[2] * self.group5()[2])
                    - (anti_reverse.group7()[2] * self.group9()[3])
                    + (self.group7()[1] * anti_reverse.group9()[2])
                    + (self.group7()[2] * anti_reverse.group9()[3])
                    - (anti_reverse.group6()[1] * self.group3()[1])
                    - (anti_reverse.group6()[2] * self.group3()[2])),
                (-(anti_reverse.group4()[1] * self.group7()[2]) + (anti_reverse.group4()[2] * self.group7()[1])
                    - (anti_reverse.group5()[2] * self.group1()[1])
                    - (anti_reverse.group7()[0] * self[e45])
                    + (anti_reverse.group7()[1] * self.group4()[2])
                    - (anti_reverse.group7()[2] * self.group4()[1])
                    - (self.group5()[1] * anti_reverse.group1()[2])
                    + (self.group7()[0] * anti_reverse[e45])
                    + (anti_reverse.group6()[0] * self.group3()[3])
                    + (anti_reverse.group6()[2] * self.group9()[2])
                    - (anti_reverse.group9()[2] * self.group6()[2])
                    + (self.group3()[0] * anti_reverse[e1])),
                ((anti_reverse.group4()[0] * self.group7()[2])
                    - (anti_reverse.group4()[2] * self.group7()[0])
                    - (anti_reverse.group5()[0] * self.group1()[2])
                    - (anti_reverse.group7()[0] * self.group4()[2])
                    - (anti_reverse.group7()[1] * self[e45])
                    + (anti_reverse.group7()[2] * self.group4()[0])
                    - (self.group5()[2] * anti_reverse.group1()[0])
                    + (self.group7()[1] * anti_reverse[e45])
                    + (anti_reverse.group6()[0] * self.group9()[3])
                    + (anti_reverse.group6()[1] * self.group3()[3])
                    - (anti_reverse.group9()[3] * self.group6()[0])
                    + (self.group3()[1] * anti_reverse[e1])),
                (-(anti_reverse.group4()[0] * self.group7()[1]) + (anti_reverse.group4()[1] * self.group7()[0]) - (anti_reverse.group5()[1] * self.group1()[0])
                    + (anti_reverse.group7()[0] * self.group4()[1])
                    - (anti_reverse.group7()[1] * self.group4()[0])
                    - (anti_reverse.group7()[2] * self[e45])
                    - (self.group5()[0] * anti_reverse.group1()[1])
                    + (self.group7()[2] * anti_reverse[e45])
                    + (anti_reverse.group6()[1] * self.group9()[1])
                    + (anti_reverse.group6()[2] * self.group3()[3])
                    - (anti_reverse.group9()[1] * self.group6()[1])
                    + (self.group3()[2] * anti_reverse[e1])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group9())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group9())
                - (Simd32x4::from(anti_reverse.group9()[0]) * Simd32x4::from([self.group6()[3], self.group8()[0], self.group8()[1], self.group8()[2]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse.group4()[2]]))
                + (Simd32x4::from(self.group9()[0]) * Simd32x4::from([anti_reverse.group6()[3], anti_reverse.group8()[0], anti_reverse.group8()[1], anti_reverse.group8()[2]]))
                + (Simd32x4::from([self.group0()[0], self.group5()[2], self.group5()[0], self.group5()[1]]) * swizzle!(anti_reverse.group1(), 3, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group7()[0], anti_reverse.group6()[1], anti_reverse.group6()[2], anti_reverse.group6()[0]]) * swizzle!(self.group9(), 1, 3, 1, 2))
                + (Simd32x4::from([self.group7()[0], self.group6()[1], self.group6()[2], self.group6()[0]]) * swizzle!(anti_reverse.group9(), 1, 3, 1, 2))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group8()[1], anti_reverse.group8()[2], anti_reverse.group8()[0]]) * swizzle!(self.group3(), 1, 2, 0, 1))
                + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group8()[2], anti_reverse.group8()[0], anti_reverse.group8()[1]]) * swizzle!(self.group3(), 3, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group5()[0], anti_reverse.group5()[1], anti_reverse.group5()[2]]) * swizzle!(self.group6(), 0, 3, 3, 3))
                + (Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group5()[1], anti_reverse.group5()[2], anti_reverse.group5()[0]]) * swizzle!(self.group1(), 0, 2, 0, 1))
                - (Simd32x4::from([anti_reverse.group3()[3], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 3, 0, 1, 2))
                + (Simd32x4::from([self.group1()[1], self.group8()[2], self.group8()[0], self.group8()[1]]) * swizzle!(anti_reverse.group3(), 1, 1, 2, 0))
                + (Simd32x4::from([self.group1()[2], self.group6()[0], self.group6()[1], self.group6()[2]]) * swizzle!(anti_reverse.group3(), 2, 3, 3, 3))
                - (Simd32x4::from([self.group3()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 2))
                - (Simd32x4::from([self.group3()[0], self.group5()[0], self.group5()[1], self.group5()[2]]) * swizzle!(anti_reverse.group6(), 0, 3, 3, 3))
                - (Simd32x4::from([self.group3()[2], self.group4()[0], self.group4()[1], self.group4()[2]]) * swizzle!(anti_reverse.group1(), 2, 3, 3, 3))
                - (Simd32x4::from([self.group6()[1], self.group8()[1], self.group8()[2], self.group8()[0]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group6()[2], self[e1], self[e1], self[e1]]) * swizzle!(anti_reverse.group3(), 2, 0, 1, 2))),
            // e3215
            ((anti_reverse.group0()[0] * self[e1]) + (anti_reverse.group0()[1] * self[e45]) + (self.group0()[0] * anti_reverse[e1]) + (self.group0()[1] * anti_reverse[e45])
                - (anti_reverse.group4()[0] * self.group1()[0])
                - (anti_reverse.group4()[0] * self.group6()[0])
                - (anti_reverse.group4()[1] * self.group1()[1])
                - (anti_reverse.group4()[1] * self.group6()[1])
                - (anti_reverse.group4()[2] * self.group1()[2])
                - (anti_reverse.group4()[2] * self.group6()[2])
                - (anti_reverse.group5()[0] * self.group8()[0])
                - (anti_reverse.group5()[1] * self.group8()[1])
                - (anti_reverse.group5()[2] * self.group8()[2])
                - (anti_reverse.group8()[0] * self.group5()[0])
                + (anti_reverse.group8()[0] * self.group9()[1])
                - (anti_reverse.group8()[1] * self.group5()[1])
                + (anti_reverse.group8()[1] * self.group9()[2])
                - (anti_reverse.group8()[2] * self.group5()[2])
                + (anti_reverse.group8()[2] * self.group9()[3])
                + (self.group4()[0] * anti_reverse.group1()[0])
                - (self.group4()[0] * anti_reverse.group6()[0])
                + (self.group4()[1] * anti_reverse.group1()[1])
                - (self.group4()[1] * anti_reverse.group6()[1])
                + (self.group4()[2] * anti_reverse.group1()[2])
                - (self.group4()[2] * anti_reverse.group6()[2])
                - (self.group8()[0] * anti_reverse.group9()[1])
                - (self.group8()[1] * anti_reverse.group9()[2])
                - (self.group8()[2] * anti_reverse.group9()[3])
                + (anti_reverse.group3()[3] * self[e1])
                - (anti_reverse.group6()[3] * self[e45])
                - (self.group3()[3] * anti_reverse[e1])
                + (self.group6()[3] * anti_reverse[e45])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group5()[0], 2) + f32::powi(self.group5()[1], 2) + f32::powi(self.group5()[2], 2)
                - f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group3()[3], 2)
                - f32::powi(self.group6()[0], 2)
                - f32::powi(self.group6()[1], 2)
                - f32::powi(self.group6()[2], 2)
                + f32::powi(self.group6()[3], 2)
                + f32::powi(self.group9()[1], 2)
                + f32::powi(self.group9()[2], 2)
                + f32::powi(self.group9()[3], 2)
                + 2.0 * (self.group4()[0] * self.group3()[0])
                + 2.0 * (self.group4()[1] * self.group3()[1])
                + 2.0 * (self.group4()[2] * self.group3()[2])
                - 2.0 * (self.group7()[0] * self.group8()[0])
                - 2.0 * (self.group7()[1] * self.group8()[1])
                - 2.0 * (self.group7()[2] * self.group8()[2])
                + 2.0 * (self.group1()[3] * self[e1])
                - 2.0 * (self.group9()[0] * self[e45])),
        );
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
            // e1, e2, e3, e4
            geometric_anti_product.group1(),
            // e5
            geometric_anti_product[e1],
            // e41, e42, e43, e45
            geometric_anti_product.group3(),
            // e15, e25, e35
            geometric_anti_product.group4(),
            // e23, e31, e12
            geometric_anti_product.group5(),
            // e415, e425, e435, e321
            geometric_anti_product.group6(),
            // e423, e431, e412
            geometric_anti_product.group7(),
            // e235, e315, e125
            geometric_anti_product.group8(),
            // e1234, e4235, e4315, e4125
            geometric_anti_product.group9(),
            // e3215
            geometric_anti_product[e45],
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryCircle {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       13       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryCircle::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
                (anti_reverse.group0()[3] * self.group0()[0]),
                (anti_reverse.group0()[3] * self.group0()[1]),
                (anti_reverse.group0()[3] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * swizzle!(anti_reverse.group0(), 3, 0, 1, 2))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[1] * self.group0()[0])),
                0.0,
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([geometric_anti_product.group1()[0], geometric_anti_product.group1()[1], geometric_anti_product.group1()[2], 0.0]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryCircleRotor {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       14       17        0
    //  no simd       23       29        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e12345 */ self[e425]);
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])
                    + (anti_reverse[e425] * self[e425])),
                (anti_reverse.group0()[3] * self.group0()[0]),
                (anti_reverse.group0()[3] * self.group0()[1]),
                (anti_reverse.group0()[3] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * swizzle!(anti_reverse.group0(), 3, 0, 1, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group0()[2]) + (self.group0()[0] * anti_reverse[e425])),
                ((anti_reverse.group0()[1] * self[e425]) - (anti_reverse.group0()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[2] * self[e425])),
                0.0,
            ]) + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse[e425], anti_reverse[e425], anti_reverse[e425]]) * swizzle!(self.group0(), 1, 1, 2, 3))
                + (Simd32x4::from([self[e425], self.group0()[2], self.group0()[0], self[e425]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2) + f32::powi(self[e425], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryDipole {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       16       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([((anti_reverse.group0()[2] * self.group0()[2]) - (anti_reverse.group0()[3] * self.group0()[3])), 0.0, 0.0, 0.0])
                + (swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * swizzle!(self.group0(), 0, 3, 3, 3))
                + (swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * swizzle!(self.group0(), 1, 0, 1, 2))),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[1])),
                (-(anti_reverse.group0()[0] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[0])),
                ((anti_reverse.group0()[0] * self.group0()[1]) - (anti_reverse.group0()[1] * self.group0()[0])),
                0.0,
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([geometric_anti_product.group1()[0], geometric_anti_product.group1()[1], geometric_anti_product.group1()[2], 0.0]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryDipoleInversion {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       21        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       29        0
    //  no simd       48       53        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)), /* e4235, e4315, e4125 */ self.group1());
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                ((anti_reverse.group1()[0] * self.group1()[0]) + (anti_reverse.group1()[1] * self.group1()[1]) + (anti_reverse.group1()[2] * self.group1()[2])),
                (-(self.group1()[2] * anti_reverse.group0()[1]) + (anti_reverse.group0()[3] * self.group0()[0])),
                (-(self.group1()[0] * anti_reverse.group0()[2]) + (anti_reverse.group0()[3] * self.group0()[1])),
                (-(self.group1()[1] * anti_reverse.group0()[0]) + (anti_reverse.group0()[3] * self.group0()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 3, 2, 0, 1))
                + (Simd32x4::from([self.group0()[1], self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 1))
                + (swizzle!(anti_reverse.group0(), 2, 0, 1, 2) * swizzle!(self.group0(), 2, 3, 3, 3))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group1()[2]) - (anti_reverse.group1()[2] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group0()[2])),
                (-(anti_reverse.group1()[0] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group1()[0]) + (anti_reverse.group0()[2] * self.group0()[0])),
                ((anti_reverse.group1()[0] * self.group1()[1]) - (anti_reverse.group1()[1] * self.group1()[0]) + (anti_reverse.group0()[0] * self.group0()[1])),
                (-(anti_reverse.group1()[2] * self.group0()[2]) - (self.group1()[1] * anti_reverse.group0()[1]) - (self.group1()[2] * anti_reverse.group0()[2])),
            ]) - (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 1, 2, 0, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)
                + f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryQuadNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryQuadNum::from_groups(/* e321, e12345 */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1])));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)));
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryVersorEven {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       32        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       40       41        0
    //  no simd       64       68        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ (self.group1() * Simd32x4::from(-1.0)));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[3] * self.group0()[3])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])),
                ((anti_reverse.group0()[1] * self.group0()[0])
                    + (anti_reverse.group1()[0] * self.group1()[3])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group1()[0])),
                ((anti_reverse.group0()[2] * self.group0()[0])
                    + (anti_reverse.group1()[0] * self.group0()[3])
                    + (anti_reverse.group1()[1] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[1])),
                ((anti_reverse.group0()[3] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[0]) * self.group0())
                - (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 2, 3, 1, 2))
                + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group0()[3], anti_reverse.group0()[1], anti_reverse.group0()[2]]) * swizzle!(self.group1(), 3, 1, 2, 0))
                - (Simd32x4::from([self.group0()[1], self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 1, 2, 3, 1))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    + (anti_reverse.group1()[3] * self.group0()[1])),
                ((anti_reverse.group0()[2] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group1()[2])
                    + (anti_reverse.group1()[1] * self.group0()[0])
                    + (anti_reverse.group1()[3] * self.group0()[2])),
                ((anti_reverse.group0()[3] * self.group1()[3])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[2] * self.group0()[0])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
                (-(anti_reverse.group0()[3] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[3])),
            ]) + (Simd32x4::from(anti_reverse.group0()[0]) * self.group1())
                + (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[3]]) * swizzle!(self.group0(), 2, 3, 1, 0))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group0()[2]]) * swizzle!(self.group1(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group0()[3], self.group0()[1], self.group0()[2], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 2, 3, 1, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)
                - f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[3], 2)
                - f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4       14       17        0
    // Totals...
    // yes simd       22       17        0
    //  no simd       64       68        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (-(Simd32x4::from([self.group0()[0], self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 0, 2, 3, 1))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 2, 1, 1, 2))
                + (Simd32x4::from([self.group0()[3], self.group1()[1], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group0(), 3, 3, 2, 3))
                + (Simd32x4::from([self.group1()[0], self.group1()[3], self.group0()[3], self.group0()[1]]) * swizzle!(anti_reverse.group1(), 0, 0, 0, 1))
                + (Simd32x4::from([self.group1()[1], self.group0()[2], self.group1()[3], self.group1()[3]]) * swizzle!(anti_reverse.group1(), 1, 2, 1, 2))
                - (Simd32x4::from([self.group1()[3], self.group0()[3], self.group0()[1], self.group0()[2]]) * swizzle!(anti_reverse.group1(), 3, 1, 2, 0))
                + (swizzle!(anti_reverse.group0(), 1, 0, 0, 0) * swizzle!(self.group0(), 1, 1, 2, 3))
                + (swizzle!(anti_reverse.group1(), 2, 3, 3, 3) * swizzle!(self.group1(), 2, 0, 1, 2))),
            // e415, e425, e435, e321
            (-(Simd32x4::from([self.group0()[0], self.group1()[2], self.group1()[0], self.group0()[1]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 0))
                - (Simd32x4::from([self.group0()[2], self.group1()[3], self.group1()[3], self.group1()[2]]) * swizzle!(anti_reverse.group0(), 3, 2, 3, 3))
                + (Simd32x4::from([self.group0()[3], self.group0()[1], self.group0()[2], self.group1()[3]]) * swizzle!(anti_reverse.group0(), 2, 3, 1, 0))
                - (Simd32x4::from([self.group1()[1], self.group0()[0], self.group0()[0], self.group0()[2]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group1()[3], self.group0()[3], self.group0()[1], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 1, 1, 2, 2))
                - (swizzle!(anti_reverse.group0(), 0, 0, 0, 1) * swizzle!(self.group1(), 0, 1, 2, 0))
                - (swizzle!(anti_reverse.group1(), 3, 3, 3, 2) * swizzle!(self.group0(), 1, 2, 3, 3))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)
                + f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[3], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryVersorRoundPoint {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = MysteryVersorRoundPoint::from_groups(/* e1, e2, e3, e12345 */ Simd32x4::from([
            (self.group0()[0] * self.group0()[3] * 2.0),
            (self.group0()[1] * self.group0()[3] * 2.0),
            (self.group0()[2] * self.group0()[3] * 2.0),
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        ]));
        let subtraction = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for MysteryVersorSphere {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = MysteryVersorRoundPoint::from_groups(/* e1, e2, e3, e12345 */ Simd32x4::from([
            (self.group0()[0] * self.group0()[3] * 2.0),
            (self.group0()[1] * self.group0()[3] * 2.0),
            (self.group0()[2] * self.group0()[3] * 2.0),
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        ]));
        let subtraction = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
        ]));
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
impl AntiConstraintViolation for PlaneOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for QuadNum {
    type Output = VersorRoundPointAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       10       13        0
    //  no simd       12       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = QuadNum::from_groups(
            // e4, e5, e321, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_anti_product = VersorRoundPointAligningOrigin::from_groups(
            // e4, e5, e12345
            (Simd32x3::from([
                (-(anti_reverse.group0()[0] * self.group0()[2]) + (anti_reverse.group0()[2] * self.group0()[0]) + (anti_reverse.group0()[3] * self.group0()[0])),
                ((anti_reverse.group0()[1] * self.group0()[2]) - (anti_reverse.group0()[2] * self.group0()[1]) + (anti_reverse.group0()[3] * self.group0()[1])),
                ((anti_reverse.group0()[0] * self.group0()[1]) + (anti_reverse.group0()[1] * self.group0()[0]) + (anti_reverse.group0()[2] * self.group0()[2])),
            ]) + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[3]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2) + 2.0 * (self.group0()[0] * self.group0()[1])),
        );
        let subtraction = VersorRoundPointAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            (geometric_anti_product.group0()[2] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for QuadNumAtInfinity {
    type Output = VersorRoundPointAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd2        2        2        0
    // Totals...
    // yes simd        5        5        0
    //  no simd        7        7        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = QuadNumAtInfinity::from_groups(/* e5, e321, e12345 */ Simd32x3::from([self.group0()[0], (self.group0()[1] * -1.0), self.group0()[2]]));
        let geometric_anti_product = VersorRoundPointAligningOriginAtInfinity::from_groups(
            // e5, e12345
            (Simd32x2::from([(-(anti_reverse.group0()[1] * self.group0()[0]) + (anti_reverse.group0()[2] * self.group0()[0])), 0.0])
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group0()[1]]))
                + (Simd32x2::from(self.group0()[2]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group0()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let subtraction = VersorRoundPointAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            geometric_anti_product.group0()[0],
            (geometric_anti_product.group0()[1] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for QuadNumOrthogonalOrigin {
    type Output = VersorRoundPointAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        1        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = QuadNumOrthogonalOrigin::from_groups(/* e4, e5, e321 */ Simd32x3::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0)]));
        let geometric_anti_product = VersorRoundPointAligningOrigin::from_groups(
            // e4, e5, e12345
            (Simd32x3::from([
                ((anti_reverse.group0()[0] * self.group0()[2]) * -1.0),
                ((anti_reverse.group0()[2] * self.group0()[1]) * -1.0),
                ((anti_reverse.group0()[1] * self.group0()[0]) + (anti_reverse.group0()[2] * self.group0()[2])),
            ]) + (swizzle!(anti_reverse.group0(), 2, 1, 0) * swizzle!(self.group0(), 0, 2, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[2], 2) + 2.0 * (self.group0()[0] * self.group0()[1])));
        let subtraction = VersorRoundPointAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            (geometric_anti_product.group0()[2] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for RoundPoint {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for RoundPointAtOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for Scalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for Sphere {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for SphereAtOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for SphereOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84      102        0
    //    simd4       42       43        0
    // Totals...
    // yes simd      126      145        0
    //  no simd      252      274        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((anti_reverse.group0()[3] * self.group0()[0])
                    + (anti_reverse.group1()[0] * self.group3()[3])
                    + (anti_reverse.group1()[2] * self.group0()[1])
                    + (anti_reverse.group1()[3] * self.group0()[0])
                    + (anti_reverse.group3()[2] * self.group0()[1])
                    + (anti_reverse.group3()[3] * self.group1()[0])),
                ((anti_reverse.group0()[1] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[1])
                    + (anti_reverse.group1()[0] * self.group0()[2])
                    + (anti_reverse.group1()[1] * self.group3()[3])
                    + (anti_reverse.group1()[3] * self.group0()[1])
                    + (anti_reverse.group3()[1] * self.group3()[3])),
                ((anti_reverse.group0()[2] * self.group0()[3])
                    + (anti_reverse.group0()[3] * self.group0()[2])
                    + (anti_reverse.group1()[1] * self.group0()[0])
                    + (anti_reverse.group1()[3] * self.group0()[2])
                    + (anti_reverse.group3()[2] * self.group3()[3])
                    + (anti_reverse.group3()[3] * self.group1()[2])),
                (-(anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group3()[1] * self.group3()[1])
                    - (anti_reverse.group3()[2] * self.group3()[2])),
            ]) - (Simd32x4::from(anti_reverse.group0()[0]) * Simd32x4::from([self.group1()[3], self.group3()[2], self.group1()[1], self.group2()[0]]))
                - (Simd32x4::from(anti_reverse.group0()[1]) * Simd32x4::from([self.group1()[2], self.group1()[3], self.group3()[0], self.group2()[1]]))
                - (Simd32x4::from(anti_reverse.group0()[2]) * Simd32x4::from([self.group3()[1], self.group1()[0], self.group1()[3], self.group2()[2]]))
                + (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[2], anti_reverse.group2()[3]]) * swizzle!(self.group3(), 2, 0, 3, 3))
                + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group3()[3], anti_reverse.group0()[1], anti_reverse.group1()[3]]) * swizzle!(self.group1(), 1, 1, 0, 3))
                - (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group2()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[3], self.group1()[2], self.group3()[1], self.group0()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 0, 3))
                + (Simd32x4::from([self.group3()[3], self.group0()[2], self.group0()[0], self.group2()[3]]) * swizzle!(anti_reverse.group3(), 0, 0, 1, 3))
                - (swizzle!(anti_reverse.group3(), 3, 3, 3, 0) * swizzle!(self.group3(), 0, 1, 2, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[3] * self.group1()[0])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    + (anti_reverse.group1()[3] * self.group3()[0])
                    + (anti_reverse.group3()[0] * self.group1()[3])
                    + (anti_reverse.group3()[2] * self.group3()[1])
                    + (anti_reverse.group3()[3] * self.group2()[0])),
                ((anti_reverse.group0()[3] * self.group1()[1])
                    + (anti_reverse.group1()[3] * self.group3()[1])
                    + (anti_reverse.group2()[3] * self.group0()[1])
                    + (anti_reverse.group3()[0] * self.group3()[2])
                    + (anti_reverse.group3()[1] * self.group1()[3])
                    + (anti_reverse.group3()[3] * self.group2()[1])),
                ((anti_reverse.group0()[3] * self.group1()[2])
                    + (anti_reverse.group1()[3] * self.group3()[2])
                    + (anti_reverse.group2()[3] * self.group0()[2])
                    + (anti_reverse.group3()[1] * self.group3()[0])
                    + (anti_reverse.group3()[2] * self.group1()[3])
                    + (anti_reverse.group3()[3] * self.group2()[2])),
                (-(anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group1()[2] * self.group3()[2])
                    - (anti_reverse.group3()[0] * self.group1()[0])
                    - (anti_reverse.group3()[1] * self.group1()[1])
                    - (anti_reverse.group3()[2] * self.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group3()[3]]) * swizzle!(self.group2(), 1, 3, 3, 3))
                + (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group2()[1]]) * swizzle!(self.group0(), 1, 3, 3, 1))
                - (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group1()[1]]) * swizzle!(self.group3(), 2, 0, 1, 1))
                + (Simd32x4::from([self.group0()[0], self.group3()[3], self.group3()[3], self.group0()[2]]) * swizzle!(anti_reverse.group2(), 3, 1, 2, 2))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[3]]) * swizzle!(anti_reverse.group2(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group0()[3]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 3))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group3()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group2()[3], self.group2()[2], self.group2()[0], self.group1()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group3()[3], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 0))
                - (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group2()[2] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group3()[1])
                    + (anti_reverse.group2()[3] * self.group1()[0])
                    + (anti_reverse.group2()[3] * self.group3()[0])),
                ((anti_reverse.group2()[1] * self.group0()[3])
                    + (anti_reverse.group2()[1] * self.group1()[3])
                    + (anti_reverse.group2()[3] * self.group1()[1])
                    + (anti_reverse.group2()[3] * self.group3()[1])),
                ((anti_reverse.group2()[2] * self.group0()[3])
                    + (anti_reverse.group2()[2] * self.group1()[3])
                    + (anti_reverse.group2()[3] * self.group1()[2])
                    + (anti_reverse.group2()[3] * self.group3()[2])),
                (-(anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group3()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group3()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[3]) * self.group2())
                + (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group3()[0]]) * swizzle!(self.group2(), 3, 2, 0, 0))
                + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group3()[1]]) * swizzle!(self.group2(), 1, 3, 3, 1))
                - (Simd32x4::from([anti_reverse.group3()[0], anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group1()[2]]) * swizzle!(self.group2(), 3, 2, 0, 2))
                - (Simd32x4::from([anti_reverse.group3()[2], anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group1()[3]]) * swizzle!(self.group2(), 1, 3, 3, 3))
                + (Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group0()[3]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group1()[3], self.group3()[2], self.group3()[0], self.group1()[3]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 3))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (swizzle!(anti_reverse.group1(), 3, 3, 3, 1) * swizzle!(self.group2(), 0, 1, 2, 1))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group3(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group3(), 1, 2, 0, 2) * swizzle!(self.group2(), 2, 0, 1, 2))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((anti_reverse.group1()[2] * self.group3()[1]) + (anti_reverse.group1()[3] * self.group1()[0]) - (anti_reverse.group2()[0] * self.group3()[3])
                    + (anti_reverse.group3()[0] * self.group0()[3])
                    + (anti_reverse.group3()[2] * self.group1()[1])
                    + (anti_reverse.group3()[3] * self.group2()[0])),
                ((anti_reverse.group1()[1] * self.group1()[3]) + (anti_reverse.group1()[3] * self.group1()[1]) - (anti_reverse.group2()[1] * self.group3()[3])
                    + (anti_reverse.group3()[0] * self.group1()[2])
                    + (anti_reverse.group3()[1] * self.group0()[3])
                    + (anti_reverse.group3()[3] * self.group2()[1])),
                ((anti_reverse.group1()[2] * self.group1()[3]) + (anti_reverse.group1()[3] * self.group1()[2]) - (anti_reverse.group2()[2] * self.group3()[3])
                    + (anti_reverse.group3()[1] * self.group1()[0])
                    + (anti_reverse.group3()[2] * self.group0()[3])
                    + (anti_reverse.group3()[3] * self.group2()[2])),
                (-(anti_reverse.group0()[2] * self.group1()[2]) + (anti_reverse.group0()[3] * self.group3()[3])
                    - (anti_reverse.group3()[0] * self.group0()[0])
                    - (anti_reverse.group3()[1] * self.group0()[1])
                    - (anti_reverse.group3()[2] * self.group0()[2])
                    - (anti_reverse.group3()[3] * self.group1()[3])),
            ]) + (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group3()[3]]) * swizzle!(self.group0(), 2, 0, 1, 3))
                - (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 1, 2, 0, 1))
                - (Simd32x4::from([anti_reverse.group2()[3], anti_reverse.group2()[3], anti_reverse.group2()[3], anti_reverse.group1()[2]]) * swizzle!(self.group0(), 0, 1, 2, 2))
                - (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group0()[1]]) * swizzle!(self.group1(), 2, 0, 1, 1))
                + (Simd32x4::from([self.group1()[3], self.group3()[2], self.group3()[0], self.group3()[3]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group2()[1], self.group2()[3], self.group2()[3], self.group3()[1]]) * swizzle!(anti_reverse.group0(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group2()[3], self.group2()[2], self.group2()[0], self.group3()[0]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 0))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))
                + (swizzle!(anti_reverse.group0(), 3, 3, 3, 2) * swizzle!(self.group3(), 0, 1, 2, 2))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group3()[0], 2)
                - f32::powi(self.group3()[1], 2)
                - f32::powi(self.group3()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorEvenAligningOrigin {
    type Output = VersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       29        0
    //    simd2        8        8        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       29       43        0
    //  no simd       55       69        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group2()[1] * self.group0()[2])),
                ((anti_reverse.group0()[1] * self.group2()[3]) + (anti_reverse.group2()[2] * self.group0()[0])),
                ((anti_reverse.group0()[2] * self.group2()[3]) + (anti_reverse.group2()[0] * self.group0()[1])),
                (-(anti_reverse.group0()[2] * self.group1()[2]) - (anti_reverse.group1()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(anti_reverse.group1()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[3]]))
                - (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group0()[1]]) * swizzle!(self.group1(), 3, 3, 3, 1))
                - (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([anti_reverse.group2()[3], anti_reverse.group2()[3], anti_reverse.group2()[3], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 0, 1, 2, 1))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group2()[3], self.group2()[2], self.group2()[0], self.group1()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))),
            // e5, e12345
            (Simd32x2::from([
                0.0,
                ((anti_reverse.group1()[3] * self.group2()[3])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])),
            ]) + (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group2()[3], self.group0()[3]]))
                + (Simd32x2::from(anti_reverse.group2()[3]) * Simd32x2::from([self.group0()[3], self.group1()[3]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([anti_reverse.group2()[0], anti_reverse.group1()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([anti_reverse.group2()[1], anti_reverse.group1()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([anti_reverse.group2()[2], anti_reverse.group1()[2]]))
                - (Simd32x2::from(self.group2()[0]) * Simd32x2::from([anti_reverse.group1()[0], anti_reverse.group0()[0]]))
                - (Simd32x2::from(self.group2()[1]) * Simd32x2::from([anti_reverse.group1()[1], anti_reverse.group0()[1]]))
                - (Simd32x2::from(self.group2()[2]) * Simd32x2::from([anti_reverse.group1()[2], anti_reverse.group0()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2)
                - f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group1()[3] * self.group2()[3])),
        );
        let subtraction = VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            geometric_anti_product.group0(),
            // e5, e12345
            Simd32x2::from([geometric_anti_product.group1()[0], (geometric_anti_product.group1()[1] - anti_scalar_product[e12345])]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       51        0
    //    simd4       20       21        0
    // Totals...
    // yes simd       64       72        0
    //  no simd      124      135        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[3] * self.group0()[3])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])),
                ((anti_reverse.group0()[1] * self.group0()[0])
                    + (anti_reverse.group1()[0] * self.group1()[3])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group1()[0])),
                ((anti_reverse.group0()[2] * self.group0()[0])
                    + (anti_reverse.group1()[0] * self.group0()[3])
                    + (anti_reverse.group1()[1] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[1])),
                ((anti_reverse.group0()[3] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group1()[3])
                    + (anti_reverse.group1()[3] * self.group1()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[0]) * self.group0())
                - (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 2, 3, 1, 2))
                + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group0()[3], anti_reverse.group0()[1], anti_reverse.group0()[2]]) * swizzle!(self.group1(), 3, 1, 2, 0))
                - (Simd32x4::from([self.group0()[1], self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 1, 2, 3, 1))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[1] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[2] * self.group1()[1])
                    + (anti_reverse.group1()[3] * self.group0()[1])),
                ((anti_reverse.group0()[2] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group1()[2])
                    + (anti_reverse.group1()[1] * self.group0()[0])
                    + (anti_reverse.group1()[3] * self.group0()[2])),
                ((anti_reverse.group0()[3] * self.group1()[3])
                    + (anti_reverse.group1()[1] * self.group1()[0])
                    + (anti_reverse.group1()[2] * self.group0()[0])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
                (-(anti_reverse.group0()[3] * self.group1()[2])
                    - (anti_reverse.group1()[0] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group0()[3])),
            ]) + (Simd32x4::from(anti_reverse.group0()[0]) * self.group1())
                + (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[3]]) * swizzle!(self.group0(), 2, 3, 1, 0))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group0()[2]]) * swizzle!(self.group1(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group0()[3], self.group0()[1], self.group0()[2], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 2, 3, 1, 1))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group2()[2] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group1()[1])
                    + (anti_reverse.group2()[3] * self.group0()[1])
                    + (anti_reverse.group2()[3] * self.group1()[0])),
                ((anti_reverse.group2()[1] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group1()[3])
                    + (anti_reverse.group2()[3] * self.group0()[2])
                    + (anti_reverse.group2()[3] * self.group1()[1])),
                ((anti_reverse.group2()[2] * self.group0()[0])
                    + (anti_reverse.group2()[2] * self.group1()[3])
                    + (anti_reverse.group2()[3] * self.group0()[3])
                    + (anti_reverse.group2()[3] * self.group1()[2])),
                (-(anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[2] * self.group0()[3])
                    - (anti_reverse.group2()[2] * self.group1()[2])),
            ]) + (Simd32x4::from(anti_reverse.group0()[0]) * self.group2())
                - (Simd32x4::from(anti_reverse.group1()[3]) * self.group2())
                - (Simd32x4::from([anti_reverse.group0()[1], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[0]]) * swizzle!(self.group2(), 3, 2, 0, 0))
                - (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[2], anti_reverse.group0()[3], anti_reverse.group1()[1]]) * swizzle!(self.group2(), 1, 3, 3, 1))
                + (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group0()[2]]) * swizzle!(self.group2(), 3, 2, 0, 1))
                + (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group0()[3]]) * swizzle!(self.group2(), 1, 3, 3, 2))
                + (swizzle!(anti_reverse.group0(), 2, 3, 1, 1) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 2) * swizzle!(self.group2(), 2, 0, 1, 2))
                + (swizzle!(anti_reverse.group2(), 0, 0, 1, 3) * swizzle!(self.group0(), 0, 3, 1, 0))
                + (swizzle!(anti_reverse.group2(), 0, 0, 1, 3) * swizzle!(self.group1(), 3, 2, 0, 3))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group0(), 3, 1, 2, 1))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)
                - f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[3], 2)
                - f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorEvenAtOrigin {
    type Output = MysteryVersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       22        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       32       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = MysteryVersorRoundPoint::from_groups(
            // e1, e2, e3, e12345
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group0()[3] * self.group1()[0])),
                ((anti_reverse.group0()[1] * self.group1()[3]) + (anti_reverse.group0()[3] * self.group1()[1])),
                ((anti_reverse.group0()[2] * self.group1()[3]) + (anti_reverse.group0()[3] * self.group1()[2])),
                (-(anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2])),
            ]) + (swizzle!(anti_reverse.group0(), 0, 0, 1, 3) * swizzle!(self.group1(), 3, 2, 0, 3))
                - (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                - (swizzle!(anti_reverse.group1(), 0, 0, 1, 0) * swizzle!(self.group0(), 3, 2, 0, 0))
                + (swizzle!(anti_reverse.group1(), 1, 2, 0, 3) * swizzle!(self.group0(), 2, 0, 1, 3))
                - (swizzle!(anti_reverse.group1(), 2, 1, 2, 1) * swizzle!(self.group0(), 1, 3, 3, 1))
                - (swizzle!(anti_reverse.group1(), 3, 3, 3, 2) * swizzle!(self.group0(), 0, 1, 2, 2))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-2.0 * (self.group0()[0] * self.group1()[0]) - 2.0 * (self.group0()[1] * self.group1()[1]) - 2.0 * (self.group0()[2] * self.group1()[2])
                + 2.0 * (self.group0()[3] * self.group1()[3])),
        );
        let subtraction = MysteryVersorRoundPoint::from_groups(/* e1, e2, e3, e12345 */ Simd32x4::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
            (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorEvenOnOrigin {
    type Output = VersorRoundPointOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        4        4        0
    // Totals...
    // yes simd       11       14        0
    //  no simd       15       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = VersorRoundPointOnOrigin::from_groups(
            // e4, e12345
            (Simd32x2::from([
                (-(anti_reverse.group1()[0] * self.group0()[0]) - (anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
                0.0,
            ]) + (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group1()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([anti_reverse.group0()[1], anti_reverse.group1()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([anti_reverse.group0()[2], anti_reverse.group1()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)),
        );
        let subtraction =
            VersorRoundPointOnOrigin::from_groups(
                // e4, e12345
                Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
            );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       75        0
    //    simd4       20       21        0
    // Totals...
    // yes simd       76       96        0
    //  no simd      136      159        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e1, e2, e3, e4
            self.group2(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                ((anti_reverse.group0()[3] * self.group0()[0]) + (anti_reverse.group2()[2] * self.group0()[1])),
                ((anti_reverse.group0()[3] * self.group0()[1]) + (anti_reverse.group2()[1] * self.group2()[3])),
                ((anti_reverse.group0()[3] * self.group0()[2]) + (anti_reverse.group2()[2] * self.group2()[3])),
                (-(anti_reverse.group0()[2] * self.group1()[2]) - (anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group2()[3])
                    - (anti_reverse.group2()[1] * self.group2()[1])
                    - (anti_reverse.group2()[2] * self.group2()[2])),
            ]) - (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group0()[3], self.group2()[2], self.group2()[0], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 0))
                - (Simd32x4::from([self.group2()[1], self.group0()[3], self.group0()[3], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 2, 1, 2, 1))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group0()[3]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group2()[3], self.group0()[2], self.group0()[0], self.group1()[3]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 3))
                - (swizzle!(anti_reverse.group2(), 3, 3, 3, 0) * swizzle!(self.group2(), 0, 1, 2, 0))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                ((anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group0()[3] * self.group2()[0]) + (anti_reverse.group2()[0] * self.group0()[3])
                    - (anti_reverse.group2()[1] * self.group2()[2])
                    + (anti_reverse.group2()[2] * self.group2()[1])
                    + (anti_reverse.group2()[3] * self.group1()[0])),
                ((anti_reverse.group0()[1] * self.group1()[3])
                    + (anti_reverse.group0()[3] * self.group2()[1])
                    + (anti_reverse.group2()[0] * self.group2()[2])
                    + (anti_reverse.group2()[1] * self.group0()[3])
                    - (anti_reverse.group2()[2] * self.group2()[0])
                    + (anti_reverse.group2()[3] * self.group1()[1])),
                ((anti_reverse.group0()[2] * self.group1()[3]) + (anti_reverse.group0()[3] * self.group2()[2]) - (anti_reverse.group2()[0] * self.group2()[1])
                    + (anti_reverse.group2()[1] * self.group2()[0])
                    + (anti_reverse.group2()[2] * self.group0()[3])
                    + (anti_reverse.group2()[3] * self.group1()[2])),
                (-(anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group2()[3]]) * swizzle!(self.group1(), 3, 2, 0, 3))
                + (Simd32x4::from([self.group0()[1], self.group2()[3], self.group2()[3], self.group0()[1]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group2()[3]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group2()[3], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 0))
                - (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group1(), 3, 3, 3, 2) * swizzle!(self.group0(), 0, 1, 2, 2))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((anti_reverse.group1()[2] * self.group2()[1]) + (anti_reverse.group1()[3] * self.group2()[0])
                    - (anti_reverse.group2()[0] * self.group1()[3])
                    - (anti_reverse.group2()[2] * self.group1()[1])),
                ((anti_reverse.group1()[1] * self.group0()[3]) + (anti_reverse.group1()[3] * self.group2()[1])
                    - (anti_reverse.group2()[0] * self.group1()[2])
                    - (anti_reverse.group2()[1] * self.group1()[3])),
                ((anti_reverse.group1()[2] * self.group0()[3]) + (anti_reverse.group1()[3] * self.group2()[2])
                    - (anti_reverse.group2()[1] * self.group1()[0])
                    - (anti_reverse.group2()[2] * self.group1()[3])),
                (-(anti_reverse.group1()[1] * self.group2()[1]) - (anti_reverse.group1()[2] * self.group2()[2])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])),
            ]) - (Simd32x4::from(anti_reverse.group0()[3]) * self.group1())
                + (Simd32x4::from([self.group0()[3], self.group2()[2], self.group2()[0], self.group0()[3]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 3))
                - (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))
                + (swizzle!(anti_reverse.group2(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group1()[2]) - (anti_reverse.group1()[0] * self.group2()[3])
                    + (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group2()[3] * self.group1()[0])),
                (-(anti_reverse.group0()[2] * self.group1()[0]) - (anti_reverse.group1()[1] * self.group2()[3])
                    + (anti_reverse.group1()[2] * self.group0()[0])
                    + (anti_reverse.group2()[3] * self.group1()[1])),
                (-(anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group1()[0] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group1()[2])),
                ((anti_reverse.group0()[2] * self.group2()[2]) + (anti_reverse.group0()[3] * self.group2()[3])
                    - (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group0()[3])),
            ]) - (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group2()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group2()[1]]) * swizzle!(self.group0(), 0, 1, 2, 1))
                + (Simd32x4::from([self.group1()[1], self.group1()[3], self.group1()[3], self.group2()[1]]) * swizzle!(anti_reverse.group0(), 2, 1, 2, 1))
                + (Simd32x4::from([self.group1()[3], self.group1()[2], self.group1()[0], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 0))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2)
                - f32::powi(self.group2()[0], 2)
                - f32::powi(self.group2()[1], 2)
                - f32::powi(self.group2()[2], 2)
                - 2.0 * (self.group0()[0] * self.group1()[0])
                - 2.0 * (self.group0()[1] * self.group1()[1])
                - 2.0 * (self.group0()[2] * self.group1()[2])
                + 2.0 * (self.group1()[3] * self.group2()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       78        0
    //    simd4       48       49        0
    // Totals...
    // yes simd      108      127        0
    //  no simd      252      274        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group1()[1])
                    - (anti_reverse.group0()[3] * self.group0()[0])
                    - (anti_reverse.group1()[2] * self.group0()[1])
                    - (anti_reverse.group2()[3] * self.group1()[0])
                    - (anti_reverse.group2()[3] * self.group3()[0])
                    - (anti_reverse.group3()[1] * self.group0()[2])),
                (-(anti_reverse.group0()[1] * self.group0()[3])
                    - (anti_reverse.group0()[3] * self.group0()[1])
                    - (anti_reverse.group1()[0] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group1()[1])
                    - (anti_reverse.group2()[3] * self.group3()[1])
                    - (anti_reverse.group3()[2] * self.group0()[0])),
                (-(anti_reverse.group0()[2] * self.group0()[3])
                    - (anti_reverse.group0()[3] * self.group0()[2])
                    - (anti_reverse.group1()[1] * self.group0()[0])
                    - (anti_reverse.group2()[3] * self.group1()[2])
                    - (anti_reverse.group2()[3] * self.group3()[2])
                    - (anti_reverse.group3()[0] * self.group0()[1])),
                ((anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])
                    + (anti_reverse.group3()[2] * self.group3()[2])),
            ]) - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[3]]))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group3()[3]]))
                - (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group2()[3]]) * swizzle!(self.group3(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(anti_reverse.group1(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group0()[1], self.group2()[3], self.group2()[3], self.group3()[1]]) * swizzle!(anti_reverse.group3(), 2, 1, 2, 1))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group0()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group2()[3], self.group0()[2], self.group0()[0], self.group3()[0]]) * swizzle!(anti_reverse.group3(), 0, 0, 1, 0))
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[1]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 1))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group3()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[0] * self.group2()[2]) - (anti_reverse.group3()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[1] * self.group2()[0]) - (anti_reverse.group3()[3] * self.group0()[2])),
                ((anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2])),
            ]) - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[1], anti_reverse.group0()[2], anti_reverse.group1()[0]]) * swizzle!(self.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group0()[3], anti_reverse.group3()[0]]) * swizzle!(self.group1(), 0, 1, 2, 0))
                + (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group0()[3]]) * swizzle!(self.group1(), 2, 0, 1, 3))
                - (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group2()[3]]) * self.group3())
                + (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group1()[3]]) * swizzle!(self.group0(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[1], self.group2()[3], self.group2()[3], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group3()[1]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 1))
                - (Simd32x4::from([self.group1()[1], self.group0()[3], self.group0()[3], self.group3()[2]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 2))
                - (Simd32x4::from([self.group1()[3], self.group3()[2], self.group3()[0], self.group1()[1]]) * swizzle!(anti_reverse.group3(), 0, 0, 1, 1))
                - (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[2]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group2()[3], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 0))
                - (Simd32x4::from([self.group3()[1], self.group1()[3], self.group1()[3], self.group1()[2]]) * swizzle!(anti_reverse.group3(), 2, 1, 2, 2))
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 3))
                + (swizzle!(anti_reverse.group0(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[2] * self.group2()[1])
                    - (anti_reverse.group1()[3] * self.group2()[0])
                    - (anti_reverse.group3()[2] * self.group2()[1])
                    - (anti_reverse.group3()[3] * self.group1()[0])),
                (-(anti_reverse.group1()[3] * self.group2()[1])
                    - (anti_reverse.group2()[2] * self.group3()[0])
                    - (anti_reverse.group3()[1] * self.group3()[3])
                    - (anti_reverse.group3()[3] * self.group1()[1])),
                (-(anti_reverse.group1()[3] * self.group2()[2])
                    - (anti_reverse.group2()[2] * self.group0()[3])
                    - (anti_reverse.group3()[2] * self.group3()[3])
                    - (anti_reverse.group3()[3] * self.group1()[2])),
                ((anti_reverse.group1()[2] * self.group2()[2])
                    + (anti_reverse.group3()[1] * self.group2()[1])
                    + (anti_reverse.group3()[2] * self.group2()[2])
                    + (anti_reverse.group3()[3] * self.group1()[3])),
            ]) - (Simd32x4::from(anti_reverse.group0()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]))
                - (Simd32x4::from(anti_reverse.group2()[0]) * Simd32x4::from([self.group0()[3], self.group1()[2], self.group3()[1], self.group3()[0]]))
                + (Simd32x4::from(anti_reverse.group2()[0]) * Simd32x4::from([self.group1()[3], self.group3()[2], self.group1()[1], self.group1()[0]]))
                + (Simd32x4::from(anti_reverse.group2()[1]) * Simd32x4::from([self.group1()[2], self.group1()[3], self.group3()[0], self.group1()[1]]))
                + (Simd32x4::from(anti_reverse.group2()[2]) * Simd32x4::from([self.group3()[1], self.group1()[0], self.group1()[3], self.group1()[2]]))
                - (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group2()[1]]) * swizzle!(self.group3(), 2, 3, 3, 1))
                + (Simd32x4::from([anti_reverse.group3()[1], anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group1()[1]]) * swizzle!(self.group2(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group1()[1], self.group0()[3], self.group1()[0], self.group3()[2]]) * swizzle!(anti_reverse.group2(), 2, 1, 1, 2))
                + (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[0]]) * swizzle!(anti_reverse.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group3()[3], self.group2()[2], self.group2()[0], self.group0()[3]]) * swizzle!(anti_reverse.group3(), 0, 0, 1, 3))
                - (Simd32x4::from([self.group3()[3], self.group2()[2], self.group2()[0], self.group3()[3]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 3))
                + (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group2(), 2, 0, 1, 0))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group2()[1]) - (anti_reverse.group1()[1] * self.group3()[2])
                    + (anti_reverse.group3()[0] * self.group0()[3])
                    + (anti_reverse.group3()[3] * self.group0()[0])),
                (-(anti_reverse.group0()[1] * self.group3()[3]) - (anti_reverse.group1()[2] * self.group3()[0])
                    + (anti_reverse.group3()[1] * self.group0()[3])
                    + (anti_reverse.group3()[3] * self.group0()[1])),
                (-(anti_reverse.group0()[2] * self.group3()[3]) - (anti_reverse.group1()[0] * self.group3()[1])
                    + (anti_reverse.group3()[2] * self.group0()[3])
                    + (anti_reverse.group3()[3] * self.group0()[2])),
                ((anti_reverse.group0()[1] * self.group3()[1]) + (anti_reverse.group0()[2] * self.group3()[2])
                    - (anti_reverse.group3()[1] * self.group0()[1])
                    - (anti_reverse.group3()[2] * self.group0()[2])),
            ]) - (Simd32x4::from(anti_reverse.group2()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group1()[3]]))
                + (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group0()[1]]) * swizzle!(self.group1(), 0, 1, 2, 1))
                + (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group1()[2]]) * swizzle!(self.group0(), 1, 2, 0, 2))
                + (Simd32x4::from([anti_reverse.group3()[2], anti_reverse.group3()[0], anti_reverse.group3()[1], anti_reverse.group0()[2]]) * swizzle!(self.group1(), 1, 2, 0, 2))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group1()[3], self.group3()[2], self.group3()[0], self.group0()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 0))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group3()[1], self.group1()[3], self.group1()[3], self.group0()[1]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group3()[3], self.group2()[2], self.group2()[0], self.group2()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                + (swizzle!(anti_reverse.group0(), 3, 3, 3, 0) * swizzle!(self.group3(), 0, 1, 2, 0))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 3) * swizzle!(self.group0(), 2, 0, 1, 3))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group3()[0], 2)
                + f32::powi(self.group3()[1], 2)
                + f32::powi(self.group3()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                geometric_anti_product.group0()[0],
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                (geometric_anti_product.group0()[3] - anti_scalar_product[e12345]),
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       27        0
    //    simd4       26       27        0
    // Totals...
    // yes simd       46       54        0
    //  no simd      124      135        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self.group0()[0], (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_anti_product = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (-Simd32x4::from([
                (anti_reverse.group0()[0] * self.group0()[0]),
                (anti_reverse.group2()[1] * self.group1()[2]),
                (anti_reverse.group2()[2] * self.group1()[0]),
                (anti_reverse.group2()[0] * self.group1()[1]),
            ]) + (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 0, 0, 1, 2))
                + (Simd32x4::from([self.group1()[0], self.group1()[3], self.group2()[2], self.group2()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 0, 1))
                + (Simd32x4::from([self.group1()[1], self.group2()[1], self.group1()[3], self.group1()[3]]) * swizzle!(anti_reverse.group1(), 1, 2, 1, 2))
                - (Simd32x4::from([self.group1()[3], self.group2()[2], self.group2()[0], self.group2()[1]]) * swizzle!(anti_reverse.group1(), 3, 1, 2, 0))
                + (Simd32x4::from([self.group2()[1], self.group0()[0], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 0, 0, 1))
                + (Simd32x4::from([self.group2()[2], self.group1()[1], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 2, 1, 2))
                + (swizzle!(anti_reverse.group1(), 2, 3, 3, 3) * swizzle!(self.group1(), 2, 0, 1, 2))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (anti_reverse.group2()[1] * self.group2()[2]),
                (anti_reverse.group2()[2] * self.group2()[0]),
                (anti_reverse.group2()[0] * self.group2()[1]),
                (anti_reverse.group1()[3] * self.group0()[0]),
            ]) - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group2()[0]]) * swizzle!(self.group1(), 0, 1, 2, 0))
                + (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group0()[0]]) * swizzle!(self.group1(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[0], self.group1()[2], self.group1()[0], self.group2()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 0))
                - (Simd32x4::from([self.group1()[1], self.group0()[0], self.group0()[0], self.group2()[1]]) * swizzle!(anti_reverse.group1(), 2, 1, 2, 1))
                - (Simd32x4::from([self.group1()[3], self.group2()[2], self.group2()[0], self.group1()[1]]) * swizzle!(anti_reverse.group2(), 0, 0, 1, 1))
                - (Simd32x4::from([self.group2()[1], self.group1()[3], self.group1()[3], self.group1()[2]]) * swizzle!(anti_reverse.group2(), 2, 1, 2, 2))
                - (swizzle!(anti_reverse.group1(), 3, 3, 3, 2) * swizzle!(self.group2(), 0, 1, 2, 2))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-(anti_reverse.group1()[3] * self.group0()[1])
                    - (anti_reverse.group2()[0] * self.group2()[3])
                    - (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group1()[0])),
                (-(anti_reverse.group1()[1] * self.group2()[3])
                    - (anti_reverse.group2()[0] * self.group0()[3])
                    - (anti_reverse.group2()[1] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group1()[1])),
                (-(anti_reverse.group1()[2] * self.group2()[3])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group2()[3])
                    - (anti_reverse.group2()[3] * self.group1()[2])),
                ((anti_reverse.group1()[2] * self.group0()[3])
                    + (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[3])
                    + (anti_reverse.group2()[3] * self.group1()[3])),
            ]) - (Simd32x4::from(anti_reverse.group0()[0]) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], self.group2()[3]]))
                - (Simd32x4::from(anti_reverse.group0()[1]) * Simd32x4::from([self.group0()[0], self.group1()[2], self.group2()[1], self.group2()[0]]))
                + (Simd32x4::from(anti_reverse.group0()[1]) * Simd32x4::from([self.group1()[3], self.group2()[2], self.group1()[1], self.group1()[0]]))
                + (Simd32x4::from(anti_reverse.group0()[2]) * Simd32x4::from([self.group1()[2], self.group1()[3], self.group2()[0], self.group1()[1]]))
                - (Simd32x4::from(anti_reverse.group0()[2]) * Simd32x4::from([self.group2()[2], self.group0()[0], self.group1()[0], self.group2()[1]]))
                - (Simd32x4::from(anti_reverse.group0()[3]) * Simd32x4::from([self.group1()[1], self.group2()[0], self.group0()[0], self.group2()[2]]))
                + (Simd32x4::from(anti_reverse.group0()[3]) * Simd32x4::from([self.group2()[1], self.group1()[0], self.group1()[3], self.group1()[2]]))
                - (Simd32x4::from([anti_reverse.group1()[2], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group2()[3]]) * swizzle!(self.group0(), 2, 2, 3, 0))
                + (Simd32x4::from([anti_reverse.group2()[1], anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 3, 1, 2, 2))
                + (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[1]]) * swizzle!(anti_reverse.group2(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group2()[3], self.group0()[3], self.group0()[1], self.group2()[3]]) * swizzle!(anti_reverse.group1(), 0, 0, 1, 3))
                + (swizzle!(anti_reverse.group1(), 1, 2, 0, 0) * swizzle!(self.group0(), 3, 1, 2, 1))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group2()[0], 2)
                + f32::powi(self.group2()[1], 2)
                + f32::powi(self.group2()[2], 2)),
        );
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            geometric_anti_product.group1(),
            // e235, e315, e125, e5
            geometric_anti_product.group2(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = VersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd2        8        8        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       32       46        0
    //  no simd       55       69        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_anti_product = VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(anti_reverse.group0()[2] * self.group2()[1]) + (anti_reverse.group2()[0] * self.group2()[3]) - (anti_reverse.group2()[3] * self.group2()[0])),
                (-(anti_reverse.group0()[1] * self.group1()[3]) + (anti_reverse.group2()[1] * self.group2()[3]) - (anti_reverse.group2()[3] * self.group2()[1])),
                (-(anti_reverse.group0()[2] * self.group1()[3]) + (anti_reverse.group2()[2] * self.group2()[3]) - (anti_reverse.group2()[3] * self.group2()[2])),
                ((anti_reverse.group0()[1] * self.group1()[1]) + (anti_reverse.group0()[2] * self.group1()[2]) + (anti_reverse.group1()[2] * self.group0()[2])),
            ]) + (Simd32x4::from([anti_reverse.group2()[2], anti_reverse.group2()[0], anti_reverse.group2()[1], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group1()[3], self.group2()[2], self.group2()[0], self.group2()[3]]) * swizzle!(anti_reverse.group0(), 0, 0, 1, 3))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 1, 2, 0, 0))
                + (swizzle!(anti_reverse.group1(), 3, 3, 3, 0) * swizzle!(self.group0(), 0, 1, 2, 0))
                - (swizzle!(anti_reverse.group2(), 1, 2, 0, 3) * swizzle!(self.group0(), 2, 0, 1, 3))),
            // e5, e12345
            (Simd32x2::from([
                0.0,
                ((anti_reverse.group2()[0] * self.group0()[0]) + (anti_reverse.group2()[1] * self.group0()[1]) + (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group1()[3])),
            ]) - (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                - (Simd32x2::from(anti_reverse.group1()[3]) * Simd32x2::from([self.group0()[3], self.group2()[3]]))
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([anti_reverse.group2()[0], anti_reverse.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([anti_reverse.group2()[1], anti_reverse.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([anti_reverse.group2()[2], anti_reverse.group1()[2]]))
                + (Simd32x2::from(self.group2()[0]) * Simd32x2::from([anti_reverse.group1()[0], anti_reverse.group0()[0]]))
                + (Simd32x2::from(self.group2()[1]) * Simd32x2::from([anti_reverse.group1()[1], anti_reverse.group0()[1]]))
                + (Simd32x2::from(self.group2()[2]) * Simd32x2::from([anti_reverse.group1()[2], anti_reverse.group0()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group1()[3] * self.group2()[3])),
        );
        let subtraction = VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            geometric_anti_product.group0(),
            // e5, e12345
            Simd32x2::from([geometric_anti_product.group1()[0], (geometric_anti_product.group1()[1] - anti_scalar_product[e12345])]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorRoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group1()[1]) * self.group0() * Simd32x4::from(2.0)),
            // e5, e12345
            Simd32x2::from([
                (self.group1()[0] * self.group1()[1] * 2.0),
                (f32::powi(self.group1()[1], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)
                    + 2.0 * (self.group1()[0] * self.group0()[3])),
            ]),
        );
        let subtraction = RoundPoint::from_groups(/* e1, e2, e3, e4 */ geometric_anti_product.group0(), /* e5 */ geometric_anti_product.group1()[0]);
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorRoundPointAligningOrigin {
    type Output = RoundPointAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = VersorRoundPointAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            (self.group0()[0] * self.group0()[2] * 2.0),
            (self.group0()[1] * self.group0()[2] * 2.0),
            (f32::powi(self.group0()[2], 2) + 2.0 * (self.group0()[0] * self.group0()[1])),
        ]));
        let subtraction = RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from([geometric_anti_product.group0()[0], geometric_anti_product.group0()[1]]));
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorRoundPointAligningOriginAtInfinity {
    type Output = VersorRoundPointAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = VersorRoundPointAligningOriginAtInfinity::from_groups(
            // e5, e12345
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self.group0()[1], 2));
        let subtraction = VersorRoundPointAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            geometric_anti_product.group0()[0],
            (geometric_anti_product.group0()[1] - anti_scalar_product[e12345]),
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorRoundPointAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = VersorRoundPointAtInfinity::from_groups(
            // e1, e2, e3
            (Simd32x3::from(self.group1()[1]) * self.group0() * Simd32x3::from(2.0)),
            // e5, e12345
            Simd32x2::from([
                (self.group1()[0] * self.group1()[1] * 2.0),
                (f32::powi(self.group1()[1], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)),
            ]),
        );
        let subtraction = AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
            geometric_anti_product.group1()[0],
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorRoundPointOnOrigin {
    type Output = VersorRoundPointOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = VersorRoundPointOnOrigin::from_groups(
            // e4, e12345
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self.group0()[1], 2));
        let subtraction =
            VersorRoundPointOnOrigin::from_groups(
                // e4, e12345
                Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
            );
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorSphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group1()[1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * Simd32x4::from([2.0, 2.0, 2.0, -2.0])),
            // e5, e12345
            Simd32x2::from([
                (self.group1()[1] * self.group0()[3] * -2.0),
                (-f32::powi(self.group1()[1], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)
                    - 2.0 * (self.group1()[0] * self.group0()[3])),
            ]),
        );
        let subtraction = RoundPoint::from_groups(/* e1, e2, e3, e4 */ geometric_anti_product.group0(), /* e5 */ geometric_anti_product.group1()[0]);
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorSphereAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = VersorRoundPointAtInfinity::from_groups(
            // e1, e2, e3
            (Simd32x3::from(self[e4315]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(2.0)),
            // e5, e12345
            Simd32x2::from([
                (self.group0()[3] * self[e4315] * -2.0),
                (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self[e4315], 2)),
            ]),
        );
        let subtraction = AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product.group0()[0],
            geometric_anti_product.group0()[1],
            geometric_anti_product.group0()[2],
            geometric_anti_product.group1()[0],
        ]));
        return subtraction;
    }
}
impl AntiConstraintViolation for VersorSphereOrthogonalOrigin {
    type Output = RoundPointAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = VersorRoundPointAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            (self.group0()[1] * self.group0()[2] * -2.0),
            (self.group0()[0] * self.group0()[2] * -2.0),
            (-f32::powi(self.group0()[2], 2) - 2.0 * (self.group0()[0] * self.group0()[1])),
        ]));
        let subtraction = RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from([geometric_anti_product.group0()[0], geometric_anti_product.group0()[1]]));
        return subtraction;
    }
}
