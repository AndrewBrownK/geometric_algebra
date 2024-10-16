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
//   Median:        14      13       0
//  Average:        32      35       0
//  Maximum:       293     303       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        14      16       0
//  Average:        41      47       0
//  Maximum:       396     420       0
impl AntiConstraintViolation for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       42        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       37       46        0
    //  no simd       43       57        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (self.group0()[0] * anti_reverse.group2()[0])
                    + (self.group0()[1] * anti_reverse.group2()[1])
                    + (self.group0()[2] * anti_reverse.group2()[2])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group1()[3] * self.group1()[3])
                    - (anti_reverse.group2()[3] * self.group2()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group1()[0] * self.group2()[0])
                    + (anti_reverse.group1()[1] * self.group2()[1])
                    + (anti_reverse.group1()[2] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group1()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       68        0
    //    simd3        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       69       77        0
    //  no simd       90      103        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * anti_reverse.group2()[0])
                    - (self.group0()[1] * anti_reverse.group2()[1])
                    - (self.group0()[2] * anti_reverse.group2()[2])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group1()[3] * self.group1()[3])
                    + (anti_reverse.group2()[3] * self.group3()[3])
                    - (anti_reverse.group3()[0] * self.group3()[0])
                    - (anti_reverse.group3()[1] * self.group3()[1])
                    - (anti_reverse.group3()[2] * self.group3()[2])
                    + (anti_reverse.group3()[3] * self.group2()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group1()[3] * self.group3()[3])
                    - (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[0] * self.group3()[0])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group3()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group3()[2])
                    + (anti_reverse.group3()[0] * self.group2()[0])
                    + (anti_reverse.group3()[1] * self.group2()[1])
                    + (anti_reverse.group3()[2] * self.group2()[2])
                    + (anti_reverse.group3()[3] * self.group1()[3])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([-2.0, -1.0])),
        );
        let subtraction = DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([geometric_anti_product.group0()[0], 0.0]));
        return subtraction;
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
        let anti_reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3]));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self.group0()[3], 2));
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
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
        let anti_reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3, e5 */ self.group1());
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[3] * self.group0()[3])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group0()[3] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
        return subtraction;
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
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])
                    - (anti_reverse.group0()[3] * self.group0()[3])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group1()[0]) + (anti_reverse.group0()[1] * self.group1()[1]) + (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group0()[3] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    - (anti_reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
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
impl AntiConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
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
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group1()[3] * self.group1()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    - (self.group2()[0] * anti_reverse.group1()[0])
                    - (self.group2()[1] * anti_reverse.group1()[1])
                    - (self.group2()[2] * anti_reverse.group1()[2])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       50        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       43       52        0
    //  no simd       43       57        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * anti_reverse.group2()[0])
                    - (self.group0()[1] * anti_reverse.group2()[1])
                    - (self.group0()[2] * anti_reverse.group2()[2])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group1()[3] * self.group1()[3])
                    + (anti_reverse.group2()[3] * self.group2()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Dipole {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       38        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       35       43        0
    //  no simd       41       56        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group1()[3] * self.group1()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group2()[0] * self.group1()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])
                    + (self.group2()[0] * anti_reverse.group1()[0])
                    + (self.group2()[1] * anti_reverse.group1()[1])
                    + (self.group2()[2] * anti_reverse.group1()[2])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       68        0
    //    simd3        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       69       77        0
    //  no simd       90      103        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (self.group0()[0] * anti_reverse.group2()[0])
                    + (self.group0()[1] * anti_reverse.group2()[1])
                    + (self.group0()[2] * anti_reverse.group2()[2])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group1()[3] * self.group1()[3])
                    - (anti_reverse.group2()[3] * self.group3()[3])
                    + (anti_reverse.group3()[0] * self.group3()[0])
                    + (anti_reverse.group3()[1] * self.group3()[1])
                    + (anti_reverse.group3()[2] * self.group3()[2])
                    - (anti_reverse.group3()[3] * self.group2()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group1()[0] * self.group2()[0]) + (anti_reverse.group1()[1] * self.group2()[1]) + (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group1()[3] * self.group3()[3])
                    + (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[0] * self.group3()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group3()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group3()[2])
                    + (anti_reverse.group3()[0] * self.group2()[0])
                    + (anti_reverse.group3()[1] * self.group2()[1])
                    + (anti_reverse.group3()[2] * self.group2()[2])
                    + (anti_reverse.group3()[3] * self.group1()[3])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
        let subtraction = DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([geometric_anti_product.group0()[0], 0.0]));
        return subtraction;
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
        let anti_reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3] * -1.0));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
        return subtraction;
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
        let anti_reverse = Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[3] * self.group0()[3])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group1()[0])
                    - (anti_reverse.group0()[1] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group0()[3] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
        return subtraction;
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
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])
                    + (anti_reverse.group0()[3] * self.group0()[3])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group1()[0]) - (anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group0()[3] * self.group1()[3])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      248      252        0
    //    simd2       16       16        0
    //    simd3        0        4        0
    //    simd4       29       31        0
    // Totals...
    // yes simd      293      303        0
    //  no simd      396      420        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e15, e25, e35, e45
            (self.group3() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (self.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (self.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group8() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e45],
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e12345
            (Simd32x2::from([
                ((anti_reverse.group0()[1] * self.group0()[0])
                    - (anti_reverse.group7()[0] * self.group3()[0])
                    - (anti_reverse.group7()[1] * self.group3()[1])
                    - (anti_reverse.group7()[2] * self.group3()[2])
                    - (self.group5()[0] * anti_reverse.group6()[0])
                    - (self.group5()[1] * anti_reverse.group6()[1])
                    - (self.group5()[2] * anti_reverse.group6()[2])
                    - (self.group7()[0] * anti_reverse.group3()[0])
                    - (self.group7()[1] * anti_reverse.group3()[1])
                    - (self.group7()[2] * anti_reverse.group3()[2])
                    - (anti_reverse.group6()[3] * self.group3()[3])
                    + (anti_reverse.group9()[0] * self.group1()[0])
                    + (anti_reverse.group9()[1] * self.group1()[1])
                    + (anti_reverse.group9()[2] * self.group1()[2])
                    + (anti_reverse[e1] * self[e45])
                    + (anti_reverse[e45] * self[e1])),
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
                    - (anti_reverse.group9()[3] * self[e45])
                    - (self.group9()[3] * anti_reverse[e45])),
            ]) + (Simd32x2::from(self.group0()[1]) * anti_reverse.group0())
                - (Simd32x2::from(anti_reverse.group8()[0]) * Simd32x2::from([self.group4()[0], self.group7()[0]]))
                - (Simd32x2::from(anti_reverse.group8()[1]) * Simd32x2::from([self.group4()[1], self.group7()[1]]))
                - (Simd32x2::from(anti_reverse.group8()[2]) * Simd32x2::from([self.group4()[2], self.group7()[2]]))
                - (Simd32x2::from(self.group8()[0]) * Simd32x2::from([anti_reverse.group4()[0], anti_reverse.group7()[0]]))
                - (Simd32x2::from(self.group8()[1]) * Simd32x2::from([anti_reverse.group4()[1], anti_reverse.group7()[1]]))
                - (Simd32x2::from(self.group8()[2]) * Simd32x2::from([anti_reverse.group4()[2], anti_reverse.group7()[2]]))
                + (Simd32x2::from(anti_reverse.group1()[3]) * Simd32x2::from([self.group9()[3], self[e1]]))
                - (Simd32x2::from(anti_reverse.group3()[3]) * Simd32x2::from([self.group6()[3], self.group3()[3]]))
                + (Simd32x2::from(self.group1()[3]) * Simd32x2::from([anti_reverse.group9()[3], anti_reverse[e1]]))
                - (Simd32x2::from(self.group6()[0]) * Simd32x2::from([anti_reverse.group5()[0], anti_reverse.group6()[0]]))
                - (Simd32x2::from(self.group6()[1]) * Simd32x2::from([anti_reverse.group5()[1], anti_reverse.group6()[1]]))
                - (Simd32x2::from(self.group6()[2]) * Simd32x2::from([anti_reverse.group5()[2], anti_reverse.group6()[2]]))
                + (Simd32x2::from(self.group9()[0]) * Simd32x2::from([anti_reverse.group1()[0], anti_reverse.group9()[0]]))
                + (Simd32x2::from(self.group9()[1]) * Simd32x2::from([anti_reverse.group1()[1], anti_reverse.group9()[1]]))
                + (Simd32x2::from(self.group9()[2]) * Simd32x2::from([anti_reverse.group1()[2], anti_reverse.group9()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * anti_reverse.group9()[0]) - (anti_reverse.group4()[0] * self.group9()[3])
                    + (anti_reverse.group4()[1] * self.group3()[2])
                    + (anti_reverse.group5()[0] * self.group3()[3])
                    - (anti_reverse.group5()[1] * self.group9()[2])
                    + (anti_reverse.group7()[0] * self[e1])
                    - (anti_reverse.group7()[1] * self.group8()[2])
                    + (anti_reverse.group7()[2] * self.group8()[1])
                    - (anti_reverse.group8()[0] * self.group1()[3])
                    + (anti_reverse.group8()[1] * self.group7()[2])
                    - (anti_reverse.group8()[2] * self.group7()[1])
                    + (self.group4()[0] * anti_reverse.group9()[3])
                    - (self.group4()[2] * anti_reverse.group3()[1])
                    + (self.group5()[0] * anti_reverse.group3()[3])
                    + (self.group5()[1] * anti_reverse.group9()[2])
                    - (self.group7()[0] * anti_reverse[e1])
                    + (self.group8()[0] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[2] * self.group6()[1])
                    + (anti_reverse.group3()[0] * self[e45])
                    + (anti_reverse.group6()[2] * self.group1()[1])
                    + (anti_reverse.group6()[3] * self.group6()[0])),
                ((self.group0()[0] * anti_reverse.group9()[1]) - (anti_reverse.group4()[1] * self.group9()[3])
                    + (anti_reverse.group4()[2] * self.group3()[0])
                    + (anti_reverse.group5()[1] * self.group3()[3])
                    - (anti_reverse.group5()[2] * self.group9()[0])
                    + (anti_reverse.group7()[0] * self.group8()[2])
                    + (anti_reverse.group7()[1] * self[e1])
                    - (anti_reverse.group7()[2] * self.group8()[0])
                    - (anti_reverse.group8()[0] * self.group7()[2])
                    - (anti_reverse.group8()[1] * self.group1()[3])
                    + (anti_reverse.group8()[2] * self.group7()[0])
                    - (self.group4()[0] * anti_reverse.group3()[2])
                    + (self.group4()[1] * anti_reverse.group9()[3])
                    + (self.group5()[1] * anti_reverse.group3()[3])
                    + (self.group5()[2] * anti_reverse.group9()[0])
                    - (self.group7()[1] * anti_reverse[e1])
                    + (self.group8()[1] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[0] * self.group6()[2])
                    + (anti_reverse.group3()[1] * self[e45])
                    + (anti_reverse.group6()[1] * self.group6()[3])
                    + (anti_reverse.group6()[3] * self.group6()[1])),
                ((self.group0()[0] * anti_reverse.group9()[2]) + (anti_reverse.group4()[0] * self.group3()[1])
                    - (anti_reverse.group4()[2] * self.group9()[3])
                    - (anti_reverse.group5()[0] * self.group9()[1])
                    + (anti_reverse.group5()[2] * self.group3()[3])
                    - (anti_reverse.group7()[0] * self.group8()[1])
                    + (anti_reverse.group7()[1] * self.group8()[0])
                    + (anti_reverse.group7()[2] * self[e1])
                    + (anti_reverse.group8()[0] * self.group7()[1])
                    - (anti_reverse.group8()[1] * self.group7()[0])
                    - (anti_reverse.group8()[2] * self.group1()[3])
                    - (self.group4()[1] * anti_reverse.group3()[0])
                    + (self.group4()[2] * anti_reverse.group9()[3])
                    + (self.group5()[0] * anti_reverse.group9()[1])
                    + (self.group5()[2] * anti_reverse.group3()[3])
                    - (self.group7()[2] * anti_reverse[e1])
                    + (self.group8()[2] * anti_reverse.group1()[3])
                    + (anti_reverse.group1()[1] * self.group6()[0])
                    + (anti_reverse.group3()[2] * self[e45])
                    + (anti_reverse.group6()[2] * self.group6()[3])
                    + (anti_reverse.group6()[3] * self.group6()[2])),
                (-(anti_reverse.group0()[0] * self[e45])
                    + (anti_reverse.group4()[0] * self.group5()[0])
                    + (anti_reverse.group4()[1] * self.group5()[1])
                    + (anti_reverse.group4()[2] * self.group5()[2])
                    + (anti_reverse.group4()[2] * self.group9()[2])
                    + (anti_reverse.group5()[0] * self.group4()[0])
                    + (anti_reverse.group5()[1] * self.group4()[1])
                    + (anti_reverse.group5()[2] * self.group4()[2])
                    + (anti_reverse.group7()[0] * self.group1()[0])
                    + (anti_reverse.group7()[1] * self.group1()[1])
                    - (anti_reverse.group7()[1] * self.group6()[1])
                    + (anti_reverse.group7()[2] * self.group1()[2])
                    - (anti_reverse.group7()[2] * self.group6()[2])
                    - (self.group4()[1] * anti_reverse.group9()[1])
                    - (self.group4()[2] * anti_reverse.group9()[2])
                    - (self.group7()[0] * anti_reverse.group1()[0])
                    - (self.group7()[1] * anti_reverse.group1()[1])
                    - (self.group7()[1] * anti_reverse.group6()[1])
                    - (self.group7()[2] * anti_reverse.group1()[2])
                    - (self.group7()[2] * anti_reverse.group6()[2])
                    - (anti_reverse.group1()[3] * self.group6()[3])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group1())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group1())
                - (Simd32x4::from(anti_reverse[e45]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[0]]))
                + (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group4()[0]]) * swizzle!(self.group9(), 0, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group4()[2], anti_reverse.group4()[0], anti_reverse.group4()[1], anti_reverse[e45]]) * swizzle!(self.group3(), 1, 2, 0, 3))
                + (Simd32x4::from([anti_reverse.group5()[2], anti_reverse.group5()[0], anti_reverse.group5()[1], anti_reverse.group4()[1]]) * swizzle!(self.group9(), 1, 2, 0, 1))
                + (Simd32x4::from([self.group4()[1], self.group4()[2], self.group4()[0], self[e45]]) * swizzle!(anti_reverse.group3(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group4()[0]]) * swizzle!(anti_reverse.group9(), 1, 2, 0, 0))
                - (Simd32x4::from([anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group7()[0]]) * swizzle!(self.group6(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group7()[0]]) * swizzle!(anti_reverse.group6(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group6()[3], self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(anti_reverse.group6(), 0, 0, 1, 3))),
            // e5
            (-(anti_reverse.group0()[0] * self.group9()[3]) + (anti_reverse.group0()[1] * self[e1]) - (self.group0()[0] * anti_reverse.group9()[3])
                + (self.group0()[1] * anti_reverse[e1])
                + (anti_reverse.group5()[0] * self.group3()[0])
                + (anti_reverse.group5()[1] * self.group3()[1])
                + (anti_reverse.group5()[2] * self.group3()[2])
                - (anti_reverse.group8()[0] * self.group1()[0])
                - (anti_reverse.group8()[0] * self.group6()[0])
                - (anti_reverse.group8()[1] * self.group1()[1])
                - (anti_reverse.group8()[1] * self.group6()[1])
                - (anti_reverse.group8()[2] * self.group1()[2])
                - (anti_reverse.group8()[2] * self.group6()[2])
                + (self.group5()[0] * anti_reverse.group3()[0])
                + (self.group5()[1] * anti_reverse.group3()[1])
                + (self.group5()[2] * anti_reverse.group3()[2])
                + (self.group8()[0] * anti_reverse.group1()[0])
                - (self.group8()[0] * anti_reverse.group6()[0])
                + (self.group8()[1] * anti_reverse.group1()[1])
                - (self.group8()[1] * anti_reverse.group6()[1])
                + (self.group8()[2] * anti_reverse.group1()[2])
                - (self.group8()[2] * anti_reverse.group6()[2])
                - (anti_reverse.group3()[0] * self.group9()[0])
                - (anti_reverse.group3()[1] * self.group9()[1])
                - (anti_reverse.group3()[2] * self.group9()[2])
                - (anti_reverse.group3()[3] * self.group9()[3])
                - (anti_reverse.group6()[3] * self[e1])
                + (anti_reverse.group9()[0] * self.group3()[0])
                + (anti_reverse.group9()[1] * self.group3()[1])
                + (anti_reverse.group9()[2] * self.group3()[2])
                + (anti_reverse.group9()[3] * self.group3()[3])
                + (self.group6()[3] * anti_reverse[e1])),
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
            (Simd32x4::from([
                (-(self.group0()[0] * anti_reverse.group1()[0]) - (anti_reverse.group4()[0] * self[e1]) + (anti_reverse.group4()[1] * self.group8()[2])
                    - (anti_reverse.group4()[2] * self.group8()[1])
                    + (anti_reverse.group5()[1] * self.group1()[2])
                    + (anti_reverse.group8()[0] * self[e45])
                    - (anti_reverse.group8()[1] * self.group4()[2])
                    + (anti_reverse.group8()[2] * self.group4()[1])
                    - (self.group5()[1] * anti_reverse.group1()[2])
                    - (self.group8()[0] * anti_reverse[e45])
                    + (anti_reverse.group3()[0] * self.group1()[3])
                    + (anti_reverse.group3()[3] * self.group6()[0])
                    - (anti_reverse.group6()[1] * self.group9()[2])
                    + (anti_reverse.group9()[2] * self.group6()[1])),
                (-(self.group0()[0] * anti_reverse.group1()[1]) - (anti_reverse.group4()[0] * self.group8()[2]) - (anti_reverse.group4()[1] * self[e1])
                    + (anti_reverse.group4()[2] * self.group8()[0])
                    + (anti_reverse.group5()[2] * self.group1()[0])
                    + (anti_reverse.group8()[0] * self.group4()[2])
                    + (anti_reverse.group8()[1] * self[e45])
                    - (anti_reverse.group8()[2] * self.group4()[0])
                    - (self.group5()[2] * anti_reverse.group1()[0])
                    - (self.group8()[1] * anti_reverse[e45])
                    + (anti_reverse.group3()[1] * self.group1()[3])
                    + (anti_reverse.group3()[3] * self.group6()[1])
                    - (anti_reverse.group6()[2] * self.group9()[0])
                    + (anti_reverse.group9()[0] * self.group6()[2])),
                (-(self.group0()[0] * anti_reverse.group1()[2]) + (anti_reverse.group4()[0] * self.group8()[1])
                    - (anti_reverse.group4()[1] * self.group8()[0])
                    - (anti_reverse.group4()[2] * self[e1])
                    + (anti_reverse.group5()[0] * self.group1()[1])
                    - (anti_reverse.group8()[0] * self.group4()[1])
                    + (anti_reverse.group8()[1] * self.group4()[0])
                    + (anti_reverse.group8()[2] * self[e45])
                    - (self.group5()[0] * anti_reverse.group1()[1])
                    - (self.group8()[2] * anti_reverse[e45])
                    + (anti_reverse.group3()[2] * self.group1()[3])
                    + (anti_reverse.group3()[3] * self.group6()[2])
                    - (anti_reverse.group6()[0] * self.group9()[1])
                    + (anti_reverse.group9()[1] * self.group6()[0])),
                ((anti_reverse.group0()[0] * self[e1])
                    - (anti_reverse.group5()[0] * self.group8()[0])
                    - (anti_reverse.group5()[1] * self.group8()[1])
                    - (anti_reverse.group5()[2] * self.group8()[2])
                    - (anti_reverse.group8()[0] * self.group5()[0])
                    - (anti_reverse.group8()[1] * self.group5()[1])
                    + (anti_reverse.group8()[1] * self.group9()[1])
                    - (anti_reverse.group8()[2] * self.group5()[2])
                    + (anti_reverse.group8()[2] * self.group9()[2])
                    - (self.group8()[1] * anti_reverse.group9()[1])
                    - (self.group8()[2] * anti_reverse.group9()[2])
                    - (anti_reverse.group3()[2] * self.group1()[2])
                    - (anti_reverse.group3()[2] * self.group6()[2])
                    - (self.group3()[3] * anti_reverse[e1])),
            ]) + (Simd32x4::from(anti_reverse.group0()[1]) * self.group9())
                + (Simd32x4::from(self.group0()[1]) * anti_reverse.group9())
                + (Simd32x4::from(anti_reverse.group9()[3]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[3]]))
                - (Simd32x4::from(self.group9()[3]) * Simd32x4::from([anti_reverse.group7()[0], anti_reverse.group7()[1], anti_reverse.group7()[2], anti_reverse.group6()[3]]))
                + (Simd32x4::from(anti_reverse[e1]) * Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group0()[0]]))
                - (Simd32x4::from([anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group3()[0]]) * swizzle!(self.group1(), 0, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group5()[0], anti_reverse.group5()[1], anti_reverse.group5()[2], anti_reverse.group3()[0]]) * swizzle!(self.group6(), 3, 3, 3, 0))
                - (Simd32x4::from([anti_reverse.group5()[2], anti_reverse.group5()[0], anti_reverse.group5()[1], anti_reverse.group3()[1]]) * swizzle!(self.group1(), 1, 2, 0, 1))
                + (Simd32x4::from([anti_reverse.group7()[1], anti_reverse.group7()[2], anti_reverse.group7()[0], anti_reverse.group1()[0]]) * swizzle!(self.group3(), 2, 0, 1, 0))
                - (Simd32x4::from([anti_reverse.group7()[2], anti_reverse.group7()[0], anti_reverse.group7()[1], anti_reverse.group6()[0]]) * swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group5()[0], self.group5()[1], self.group5()[2], self.group3()[1]]) * swizzle!(anti_reverse.group6(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group3()[1]]) * swizzle!(anti_reverse.group1(), 1, 2, 0, 1))
                + (Simd32x4::from([self.group7()[1], self.group7()[2], self.group7()[0], self[e1]]) * swizzle!(anti_reverse.group3(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group6()[1]]) * swizzle!(anti_reverse.group3(), 1, 2, 0, 1))
                - (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group1()[3], anti_reverse.group6()[2]]) * swizzle!(self.group3(), 0, 1, 2, 2))
                + (Simd32x4::from([anti_reverse.group6()[0], anti_reverse.group6()[1], anti_reverse.group6()[2], anti_reverse.group1()[2]]) * swizzle!(self.group3(), 3, 3, 3, 2))
                + (Simd32x4::from([anti_reverse.group6()[2], anti_reverse.group6()[0], anti_reverse.group6()[1], anti_reverse.group8()[0]]) * swizzle!(self.group9(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group8()[0]]) * swizzle!(anti_reverse.group9(), 1, 2, 0, 0))),
            // e1234
            ((anti_reverse.group0()[0] * self.group1()[3])
                + (anti_reverse.group0()[1] * self[e45])
                + (self.group0()[0] * anti_reverse.group1()[3])
                + (self.group0()[1] * anti_reverse[e45])
                + (anti_reverse.group4()[0] * self.group1()[0])
                - (anti_reverse.group4()[0] * self.group6()[0])
                + (anti_reverse.group4()[1] * self.group1()[1])
                - (anti_reverse.group4()[1] * self.group6()[1])
                + (anti_reverse.group4()[2] * self.group1()[2])
                - (anti_reverse.group4()[2] * self.group6()[2])
                - (anti_reverse.group5()[0] * self.group7()[0])
                - (anti_reverse.group5()[1] * self.group7()[1])
                - (anti_reverse.group5()[2] * self.group7()[2])
                - (anti_reverse.group7()[0] * self.group5()[0])
                - (anti_reverse.group7()[0] * self.group9()[0])
                - (anti_reverse.group7()[1] * self.group5()[1])
                - (anti_reverse.group7()[1] * self.group9()[1])
                - (anti_reverse.group7()[2] * self.group5()[2])
                - (anti_reverse.group7()[2] * self.group9()[2])
                - (self.group4()[0] * anti_reverse.group1()[0])
                - (self.group4()[0] * anti_reverse.group6()[0])
                - (self.group4()[1] * anti_reverse.group1()[1])
                - (self.group4()[1] * anti_reverse.group6()[1])
                - (self.group4()[2] * anti_reverse.group1()[2])
                - (self.group4()[2] * anti_reverse.group6()[2])
                + (self.group7()[0] * anti_reverse.group9()[0])
                + (self.group7()[1] * anti_reverse.group9()[1])
                + (self.group7()[2] * anti_reverse.group9()[2])
                + (anti_reverse.group1()[3] * self.group3()[3])
                - (anti_reverse.group3()[3] * self.group1()[3])
                + (anti_reverse.group6()[3] * self[e45])
                - (self.group6()[3] * anti_reverse[e45])),
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
                + f32::powi(self.group9()[0], 2)
                + f32::powi(self.group9()[1], 2)
                + f32::powi(self.group9()[2], 2)
                + 2.0 * (self.group4()[0] * self.group3()[0])
                + 2.0 * (self.group4()[1] * self.group3()[1])
                + 2.0 * (self.group4()[2] * self.group3()[2])
                - 2.0 * (self.group7()[0] * self.group8()[0])
                - 2.0 * (self.group7()[1] * self.group8()[1])
                - 2.0 * (self.group7()[2] * self.group8()[2])
                + 2.0 * (self.group1()[3] * self[e1])
                - 2.0 * (self.group9()[3] * self[e45])),
        );
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
            // e1, e2, e3, e4
            geometric_anti_product.group1(),
            // e5
            geometric_anti_product[e1],
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
            geometric_anti_product.group9(),
            // e1234
            geometric_anti_product[e45],
        );
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
impl AntiConstraintViolation for RoundPoint {
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
impl AntiConstraintViolation for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       70        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       72       81        0
    //  no simd      102      114        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group2()[0]) - (anti_reverse.group0()[1] * self.group2()[1]) - (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group0()[3] * self.group0()[3])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group1()[3] * self.group1()[3])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])
                    + (anti_reverse.group2()[3] * self.group3()[3])
                    - (anti_reverse.group3()[0] * self.group3()[0])
                    - (anti_reverse.group3()[1] * self.group3()[1])
                    - (anti_reverse.group3()[2] * self.group3()[2])
                    + (anti_reverse.group3()[3] * self.group2()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[3] * self.group2()[3])
                    - (anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group1()[3] * self.group2()[3])
                    - (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[0] * self.group3()[0])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group3()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group3()[2])
                    + (anti_reverse.group2()[3] * self.group0()[3])
                    + (anti_reverse.group2()[3] * self.group1()[3])
                    + (anti_reverse.group3()[0] * self.group2()[0])
                    + (anti_reverse.group3()[1] * self.group2()[1])
                    + (anti_reverse.group3()[2] * self.group2()[2])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
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
    //      f32       54       62        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       66       75        0
    //  no simd      102      114        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group2()[0]) + (anti_reverse.group0()[1] * self.group2()[1]) + (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group0()[3] * self.group0()[3])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group1()[3] * self.group1()[3])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group3()[3])
                    + (anti_reverse.group3()[0] * self.group3()[0])
                    + (anti_reverse.group3()[1] * self.group3()[1])
                    + (anti_reverse.group3()[2] * self.group3()[2])
                    - (anti_reverse.group3()[3] * self.group2()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[3] * self.group3()[3])
                    + (anti_reverse.group1()[0] * self.group2()[0])
                    + (anti_reverse.group1()[1] * self.group2()[1])
                    + (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group1()[3] * self.group3()[3])
                    + (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[0] * self.group3()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[1] * self.group3()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])
                    - (anti_reverse.group2()[2] * self.group3()[2])
                    + (anti_reverse.group3()[0] * self.group2()[0])
                    + (anti_reverse.group3()[1] * self.group2()[1])
                    + (anti_reverse.group3()[2] * self.group2()[2])
                    - (anti_reverse.group3()[3] * self.group0()[3])
                    + (anti_reverse.group3()[3] * self.group1()[3])),
            ]),
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
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
        return subtraction;
    }
}
