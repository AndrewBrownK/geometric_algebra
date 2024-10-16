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
//  Average:        30      33       0
//  Maximum:       284     294       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        14      16       0
//  Average:        41      47       0
//  Maximum:       396     420       0
impl ConstraintViolation for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       42        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       37       46        0
    //  no simd       43       57        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group2()[0])
                    - (reverse.group0()[1] * self.group2()[1])
                    - (reverse.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * reverse.group2()[0])
                    - (self.group0()[1] * reverse.group2()[1])
                    - (self.group0()[2] * reverse.group2()[2])
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    + (reverse.group1()[3] * self.group1()[3])
                    + (reverse.group2()[3] * self.group2()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])
                    - (self.group0()[2] * reverse.group1()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[1] * self.group2()[2])
                    + (self.group0()[1] * reverse.group2()[2])
                    + (reverse.group1()[0] * self.group1()[3])
                    + (reverse.group1()[3] * self.group1()[0])),
                ((reverse.group0()[2] * self.group2()[0])
                    + (self.group0()[2] * reverse.group2()[0])
                    + (reverse.group1()[1] * self.group1()[3])
                    + (reverse.group1()[3] * self.group1()[1])),
                ((reverse.group0()[0] * self.group2()[1])
                    + (self.group0()[0] * reverse.group2()[1])
                    + (reverse.group1()[2] * self.group1()[3])
                    + (reverse.group1()[3] * self.group1()[2])),
                (-(reverse.group1()[1] * self.group2()[1])
                    - (reverse.group1()[2] * self.group2()[2])
                    - (reverse.group2()[1] * self.group1()[1])
                    - (reverse.group2()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 1, 2, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2) + f32::powi(self.group2()[3], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       48        0
    //    simd3        0        1        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       54       62        0
    //  no simd       90      103        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group2()[0])
                    + (reverse.group0()[1] * self.group2()[1])
                    + (reverse.group0()[2] * self.group2()[2])
                    + (self.group0()[0] * reverse.group2()[0])
                    + (self.group0()[1] * reverse.group2()[1])
                    + (self.group0()[2] * reverse.group2()[2])
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group1()[3] * self.group1()[3])
                    - (reverse.group2()[3] * self.group3()[3])
                    + (reverse.group3()[0] * self.group3()[0])
                    + (reverse.group3()[1] * self.group3()[1])
                    + (reverse.group3()[2] * self.group3()[2])
                    - (reverse.group3()[3] * self.group2()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group1()[0])
                    + (reverse.group0()[0] * self.group3()[0])
                    + (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[1] * self.group3()[1])
                    + (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group0()[2] * self.group3()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[0] * reverse.group3()[0])
                    + (self.group0()[1] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group3()[1])
                    + (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group3()[2])
                    + (reverse.group1()[3] * self.group2()[3])
                    - (reverse.group2()[3] * self.group1()[3])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-(reverse.group0()[1] * self.group2()[2]) - (reverse.group3()[2] * self.group1()[1])),
                (-(reverse.group0()[2] * self.group2()[0]) - (reverse.group3()[0] * self.group1()[2])),
                (-(reverse.group0()[0] * self.group2()[1]) - (reverse.group3()[1] * self.group1()[0])),
                ((reverse.group3()[2] * self.group2()[2]) + (reverse.group3()[3] * self.group1()[3])),
            ]) - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * swizzle!(reverse.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]) * swizzle!(reverse.group2(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[1]]) * swizzle!(self.group3(), 1, 2, 0, 1))
                + (Simd32x4::from([reverse.group1()[3], reverse.group1()[3], reverse.group1()[3], reverse.group2()[1]]) * swizzle!(self.group1(), 0, 1, 2, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * swizzle!(reverse.group3(), 1, 2, 0, 1))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group3()[1], self.group2()[1]]) * swizzle!(reverse.group1(), 0, 1, 0, 1))
                - (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[2]]) * swizzle!(reverse.group2(), 3, 3, 3, 2))
                + (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[2]]) * swizzle!(reverse.group2(), 0, 1, 2, 2))
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group1()[3], self.group2()[2]]) * swizzle!(reverse.group1(), 1, 2, 2, 2))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group3()[0], 2)
                + f32::powi(self.group3()[1], 2)
                + f32::powi(self.group3()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
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
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
        let subtraction = AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product.group0()[0], 0.0]));
        return subtraction;
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
        let reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3] * -1.0));
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       14       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3, e5 */ self.group1());
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[3] * self.group0()[3])
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    - (reverse.group0()[3] * self.group1()[3])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       10       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiLine::from_groups(
            // e23, e31, e12
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       18        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])
                    + (reverse.group0()[3] * self.group0()[3])),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0]) - (reverse.group0()[1] * self.group1()[1]) - (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group0()[3] * self.group1()[3])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiPlane {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       38        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       35       43        0
    //  no simd       41       56        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Circle::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group2()[0])
                    + (reverse.group0()[1] * self.group2()[1])
                    + (reverse.group0()[2] * self.group2()[2])
                    + (reverse.group2()[0] * self.group0()[0])
                    + (reverse.group2()[1] * self.group0()[1])
                    + (reverse.group2()[2] * self.group0()[2])
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group1()[3] * self.group1()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group1()[0])
                    + (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[2] * self.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])
                    + (self.group0()[2] * reverse.group1()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-(reverse.group0()[1] * self.group2()[2]) + (reverse.group0()[2] * self.group2()[1]) + (reverse.group2()[1] * self.group0()[2])
                    - (reverse.group2()[2] * self.group0()[1])),
                ((reverse.group0()[0] * self.group2()[2]) - (reverse.group0()[2] * self.group2()[0]) - (reverse.group2()[0] * self.group0()[2])
                    + (reverse.group2()[2] * self.group0()[0])),
                (-(reverse.group0()[0] * self.group2()[1]) + (reverse.group0()[1] * self.group2()[0]) + (reverse.group2()[0] * self.group0()[1])
                    - (reverse.group2()[1] * self.group0()[0])),
                ((reverse.group2()[2] * self.group1()[2])
                    + (self.group2()[0] * reverse.group1()[0])
                    + (self.group2()[1] * reverse.group1()[1])
                    + (self.group2()[2] * reverse.group1()[2])),
            ]) + (Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[0]]) * swizzle!(self.group1(), 3, 3, 3, 0))
                + (Simd32x4::from([reverse.group1()[3], reverse.group1()[3], reverse.group1()[3], reverse.group2()[1]]) * swizzle!(self.group1(), 0, 1, 2, 1))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       34        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       31       40        0
    //  no simd       43       57        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group2()[0])
                    + (reverse.group0()[1] * self.group2()[1])
                    + (reverse.group0()[2] * self.group2()[2])
                    + (self.group0()[0] * reverse.group2()[0])
                    + (self.group0()[1] * reverse.group2()[1])
                    + (self.group0()[2] * reverse.group2()[2])
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group1()[3] * self.group1()[3])
                    - (reverse.group2()[3] * self.group2()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group1()[0])
                    + (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[2] * self.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])
                    + (self.group0()[2] * reverse.group1()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-(reverse.group0()[1] * self.group2()[2]) - (self.group0()[1] * reverse.group2()[2])),
                (-(reverse.group0()[2] * self.group2()[0]) - (self.group0()[2] * reverse.group2()[0])),
                (-(reverse.group0()[0] * self.group2()[1]) - (self.group0()[0] * reverse.group2()[1])),
                ((reverse.group2()[1] * self.group1()[1]) + (reverse.group2()[2] * self.group1()[2])),
            ]) + (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[2]]) * swizzle!(reverse.group1(), 3, 3, 3, 2))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[1]]) * swizzle!(reverse.group1(), 0, 1, 2, 1))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2) - f32::powi(self.group2()[3], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       46        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       41       49        0
    //  no simd       41       56        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Dipole::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(-1.0)),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group2()[0])
                    - (reverse.group0()[1] * self.group2()[1])
                    - (reverse.group0()[2] * self.group2()[2])
                    - (reverse.group2()[0] * self.group0()[0])
                    - (reverse.group2()[1] * self.group0()[1])
                    - (reverse.group2()[2] * self.group0()[2])
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    + (reverse.group1()[3] * self.group1()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])
                    - (self.group0()[2] * reverse.group1()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((reverse.group0()[1] * self.group2()[2]) - (reverse.group0()[2] * self.group2()[1]) - (reverse.group2()[1] * self.group0()[2])
                    + (reverse.group2()[2] * self.group0()[1])
                    + (reverse.group1()[0] * self.group1()[3])
                    + (reverse.group1()[3] * self.group1()[0])),
                (-(reverse.group0()[0] * self.group2()[2]) + (reverse.group0()[2] * self.group2()[0]) + (reverse.group2()[0] * self.group0()[2])
                    - (reverse.group2()[2] * self.group0()[0])
                    + (reverse.group1()[1] * self.group1()[3])
                    + (reverse.group1()[3] * self.group1()[1])),
                ((reverse.group0()[0] * self.group2()[1]) - (reverse.group0()[1] * self.group2()[0]) - (reverse.group2()[0] * self.group0()[1])
                    + (reverse.group2()[1] * self.group0()[0])
                    + (reverse.group1()[2] * self.group1()[3])
                    + (reverse.group1()[3] * self.group1()[2])),
                (-(reverse.group2()[0] * self.group1()[0])
                    - (reverse.group2()[1] * self.group1()[1])
                    - (reverse.group2()[2] * self.group1()[2])
                    - (self.group2()[0] * reverse.group1()[0])
                    - (self.group2()[1] * reverse.group1()[1])
                    - (self.group2()[2] * reverse.group1()[2])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       60        0
    //    simd3        0        1        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       63       71        0
    //  no simd       90      103        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group2()[0])
                    - (reverse.group0()[1] * self.group2()[1])
                    - (reverse.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * reverse.group2()[0])
                    - (self.group0()[1] * reverse.group2()[1])
                    - (self.group0()[2] * reverse.group2()[2])
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    + (reverse.group1()[3] * self.group1()[3])
                    + (reverse.group2()[3] * self.group3()[3])
                    - (reverse.group3()[0] * self.group3()[0])
                    - (reverse.group3()[1] * self.group3()[1])
                    - (reverse.group3()[2] * self.group3()[2])
                    + (reverse.group3()[3] * self.group2()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0]) + (reverse.group0()[0] * self.group3()[0]) - (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[1] * self.group3()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group0()[2] * self.group3()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[0] * reverse.group3()[0])
                    - (self.group0()[1] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group3()[1])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group3()[2])
                    + (reverse.group1()[3] * self.group2()[3])
                    - (reverse.group2()[3] * self.group1()[3])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[0] * self.group3()[3]) - (self.group0()[0] * reverse.group3()[3])
                    + (self.group0()[1] * reverse.group2()[2])
                    + (reverse.group1()[1] * self.group3()[2])
                    + (reverse.group1()[3] * self.group1()[0])),
                ((reverse.group0()[1] * self.group3()[3]) - (self.group0()[1] * reverse.group3()[3])
                    + (self.group0()[2] * reverse.group2()[0])
                    + (reverse.group1()[2] * self.group3()[0])
                    + (reverse.group1()[3] * self.group1()[1])),
                ((reverse.group0()[2] * self.group3()[3]) + (self.group0()[0] * reverse.group2()[1]) - (self.group0()[2] * reverse.group3()[3])
                    + (reverse.group1()[0] * self.group3()[1])
                    + (reverse.group1()[3] * self.group1()[2])),
                (-(reverse.group1()[3] * self.group3()[3])
                    - (reverse.group2()[0] * self.group3()[0])
                    - (reverse.group2()[1] * self.group3()[1])
                    - (reverse.group2()[2] * self.group1()[2])
                    - (reverse.group2()[2] * self.group3()[2])),
            ]) + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group3()[3]]))
                + (Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse.group0()[0], reverse.group3()[0]]) * swizzle!(self.group2(), 2, 0, 1, 0))
                - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group1()[2]]) * swizzle!(self.group2(), 3, 3, 3, 2))
                + (Simd32x4::from([reverse.group2()[3], reverse.group2()[3], reverse.group2()[3], reverse.group3()[1]]) * swizzle!(self.group2(), 0, 1, 2, 1))
                - (Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group2()[1]]) * swizzle!(self.group1(), 1, 2, 0, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]) * swizzle!(reverse.group3(), 1, 2, 0, 2))
                - (Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group2()[1]]) * swizzle!(reverse.group1(), 2, 0, 1, 1))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group3()[0], 2)
                - f32::powi(self.group3()[1], 2)
                - f32::powi(self.group3()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
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
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([(self.group0()[0] * self.group0()[1]), f32::powi(self.group0()[1], 2)]) * Simd32x2::from([-2.0, -1.0])),
        );
        let subtraction = AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product.group0()[0], 0.0]));
        return subtraction;
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
        let reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3]));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[3], 2));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       14       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[3] * self.group0()[3])
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    - (reverse.group0()[3] * self.group1()[3])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       10       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group1()[0])
                    + (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       18        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])
                    - (reverse.group0()[3] * self.group0()[3])),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group1()[0]) + (reverse.group0()[1] * self.group1()[1]) + (reverse.group0()[2] * self.group1()[2])
                    - (reverse.group0()[3] * self.group1()[3])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])
                    - (reverse.group1()[3] * self.group0()[3])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group1()[3]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      236      240        0
    //    simd2       16       16        0
    //    simd3        0        4        0
    //    simd4       32       34        0
    // Totals...
    // yes simd      284      294        0
    //  no simd      396      420        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MultiVector::from_groups(
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
        let geometric_product = MultiVector::from_groups(
            // scalar, e12345
            (Simd32x2::from([
                (-(reverse.group0()[1] * self.group0()[1])
                    + (reverse.group7()[0] * self.group8()[0])
                    + (reverse.group7()[1] * self.group8()[1])
                    + (reverse.group7()[2] * self.group8()[2])
                    + (reverse.group8()[0] * self.group7()[0])
                    + (reverse.group8()[1] * self.group7()[1])
                    + (reverse.group8()[2] * self.group7()[2])
                    - (reverse.group1()[3] * self[e1])
                    + (reverse.group3()[3] * self.group3()[3])
                    + (reverse.group6()[0] * self.group6()[0])
                    + (reverse.group6()[1] * self.group6()[1])
                    + (reverse.group6()[2] * self.group6()[2])
                    - (reverse.group9()[0] * self.group9()[0])
                    - (reverse.group9()[1] * self.group9()[1])
                    - (reverse.group9()[2] * self.group9()[2])
                    - (self.group1()[3] * reverse[e1])),
                ((reverse.group0()[1] * self.group0()[0])
                    - (reverse.group7()[0] * self.group3()[0])
                    - (reverse.group7()[1] * self.group3()[1])
                    - (reverse.group7()[2] * self.group3()[2])
                    - (self.group5()[0] * reverse.group6()[0])
                    - (self.group5()[1] * reverse.group6()[1])
                    - (self.group5()[2] * reverse.group6()[2])
                    - (self.group7()[0] * reverse.group3()[0])
                    - (self.group7()[1] * reverse.group3()[1])
                    - (self.group7()[2] * reverse.group3()[2])
                    - (reverse.group6()[3] * self.group3()[3])
                    + (reverse.group9()[0] * self.group1()[0])
                    + (reverse.group9()[1] * self.group1()[1])
                    + (reverse.group9()[2] * self.group1()[2])
                    + (reverse[e1] * self[e45])
                    + (reverse[e45] * self[e1])),
            ]) + (Simd32x2::from(reverse.group0()[0]) * self.group0())
                - (Simd32x2::from(reverse.group4()[0]) * Simd32x2::from([self.group3()[0], self.group8()[0]]))
                - (Simd32x2::from(reverse.group4()[1]) * Simd32x2::from([self.group3()[1], self.group8()[1]]))
                - (Simd32x2::from(reverse.group4()[2]) * Simd32x2::from([self.group3()[2], self.group8()[2]]))
                - (Simd32x2::from(reverse.group5()[0]) * Simd32x2::from([self.group5()[0], self.group6()[0]]))
                - (Simd32x2::from(reverse.group5()[1]) * Simd32x2::from([self.group5()[1], self.group6()[1]]))
                - (Simd32x2::from(reverse.group5()[2]) * Simd32x2::from([self.group5()[2], self.group6()[2]]))
                - (Simd32x2::from(self.group4()[0]) * Simd32x2::from([reverse.group3()[0], reverse.group8()[0]]))
                - (Simd32x2::from(self.group4()[1]) * Simd32x2::from([reverse.group3()[1], reverse.group8()[1]]))
                - (Simd32x2::from(self.group4()[2]) * Simd32x2::from([reverse.group3()[2], reverse.group8()[2]]))
                + (Simd32x2::from(reverse.group1()[0]) * Simd32x2::from([self.group1()[0], self.group9()[0]]))
                + (Simd32x2::from(reverse.group1()[1]) * Simd32x2::from([self.group1()[1], self.group9()[1]]))
                + (Simd32x2::from(reverse.group1()[2]) * Simd32x2::from([self.group1()[2], self.group9()[2]]))
                + (Simd32x2::from(reverse.group9()[3]) * Simd32x2::from([self[e45], self.group1()[3]]))
                - (Simd32x2::from(self.group6()[3]) * Simd32x2::from([reverse.group6()[3], reverse.group3()[3]]))
                + (Simd32x2::from(self.group9()[3]) * Simd32x2::from([reverse[e45], reverse.group1()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-(reverse.group0()[1] * self.group9()[0]) - (reverse.group4()[0] * self[e1]) - (reverse.group4()[1] * self.group8()[2])
                    + (reverse.group4()[2] * self.group8()[1])
                    - (reverse.group7()[0] * self.group9()[3])
                    + (reverse.group7()[2] * self.group3()[1])
                    + (reverse.group8()[1] * self.group4()[2])
                    - (reverse.group8()[2] * self.group4()[1])
                    + (self.group4()[0] * reverse[e1])
                    + (self.group5()[0] * reverse.group6()[3])
                    - (self.group5()[1] * reverse.group1()[2])
                    + (self.group7()[0] * reverse.group9()[3])
                    - (self.group7()[1] * reverse.group3()[2])
                    + (self.group7()[2] * reverse.group3()[1])
                    - (self.group8()[0] * reverse[e45])
                    - (reverse.group1()[3] * self.group3()[0])
                    + (reverse.group3()[0] * self.group1()[3])
                    + (reverse.group9()[2] * self.group6()[1])),
                (-(reverse.group0()[1] * self.group9()[1]) + (reverse.group4()[0] * self.group8()[2])
                    - (reverse.group4()[1] * self[e1])
                    - (reverse.group4()[2] * self.group8()[0])
                    + (reverse.group7()[0] * self.group3()[2])
                    - (reverse.group7()[1] * self.group9()[3])
                    - (reverse.group8()[0] * self.group4()[2])
                    + (reverse.group8()[2] * self.group4()[0])
                    + (self.group4()[1] * reverse[e1])
                    + (self.group5()[1] * reverse.group6()[3])
                    - (self.group5()[2] * reverse.group1()[0])
                    + (self.group7()[0] * reverse.group3()[2])
                    + (self.group7()[1] * reverse.group9()[3])
                    - (self.group7()[2] * reverse.group3()[0])
                    - (self.group8()[1] * reverse[e45])
                    - (reverse.group1()[3] * self.group3()[1])
                    + (reverse.group3()[1] * self.group1()[3])
                    + (reverse.group9()[0] * self.group6()[2])),
                (-(reverse.group0()[1] * self.group9()[2]) - (reverse.group4()[0] * self.group8()[1]) + (reverse.group4()[1] * self.group8()[0])
                    - (reverse.group4()[2] * self[e1])
                    + (reverse.group7()[1] * self.group3()[0])
                    - (reverse.group7()[2] * self.group9()[3])
                    + (reverse.group8()[0] * self.group4()[1])
                    - (reverse.group8()[1] * self.group4()[0])
                    + (self.group4()[2] * reverse[e1])
                    - (self.group5()[0] * reverse.group1()[1])
                    + (self.group5()[2] * reverse.group6()[3])
                    - (self.group7()[0] * reverse.group3()[1])
                    + (self.group7()[1] * reverse.group3()[0])
                    + (self.group7()[2] * reverse.group9()[3])
                    - (self.group8()[2] * reverse[e45])
                    - (reverse.group1()[3] * self.group3()[2])
                    + (reverse.group3()[2] * self.group1()[3])
                    + (reverse.group9()[1] * self.group6()[0])),
                ((self.group0()[1] * reverse[e45])
                    - (reverse.group4()[1] * self.group1()[1])
                    - (reverse.group4()[2] * self.group1()[2])
                    - (reverse.group4()[2] * self.group6()[2])
                    - (reverse.group5()[0] * self.group7()[0])
                    - (reverse.group5()[1] * self.group7()[1])
                    - (reverse.group5()[2] * self.group7()[2])
                    - (reverse.group7()[0] * self.group5()[0])
                    - (reverse.group7()[1] * self.group5()[1])
                    + (reverse.group7()[1] * self.group9()[1])
                    - (reverse.group7()[2] * self.group5()[2])
                    + (reverse.group7()[2] * self.group9()[2])
                    + (self.group4()[1] * reverse.group1()[1])
                    + (self.group4()[2] * reverse.group1()[2])
                    - (self.group4()[2] * reverse.group6()[2])
                    - (self.group7()[1] * reverse.group9()[1])
                    - (self.group7()[2] * reverse.group9()[2])
                    - (reverse.group6()[3] * self[e45])),
            ]) + (Simd32x4::from(reverse.group0()[0]) * self.group1())
                + (Simd32x4::from(self.group0()[0]) * reverse.group1())
                + (Simd32x4::from(self.group6()[3]) * Simd32x4::from([reverse.group5()[0], reverse.group5()[1], reverse.group5()[2], reverse[e45]]))
                + (Simd32x4::from(self[e45]) * Simd32x4::from([reverse.group8()[0], reverse.group8()[1], reverse.group8()[2], reverse.group0()[1]]))
                - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group7()[0]]) * swizzle!(reverse.group9(), 0, 1, 2, 0))
                + (Simd32x4::from([reverse.group5()[1], reverse.group5()[2], reverse.group5()[0], reverse.group3()[3]]) * swizzle!(self.group1(), 2, 0, 1, 3))
                - (Simd32x4::from([reverse.group5()[2], reverse.group5()[0], reverse.group5()[1], reverse.group4()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group7()[1], reverse.group7()[2], reverse.group7()[0], reverse.group1()[3]]) * swizzle!(self.group3(), 2, 0, 1, 3))
                + (Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group4()[0]]) * swizzle!(reverse.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group3()[3], reverse.group3()[3], reverse.group3()[3], reverse.group4()[0]]) * swizzle!(self.group6(), 0, 1, 2, 0))
                + (Simd32x4::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1], reverse.group7()[0]]) * swizzle!(self.group9(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group9()[1], reverse.group9()[2], reverse.group9()[0], reverse.group4()[1]]) * swizzle!(self.group6(), 2, 0, 1, 1))
                - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group9()[1], self.group4()[0]]) * swizzle!(reverse.group6(), 0, 1, 0, 0))
                - (Simd32x4::from([self.group9()[2], self.group9()[0], self.group3()[3], self.group4()[1]]) * swizzle!(reverse.group6(), 1, 2, 2, 1))),
            // e5
            ((reverse.group0()[0] * self[e1]) + (reverse.group0()[1] * self.group9()[3]) + (self.group0()[0] * reverse[e1]) + (self.group0()[1] * reverse.group9()[3])
                - (reverse.group5()[0] * self.group8()[0])
                - (reverse.group5()[1] * self.group8()[1])
                - (reverse.group5()[2] * self.group8()[2])
                - (reverse.group8()[0] * self.group5()[0])
                - (reverse.group8()[0] * self.group9()[0])
                - (reverse.group8()[1] * self.group5()[1])
                - (reverse.group8()[1] * self.group9()[1])
                - (reverse.group8()[2] * self.group5()[2])
                - (reverse.group8()[2] * self.group9()[2])
                + (self.group8()[0] * reverse.group9()[0])
                + (self.group8()[1] * reverse.group9()[1])
                + (self.group8()[2] * reverse.group9()[2])
                - (reverse.group1()[0] * self.group3()[0])
                - (reverse.group1()[1] * self.group3()[1])
                - (reverse.group1()[2] * self.group3()[2])
                + (reverse.group3()[0] * self.group1()[0])
                - (reverse.group3()[0] * self.group6()[0])
                + (reverse.group3()[1] * self.group1()[1])
                - (reverse.group3()[1] * self.group6()[1])
                + (reverse.group3()[2] * self.group1()[2])
                - (reverse.group3()[2] * self.group6()[2])
                - (reverse.group3()[3] * self[e1])
                - (reverse.group6()[0] * self.group3()[0])
                - (reverse.group6()[1] * self.group3()[1])
                - (reverse.group6()[2] * self.group3()[2])
                + (reverse.group6()[3] * self.group9()[3])
                - (reverse.group9()[3] * self.group6()[3])
                + (self.group3()[3] * reverse[e1])),
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
                ((reverse.group0()[1] * self.group1()[0]) + (reverse.group4()[0] * self.group9()[3]) + (reverse.group5()[1] * self.group9()[2])
                    - (reverse.group7()[1] * self.group8()[2])
                    + (reverse.group7()[2] * self.group8()[1])
                    + (reverse.group8()[0] * self.group1()[3])
                    + (reverse.group8()[1] * self.group7()[2])
                    - (reverse.group8()[2] * self.group7()[1])
                    - (self.group4()[0] * reverse.group9()[3])
                    + (self.group4()[1] * reverse.group3()[2])
                    + (self.group5()[0] * reverse.group3()[3])
                    - (self.group5()[1] * reverse.group9()[2])
                    - (self.group8()[0] * reverse.group1()[3])
                    - (reverse.group1()[2] * self.group6()[1])),
                ((reverse.group0()[1] * self.group1()[1])
                    + (reverse.group4()[1] * self.group9()[3])
                    + (reverse.group5()[2] * self.group9()[0])
                    + (reverse.group7()[0] * self.group8()[2])
                    - (reverse.group7()[2] * self.group8()[0])
                    - (reverse.group8()[0] * self.group7()[2])
                    + (reverse.group8()[1] * self.group1()[3])
                    + (reverse.group8()[2] * self.group7()[0])
                    - (self.group4()[1] * reverse.group9()[3])
                    + (self.group4()[2] * reverse.group3()[0])
                    + (self.group5()[1] * reverse.group3()[3])
                    - (self.group5()[2] * reverse.group9()[0])
                    - (self.group8()[1] * reverse.group1()[3])
                    - (reverse.group1()[0] * self.group6()[2])),
                ((reverse.group0()[1] * self.group1()[2]) + (reverse.group4()[2] * self.group9()[3]) + (reverse.group5()[0] * self.group9()[1])
                    - (reverse.group7()[0] * self.group8()[1])
                    + (reverse.group7()[1] * self.group8()[0])
                    + (reverse.group8()[0] * self.group7()[1])
                    - (reverse.group8()[1] * self.group7()[0])
                    + (reverse.group8()[2] * self.group1()[3])
                    + (self.group4()[0] * reverse.group3()[1])
                    - (self.group4()[2] * reverse.group9()[3])
                    - (self.group5()[0] * reverse.group9()[1])
                    + (self.group5()[2] * reverse.group3()[3])
                    - (self.group8()[2] * reverse.group1()[3])
                    - (reverse.group1()[1] * self.group6()[0])),
                (-(self.group0()[1] * reverse[e1])
                    - (reverse.group5()[1] * self.group3()[1])
                    - (reverse.group5()[2] * self.group3()[2])
                    - (reverse.group8()[1] * self.group1()[1])
                    - (reverse.group8()[2] * self.group1()[2])
                    + (reverse.group8()[2] * self.group6()[2])
                    - (self.group5()[2] * reverse.group3()[2])
                    + (self.group8()[1] * reverse.group1()[1])
                    + (self.group8()[2] * reverse.group1()[2])
                    + (self.group8()[2] * reverse.group6()[2])
                    - (reverse.group3()[1] * self.group9()[1])
                    - (reverse.group3()[2] * self.group9()[2])
                    - (reverse.group3()[3] * self.group9()[3])
                    - (reverse.group6()[3] * self[e1])),
            ]) + (Simd32x4::from(reverse.group0()[0]) * self.group9())
                + (Simd32x4::from(self.group0()[0]) * reverse.group9())
                + (Simd32x4::from(reverse[e1]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[3]]))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([reverse.group7()[0], reverse.group7()[1], reverse.group7()[2], reverse.group0()[1]]))
                + (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group8()[0]]) * swizzle!(reverse.group1(), 0, 1, 2, 0))
                + (Simd32x4::from([reverse.group4()[1], reverse.group4()[2], reverse.group4()[0], reverse.group9()[0]]) * swizzle!(self.group3(), 2, 0, 1, 0))
                - (Simd32x4::from([reverse.group4()[2], reverse.group4()[0], reverse.group4()[1], reverse.group5()[0]]) * swizzle!(self.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group5()[0], reverse.group5()[1], reverse.group5()[2], reverse.group9()[1]]) * swizzle!(self.group3(), 3, 3, 3, 1))
                - (Simd32x4::from([reverse.group5()[2], reverse.group5()[0], reverse.group5()[1], reverse.group3()[0]]) * swizzle!(self.group9(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * swizzle!(reverse.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group3()[2]]) * swizzle!(reverse.group9(), 1, 2, 0, 2))
                + (Simd32x4::from([reverse.group1()[1], reverse.group1()[2], reverse.group1()[0], reverse.group8()[0]]) * swizzle!(self.group6(), 2, 0, 1, 0))
                - (Simd32x4::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1], reverse.group8()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group6()[3], reverse.group6()[3], reverse.group6()[3], reverse.group8()[1]]) * swizzle!(self.group6(), 0, 1, 2, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group6()[3], self.group8()[1]]) * swizzle!(reverse.group6(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group6()[3], self.group6()[3], self.group1()[1], self.group8()[0]]) * swizzle!(reverse.group6(), 0, 1, 0, 0))
                + (Simd32x4::from([reverse[e45], reverse[e45], reverse[e45], reverse.group9()[3]]) * self.group3())
                - (Simd32x4::from([self[e45], self[e45], self[e45], self.group5()[1]]) * swizzle!(reverse.group3(), 0, 1, 2, 1))),
            // e1234
            ((reverse.group0()[0] * self[e45]) - (reverse.group0()[1] * self.group1()[3]) + (self.group0()[0] * reverse[e45])
                - (self.group0()[1] * reverse.group1()[3])
                - (reverse.group4()[0] * self.group5()[0])
                + (reverse.group4()[0] * self.group9()[0])
                - (reverse.group4()[1] * self.group5()[1])
                + (reverse.group4()[1] * self.group9()[1])
                - (reverse.group4()[2] * self.group5()[2])
                + (reverse.group4()[2] * self.group9()[2])
                - (reverse.group5()[0] * self.group4()[0])
                - (reverse.group5()[1] * self.group4()[1])
                - (reverse.group5()[2] * self.group4()[2])
                + (reverse.group7()[0] * self.group1()[0])
                + (reverse.group7()[0] * self.group6()[0])
                + (reverse.group7()[1] * self.group1()[1])
                + (reverse.group7()[1] * self.group6()[1])
                + (reverse.group7()[2] * self.group1()[2])
                + (reverse.group7()[2] * self.group6()[2])
                - (self.group4()[0] * reverse.group9()[0])
                - (self.group4()[1] * reverse.group9()[1])
                - (self.group4()[2] * reverse.group9()[2])
                - (self.group7()[0] * reverse.group1()[0])
                + (self.group7()[0] * reverse.group6()[0])
                - (self.group7()[1] * reverse.group1()[1])
                + (self.group7()[1] * reverse.group6()[1])
                - (self.group7()[2] * reverse.group1()[2])
                + (self.group7()[2] * reverse.group6()[2])
                - (reverse.group1()[3] * self.group6()[3])
                + (reverse.group3()[3] * self[e45])
                + (reverse.group6()[3] * self.group1()[3])
                - (self.group3()[3] * reverse[e45])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group5()[0], 2) - f32::powi(self.group5()[1], 2) - f32::powi(self.group5()[2], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group3()[3], 2)
                + f32::powi(self.group6()[0], 2)
                + f32::powi(self.group6()[1], 2)
                + f32::powi(self.group6()[2], 2)
                - f32::powi(self.group6()[3], 2)
                - f32::powi(self.group9()[0], 2)
                - f32::powi(self.group9()[1], 2)
                - f32::powi(self.group9()[2], 2)
                - 2.0 * (self.group4()[0] * self.group3()[0])
                - 2.0 * (self.group4()[1] * self.group3()[1])
                - 2.0 * (self.group4()[2] * self.group3()[2])
                + 2.0 * (self.group7()[0] * self.group8()[0])
                + 2.0 * (self.group7()[1] * self.group8()[1])
                + 2.0 * (self.group7()[2] * self.group8()[2])
                - 2.0 * (self.group1()[3] * self[e1])
                + 2.0 * (self.group9()[3] * self[e45])),
        );
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(geometric_product.group0()[0] - scalar_product[scalar]), geometric_product.group0()[1]]),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e5
            geometric_product[e1],
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
            geometric_product[e45],
        );
        return subtraction;
    }
}
impl ConstraintViolation for Plane {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for RoundPoint {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
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
impl ConstraintViolation for Sphere {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       58        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       63       72        0
    //  no simd      102      114        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group2()[0]) + (reverse.group0()[1] * self.group2()[1]) + (reverse.group0()[2] * self.group2()[2])
                    - (reverse.group0()[3] * self.group0()[3])
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group1()[3] * self.group1()[3])
                    + (reverse.group2()[0] * self.group0()[0])
                    + (reverse.group2()[1] * self.group0()[1])
                    + (reverse.group2()[2] * self.group0()[2])
                    - (reverse.group2()[3] * self.group3()[3])
                    + (reverse.group3()[0] * self.group3()[0])
                    + (reverse.group3()[1] * self.group3()[1])
                    + (reverse.group3()[2] * self.group3()[2])
                    - (reverse.group3()[3] * self.group2()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group1()[0])
                    + (reverse.group0()[0] * self.group3()[0])
                    + (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[1] * self.group3()[1])
                    + (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group0()[2] * self.group3()[2])
                    - (reverse.group0()[3] * self.group3()[3])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group3()[3])
                    - (reverse.group3()[0] * self.group0()[0])
                    - (reverse.group3()[1] * self.group0()[1])
                    - (reverse.group3()[2] * self.group0()[2])
                    - (reverse.group3()[3] * self.group0()[3])
                    - (reverse.group3()[3] * self.group1()[3])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[3] * self.group3()[0]) - (reverse.group3()[2] * self.group1()[1]) - (reverse.group3()[3] * self.group2()[0])),
                ((reverse.group0()[3] * self.group3()[1]) - (reverse.group3()[0] * self.group1()[2]) - (reverse.group3()[3] * self.group2()[1])),
                ((reverse.group0()[3] * self.group3()[2]) - (reverse.group3()[1] * self.group1()[0]) - (reverse.group3()[3] * self.group2()[2])),
                (-(reverse.group2()[2] * self.group3()[2]) - (reverse.group2()[3] * self.group0()[3]) + (reverse.group3()[2] * self.group2()[2])),
            ]) + (Simd32x4::from(reverse.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                - (Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse.group0()[2], reverse.group1()[3]]) * swizzle!(self.group2(), 2, 0, 3, 3))
                + (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[0]]) * swizzle!(self.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group1()[3], reverse.group1()[3], reverse.group1()[3], reverse.group2()[0]]) * swizzle!(self.group1(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[1]]) * swizzle!(reverse.group2(), 2, 0, 1, 1))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group3()[3], self.group1()[2]]) * swizzle!(reverse.group2(), 1, 2, 2, 2))
                + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group1()[1], self.group2()[0]]) * swizzle!(reverse.group3(), 0, 1, 0, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group0()[3], self.group2()[1]]) * swizzle!(reverse.group3(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group3()[1], self.group2()[1]]) * swizzle!(reverse.group1(), 0, 1, 0, 1))
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group1()[3], self.group2()[2]]) * swizzle!(reverse.group1(), 1, 2, 2, 2))
                + (Simd32x4::from([self.group3()[3], self.group3()[3], self.group0()[1], self.group1()[1]]) * swizzle!(reverse.group2(), 0, 1, 0, 1))
                - (swizzle!(reverse.group0(), 0, 1, 0, 3) * swizzle!(self.group2(), 3, 3, 1, 3))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group3()[0], 2)
                + f32::powi(self.group3()[1], 2)
                + f32::powi(self.group3()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       66        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       69       78        0
    //  no simd      102      114        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group2()[0]) - (reverse.group0()[1] * self.group2()[1]) - (reverse.group0()[2] * self.group2()[2])
                    + (reverse.group0()[3] * self.group0()[3])
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    + (reverse.group1()[3] * self.group1()[3])
                    - (reverse.group2()[0] * self.group0()[0])
                    - (reverse.group2()[1] * self.group0()[1])
                    - (reverse.group2()[2] * self.group0()[2])
                    + (reverse.group2()[3] * self.group3()[3])
                    - (reverse.group3()[0] * self.group3()[0])
                    - (reverse.group3()[1] * self.group3()[1])
                    - (reverse.group3()[2] * self.group3()[2])
                    + (reverse.group3()[3] * self.group2()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0]) + (reverse.group0()[0] * self.group3()[0]) - (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[1] * self.group3()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group0()[2] * self.group3()[2])
                    + (reverse.group0()[3] * self.group2()[3])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group2()[3])
                    + (reverse.group2()[3] * self.group0()[3])
                    - (reverse.group2()[3] * self.group1()[3])
                    - (reverse.group3()[0] * self.group0()[0])
                    - (reverse.group3()[1] * self.group0()[1])
                    - (reverse.group3()[2] * self.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[3] * self.group3()[0])
                    + (reverse.group1()[1] * self.group3()[2])
                    + (reverse.group1()[3] * self.group1()[0])
                    + (reverse.group2()[3] * self.group2()[0])
                    - (reverse.group3()[3] * self.group0()[0])),
                ((reverse.group0()[3] * self.group3()[1])
                    + (reverse.group1()[2] * self.group3()[0])
                    + (reverse.group1()[3] * self.group1()[1])
                    + (reverse.group2()[3] * self.group2()[1])
                    - (reverse.group3()[3] * self.group0()[1])),
                ((reverse.group0()[2] * self.group3()[3])
                    + (reverse.group0()[3] * self.group3()[2])
                    + (reverse.group1()[0] * self.group3()[1])
                    + (reverse.group1()[3] * self.group1()[2])
                    - (reverse.group3()[3] * self.group0()[2])),
                (-(reverse.group1()[2] * self.group2()[2])
                    - (reverse.group1()[3] * self.group3()[3])
                    - (reverse.group2()[1] * self.group3()[1])
                    - (reverse.group2()[2] * self.group1()[2])
                    - (reverse.group2()[2] * self.group3()[2])),
            ]) + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group3()[3]]))
                + (Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse.group2()[3], reverse.group3()[0]]) * swizzle!(self.group2(), 2, 0, 2, 0))
                - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group3()[3]]) * swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group2()[1]]) * swizzle!(self.group1(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group2()[3], self.group3()[0]]) * swizzle!(reverse.group2(), 1, 2, 2, 0))
                + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group1()[1], self.group2()[1]]) * swizzle!(reverse.group3(), 0, 1, 0, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group0()[3], self.group2()[2]]) * swizzle!(reverse.group3(), 1, 2, 2, 2))
                - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 0, 1, 0, 0))
                - (Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group2()[1]]) * swizzle!(reverse.group1(), 2, 0, 1, 1))
                + (Simd32x4::from([self.group3()[3], self.group3()[3], self.group2()[1], self.group3()[3]]) * swizzle!(reverse.group0(), 0, 1, 0, 3))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group3()[0], 2)
                - f32::powi(self.group3()[1], 2)
                - f32::powi(self.group3()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group2()[3] * self.group3()[3])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product.group2()[3]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
