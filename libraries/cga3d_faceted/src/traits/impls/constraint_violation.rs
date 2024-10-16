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
//  Average:        19      22       0
//  Maximum:       260     270       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        15      17       0
//  Average:        26      32       0
//  Maximum:       396     420       0
impl ConstraintViolation for AntiCircleOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd2        3        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        8        0
    //  no simd       11       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                (-(reverse.group1()[0] * self.group0()[0]) - (reverse.group1()[1] * self.group0()[1]) - (reverse.group1()[2] * self.group0()[2])),
                0.0,
            ]) - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group0()[0], reverse.group1()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group0()[1], reverse.group1()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group0()[2], reverse.group1()[2]]))),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)));
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
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
impl ConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       35        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       35       49        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
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
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    - (self.group0()[0] * reverse.group2()[0])
                    - (self.group0()[1] * reverse.group2()[1])
                    - (self.group0()[2] * reverse.group2()[2])
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
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[1] * self.group2()[2]) + (self.group0()[1] * reverse.group2()[2])),
                ((reverse.group0()[2] * self.group2()[0]) + (self.group0()[2] * reverse.group2()[0])),
                ((reverse.group0()[0] * self.group2()[1]) + (self.group0()[0] * reverse.group2()[1])),
                (-(reverse.group1()[1] * self.group2()[1])
                    - (reverse.group1()[2] * self.group2()[2])
                    - (self.group1()[1] * reverse.group2()[1])
                    - (self.group1()[2] * reverse.group2()[2])),
            ]) - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 1, 2, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group2()[3], 2)
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
impl ConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       13        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       14        0
    //  no simd       12       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group1()[3])),
            ]),
            // e15, e25, e35, e3215
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
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group1()[3], 2)),
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
impl ConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       20        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       21        0
    //  no simd       17       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])
                    + (reverse.group0()[3] * self.group0()[3])
                    + (reverse.group1()[3] * self.group1()[3])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((reverse.group0()[0] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[0])),
                ((reverse.group0()[1] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[1])),
                ((reverse.group0()[2] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[2])),
                (-(reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd       12       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                (-(self.group1()[0] * reverse.group0()[0]) - (self.group1()[1] * reverse.group0()[1]) - (self.group1()[2] * reverse.group0()[2])),
                (reverse.group0()[3] * self.group0()[3]),
            ]) - (Simd32x2::from(reverse.group1()[0]) * Simd32x2::from([self.group0()[0], self.group1()[0]]))
                - (Simd32x2::from(reverse.group1()[1]) * Simd32x2::from([self.group0()[1], self.group1()[1]]))
                - (Simd32x2::from(reverse.group1()[2]) * Simd32x2::from([self.group0()[2], self.group1()[2]]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
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
impl ConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       22        0
    //    simd3        0        1        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       26       29        0
    //  no simd       41       49        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
            // e1, e2, e3, e5
            self.group2(),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])
                    - (reverse.group0()[3] * self.group0()[3])
                    + (reverse.group2()[0] * self.group2()[0])
                    + (reverse.group2()[1] * self.group2()[1])
                    + (reverse.group2()[2] * self.group2()[2])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group2()[2] * self.group0()[1]) * -1.0),
                ((reverse.group2()[0] * self.group0()[2]) * -1.0),
                ((reverse.group2()[1] * self.group0()[0]) * -1.0),
                (-(reverse.group1()[1] * self.group2()[1]) + (reverse.group1()[2] * self.group0()[2]) - (reverse.group1()[2] * self.group2()[2])
                    + (self.group1()[0] * reverse.group2()[0])
                    + (self.group1()[1] * reverse.group2()[1])
                    + (self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[2] * reverse.group2()[2])
                    - (reverse.group0()[3] * self.group2()[3])
                    + (reverse.group2()[3] * self.group0()[3])),
            ]) - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group0()[3], reverse.group0()[3], reverse.group0()[3], reverse.group1()[0]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                + (Simd32x4::from([reverse.group2()[1], reverse.group2()[2], reverse.group2()[0], reverse.group1()[1]]) * swizzle!(self.group0(), 2, 0, 1, 1))
                + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group2()[1], self.group1()[0]]) * swizzle!(reverse.group0(), 0, 1, 0, 0))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group0()[3], self.group1()[1]]) * swizzle!(reverse.group0(), 1, 2, 2, 1))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)
                + f32::powi(self.group2()[0], 2)
                + f32::powi(self.group2()[1], 2)
                + f32::powi(self.group2()[2], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        0
    //  no simd       15       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e4, e1, e2, e3 */ self.group1());
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                ((reverse.group0()[3] * self.group1()[0])
                    - (reverse.group1()[1] * self.group0()[0])
                    - (reverse.group1()[2] * self.group0()[1])
                    - (reverse.group1()[3] * self.group0()[2])),
                0.0,
            ]) - (Simd32x2::from(self.group0()[3]) * Simd32x2::from([reverse.group1()[0], reverse.group0()[3]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group0()[0], reverse.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group0()[1], reverse.group1()[2]]))
                + (Simd32x2::from(self.group1()[3]) * Simd32x2::from([reverse.group0()[2], reverse.group1()[3]]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       39       53        0
    //  no simd       48       64        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    + (reverse.group0()[0] * self.group2()[0])
                    + (reverse.group0()[1] * self.group2()[1])
                    + (reverse.group0()[2] * self.group2()[2])
                    - (reverse.group0()[3] * self.group2()[3])
                    + (reverse.group2()[0] * self.group0()[0])
                    + (reverse.group2()[1] * self.group0()[1])
                    + (reverse.group2()[2] * self.group0()[2])
                    - (reverse.group2()[3] * self.group0()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[2] * reverse.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-(reverse.group0()[0] * self.group0()[3]) - (reverse.group0()[1] * self.group2()[2]) + (reverse.group0()[3] * self.group0()[0])
                    - (reverse.group2()[2] * self.group0()[1])
                    - (reverse.group2()[3] * self.group2()[0])),
                (-(reverse.group0()[1] * self.group0()[3]) - (reverse.group0()[2] * self.group2()[0]) + (reverse.group0()[3] * self.group0()[1])
                    - (reverse.group2()[0] * self.group0()[2])
                    - (reverse.group2()[3] * self.group2()[1])),
                (-(reverse.group0()[0] * self.group2()[1]) - (reverse.group0()[2] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[2])
                    - (reverse.group2()[1] * self.group0()[0])
                    - (reverse.group2()[3] * self.group2()[2])),
                ((reverse.group1()[1] * self.group2()[1]) + (reverse.group1()[2] * self.group2()[2]) + (self.group1()[2] * reverse.group2()[2])),
            ]) + (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group2()[3], self.group1()[1]]) * swizzle!(reverse.group2(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group2()[3], self.group2()[3], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 0, 1, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group0()[3] * self.group2()[3])),
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
impl ConstraintViolation for AntiDipoleOnOrigin {
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
        let reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3] * -1.0));
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiDualNum {
    type Output = NullSphereAtOrigin;
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
        let subtraction = NullSphereAtOrigin::from_groups(/* e1234 */ geometric_product.group0()[0]);
        return subtraction;
    }
}
impl ConstraintViolation for AntiFlatOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * -1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse[e321] * self[e321] * -1.0));
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self[e321], 2) * -1.0));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
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
impl ConstraintViolation for AntiFlectorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            (-(reverse.group0()[0] * self.group0()[0])
                + (reverse.group0()[1] * self.group0()[1])
                + (reverse.group0()[2] * self.group0()[2])
                + (reverse.group0()[3] * self.group0()[3])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
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
impl ConstraintViolation for AntiLineOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        5        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_product = Scalar::from_groups(
            // scalar
            (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
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
impl ConstraintViolation for AntiMotorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])
                + (reverse.group0()[3] * self.group0()[3])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiMysteryCircleRotor {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9        9        0
    //  no simd       12       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)), /* scalar */ self[e31]);
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2]) + (reverse[e31] * self[e31])),
                (reverse.group0()[3] * self.group0()[0]),
                (reverse.group0()[3] * self.group0()[1]),
                (reverse.group0()[3] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * swizzle!(reverse.group0(), 3, 0, 1, 2))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2) + f32::powi(self[e31], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiMysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12        9        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       16       14        0
    //  no simd       28       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3 */ self.group1());
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                ((reverse.group1()[0] * self.group1()[0]) + (reverse.group1()[1] * self.group1()[1]) + (reverse.group1()[2] * self.group1()[2])),
                (-(self.group1()[1] * reverse.group0()[2]) + (reverse.group0()[3] * self.group0()[0])),
                (-(self.group1()[2] * reverse.group0()[0]) + (reverse.group0()[3] * self.group0()[1])),
                (-(self.group1()[0] * reverse.group0()[1]) + (reverse.group0()[3] * self.group0()[2])),
            ]) + (Simd32x4::from([reverse.group0()[0], reverse.group1()[1], reverse.group1()[2], reverse.group1()[0]]) * swizzle!(self.group0(), 0, 2, 0, 1))
                - (Simd32x4::from([reverse.group0()[3], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]) * swizzle!(self.group0(), 3, 1, 2, 0))
                + (Simd32x4::from([self.group0()[1], self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(reverse.group0(), 1, 1, 2, 0))
                + (swizzle!(reverse.group0(), 2, 0, 1, 2) * swizzle!(self.group0(), 2, 3, 3, 3))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)
                + f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
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
impl ConstraintViolation for AntiPlaneOnOrigin {
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
impl ConstraintViolation for AntiSphereOnOrigin {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        4        4        0
    // Totals...
    // yes simd       11       14        0
    //  no simd       15       18        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                (-(reverse.group1()[0] * self.group0()[0]) - (reverse.group1()[1] * self.group0()[1]) - (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group0()[3])),
                0.0,
            ]) + (Simd32x2::from(reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group0()[0], reverse.group1()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group0()[1], reverse.group1()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group0()[2], reverse.group1()[2]]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)),
        );
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
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
impl ConstraintViolation for CircleAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       39        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       33       48        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleAligningOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
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
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    + (reverse.group2()[0] * self.group0()[0])
                    + (reverse.group2()[1] * self.group0()[1])
                    + (reverse.group2()[2] * self.group0()[2])),
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
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-(reverse.group0()[1] * self.group2()[2]) + (reverse.group0()[2] * self.group2()[1]) + (reverse.group2()[1] * self.group0()[2])
                    - (reverse.group2()[2] * self.group0()[1])),
                ((reverse.group0()[0] * self.group2()[2]) - (reverse.group0()[2] * self.group2()[0]) - (reverse.group2()[0] * self.group0()[2])
                    + (reverse.group2()[2] * self.group0()[0])),
                (-(reverse.group0()[0] * self.group2()[1]) + (reverse.group0()[1] * self.group2()[0]) + (reverse.group2()[0] * self.group0()[1])
                    - (reverse.group2()[1] * self.group0()[0])),
                ((reverse.group1()[0] * self.group2()[0])
                    + (reverse.group1()[1] * self.group2()[1])
                    + (reverse.group1()[2] * self.group2()[2])
                    + (reverse.group2()[0] * self.group1()[0])
                    + (reverse.group2()[1] * self.group1()[1])
                    + (reverse.group2()[2] * self.group1()[2])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
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
impl ConstraintViolation for CircleAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       18       23        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])
                    - (reverse.group0()[3] * self.group0()[3])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group1()[2] * self.group0()[2])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[2] * reverse.group0()[2])),
            ]) + (Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                + (Simd32x4::from([reverse.group0()[3], reverse.group0()[3], reverse.group0()[3], reverse.group1()[1]]) * swizzle!(self.group0(), 0, 1, 2, 1))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleAtOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       17       30        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleAtOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                ((reverse.group0()[0] * self.group1()[0])
                    + (reverse.group0()[1] * self.group1()[1])
                    + (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])),
                (-(reverse.group0()[1] * self.group1()[2]) + (reverse.group0()[2] * self.group1()[1]) + (reverse.group1()[1] * self.group0()[2])
                    - (reverse.group1()[2] * self.group0()[1])),
                ((reverse.group0()[0] * self.group1()[2]) - (reverse.group0()[2] * self.group1()[0]) - (reverse.group1()[0] * self.group0()[2])
                    + (reverse.group1()[2] * self.group0()[0])),
                (-(reverse.group0()[0] * self.group1()[1]) + (reverse.group0()[1] * self.group1()[0]) + (reverse.group1()[0] * self.group0()[1])
                    - (reverse.group1()[1] * self.group0()[0])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (2.0 * (self.group0()[0] * self.group1()[0]) + 2.0 * (self.group0()[1] * self.group1()[1]) + 2.0 * (self.group0()[2] * self.group1()[2])),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd2        3        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8        8        0
    //  no simd       11       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                ((reverse.group1()[0] * self.group0()[0]) + (reverse.group1()[1] * self.group0()[1]) + (reverse.group1()[2] * self.group0()[2])),
                0.0,
            ]) + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group0()[0], reverse.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group0()[1], reverse.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group0()[2], reverse.group1()[2]]))),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)));
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleOrthogonalOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       19       35        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                ((reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[2] * reverse.group0()[2])),
                ((self.group1()[2] * reverse.group0()[1]) * -1.0),
                ((self.group1()[0] * reverse.group0()[2]) * -1.0),
                ((self.group1()[1] * reverse.group0()[0]) * -1.0),
            ]) + (Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group1()[0]]) * swizzle!(self.group0(), 0, 2, 0, 1))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(reverse.group0(), 0, 2, 0, 1))
                - (Simd32x4::from([reverse.group0()[3], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]) * swizzle!(self.group0(), 3, 1, 2, 0))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2)
                + 2.0 * (self.group1()[0] * self.group0()[0])
                + 2.0 * (self.group1()[1] * self.group0()[1])
                + 2.0 * (self.group1()[2] * self.group0()[2])),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
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
impl ConstraintViolation for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       35        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       35       49        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
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
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    + (self.group0()[0] * reverse.group2()[0])
                    + (self.group0()[1] * reverse.group2()[1])
                    + (self.group0()[2] * reverse.group2()[2])
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
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-(reverse.group0()[1] * self.group2()[2]) - (self.group0()[1] * reverse.group2()[2])),
                (-(reverse.group0()[2] * self.group2()[0]) - (self.group0()[2] * reverse.group2()[0])),
                (-(reverse.group0()[0] * self.group2()[1]) - (self.group0()[0] * reverse.group2()[1])),
                ((reverse.group1()[1] * self.group2()[1])
                    + (reverse.group1()[2] * self.group2()[2])
                    + (self.group1()[1] * reverse.group2()[1])
                    + (self.group1()[2] * reverse.group2()[2])),
            ]) + (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 1, 2, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group2()[3], 2)
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
impl ConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       13        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       14        0
    //  no simd       12       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])
                    - (reverse.group1()[3] * self.group1()[3])),
            ]),
            // e15, e25, e35, e3215
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
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group1()[3], 2)),
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
impl ConstraintViolation for CircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       12        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       14       15        0
    //  no simd       20       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])
                    - (reverse.group0()[3] * self.group0()[3])
                    - (reverse.group1()[3] * self.group1()[3])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[2] * self.group1()[2])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(reverse.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[0]]) * swizzle!(reverse.group0(), 0, 1, 2, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleRotorOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       12        0
    //  no simd       12       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                ((self.group1()[0] * reverse.group0()[0]) + (self.group1()[1] * reverse.group0()[1]) + (self.group1()[2] * reverse.group0()[2])),
                ((reverse.group0()[3] * self.group0()[3]) * -1.0),
            ]) + (Simd32x2::from(reverse.group1()[0]) * Simd32x2::from([self.group0()[0], self.group1()[0]]))
                + (Simd32x2::from(reverse.group1()[1]) * Simd32x2::from([self.group0()[1], self.group1()[1]]))
                + (Simd32x2::from(reverse.group1()[2]) * Simd32x2::from([self.group0()[2], self.group1()[2]]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
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
impl ConstraintViolation for DipoleAligningOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       19       32        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                (-(reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[2] * reverse.group0()[2])),
                (self.group1()[2] * reverse.group0()[1]),
                (self.group1()[0] * reverse.group0()[2]),
                (self.group1()[1] * reverse.group0()[0]),
            ]) - (Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group1()[0]]) * swizzle!(self.group0(), 0, 2, 0, 1))
                - (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(reverse.group0(), 0, 2, 0, 1))
                + (Simd32x4::from([reverse.group0()[3], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]) * swizzle!(self.group0(), 3, 1, 2, 0))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2)
                - 2.0 * (self.group1()[0] * self.group0()[0])
                - 2.0 * (self.group1()[1] * self.group0()[1])
                - 2.0 * (self.group1()[2] * self.group0()[2])),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       16        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       18        0
    //  no simd       15       23        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])
                    + (reverse.group0()[3] * self.group0()[3])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((reverse.group0()[0] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[0])),
                ((reverse.group0()[1] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[1])),
                ((reverse.group0()[2] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[2])),
                (-(reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[2] * reverse.group0()[2])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleAtOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       17       30        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAtOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (-(reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])),
                ((reverse.group0()[1] * self.group1()[2]) - (reverse.group0()[2] * self.group1()[1]) - (reverse.group1()[1] * self.group0()[2])
                    + (reverse.group1()[2] * self.group0()[1])),
                (-(reverse.group0()[0] * self.group1()[2]) + (reverse.group0()[2] * self.group1()[0]) + (reverse.group1()[0] * self.group0()[2])
                    - (reverse.group1()[2] * self.group0()[0])),
                ((reverse.group0()[0] * self.group1()[1]) - (reverse.group0()[1] * self.group1()[0]) - (reverse.group1()[0] * self.group0()[1])
                    + (reverse.group1()[1] * self.group0()[0])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-2.0 * (self.group0()[0] * self.group1()[0]) - 2.0 * (self.group0()[1] * self.group1()[1]) - 2.0 * (self.group0()[2] * self.group1()[2])),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
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
impl ConstraintViolation for DipoleInversionAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       39        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       36       46        0
    //  no simd       54       67        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0]) - (reverse.group0()[1] * self.group1()[1]) - (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group0()[3] * self.group0()[3])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group1()[3] * self.group2()[3])
                    - (reverse.group2()[0] * self.group2()[0])
                    - (reverse.group2()[1] * self.group2()[1])
                    - (reverse.group2()[2] * self.group2()[2])
                    + (reverse.group2()[3] * self.group1()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group2()[0])
                    + (reverse.group0()[1] * self.group2()[1])
                    + (reverse.group0()[2] * self.group2()[2])
                    + (reverse.group0()[3] * self.group1()[3])
                    - (reverse.group1()[3] * self.group0()[3])
                    - (reverse.group2()[0] * self.group0()[0])
                    - (reverse.group2()[1] * self.group0()[1])
                    - (reverse.group2()[2] * self.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[0] * self.group2()[3]) - (reverse.group2()[3] * self.group0()[0])),
                ((reverse.group0()[1] * self.group2()[3]) - (reverse.group2()[3] * self.group0()[1])),
                ((reverse.group0()[2] * self.group2()[3]) - (reverse.group2()[3] * self.group0()[2])),
                (-(reverse.group1()[2] * self.group2()[2]) + (reverse.group2()[2] * self.group1()[2])),
            ]) + (Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse.group0()[0], reverse.group2()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[3]]) * swizzle!(self.group0(), 1, 2, 0, 3))
                + (Simd32x4::from([reverse.group1()[3], reverse.group1()[3], reverse.group1()[3], reverse.group2()[1]]) * swizzle!(self.group1(), 0, 1, 2, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group1()[3], self.group2()[1]]) * swizzle!(reverse.group1(), 1, 2, 2, 1))
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[3]]) * swizzle!(reverse.group0(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group1()[3], self.group1()[3], self.group0()[1], self.group2()[0]]) * swizzle!(reverse.group1(), 0, 1, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2)
                - f32::powi(self.group2()[0], 2)
                - f32::powi(self.group2()[1], 2)
                - f32::powi(self.group2()[2], 2)
                - 2.0 * (self.group0()[0] * self.group1()[0])
                - 2.0 * (self.group0()[1] * self.group1()[1])
                - 2.0 * (self.group0()[2] * self.group1()[2])
                + 2.0 * (self.group1()[3] * self.group2()[3])),
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
impl ConstraintViolation for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       23        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       29       29        0
    //  no simd       41       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])
                    + (reverse.group0()[3] * self.group0()[3])
                    - (reverse.group2()[0] * self.group2()[0])
                    - (reverse.group2()[1] * self.group2()[1])
                    - (reverse.group2()[2] * self.group2()[2])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[1] * self.group2()[2]) + (reverse.group0()[3] * self.group0()[0])),
                ((reverse.group0()[2] * self.group2()[0]) + (reverse.group0()[3] * self.group0()[1])),
                ((reverse.group0()[0] * self.group2()[1]) + (reverse.group0()[3] * self.group0()[2])),
                (-(reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[1] * self.group2()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    - (reverse.group1()[2] * self.group2()[2])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[1] * reverse.group2()[1])
                    - (self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[2] * reverse.group2()[2])
                    - (reverse.group0()[3] * self.group2()[3])),
            ]) + (Simd32x4::from(self.group0()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 1, 2, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)
                - f32::powi(self.group2()[0], 2)
                - f32::powi(self.group2()[1], 2)
                - f32::powi(self.group2()[2], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleInversionAtOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       22        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       32       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                (-(reverse.group1()[1] * self.group0()[1]) - (reverse.group1()[2] * self.group0()[2])),
                ((reverse.group0()[1] * self.group1()[2]) + (reverse.group1()[3] * self.group1()[0])),
                ((reverse.group0()[2] * self.group1()[0]) + (reverse.group1()[3] * self.group1()[1])),
                ((reverse.group1()[1] * self.group0()[0]) + (reverse.group1()[3] * self.group1()[2])),
            ]) - (Simd32x4::from([reverse.group0()[2], reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]) * swizzle!(self.group1(), 2, 3, 3, 3))
                + (Simd32x4::from([reverse.group1()[3], reverse.group1()[2], reverse.group1()[0], reverse.group0()[2]]) * swizzle!(self.group0(), 3, 1, 2, 3))
                - (Simd32x4::from([self.group1()[1], self.group0()[0], self.group0()[1], self.group0()[2]]) * swizzle!(reverse.group0(), 1, 3, 3, 3))
                + (Simd32x4::from([self.group1()[3], self.group0()[3], self.group0()[3], self.group1()[1]]) * swizzle!(reverse.group0(), 3, 0, 1, 0))
                - (swizzle!(reverse.group0(), 0, 2, 0, 1) * swizzle!(self.group1(), 0, 1, 2, 0))
                - (swizzle!(reverse.group1(), 0, 1, 2, 0) * swizzle!(self.group0(), 0, 2, 0, 1))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-2.0 * (self.group0()[0] * self.group1()[0]) - 2.0 * (self.group0()[1] * self.group1()[1]) - 2.0 * (self.group0()[2] * self.group1()[2])
                + 2.0 * (self.group0()[3] * self.group1()[3])),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleInversionOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        9        0
    //  no simd       15       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e1234, e4235, e4315, e4125
            self.group1(),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                ((reverse.group0()[0] * self.group1()[1]) + (reverse.group0()[1] * self.group1()[2]) + (reverse.group0()[2] * self.group1()[3])
                    - (reverse.group1()[0] * self.group0()[3])),
                0.0,
            ]) + (Simd32x2::from(reverse.group0()[3]) * Simd32x2::from([self.group1()[0], self.group0()[3]]))
                - (Simd32x2::from(reverse.group1()[1]) * Simd32x2::from([self.group0()[0], self.group1()[1]]))
                - (Simd32x2::from(reverse.group1()[2]) * Simd32x2::from([self.group0()[1], self.group1()[2]]))
                - (Simd32x2::from(reverse.group1()[3]) * Simd32x2::from([self.group0()[2], self.group1()[3]]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       39       53        0
    //  no simd       48       64        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group0()[0] * self.group2()[0])
                    - (reverse.group0()[1] * self.group2()[1])
                    - (reverse.group0()[2] * self.group2()[2])
                    + (reverse.group0()[3] * self.group2()[3])
                    - (reverse.group2()[0] * self.group0()[0])
                    - (reverse.group2()[1] * self.group0()[1])
                    - (reverse.group2()[2] * self.group0()[2])
                    + (reverse.group2()[3] * self.group0()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[2] * reverse.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[0] * self.group0()[3]) + (reverse.group0()[1] * self.group2()[2]) - (reverse.group0()[3] * self.group0()[0])
                    + (reverse.group2()[2] * self.group0()[1])
                    + (reverse.group2()[3] * self.group2()[0])),
                ((reverse.group0()[1] * self.group0()[3]) + (reverse.group0()[2] * self.group2()[0]) - (reverse.group0()[3] * self.group0()[1])
                    + (reverse.group2()[0] * self.group0()[2])
                    + (reverse.group2()[3] * self.group2()[1])),
                ((reverse.group0()[0] * self.group2()[1]) + (reverse.group0()[2] * self.group0()[3]) - (reverse.group0()[3] * self.group0()[2])
                    + (reverse.group2()[1] * self.group0()[0])
                    + (reverse.group2()[3] * self.group2()[2])),
                (-(reverse.group1()[1] * self.group2()[1]) - (reverse.group1()[2] * self.group2()[2]) - (self.group1()[2] * reverse.group2()[2])),
            ]) - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group2()[3], self.group1()[1]]) * swizzle!(reverse.group2(), 1, 2, 2, 1))
                - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 0, 1, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group0()[3] * self.group2()[3])),
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
impl ConstraintViolation for DipoleOnOrigin {
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
        let reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse.group0()[3] * self.group0()[3]));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[3], 2));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for DipoleOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       39        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       33       48        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
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
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group2()[0] * self.group0()[0])
                    - (reverse.group2()[1] * self.group0()[1])
                    - (reverse.group2()[2] * self.group0()[2])),
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
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((reverse.group0()[1] * self.group2()[2]) - (reverse.group0()[2] * self.group2()[1]) - (reverse.group2()[1] * self.group0()[2])
                    + (reverse.group2()[2] * self.group0()[1])),
                (-(reverse.group0()[0] * self.group2()[2]) + (reverse.group0()[2] * self.group2()[0]) + (reverse.group2()[0] * self.group0()[2])
                    - (reverse.group2()[2] * self.group0()[0])),
                ((reverse.group0()[0] * self.group2()[1]) - (reverse.group0()[1] * self.group2()[0]) - (reverse.group2()[0] * self.group0()[1])
                    + (reverse.group2()[1] * self.group0()[0])),
                (-(reverse.group1()[0] * self.group2()[0])
                    - (reverse.group1()[1] * self.group2()[1])
                    - (reverse.group1()[2] * self.group2()[2])
                    - (reverse.group2()[0] * self.group1()[0])
                    - (reverse.group2()[1] * self.group1()[1])
                    - (reverse.group2()[2] * self.group1()[2])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
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
impl ConstraintViolation for DualNum {
    type Output = NullSphereAtOrigin;
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
        let subtraction = NullSphereAtOrigin::from_groups(/* e1234 */ geometric_product.group0()[0]);
        return subtraction;
    }
}
impl ConstraintViolation for FlatOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = FlatOrigin::from_groups(/* e45 */ (self[e45] * -1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse[e45] * self[e45]));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
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
impl ConstraintViolation for FlectorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            ((reverse.group0()[0] * self.group0()[0])
                - (reverse.group0()[1] * self.group0()[1])
                - (reverse.group0()[2] * self.group0()[2])
                - (reverse.group0()[3] * self.group0()[3])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
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
impl ConstraintViolation for LineOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        5        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_product = Scalar::from_groups(
            // scalar
            ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
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
impl ConstraintViolation for MotorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        7        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = Scalar::from_groups(
            // scalar
            ((reverse.group0()[0] * self.group0()[0]) + (reverse.group0()[1] * self.group0()[1]) + (reverse.group0()[2] * self.group0()[2])
                - (reverse.group0()[3] * self.group0()[3])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      204      208        0
    //    simd2       16       16        0
    //    simd3        0        4        0
    //    simd4       40       42        0
    // Totals...
    // yes simd      260      270        0
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
                    - (reverse.group9()[1] * self.group9()[1])
                    - (reverse.group9()[2] * self.group9()[2])
                    - (reverse.group9()[3] * self.group9()[3])
                    - (self.group1()[3] * reverse[e1])),
                ((reverse.group0()[1] * self.group0()[0])
                    - (reverse.group8()[0] * self.group3()[0])
                    - (reverse.group8()[1] * self.group3()[1])
                    - (reverse.group8()[2] * self.group3()[2])
                    - (self.group5()[0] * reverse.group6()[0])
                    - (self.group5()[1] * reverse.group6()[1])
                    - (self.group5()[2] * reverse.group6()[2])
                    - (self.group8()[0] * reverse.group3()[0])
                    - (self.group8()[1] * reverse.group3()[1])
                    - (self.group8()[2] * reverse.group3()[2])
                    - (reverse.group6()[3] * self.group3()[3])
                    + (reverse.group9()[0] * self[e1])
                    + (reverse.group9()[1] * self.group1()[0])
                    + (reverse.group9()[2] * self.group1()[1])
                    + (reverse.group9()[3] * self.group1()[2])
                    + (self.group9()[0] * reverse[e1])),
            ]) + (Simd32x2::from(reverse.group0()[0]) * self.group0())
                - (Simd32x2::from(reverse.group4()[0]) * Simd32x2::from([self.group3()[0], self.group7()[0]]))
                - (Simd32x2::from(reverse.group4()[1]) * Simd32x2::from([self.group3()[1], self.group7()[1]]))
                - (Simd32x2::from(reverse.group4()[2]) * Simd32x2::from([self.group3()[2], self.group7()[2]]))
                - (Simd32x2::from(reverse.group5()[0]) * Simd32x2::from([self.group5()[0], self.group6()[0]]))
                - (Simd32x2::from(reverse.group5()[1]) * Simd32x2::from([self.group5()[1], self.group6()[1]]))
                - (Simd32x2::from(reverse.group5()[2]) * Simd32x2::from([self.group5()[2], self.group6()[2]]))
                - (Simd32x2::from(self.group4()[0]) * Simd32x2::from([reverse.group3()[0], reverse.group7()[0]]))
                - (Simd32x2::from(self.group4()[1]) * Simd32x2::from([reverse.group3()[1], reverse.group7()[1]]))
                - (Simd32x2::from(self.group4()[2]) * Simd32x2::from([reverse.group3()[2], reverse.group7()[2]]))
                + (Simd32x2::from(reverse.group1()[0]) * Simd32x2::from([self.group1()[0], self.group9()[1]]))
                + (Simd32x2::from(reverse.group1()[1]) * Simd32x2::from([self.group1()[1], self.group9()[2]]))
                + (Simd32x2::from(reverse.group1()[2]) * Simd32x2::from([self.group1()[2], self.group9()[3]]))
                - (Simd32x2::from(self.group6()[3]) * Simd32x2::from([reverse.group6()[3], reverse.group3()[3]]))
                + (Simd32x2::from(reverse[e45]) * Simd32x2::from([self.group9()[0], self.group1()[3]]))
                + (Simd32x2::from(self[e45]) * Simd32x2::from([reverse.group9()[0], reverse.group1()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((reverse.group4()[1] * self.group7()[2]) - (reverse.group4()[2] * self.group7()[1]) + (reverse.group5()[1] * self.group1()[2])
                    - (reverse.group7()[0] * self[e45])
                    - (reverse.group7()[1] * self.group4()[2])
                    + (reverse.group7()[2] * self.group4()[1])
                    - (self.group4()[0] * reverse.group1()[3])
                    + (self.group5()[0] * reverse.group6()[3])
                    - (self.group5()[1] * reverse.group1()[2])
                    + (self.group7()[0] * reverse[e45])
                    + (self.group8()[1] * reverse.group3()[2])),
                (-(reverse.group4()[0] * self.group7()[2])
                    + (reverse.group4()[2] * self.group7()[0])
                    + (reverse.group5()[2] * self.group1()[0])
                    + (reverse.group7()[0] * self.group4()[2])
                    - (reverse.group7()[1] * self[e45])
                    - (reverse.group7()[2] * self.group4()[0])
                    - (self.group4()[1] * reverse.group1()[3])
                    + (self.group5()[1] * reverse.group6()[3])
                    - (self.group5()[2] * reverse.group1()[0])
                    + (self.group7()[1] * reverse[e45])
                    + (self.group8()[2] * reverse.group3()[0])),
                ((reverse.group4()[0] * self.group7()[1]) - (reverse.group4()[1] * self.group7()[0]) + (reverse.group5()[0] * self.group1()[1])
                    - (reverse.group7()[0] * self.group4()[1])
                    + (reverse.group7()[1] * self.group4()[0])
                    - (reverse.group7()[2] * self[e45])
                    - (self.group4()[2] * reverse.group1()[3])
                    - (self.group5()[0] * reverse.group1()[1])
                    + (self.group5()[2] * reverse.group6()[3])
                    + (self.group7()[2] * reverse[e45])
                    + (self.group8()[0] * reverse.group3()[1])),
                (-(reverse.group5()[0] * self.group7()[0])
                    - (reverse.group5()[1] * self.group7()[1])
                    - (reverse.group5()[2] * self.group7()[2])
                    - (reverse.group7()[0] * self.group5()[0])
                    - (reverse.group7()[1] * self.group5()[1])
                    + (reverse.group7()[1] * self.group9()[2])
                    - (reverse.group7()[2] * self.group5()[2])
                    + (reverse.group7()[2] * self.group9()[3])
                    - (reverse.group3()[2] * self.group1()[2])
                    - (reverse.group3()[2] * self.group6()[2])
                    - (reverse.group6()[2] * self.group3()[2])),
            ]) + (Simd32x4::from(reverse.group0()[0]) * self.group1())
                + (Simd32x4::from(self.group0()[0]) * reverse.group1())
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group4()[0], reverse.group4()[1], reverse.group4()[2], reverse.group3()[3]]))
                + (Simd32x4::from(self.group6()[3]) * Simd32x4::from([reverse.group5()[0], reverse.group5()[1], reverse.group5()[2], reverse.group9()[0]]))
                + (Simd32x4::from(self.group9()[0]) * Simd32x4::from([reverse.group8()[0], reverse.group8()[1], reverse.group8()[2], reverse.group0()[1]]))
                - (Simd32x4::from([reverse.group0()[1], reverse.group0()[1], reverse.group0()[1], reverse.group6()[3]]) * swizzle!(self.group9(), 1, 2, 3, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group7()[0]]) * swizzle!(reverse.group9(), 1, 2, 3, 1))
                - (Simd32x4::from([reverse.group5()[2], reverse.group5()[0], reverse.group5()[1], reverse.group3()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group8()[1], reverse.group8()[2], reverse.group8()[0], reverse.group1()[0]]) * swizzle!(self.group3(), 2, 0, 1, 0))
                - (Simd32x4::from([reverse.group8()[2], reverse.group8()[0], reverse.group8()[1], reverse.group1()[3]]) * swizzle!(self.group3(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group3()[1]]) * swizzle!(reverse.group1(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], self.group7()[1]]) * swizzle!(reverse.group9(), 0, 0, 0, 2))
                - (Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self.group6()[0]]) * swizzle!(reverse.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1], reverse.group7()[0]]) * swizzle!(self.group9(), 2, 3, 1, 1))
                - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group9()[2], self.group3()[0]]) * swizzle!(reverse.group6(), 0, 1, 0, 0))
                + (Simd32x4::from([self.group6()[1], self.group6()[2], self.group6()[0], self.group0()[1]]) * swizzle!(reverse.group9(), 3, 1, 2, 0))
                - (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group7()[2]]) * swizzle!(reverse.group9(), 2, 3, 1, 3))
                - (Simd32x4::from([self.group9()[3], self.group9()[1], self.group3()[3], self.group3()[1]]) * swizzle!(reverse.group6(), 1, 2, 2, 1))
                + (Simd32x4::from([reverse[e1], reverse[e1], reverse[e1], reverse.group1()[2]]) * swizzle!(self.group3(), 0, 1, 2, 2))
                - (Simd32x4::from([self[e1], self[e1], self[e1], self.group1()[1]]) * swizzle!(reverse.group3(), 0, 1, 2, 1))
                - (swizzle!(reverse.group3(), 3, 3, 3, 1) * swizzle!(self.group6(), 0, 1, 2, 1))),
            // e5
            ((reverse.group0()[0] * self[e1])
                + (reverse.group0()[1] * self[e45])
                + (self.group0()[0] * reverse[e1])
                + (self.group0()[1] * reverse[e45])
                + (reverse.group4()[0] * self.group1()[0])
                - (reverse.group4()[0] * self.group6()[0])
                + (reverse.group4()[1] * self.group1()[1])
                - (reverse.group4()[1] * self.group6()[1])
                + (reverse.group4()[2] * self.group1()[2])
                - (reverse.group4()[2] * self.group6()[2])
                - (reverse.group5()[0] * self.group8()[0])
                - (reverse.group5()[1] * self.group8()[1])
                - (reverse.group5()[2] * self.group8()[2])
                - (reverse.group8()[0] * self.group5()[0])
                - (reverse.group8()[0] * self.group9()[1])
                - (reverse.group8()[1] * self.group5()[1])
                - (reverse.group8()[1] * self.group9()[2])
                - (reverse.group8()[2] * self.group5()[2])
                - (reverse.group8()[2] * self.group9()[3])
                - (self.group4()[0] * reverse.group1()[0])
                - (self.group4()[0] * reverse.group6()[0])
                - (self.group4()[1] * reverse.group1()[1])
                - (self.group4()[1] * reverse.group6()[1])
                - (self.group4()[2] * reverse.group1()[2])
                - (self.group4()[2] * reverse.group6()[2])
                + (self.group8()[0] * reverse.group9()[1])
                + (self.group8()[1] * reverse.group9()[2])
                + (self.group8()[2] * reverse.group9()[3])
                - (reverse.group3()[3] * self[e1])
                + (reverse.group6()[3] * self[e45])
                + (self.group3()[3] * reverse[e1])
                - (self.group6()[3] * reverse[e45])),
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
            (Simd32x4::from([
                (-(reverse.group5()[2] * self.group3()[2]) - (self.group5()[1] * reverse.group3()[1]) - (self.group5()[2] * reverse.group3()[2])
                    + (self.group7()[0] * reverse.group6()[0])
                    - (self.group7()[1] * reverse.group1()[1])
                    + (self.group7()[1] * reverse.group6()[1])
                    - (self.group7()[2] * reverse.group1()[2])
                    + (self.group7()[2] * reverse.group6()[2])
                    - (reverse.group1()[3] * self.group6()[3])
                    + (reverse.group6()[3] * self.group1()[3])
                    - (reverse.group9()[1] * self.group3()[0])
                    - (reverse.group9()[2] * self.group3()[1])
                    - (reverse.group9()[3] * self.group3()[2])),
                ((self.group0()[1] * reverse.group1()[0]) - (reverse.group4()[0] * self.group9()[0])
                    + (reverse.group4()[2] * self.group3()[1])
                    + (reverse.group5()[0] * self.group3()[3])
                    - (reverse.group5()[2] * self.group9()[2])
                    - (reverse.group7()[0] * self[e1])
                    - (reverse.group7()[1] * self.group8()[2])
                    + (reverse.group7()[2] * self.group8()[1])
                    + (reverse.group8()[1] * self.group7()[2])
                    - (reverse.group8()[2] * self.group7()[1])
                    + (self.group4()[0] * reverse.group9()[0])
                    + (self.group5()[2] * reverse.group9()[2])
                    + (self.group7()[0] * reverse[e1])),
                ((self.group0()[1] * reverse.group1()[1]) + (reverse.group4()[0] * self.group3()[2])
                    - (reverse.group4()[1] * self.group9()[0])
                    - (reverse.group5()[0] * self.group9()[3])
                    + (reverse.group5()[1] * self.group3()[3])
                    + (reverse.group7()[0] * self.group8()[2])
                    - (reverse.group7()[1] * self[e1])
                    - (reverse.group7()[2] * self.group8()[0])
                    - (reverse.group8()[0] * self.group7()[2])
                    + (reverse.group8()[2] * self.group7()[0])
                    + (self.group4()[1] * reverse.group9()[0])
                    + (self.group5()[0] * reverse.group9()[3])
                    + (self.group7()[1] * reverse[e1])),
                ((self.group0()[1] * reverse.group1()[2]) + (reverse.group4()[1] * self.group3()[0])
                    - (reverse.group4()[2] * self.group9()[0])
                    - (reverse.group5()[1] * self.group9()[1])
                    + (reverse.group5()[2] * self.group3()[3])
                    - (reverse.group7()[0] * self.group8()[1])
                    + (reverse.group7()[1] * self.group8()[0])
                    - (reverse.group7()[2] * self[e1])
                    + (reverse.group8()[0] * self.group7()[1])
                    - (reverse.group8()[1] * self.group7()[0])
                    + (self.group4()[2] * reverse.group9()[0])
                    + (self.group5()[1] * reverse.group9()[1])
                    + (self.group7()[2] * reverse[e1])),
            ]) + (Simd32x4::from(reverse.group0()[0]) * self.group9())
                + (Simd32x4::from(self.group0()[0]) * reverse.group9())
                - (Simd32x4::from(reverse.group1()[3]) * Simd32x4::from([self.group0()[1], self.group8()[0], self.group8()[1], self.group8()[2]]))
                - (Simd32x4::from([reverse.group0()[1], reverse.group6()[2], reverse.group6()[0], reverse.group6()[1]]) * swizzle!(self.group1(), 3, 1, 2, 0))
                - (Simd32x4::from([reverse.group5()[0], reverse.group4()[1], reverse.group4()[2], reverse.group4()[0]]) * swizzle!(self.group3(), 0, 2, 0, 1))
                - (Simd32x4::from([reverse.group5()[1], reverse[e45], reverse[e45], reverse[e45]]) * swizzle!(self.group3(), 1, 0, 1, 2))
                + (Simd32x4::from([reverse.group7()[0], reverse.group0()[1], reverse.group0()[1], reverse.group0()[1]]) * swizzle!(self.group1(), 0, 0, 1, 2))
                + (Simd32x4::from([reverse.group7()[0], reverse.group1()[1], reverse.group1()[2], reverse.group1()[0]]) * swizzle!(self.group6(), 0, 2, 0, 1))
                + (Simd32x4::from([reverse.group7()[1], reverse.group8()[0], reverse.group8()[1], reverse.group8()[2]]) * swizzle!(self.group1(), 1, 3, 3, 3))
                + (Simd32x4::from([reverse.group7()[1], reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]) * swizzle!(self.group6(), 1, 3, 3, 3))
                + (Simd32x4::from([reverse.group7()[2], reverse.group6()[1], reverse.group6()[2], reverse.group6()[0]]) * swizzle!(self.group1(), 2, 2, 0, 1))
                + (Simd32x4::from([reverse.group7()[2], reverse.group6()[3], reverse.group6()[3], reverse.group6()[3]]) * swizzle!(self.group6(), 2, 0, 1, 2))
                - (Simd32x4::from([self.group5()[0], self.group4()[1], self.group4()[2], self.group4()[0]]) * swizzle!(reverse.group3(), 0, 2, 0, 1))
                - (Simd32x4::from([self.group7()[0], self.group6()[1], self.group6()[2], self.group6()[0]]) * swizzle!(reverse.group1(), 0, 2, 0, 1))
                + (Simd32x4::from([reverse.group3()[0], reverse.group5()[1], reverse.group5()[2], reverse.group5()[0]]) * swizzle!(self.group9(), 1, 3, 1, 2))
                - (Simd32x4::from([self.group3()[3], self.group5()[1], self.group5()[2], self.group5()[0]]) * swizzle!(reverse.group9(), 0, 3, 1, 2))
                + (Simd32x4::from([self.group9()[0], self[e45], self[e45], self[e45]]) * swizzle!(reverse.group3(), 3, 0, 1, 2))
                + (Simd32x4::from([self.group9()[2], self.group4()[2], self.group4()[0], self.group4()[1]]) * swizzle!(reverse.group3(), 1, 1, 2, 0))
                + (Simd32x4::from([self.group9()[3], self.group5()[0], self.group5()[1], self.group5()[2]]) * swizzle!(reverse.group3(), 2, 3, 3, 3))),
            // e3215
            ((reverse.group0()[0] * self[e45]) - (reverse.group0()[1] * self[e1]) + (self.group0()[0] * reverse[e45])
                - (self.group0()[1] * reverse[e1])
                - (reverse.group4()[0] * self.group5()[0])
                - (reverse.group4()[0] * self.group9()[1])
                - (reverse.group4()[1] * self.group5()[1])
                - (reverse.group4()[1] * self.group9()[2])
                - (reverse.group4()[2] * self.group5()[2])
                - (reverse.group4()[2] * self.group9()[3])
                - (reverse.group5()[0] * self.group4()[0])
                - (reverse.group5()[1] * self.group4()[1])
                - (reverse.group5()[2] * self.group4()[2])
                - (reverse.group8()[0] * self.group1()[0])
                + (reverse.group8()[0] * self.group6()[0])
                - (reverse.group8()[1] * self.group1()[1])
                + (reverse.group8()[1] * self.group6()[1])
                - (reverse.group8()[2] * self.group1()[2])
                + (reverse.group8()[2] * self.group6()[2])
                + (self.group4()[0] * reverse.group9()[1])
                + (self.group4()[1] * reverse.group9()[2])
                + (self.group4()[2] * reverse.group9()[3])
                + (self.group8()[0] * reverse.group1()[0])
                + (self.group8()[0] * reverse.group6()[0])
                + (self.group8()[1] * reverse.group1()[1])
                + (self.group8()[1] * reverse.group6()[1])
                + (self.group8()[2] * reverse.group1()[2])
                + (self.group8()[2] * reverse.group6()[2])
                - (reverse.group3()[3] * self[e45])
                - (reverse.group6()[3] * self[e1])
                + (self.group3()[3] * reverse[e45])
                + (self.group6()[3] * reverse[e1])),
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
                - f32::powi(self.group9()[1], 2)
                - f32::powi(self.group9()[2], 2)
                - f32::powi(self.group9()[3], 2)
                - 2.0 * (self.group4()[0] * self.group3()[0])
                - 2.0 * (self.group4()[1] * self.group3()[1])
                - 2.0 * (self.group4()[2] * self.group3()[2])
                + 2.0 * (self.group7()[0] * self.group8()[0])
                + 2.0 * (self.group7()[1] * self.group8()[1])
                + 2.0 * (self.group7()[2] * self.group8()[2])
                - 2.0 * (self.group1()[3] * self[e1])
                + 2.0 * (self.group9()[0] * self[e45])),
        );
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(geometric_product.group0()[0] - scalar_product[scalar]), geometric_product.group0()[1]]),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e5
            geometric_product[e1],
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
            geometric_product.group9(),
            // e3215
            geometric_product[e45],
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryCircle {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        7        5        0
    //  no simd       13       14        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircle::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([((reverse.group0()[2] * self.group0()[2]) - (reverse.group0()[3] * self.group0()[3])), 0.0, 0.0, 0.0])
                + (swizzle!(reverse.group0(), 0, 0, 1, 2) * swizzle!(self.group0(), 0, 3, 3, 3))
                + (swizzle!(reverse.group0(), 1, 3, 3, 3) * swizzle!(self.group0(), 1, 0, 1, 2))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryCircleRotor {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9        6        0
    //  no simd       15       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e12345 */ self[e425]);
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                ((reverse.group0()[2] * self.group0()[2]) - (reverse.group0()[3] * self.group0()[3]) - (reverse[e425] * self[e425])),
                0.0,
                0.0,
                0.0,
            ]) + (swizzle!(reverse.group0(), 0, 0, 1, 2) * swizzle!(self.group0(), 0, 3, 3, 3))
                + (swizzle!(reverse.group0(), 1, 3, 3, 3) * swizzle!(self.group0(), 1, 0, 1, 2))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2) - f32::powi(self[e425], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryDipole {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       10       14        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                (-(reverse.group0()[0] * self.group0()[0]) - (reverse.group0()[1] * self.group0()[1]) - (reverse.group0()[2] * self.group0()[2])),
                (reverse.group0()[3] * self.group0()[0]),
                (reverse.group0()[3] * self.group0()[1]),
                (reverse.group0()[3] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * swizzle!(reverse.group0(), 3, 0, 1, 2))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       19       17        0
    //  no simd       28       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)), /* e4235, e4315, e4125 */ self.group1());
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                (-(reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group0()[2] * self.group0()[2])),
                ((self.group1()[2] * reverse.group0()[1]) + (reverse.group0()[0] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[0])),
                ((self.group1()[0] * reverse.group0()[2]) + (reverse.group0()[1] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[1])),
                ((self.group1()[1] * reverse.group0()[0]) + (reverse.group0()[2] * self.group0()[3]) + (reverse.group0()[3] * self.group0()[2])),
            ]) - (Simd32x4::from([reverse.group0()[0], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                + (Simd32x4::from([reverse.group0()[3], reverse.group1()[1], reverse.group1()[2], reverse.group1()[0]]) * swizzle!(self.group0(), 3, 2, 0, 1))
                - (Simd32x4::from([self.group0()[1], self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(reverse.group0(), 1, 2, 0, 1))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)
                - f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[3], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       15        9        0
    //  no simd       36       36        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ (self.group1() * Simd32x4::from(-1.0)));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (-(Simd32x4::from([self.group0()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(reverse.group0(), 0, 3, 1, 2))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[0], self.group1()[1]]) * swizzle!(reverse.group0(), 2, 1, 2, 1))
                + (Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group0()[0]]) * swizzle!(reverse.group0(), 3, 2, 3, 3))
                + (Simd32x4::from([self.group1()[0], self.group1()[3], self.group1()[3], self.group0()[2]]) * swizzle!(reverse.group1(), 0, 0, 1, 0))
                + (Simd32x4::from([self.group1()[1], self.group0()[3], self.group0()[1], self.group1()[3]]) * swizzle!(reverse.group1(), 1, 1, 2, 2))
                - (Simd32x4::from([self.group1()[3], self.group0()[2], self.group0()[3], self.group0()[1]]) * swizzle!(reverse.group1(), 3, 2, 0, 1))
                + (swizzle!(reverse.group0(), 1, 0, 0, 0) * swizzle!(self.group0(), 1, 1, 2, 3))
                + (swizzle!(reverse.group1(), 2, 3, 3, 3) * swizzle!(self.group1(), 2, 0, 1, 2))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)
                + f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[3], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       16        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       24       21        0
    //  no simd       36       36        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0(),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                (-(reverse.group0()[3] * self.group0()[3])
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])),
                ((reverse.group0()[1] * self.group0()[0])
                    + (reverse.group1()[0] * self.group1()[3])
                    + (reverse.group1()[1] * self.group0()[3])
                    + (reverse.group1()[3] * self.group1()[0])),
                ((reverse.group0()[2] * self.group0()[0])
                    + (reverse.group1()[1] * self.group1()[3])
                    + (reverse.group1()[2] * self.group0()[1])
                    + (reverse.group1()[3] * self.group1()[1])),
                ((reverse.group0()[3] * self.group0()[0])
                    + (reverse.group1()[0] * self.group0()[2])
                    + (reverse.group1()[2] * self.group1()[3])
                    + (reverse.group1()[3] * self.group1()[2])),
            ]) + (Simd32x4::from(reverse.group0()[0]) * self.group0())
                - (Simd32x4::from([reverse.group0()[2], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]) * swizzle!(self.group0(), 2, 2, 3, 1))
                + (Simd32x4::from([reverse.group1()[3], reverse.group0()[2], reverse.group0()[3], reverse.group0()[1]]) * swizzle!(self.group1(), 3, 2, 0, 1))
                - (Simd32x4::from([self.group0()[1], self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(reverse.group0(), 1, 3, 1, 2))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)
                - f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[3], 2)
                - f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[3], 2)),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
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
impl ConstraintViolation for PlaneOnOrigin {
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
impl ConstraintViolation for RoundPointAtOrigin {
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
impl ConstraintViolation for SphereAtOrigin {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for SphereOnOrigin {
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
impl ConstraintViolation for VersorEvenAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       45        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       36       51        0
    //  no simd       54       69        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
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
                    - (reverse.group1()[3] * self.group2()[3])
                    + (reverse.group2()[0] * self.group0()[0])
                    + (reverse.group2()[1] * self.group0()[1])
                    + (reverse.group2()[2] * self.group0()[2])
                    - (reverse.group2()[3] * self.group1()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
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
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-(reverse.group0()[1] * self.group2()[2]) - (reverse.group1()[3] * self.group2()[0])),
                (-(reverse.group0()[2] * self.group2()[0]) - (reverse.group1()[3] * self.group2()[1])),
                (-(reverse.group0()[2] * self.group2()[3]) - (reverse.group1()[3] * self.group2()[2])),
                ((reverse.group1()[1] * self.group2()[1]) + (reverse.group1()[2] * self.group2()[2])),
            ]) + (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[2]]) * swizzle!(reverse.group2(), 3, 3, 3, 2))
                + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group1()[3], self.group1()[1]]) * swizzle!(reverse.group2(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 0, 1, 0, 0))
                - (swizzle!(reverse.group0(), 0, 1, 0, 3) * swizzle!(self.group2(), 3, 3, 1, 3))
                - (swizzle!(reverse.group2(), 2, 0, 1, 3) * swizzle!(self.group0(), 1, 2, 0, 3))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                + 2.0 * (self.group0()[0] * self.group2()[0])
                + 2.0 * (self.group0()[1] * self.group2()[1])
                + 2.0 * (self.group0()[2] * self.group2()[2])
                - 2.0 * (self.group1()[3] * self.group2()[3])),
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
impl ConstraintViolation for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       19        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       30       28        0
    //  no simd       54       55        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (-(reverse.group0()[0] * self.group0()[0])
                    + (reverse.group0()[1] * self.group0()[1])
                    + (reverse.group0()[2] * self.group0()[2])
                    + (reverse.group0()[3] * self.group0()[3])
                    + (reverse.group1()[0] * self.group1()[0])
                    + (reverse.group1()[1] * self.group1()[1])
                    + (reverse.group1()[2] * self.group1()[2])
                    - (reverse.group1()[3] * self.group1()[3])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group2()[0] * self.group0()[1]) + (reverse.group2()[0] * self.group1()[0]) - (reverse.group2()[1] * self.group0()[2])
                    + (reverse.group2()[1] * self.group1()[1])
                    - (reverse.group2()[2] * self.group0()[3])
                    + (reverse.group2()[2] * self.group1()[2])
                    - (reverse.group2()[3] * self.group0()[0])
                    + (reverse.group2()[3] * self.group1()[3])),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[1], self.group2()[1]]) * swizzle!(reverse.group0(), 1, 2, 1, 2))
                + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], self.group2()[0]]) * swizzle!(reverse.group0(), 0, 0, 0, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[3], self.group0()[1], self.group2()[3]]) * swizzle!(reverse.group1(), 2, 0, 1, 3))
                + (Simd32x4::from([self.group0()[3], self.group0()[1], self.group1()[3], self.group2()[1]]) * swizzle!(reverse.group1(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[2]]) * swizzle!(reverse.group1(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[3]]) * swizzle!(reverse.group0(), 3, 1, 2, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group0()[0], self.group2()[2]]) * swizzle!(reverse.group0(), 2, 3, 3, 3))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group0()[2], self.group2()[0]]) * swizzle!(reverse.group1(), 0, 1, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)
                + f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[3], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[3], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorEvenAtOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       22        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       32       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (Simd32x4::from([
                ((reverse.group0()[1] * self.group1()[1]) + (reverse.group0()[2] * self.group1()[2])),
                (-(reverse.group0()[1] * self.group1()[2]) - (reverse.group0()[3] * self.group1()[0])),
                (-(reverse.group0()[2] * self.group1()[0]) - (reverse.group0()[3] * self.group1()[1])),
                (-(reverse.group0()[2] * self.group1()[3]) - (reverse.group0()[3] * self.group1()[2])),
            ]) + (swizzle!(reverse.group0(), 0, 2, 0, 1) * swizzle!(self.group1(), 0, 1, 2, 0))
                - (swizzle!(reverse.group0(), 3, 0, 1, 0) * swizzle!(self.group1(), 3, 3, 3, 1))
                + (swizzle!(reverse.group1(), 0, 0, 1, 0) * swizzle!(self.group0(), 0, 3, 3, 1))
                + (swizzle!(reverse.group1(), 1, 1, 2, 2) * swizzle!(self.group0(), 1, 2, 0, 3))
                + (swizzle!(reverse.group1(), 2, 3, 3, 3) * swizzle!(self.group0(), 2, 0, 1, 2))
                - (swizzle!(reverse.group1(), 3, 2, 0, 1) * swizzle!(self.group0(), 3, 1, 2, 0))),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (2.0 * (self.group0()[0] * self.group1()[0]) + 2.0 * (self.group0()[1] * self.group1()[1]) + 2.0 * (self.group0()[2] * self.group1()[2])
                - 2.0 * (self.group0()[3] * self.group1()[3])),
        );
        let subtraction = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorEvenOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        4        4        0
    // Totals...
    // yes simd       11       14        0
    //  no simd       15       18        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            (Simd32x2::from([
                ((reverse.group1()[0] * self.group0()[0]) + (reverse.group1()[1] * self.group0()[1]) + (reverse.group1()[2] * self.group0()[2])
                    - (reverse.group1()[3] * self.group0()[3])),
                0.0,
            ]) - (Simd32x2::from(reverse.group0()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group0()[0], reverse.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group0()[1], reverse.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group0()[2], reverse.group1()[2]]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)),
        );
        let subtraction = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       47        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       42       52        0
    //  no simd       54       67        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e1, e2, e3, e4
            self.group2(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group1()[0]) + (reverse.group0()[1] * self.group1()[1]) + (reverse.group0()[2] * self.group1()[2])
                    - (reverse.group0()[3] * self.group0()[3])
                    + (reverse.group1()[0] * self.group0()[0])
                    + (reverse.group1()[1] * self.group0()[1])
                    + (reverse.group1()[2] * self.group0()[2])
                    - (reverse.group1()[3] * self.group2()[3])
                    + (reverse.group2()[0] * self.group2()[0])
                    + (reverse.group2()[1] * self.group2()[1])
                    + (reverse.group2()[2] * self.group2()[2])
                    - (reverse.group2()[3] * self.group1()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((reverse.group0()[0] * self.group2()[0])
                    + (reverse.group0()[1] * self.group2()[1])
                    + (reverse.group0()[2] * self.group2()[2])
                    + (reverse.group0()[3] * self.group2()[3])
                    - (reverse.group2()[0] * self.group0()[0])
                    - (reverse.group2()[1] * self.group0()[1])
                    - (reverse.group2()[2] * self.group0()[2])
                    - (reverse.group2()[3] * self.group0()[3])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-(reverse.group0()[1] * self.group1()[2]) + (reverse.group1()[1] * self.group0()[2]) + (reverse.group1()[3] * self.group0()[0])
                    - (reverse.group2()[3] * self.group1()[0])),
                (-(reverse.group0()[2] * self.group1()[0]) + (reverse.group1()[2] * self.group0()[0]) + (reverse.group1()[3] * self.group0()[1])
                    - (reverse.group2()[3] * self.group1()[1])),
                (-(reverse.group0()[2] * self.group1()[3]) + (reverse.group1()[2] * self.group2()[3]) + (reverse.group1()[3] * self.group0()[2])
                    - (reverse.group2()[3] * self.group1()[2])),
                (-(reverse.group1()[1] * self.group2()[1]) - (reverse.group1()[2] * self.group2()[2])
                    + (reverse.group2()[1] * self.group1()[1])
                    + (reverse.group2()[2] * self.group1()[2])),
            ]) + (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group2()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group2()[0]]) * swizzle!(reverse.group1(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group2()[3], self.group2()[3], self.group0()[1], self.group0()[3]]) * swizzle!(reverse.group1(), 0, 1, 0, 3))
                - (swizzle!(reverse.group0(), 0, 1, 0, 3) * swizzle!(self.group1(), 3, 3, 1, 3))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2)
                + f32::powi(self.group2()[0], 2)
                + f32::powi(self.group2()[1], 2)
                + f32::powi(self.group2()[2], 2)
                + 2.0 * (self.group0()[0] * self.group1()[0])
                + 2.0 * (self.group0()[1] * self.group1()[1])
                + 2.0 * (self.group0()[2] * self.group1()[2])
                - 2.0 * (self.group1()[3] * self.group2()[3])),
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
impl ConstraintViolation for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       27        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       33       34        0
    //  no simd       51       55        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self.group0()[0], (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                ((reverse.group0()[0] * self.group0()[0])
                    - (reverse.group1()[0] * self.group1()[0])
                    - (reverse.group1()[1] * self.group1()[1])
                    - (reverse.group1()[2] * self.group1()[2])
                    + (reverse.group1()[3] * self.group1()[3])
                    - (reverse.group2()[0] * self.group2()[0])
                    - (reverse.group2()[1] * self.group2()[1])
                    - (reverse.group2()[2] * self.group2()[2])),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group1()[1] * self.group2()[2]) + (reverse.group1()[3] * self.group1()[0])),
                ((reverse.group1()[2] * self.group2()[0]) + (reverse.group1()[3] * self.group1()[1])),
                ((reverse.group1()[0] * self.group2()[1]) + (reverse.group1()[3] * self.group1()[2])),
                (-(reverse.group0()[2] * self.group1()[1])
                    - (reverse.group0()[2] * self.group2()[1])
                    - (reverse.group0()[3] * self.group1()[2])
                    - (reverse.group0()[3] * self.group2()[2])
                    - (reverse.group1()[0] * self.group0()[1])
                    - (reverse.group1()[1] * self.group0()[2])
                    - (reverse.group1()[2] * self.group0()[3])
                    - (reverse.group1()[3] * self.group2()[3])
                    + (reverse.group2()[2] * self.group0()[3])
                    + (reverse.group2()[3] * self.group0()[0])),
            ]) + (Simd32x4::from(reverse.group0()[0]) * self.group2())
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[3]]))
                - (Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group0()[1]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[1]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[1], self.group0()[1]]) * swizzle!(reverse.group2(), 0, 1, 0, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group0()[0], self.group0()[2]]) * swizzle!(reverse.group2(), 1, 2, 2, 1))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[0], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group2()[0], 2)
                - f32::powi(self.group2()[1], 2)
                - f32::powi(self.group2()[2], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(geometric_product.group0()[0] - scalar_product[scalar]), 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       45        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       36       51        0
    //  no simd       54       69        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
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
                    + (reverse.group1()[3] * self.group2()[3])
                    - (reverse.group2()[0] * self.group0()[0])
                    - (reverse.group2()[1] * self.group0()[1])
                    - (reverse.group2()[2] * self.group0()[2])
                    + (reverse.group2()[3] * self.group1()[3])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(reverse.group0()[0] * self.group1()[0]) - (reverse.group0()[1] * self.group1()[1]) - (reverse.group0()[2] * self.group1()[2])
                    + (reverse.group0()[3] * self.group2()[3])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2])
                    + (reverse.group2()[3] * self.group0()[3])),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                ((reverse.group0()[1] * self.group2()[2]) + (reverse.group2()[3] * self.group2()[0])),
                ((reverse.group0()[2] * self.group2()[0]) + (reverse.group2()[3] * self.group2()[1])),
                ((reverse.group0()[2] * self.group1()[3]) + (reverse.group2()[3] * self.group2()[2])),
                (-(reverse.group1()[2] * self.group2()[2]) - (reverse.group2()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[0]]) * swizzle!(self.group2(), 1, 2, 0, 0))
                + (Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group1()[3]]) * swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[1]]) * swizzle!(reverse.group1(), 3, 3, 3, 1))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group2()[3], self.group1()[1]]) * swizzle!(reverse.group2(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group2()[1], self.group1()[3]]) * swizzle!(reverse.group0(), 0, 1, 0, 3))
                - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group0()[1], self.group1()[0]]) * swizzle!(reverse.group2(), 0, 1, 0, 0))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2)
                - f32::powi(self.group1()[0], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[2], 2)
                - 2.0 * (self.group0()[0] * self.group2()[0])
                - 2.0 * (self.group0()[1] * self.group2()[1])
                - 2.0 * (self.group0()[2] * self.group2()[2])
                + 2.0 * (self.group1()[3] * self.group2()[3])),
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
