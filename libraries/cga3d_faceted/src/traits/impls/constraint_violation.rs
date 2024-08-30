// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 95
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        22      29       0
//  Average:        35      42       0
//  Maximum:       533     551       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        34      46       0
//  Average:        58      68       0
//  Maximum:      1024    1060       0
impl ConstraintViolation for AntiCircleOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       22       33        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group1()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[0])),
                ((self.group1()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[1])
                    + (self.group0()[1] * reverse.group1()[0])),
                (-(self.group1()[2] * reverse.group1()[2]) - (self.group1()[0] * reverse.group1()[0]) - (self.group1()[1] * reverse.group1()[1])),
            ]),
            // e23, e31, e12, e1234
            Simd32x4::from([
                (-(self.group1()[1] * reverse.group1()[2]) + (self.group1()[2] * reverse.group1()[1])),
                ((self.group1()[0] * reverse.group1()[2]) - (self.group1()[2] * reverse.group1()[0])),
                (-(self.group1()[0] * reverse.group1()[1]) + (self.group1()[1] * reverse.group1()[0])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2)));
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       74        0
    //    simd3        0        1        0
    //    simd4       14       15        0
    // Totals...
    // yes simd       74       90        0
    //  no simd      116      137        0
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
            ((Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                - (swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[2]]))
                + (swizzle!(self.group1(), 2, 0, 1, 3) * Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse.group0()[0], reverse.group1()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[1]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 0) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[0]]))
                + Simd32x4::from([
                    ((self.group0()[2] * reverse.group1()[1]) + (self.group0()[0] * reverse.group1()[3]) + (self.group0()[0] * reverse.group2()[3])),
                    ((self.group0()[1] * reverse.group2()[3]) + (self.group0()[0] * reverse.group1()[2]) + (self.group0()[1] * reverse.group1()[3])),
                    ((self.group0()[2] * reverse.group2()[3]) + (self.group0()[2] * reverse.group1()[3]) + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group2()[2] * reverse.group0()[2])
                        - (self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group2()[2])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group2()[3]) * reverse.group1())
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group0()[2]]))
                + (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group2()[3], reverse.group2()[3], reverse.group2()[3]]))
                + (swizzle!(reverse.group2(), 3, 2, 0, 2) * Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group1()[2]) - (self.group0()[1] * reverse.group2()[2])
                        + (self.group0()[2] * reverse.group2()[1])),
                    ((self.group2()[0] * reverse.group0()[2]) - (self.group1()[2] * reverse.group1()[0]) + (self.group1()[0] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group2()[0])),
                    ((self.group2()[1] * reverse.group0()[0]) + (self.group1()[1] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group2()[1] * reverse.group0()[1]) - (self.group2()[0] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e15, e25, e35, e1234
            (-(swizzle!(reverse.group1(), 2, 0, 3, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[2], self.group0()[2]]))
                - (swizzle!(reverse.group1(), 3, 3, 1, 0) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[0], self.group0()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[2]]))
                + Simd32x4::from([
                    ((self.group2()[3] * reverse.group2()[0])
                        + (self.group2()[2] * reverse.group1()[1])
                        + (self.group2()[0] * reverse.group2()[3])
                        + (self.group1()[3] * reverse.group2()[0])
                        + (self.group1()[2] * reverse.group2()[1])),
                    ((self.group2()[3] * reverse.group2()[1])
                        + (self.group2()[1] * reverse.group2()[3])
                        + (self.group2()[0] * reverse.group1()[2])
                        + (self.group1()[3] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group2()[2])),
                    ((self.group2()[3] * reverse.group2()[2])
                        + (self.group2()[2] * reverse.group2()[3])
                        + (self.group2()[1] * reverse.group1()[0])
                        + (self.group1()[3] * reverse.group2()[2])
                        + (self.group1()[1] * reverse.group2()[0])),
                    (-(self.group1()[1] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group0()[1])
                        + (self.group1()[3] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group1()[3])
                        + (self.group0()[1] * reverse.group2()[2])),
                    ((self.group2()[0] * reverse.group0()[2])
                        + (self.group1()[3] * reverse.group1()[1])
                        + (self.group1()[1] * reverse.group1()[3])
                        + (self.group0()[2] * reverse.group2()[0])),
                    ((self.group2()[1] * reverse.group0()[0])
                        + (self.group1()[3] * reverse.group1()[2])
                        + (self.group1()[2] * reverse.group1()[3])
                        + (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group2()[1] * reverse.group1()[1])
                        - (self.group2()[0] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group2()[0])
                        - (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group2()[3], 2) - (self.group2()[2] * self.group0()[2]) - (self.group2()[1] * self.group0()[1]) - (self.group2()[0] * self.group0()[0])
                + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       70       85        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       76       93        0
    //  no simd       94      115        0
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
            ((Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group0()[2])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group2()[3])
                        - (self.group0()[1] * reverse.group1()[2])),
                    (-(self.group1()[2] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group2()[3])),
                    ((self.group1()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[1]) + (self.group0()[2] * reverse.group2()[3])
                        - (self.group0()[0] * reverse.group1()[1])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group2()[2] * reverse.group0()[2])
                        - (self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group1()[2])
                        - (self.group1()[1] * reverse.group1()[1])
                        - (self.group1()[0] * reverse.group1()[0])
                        - (self.group0()[2] * reverse.group2()[2])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group0()[2]]))
                + (swizzle!(reverse.group2(), 3, 3, 3, 2) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]))
                + (swizzle!(reverse.group2(), 1, 2, 0, 0) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[0]]))
                + Simd32x4::from([
                    ((self.group2()[3] * reverse.group1()[0]) + (self.group2()[2] * reverse.group0()[1]) + (self.group1()[2] * reverse.group1()[1])
                        - (self.group1()[1] * reverse.group1()[2])
                        - (self.group0()[1] * reverse.group2()[2])),
                    ((self.group2()[3] * reverse.group1()[1]) + (self.group2()[0] * reverse.group0()[2]) - (self.group1()[2] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group2()[0])),
                    ((self.group2()[3] * reverse.group1()[2]) + (self.group2()[1] * reverse.group0()[0]) + (self.group1()[1] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group2()[1] * reverse.group0()[1]) - (self.group2()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group2()[3] * reverse.group2()[0]) + (self.group2()[2] * reverse.group1()[1]) - (self.group2()[1] * reverse.group1()[2])
                    + (self.group2()[0] * reverse.group2()[3])
                    - (self.group1()[1] * reverse.group2()[2])
                    + (self.group1()[2] * reverse.group2()[1])),
                ((self.group2()[3] * reverse.group2()[1]) - (self.group2()[2] * reverse.group1()[0])
                    + (self.group2()[1] * reverse.group2()[3])
                    + (self.group2()[0] * reverse.group1()[2])
                    + (self.group1()[0] * reverse.group2()[2])
                    - (self.group1()[2] * reverse.group2()[0])),
                ((self.group2()[3] * reverse.group2()[2]) + (self.group2()[2] * reverse.group2()[3]) + (self.group2()[1] * reverse.group1()[0])
                    - (self.group2()[0] * reverse.group1()[1])
                    - (self.group1()[0] * reverse.group2()[1])
                    + (self.group1()[1] * reverse.group2()[0])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group0()[1]) + (self.group0()[1] * reverse.group2()[2])),
                    ((self.group2()[0] * reverse.group0()[2]) + (self.group0()[2] * reverse.group2()[0])),
                    ((self.group2()[1] * reverse.group0()[0]) + (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group2()[1] * reverse.group1()[1])
                        - (self.group2()[0] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group2()[0])
                        - (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group2()[3], 2)
                - (self.group2()[2] * self.group0()[2])
                - (self.group2()[1] * self.group0()[1])
                - (self.group2()[0] * self.group0()[0])
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       24       31        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       35        0
    //  no simd       36       46        0
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
            ((Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + Simd32x4::from([
                    ((self.group0()[2] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[3]) - (self.group0()[1] * reverse.group0()[2])),
                    (-(self.group0()[2] * reverse.group0()[0]) + (self.group0()[0] * reverse.group0()[2]) + (self.group0()[1] * reverse.group1()[3])),
                    ((self.group0()[2] * reverse.group1()[3]) - (self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
                    (-(self.group0()[2] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
                ])),
            // e15, e25, e35, e3215
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * reverse.group1()[0])
                        + (self.group1()[2] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group1()[3])
                        + (self.group0()[2] * reverse.group1()[1])),
                    ((self.group1()[3] * reverse.group1()[1])
                        + (self.group1()[1] * reverse.group1()[3])
                        + (self.group1()[0] * reverse.group0()[2])
                        + (self.group0()[0] * reverse.group1()[2])),
                    ((self.group1()[3] * reverse.group1()[2])
                        + (self.group1()[2] * reverse.group1()[3])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[3], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       30        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       29       38        0
    //  no simd       50       62        0
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
            ((Simd32x4::from(self.group1()[3]) * swizzle!(reverse.group1(), 3, 0, 1, 2))
                + (swizzle!(reverse.group0(), 3, 1, 2, 0) * Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group1()[1]]))
                - (swizzle!(reverse.group0(), 2, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group1()[1], self.group1()[2], self.group1()[2]]))
                - (swizzle!(reverse.group0(), 0, 3, 3, 1) * Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[1], self.group1()[0]]))
                - (swizzle!(self.group0(), 1, 1, 2, 0) * Simd32x4::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    ((self.group1()[0] * reverse.group1()[3]) + (self.group0()[3] * reverse.group1()[0]) + (self.group0()[2] * reverse.group1()[1])),
                    ((self.group1()[1] * reverse.group1()[3]) + (self.group0()[3] * reverse.group1()[1]) + (self.group0()[0] * reverse.group1()[2])),
                    ((self.group1()[2] * reverse.group1()[3]) + (self.group0()[3] * reverse.group1()[2]) + (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group1()[3]) * reverse.group0())
                + (swizzle!(self.group0(), 2, 0, 2, 3) * Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse.group1()[3], reverse.group1()[3]]))
                + Simd32x4::from([
                    ((self.group0()[0] * reverse.group1()[3]) - (self.group0()[1] * reverse.group0()[2])),
                    (-(self.group0()[2] * reverse.group0()[0]) + (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
                    0.0,
                ])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self.group0()[0] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[0])),
                ((self.group0()[1] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[1])),
                ((self.group0()[2] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[2])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[3], 2) + f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       39        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       36       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            ((swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[3]]))
                + Simd32x4::from([
                    (-(self.group1()[1] * reverse.group0()[2])
                        + (self.group0()[3] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group0()[3])
                        - (self.group0()[1] * reverse.group1()[2])),
                    (-(self.group1()[2] * reverse.group0()[0]) + (self.group0()[3] * reverse.group0()[1]) - (self.group0()[2] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group1()[0] * reverse.group0()[1]) + (self.group0()[3] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[3])
                        - (self.group0()[0] * reverse.group1()[1])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group1()[2] * reverse.group1()[2]) - (self.group1()[1] * reverse.group1()[1]) - (self.group1()[0] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e1234
            Simd32x4::from([
                ((self.group1()[2] * reverse.group1()[1]) - (self.group1()[1] * reverse.group1()[2])
                    + (self.group0()[3] * reverse.group1()[0])
                    + (self.group1()[0] * reverse.group0()[3])),
                (-(self.group1()[2] * reverse.group1()[0])
                    + (self.group1()[1] * reverse.group0()[3])
                    + (self.group0()[3] * reverse.group1()[1])
                    + (self.group1()[0] * reverse.group1()[2])),
                ((self.group1()[2] * reverse.group0()[3]) + (self.group1()[1] * reverse.group1()[0]) + (self.group0()[3] * reverse.group1()[2])
                    - (self.group1()[0] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[1], 2) + f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2)),
        );
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       72        0
    //    simd3        0        1        0
    //    simd4       41       42        0
    // Totals...
    // yes simd      101      115        0
    //  no simd      224      243        0
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
            (-(swizzle!(self.group3(), 2, 1, 2, 3) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[3], reverse.group2()[3]]))
                + (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group3()[2]]))
                + (swizzle!(reverse.group3(), 0, 1, 2, 1) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group3()[1]]))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group3()[3]]))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (Simd32x4::from(self.group0()[2]) * Simd32x4::from([reverse.group3()[1], reverse.group1()[0], reverse.group1()[3], reverse.group2()[2]]))
                + (swizzle!(reverse.group1(), 3, 3, 1, 1) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group1()[1]]))
                + Simd32x4::from([
                    (-(self.group3()[0] * reverse.group2()[3])
                        - (self.group1()[2] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group2()[3])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group3()[2])
                        + (self.group0()[1] * reverse.group1()[2])),
                    (-(self.group3()[0] * reverse.group0()[2])
                        - (self.group1()[1] * reverse.group2()[3])
                        - (self.group1()[0] * reverse.group0()[2])
                        - (self.group0()[2] * reverse.group3()[0])
                        - (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group3()[2])),
                    (-(self.group3()[1] * reverse.group0()[0]) - (self.group1()[2] * reverse.group2()[3]) - (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[1] * reverse.group3()[0])
                        - (self.group0()[1] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group3()[1])),
                    ((self.group3()[0] * reverse.group3()[0])
                        + (self.group2()[2] * reverse.group0()[2])
                        + (self.group2()[1] * reverse.group0()[1])
                        + (self.group2()[0] * reverse.group0()[0])
                        + (self.group1()[0] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(Simd32x4::from(self.group3()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                - (swizzle!(self.group3(), 2, 1, 2, 2) * Simd32x4::from([reverse.group3()[1], reverse.group1()[3], reverse.group1()[3], reverse.group1()[2]]))
                + (swizzle!(reverse.group3(), 2, 0, 1, 3) * Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group2()[3]]))
                - (swizzle!(self.group3(), 0, 0, 1, 1) * Simd32x4::from([reverse.group1()[3], reverse.group3()[2], reverse.group3()[0], reverse.group1()[1]]))
                - (swizzle!(reverse.group2(), 0, 1, 2, 2) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[2]]))
                + (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group2(), 3, 3, 3, 0) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]))
                - (swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(reverse.group3(), 0, 1, 2, 2))
                - (swizzle!(reverse.group1(), 1, 2, 0, 0) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group3()[0]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[1]]))
                - (swizzle!(reverse.group3(), 3, 3, 3, 1) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group1()[2]) + (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[0] * reverse.group0()[2]) + (self.group1()[2] * reverse.group1()[0]) + (self.group0()[2] * reverse.group2()[0])),
                    (-(self.group2()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group1()[1]) + (self.group0()[0] * reverse.group2()[1])),
                    ((self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0]) - (self.group1()[0] * reverse.group3()[0])),
                ])),
            // e15, e25, e35, e1234
            (-(swizzle!(reverse.group3(), 0, 1, 2, 2) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2, 2) * Simd32x4::from([reverse.group2()[1], reverse.group3()[3], reverse.group3()[3], reverse.group0()[2]]))
                - (swizzle!(reverse.group2(), 2, 0, 1, 3) * Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group1()[3]]))
                + (swizzle!(self.group3(), 0, 0, 1, 1) * Simd32x4::from([reverse.group3()[3], reverse.group2()[2], reverse.group2()[0], reverse.group0()[1]]))
                - (swizzle!(reverse.group3(), 1, 2, 0, 1) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group0()[1]]))
                + (swizzle!(self.group2(), 1, 2, 0, 3) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group1()[3]]))
                + (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group0()[2]]))
                + (swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group0()[2]]))
                - (swizzle!(reverse.group3(), 3, 3, 3, 0) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]))
                + (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[1]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * reverse.group1()[0])
                        - (self.group2()[2] * reverse.group1()[1])
                        - (self.group2()[0] * reverse.group1()[3])
                        - (self.group1()[2] * reverse.group2()[1])),
                    (-(self.group3()[3] * reverse.group1()[1])
                        - (self.group2()[1] * reverse.group1()[3])
                        - (self.group2()[0] * reverse.group1()[2])
                        - (self.group1()[0] * reverse.group2()[2])),
                    (-(self.group3()[3] * reverse.group1()[2])
                        - (self.group2()[2] * reverse.group1()[3])
                        - (self.group2()[1] * reverse.group1()[0])
                        - (self.group1()[1] * reverse.group2()[0])),
                    ((self.group3()[0] * reverse.group0()[0])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[1] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group1()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self.group3()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (swizzle!(reverse.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[2]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[2]]))
                + (swizzle!(self.group2(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group3()[2]]))
                + (swizzle!(self.group2(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group3()[1]]))
                - (swizzle!(reverse.group2(), 3, 3, 3, 1) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[1]]))
                + (swizzle!(reverse.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[1]]))
                + (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group3()[1], reverse.group1()[3], reverse.group1()[3], reverse.group3()[3]]))
                + (swizzle!(self.group1(), 0, 0, 1, 2) * Simd32x4::from([reverse.group1()[3], reverse.group3()[2], reverse.group3()[0], reverse.group2()[2]]))
                + (swizzle!(reverse.group2(), 1, 2, 0, 0) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]))
                + (swizzle!(reverse.group3(), 3, 3, 3, 0) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]))
                - (swizzle!(reverse.group2(), 2, 0, 1, 0) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group3()[2])),
                    (-(self.group2()[0] * reverse.group0()[2]) - (self.group1()[2] * reverse.group3()[0])),
                    (-(self.group2()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group3()[1])),
                    ((self.group2()[0] * reverse.group1()[0]) + (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group3()[3] * self.group2()[3]) + f32::powi(self.group3()[2], 2) + f32::powi(self.group3()[1], 2) + f32::powi(self.group3()[0], 2)
                - (self.group2()[3] * self.group3()[3])
                + (self.group2()[2] * self.group0()[2])
                + (self.group2()[1] * self.group0()[1])
                + (self.group2()[0] * self.group0()[0])
                - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       37       44        0
    //    simd3        0        1        0
    //    simd4       16       17        0
    // Totals...
    // yes simd       53       62        0
    //  no simd      101      115        0
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
            ((swizzle!(self.group2(), 2, 2, 1, 2) * Simd32x4::from([reverse.group2()[2], reverse.group1()[1], reverse.group2()[3], reverse.group2()[3]]))
                + (swizzle!(self.group2(), 1, 0, 0, 1) * Simd32x4::from([reverse.group2()[1], reverse.group2()[3], reverse.group1()[2], reverse.group1()[0]]))
                + (swizzle!(reverse.group2(), 0, 2, 0, 1) * Simd32x4::from([self.group2()[0], self.group1()[1], self.group1()[2], self.group1()[0]]))
                - (swizzle!(reverse.group0(), 3, 0, 1, 2) * Simd32x4::from([self.group0()[3], self.group2()[3], self.group2()[3], self.group2()[3]]))
                + (swizzle!(reverse.group0(), 2, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (swizzle!(self.group0(), 0, 3, 3, 3) * Simd32x4::from([reverse.group0()[0], reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(self.group0(), 1, 1, 2, 0) * Simd32x4::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    (-(self.group2()[3] * reverse.group2()[0])
                        - (self.group2()[1] * reverse.group1()[2])
                        - (self.group1()[2] * reverse.group2()[1])
                        - (self.group1()[2] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[3])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group2()[3])),
                    (-(self.group2()[3] * reverse.group2()[1])
                        - (self.group2()[2] * reverse.group1()[0])
                        - (self.group1()[1] * reverse.group0()[3])
                        - (self.group1()[0] * reverse.group2()[2])
                        - (self.group1()[0] * reverse.group0()[2])
                        - (self.group0()[0] * reverse.group1()[2])
                        - (self.group0()[1] * reverse.group2()[3])),
                    (-(self.group2()[3] * reverse.group2()[2])
                        - (self.group2()[0] * reverse.group1()[1])
                        - (self.group1()[2] * reverse.group0()[3])
                        - (self.group1()[1] * reverse.group2()[0])
                        - (self.group1()[1] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group2()[3])
                        - (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group2(), 2, 1, 2, 2) * Simd32x4::from([reverse.group2()[1], reverse.group0()[3], reverse.group0()[3], reverse.group0()[2]]))
                - (swizzle!(self.group2(), 0, 0, 1, 1) * Simd32x4::from([reverse.group0()[3], reverse.group2()[2], reverse.group2()[0], reverse.group0()[1]]))
                - (swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(reverse.group2(), 0, 1, 2, 2))
                - (swizzle!(reverse.group0(), 1, 2, 0, 0) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group2()[0]]))
                + Simd32x4::from([
                    ((self.group2()[1] * reverse.group2()[2]) + (self.group0()[1] * reverse.group0()[2])),
                    ((self.group2()[2] * reverse.group2()[0]) + (self.group0()[2] * reverse.group0()[0])),
                    ((self.group2()[0] * reverse.group2()[1]) + (self.group0()[0] * reverse.group0()[1])),
                    (-(self.group0()[0] * reverse.group2()[0]) - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(reverse.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[2]]))
                - (swizzle!(self.group2(), 1, 2, 0, 3) * swizzle!(reverse.group0(), 2, 0, 1, 3))
                + (swizzle!(reverse.group0(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[1]]))
                + (swizzle!(self.group0(), 2, 0, 2, 3) * Simd32x4::from([reverse.group2()[1], reverse.group2()[2], reverse.group0()[3], reverse.group2()[3]]))
                + (swizzle!(self.group0(), 0, 1, 1, 2) * Simd32x4::from([reverse.group0()[3], reverse.group0()[3], reverse.group2()[0], reverse.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[1] * reverse.group2()[2]) * -1.0),
                    ((self.group0()[2] * reverse.group2()[0]) * -1.0),
                    ((self.group0()[0] * reverse.group2()[1]) * -1.0),
                    (-(self.group2()[2] * reverse.group1()[2]) - (self.group2()[1] * reverse.group1()[1]) - (self.group2()[0] * reverse.group1()[0])
                        + (self.group1()[2] * reverse.group2()[2])
                        + (self.group1()[1] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group2()[0])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group2()[2], 2) + f32::powi(self.group2()[1], 2) + f32::powi(self.group2()[0], 2) - f32::powi(self.group0()[3], 2)
                + f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       24       31        0
    //  no simd       45       55        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e4, e1, e2, e3 */ self.group1());
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (-(swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[3], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(self.group1(), 2, 3, 1, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 0, 0, 0, 2) * swizzle!(reverse.group1(), 1, 2, 3, 2))
                + (swizzle!(reverse.group1(), 2, 3, 1, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[1]]))
                + Simd32x4::from([
                    (-(self.group1()[1] * reverse.group1()[0]) - (self.group0()[3] * reverse.group0()[0]) + (self.group0()[0] * reverse.group0()[3])
                        - (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group1()[2] * reverse.group1()[0]) - (self.group0()[3] * reverse.group0()[1]) - (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group1()[3] * reverse.group1()[0]) - (self.group0()[3] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[3])
                        - (self.group0()[0] * reverse.group1()[2])),
                    0.0,
                ])),
            // e23, e31, e12, e1234
            (-(swizzle!(reverse.group1(), 2, 2, 1, 0) * Simd32x4::from([self.group1()[3], self.group0()[3], self.group1()[2], self.group0()[3]]))
                + (swizzle!(self.group1(), 2, 3, 1, 3) * Simd32x4::from([reverse.group1()[3], reverse.group1()[1], reverse.group1()[2], reverse.group0()[2]]))
                - (swizzle!(reverse.group1(), 1, 3, 3, 3) * Simd32x4::from([self.group0()[3], self.group1()[1], self.group0()[3], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[1] * reverse.group0()[3]) * -1.0),
                    ((self.group1()[2] * reverse.group0()[3]) * -1.0),
                    ((self.group1()[3] * reverse.group0()[3]) * -1.0),
                    ((self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[3])
                        - (self.group0()[0] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group1()[2])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[3], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[1], 2)),
        );
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       87        0
    //    simd3        0        1        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       80      100        0
    //  no simd      116      138        0
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
            (-(Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group0()[3]]))
                + (swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[2]]))
                - (Simd32x4::from(reverse.group2()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1]) - (self.group0()[2] * reverse.group1()[1])),
                    (-(self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[1] * reverse.group0()[0]) - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group2()[1] * reverse.group0()[1])
                        + (self.group2()[0] * reverse.group0()[0])
                        + (self.group1()[2] * reverse.group1()[2])
                        + (self.group1()[1] * reverse.group1()[1])
                        + (self.group1()[0] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(reverse.group2() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[3]]))
                + (swizzle!(self.group2(), 1, 2, 0, 3) * swizzle!(reverse.group0(), 2, 0, 1, 3))
                - (swizzle!(reverse.group2(), 3, 3, 3, 2) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[2]]))
                - (swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[0]]))
                - (swizzle!(self.group0(), 2, 0, 2, 1) * Simd32x4::from([reverse.group2()[1], reverse.group2()[2], reverse.group0()[3], reverse.group2()[1]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) - (self.group1()[2] * reverse.group1()[1]) + (self.group1()[1] * reverse.group1()[2])
                        - (self.group0()[0] * reverse.group0()[3])
                        + (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[0] * reverse.group0()[2]) + (self.group1()[2] * reverse.group1()[0]) - (self.group1()[0] * reverse.group1()[2])
                        + (self.group0()[2] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group2()[1] * reverse.group0()[0]) - (self.group1()[1] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group2()[1])
                        - (self.group0()[1] * reverse.group2()[0])),
                    ((self.group2()[2] * reverse.group0()[2]) + (self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * reverse.group1()[1]) + (self.group2()[1] * reverse.group1()[2]) - (self.group1()[2] * reverse.group2()[1])
                    + (self.group1()[1] * reverse.group2()[2])
                    - (self.group0()[3] * reverse.group1()[0])
                    - (self.group1()[0] * reverse.group0()[3])),
                ((self.group2()[2] * reverse.group1()[0]) - (self.group2()[0] * reverse.group1()[2]) + (self.group1()[2] * reverse.group2()[0])
                    - (self.group1()[1] * reverse.group0()[3])
                    - (self.group0()[3] * reverse.group1()[1])
                    - (self.group1()[0] * reverse.group2()[2])),
                (-(self.group2()[1] * reverse.group1()[0]) + (self.group2()[0] * reverse.group1()[1])
                    - (self.group1()[2] * reverse.group0()[3])
                    - (self.group1()[1] * reverse.group2()[0])
                    - (self.group0()[3] * reverse.group1()[2])
                    + (self.group1()[0] * reverse.group2()[1])),
                ((self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            ((swizzle!(self.group2(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group1()[2]]))
                + (swizzle!(self.group2(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[1]]))
                + (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) - (self.group2()[0] * reverse.group2()[3]) - (self.group0()[3] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group0()[3])
                        - (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[1] * reverse.group2()[3])
                        - (self.group2()[0] * reverse.group0()[2])
                        - (self.group0()[3] * reverse.group0()[1])
                        - (self.group0()[2] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group2()[2] * reverse.group2()[3]) - (self.group2()[1] * reverse.group0()[0]) - (self.group0()[3] * reverse.group0()[2])
                        + (self.group0()[2] * reverse.group0()[3])
                        - (self.group0()[0] * reverse.group2()[1])),
                    ((self.group2()[0] * reverse.group1()[0]) + (self.group1()[0] * reverse.group2()[0]) + (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group2()[3] * self.group0()[3])
                + (self.group2()[2] * self.group0()[2])
                + (self.group2()[1] * self.group0()[1])
                + (self.group2()[0] * self.group0()[0])
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                - (self.group0()[3] * self.group2()[3])
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDipoleOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group0()[0] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[0])),
                ((self.group0()[1] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[1])),
                ((self.group0()[2] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[2])),
                (self.group0()[3] * reverse.group0()[3] * -1.0),
            ]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
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
        let geometric_product = Scalar::from_groups(/* scalar */ (self[e321] * reverse[e321] * -1.0));
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self[e321], 2) * -1.0));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiFlatPoint {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (-(self.group0()[0] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[0])),
                (-(self.group0()[1] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[1])),
                (-(self.group0()[2] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[2])),
                (self.group0()[3] * reverse.group0()[3] * -1.0),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
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
impl ConstraintViolation for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       20       25        0
    //  no simd       44       52        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3, e5 */ self.group1());
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group1(), 2, 0, 1, 2))
                - (Simd32x4::from(self.group0()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group0()[3]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group1()[1]) - (self.group1()[0] * reverse.group0()[3])),
                    (-(self.group1()[1] * reverse.group0()[3]) - (self.group1()[0] * reverse.group1()[2])),
                    (-(self.group1()[2] * reverse.group0()[3]) - (self.group1()[1] * reverse.group1()[0])),
                    ((self.group1()[1] * reverse.group1()[1]) + (self.group1()[0] * reverse.group1()[0])),
                ])),
            // e15, e25, e35, e3215
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
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[1], 2) - f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiFlectorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       16       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_product = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            (-(swizzle!(self.group0(), 3, 2, 3, 0) * swizzle!(reverse.group0(), 2, 0, 0, 0))
                + (swizzle!(self.group0(), 2, 3, 1, 3) * swizzle!(reverse.group0(), 3, 1, 2, 3))
                + Simd32x4::from([
                    (-(self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
                    (-(self.group0()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group0()[2] * reverse.group0()[1]) - (self.group0()[0] * reverse.group0()[3])),
                    ((self.group0()[2] * reverse.group0()[2]) + (self.group0()[1] * reverse.group0()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       22       33        0
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
                (-(self.group0()[1] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[1])),
                ((self.group0()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[0])),
                (-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
                (-(self.group0()[2] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
            ]),
            // e15, e25, e35, e3215
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
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)));
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiLineOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       10        0
    //  no simd        8       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_product = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            (-(self.group0()[1] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[1])),
            ((self.group0()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[0])),
            (-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
            (-(self.group0()[2] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
        ]));
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)));
        let subtraction = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       30        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       36        0
    //  no simd       44       54        0
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
            ((Simd32x4::from(self.group0()[3]) * reverse.group0()) - (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group0()[2] * reverse.group0()[1]) + (self.group0()[0] * reverse.group0()[3])),
                    ((self.group0()[0] * reverse.group0()[2]) + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group0()[2] * reverse.group0()[3]) + (self.group0()[1] * reverse.group0()[0])),
                    (-(self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
                ])),
            // e15, e25, e35, e3215
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
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiMotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       13        0
    //  no simd       16       19        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            ((Simd32x4::from(self.group0()[3]) * reverse.group0()) - (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group0()[2] * reverse.group0()[1]) + (self.group0()[0] * reverse.group0()[3])),
                    ((self.group0()[0] * reverse.group0()[2]) + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group0()[2] * reverse.group0()[3]) + (self.group0()[1] * reverse.group0()[0])),
                    (-(self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiMysteryCircleRotor {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       14       17        0
    //  no simd       23       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)), /* scalar */ self[e31]);
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            ((swizzle!(self.group0(), 3, 0, 1, 2) * Simd32x4::from(reverse.group0()[3]))
                + Simd32x4::from([
                    ((self[e31] * reverse[e31]) - (self.group0()[2] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
                    (self.group0()[3] * reverse.group0()[0]),
                    (self.group0()[3] * reverse.group0()[1]),
                    (self.group0()[3] * reverse.group0()[2]),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self[e31]) * reverse.group0())
                + (swizzle!(self.group0(), 2, 0, 2, 3) * Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse[e31], reverse[e31]]))
                + Simd32x4::from([
                    ((self.group0()[0] * reverse[e31]) - (self.group0()[1] * reverse.group0()[2])),
                    (-(self.group0()[2] * reverse.group0()[0]) + (self.group0()[1] * reverse[e31])),
                    (-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
                    0.0,
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self[e31], 2) + f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiMysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       21        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       29        0
    //  no simd       48       53        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3 */ self.group1());
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (-(swizzle!(reverse.group0(), 3, 2, 0, 1) * Simd32x4::from([self.group0()[3], self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (swizzle!(reverse.group0(), 2, 1, 2, 0) * Simd32x4::from([self.group0()[2], self.group1()[2], self.group1()[0], self.group1()[1]]))
                + (swizzle!(self.group0(), 0, 3, 3, 3) * swizzle!(reverse.group0(), 0, 0, 1, 2))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[1], reverse.group1()[1], reverse.group1()[2], reverse.group0()[3]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group1()[2]) + (self.group1()[1] * reverse.group1()[1]) + (self.group1()[0] * reverse.group1()[0])),
                    ((self.group0()[0] * reverse.group0()[3]) - (self.group0()[1] * reverse.group1()[2])),
                    (-(self.group0()[2] * reverse.group1()[0]) + (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group0()[0] * reverse.group1()[1]) + (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(reverse.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[2]]))
                - (swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group1()[2]]))
                - (swizzle!(reverse.group0(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[1]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group1()[1]) + (self.group1()[1] * reverse.group1()[2]) + (self.group0()[1] * reverse.group0()[2])),
                    ((self.group1()[2] * reverse.group1()[0]) - (self.group1()[0] * reverse.group1()[2]) + (self.group0()[2] * reverse.group0()[0])),
                    (-(self.group1()[1] * reverse.group1()[0]) + (self.group1()[0] * reverse.group1()[1]) + (self.group0()[0] * reverse.group0()[1])),
                    (-(self.group1()[0] * reverse.group0()[0]) - (self.group0()[0] * reverse.group1()[0]) - (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[0], 2) - f32::powi(self.group0()[3], 2)
                + f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiMysteryQuadNum {
    type Output = AntiMysteryQuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd2        1        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        4        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryQuadNum::from_groups(/* e45, scalar */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_product = AntiMysteryQuadNum::from_groups(
            // e45, scalar
            ((Simd32x2::from(self.group0()[0]) * swizzle!(reverse.group0(), 1, 0)) + (Simd32x2::from(self.group0()[1]) * reverse.group0())),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)));
        let subtraction = AntiMysteryQuadNum::from_groups(
            // e45, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiPlane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from([
                ((self.group0()[1] * self.group0()[2]) - (self.group0()[2] * self.group0()[1])),
                (-(self.group0()[0] * self.group0()[2]) + (self.group0()[2] * self.group0()[0])),
                ((self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                ((self.group0()[0] * self.group0()[3]) - (self.group0()[3] * self.group0()[0])),
                ((self.group0()[1] * self.group0()[3]) - (self.group0()[3] * self.group0()[1])),
                ((self.group0()[2] * self.group0()[3]) - (self.group0()[3] * self.group0()[2])),
                (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
            ]),
        );
        let subtraction = AntiLine::from_groups(
            // e23, e31, e12
            geometric_product.group0(),
            // e15, e25, e35
            Simd32x3::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiPlaneOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            ((self.group0()[1] * self.group0()[2]) - (self.group0()[2] * self.group0()[1])),
            (-(self.group0()[0] * self.group0()[2]) + (self.group0()[2] * self.group0()[0])),
            ((self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        ]));
        let subtraction = AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       16       23        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            ((Simd32x4::from(self.group0()[3]) * reverse.group0())
                + (swizzle!(self.group0(), 0, 2, 2, 2) * swizzle!(reverse.group0(), 2, 1, 3, 2))
                + (swizzle!(self.group0(), 0, 1, 1, 0) * swizzle!(reverse.group0(), 3, 3, 0, 1))
                + (swizzle!(self.group0(), 2, 1, 0, 1) * swizzle!(reverse.group0(), 0, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) + (self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
        );
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiQuadNumAligningOrigin {
    type Output = AntiQuadNumOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        8        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            ((self.group0()[0] * self.group0()[2]) + (self.group0()[2] * self.group0()[0])),
            ((self.group0()[1] * self.group0()[2]) + (self.group0()[2] * self.group0()[1])),
            (-(self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            (f32::powi(self.group0()[2], 2) + (self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
        ]));
        let subtraction = AntiQuadNumOrthogonalOrigin::from_groups(
            // e1234, e3215, e45
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiQuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiQuadNumAligningOriginAtInfinity::from_groups(/* e3215, scalar */ Simd32x2::from([
            ((self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            f32::powi(self.group0()[1], 2),
        ]));
        let subtraction = Horizon::from_groups(/* e3215 */ geometric_product.group0()[0]);
        return subtraction;
    }
}
impl ConstraintViolation for AntiQuadNumAtInfinity {
    type Output = AntiQuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd3        2        2        0
    // Totals...
    // yes simd        5        5        0
    //  no simd        9        9        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiQuadNumAtInfinity::from_groups(/* e3215, e45, scalar */ Simd32x3::from([self.group0()[0], (self.group0()[1] * -1.0), self.group0()[2]]));
        let geometric_product = AntiQuadNumAtInfinity::from_groups(
            // e3215, e45, scalar
            ((swizzle!(self.group0(), 2, 1, 1) * swizzle!(reverse.group0(), 0, 2, 1))
                + (swizzle!(self.group0(), 1, 2, 2) * reverse.group0())
                + Simd32x3::from([(-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group0()[2])), 0.0, 0.0])),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        let subtraction = AntiQuadNumAtInfinity::from_groups(/* e3215, e45, scalar */ Simd32x3::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            (geometric_product.group0()[2] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiQuadNumOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiQuadNumOnOrigin::from_groups(/* e1234, scalar */ Simd32x2::from([
            ((self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            f32::powi(self.group0()[1], 2),
        ]));
        let subtraction = NullSphereAtOrigin::from_groups(/* e1234 */ geometric_product.group0()[0]);
        return subtraction;
    }
}
impl ConstraintViolation for AntiQuadNumOrthogonalOrigin {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiQuadNumOrthogonalOrigin::from_groups(/* e1234, e3215, e45 */ Simd32x3::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0)]));
        let geometric_product = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            ((self.group0()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[0])),
            (-(self.group0()[1] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[1])),
            (-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
            ((self.group0()[2] * reverse.group0()[2]) + (self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
        ]));
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[2], 2) + (self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
        );
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
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
    type Output = AntiCircleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group0()[0] * self.group0()[3]) + (self.group0()[3] * self.group0()[0])),
                (-(self.group0()[1] * self.group0()[3]) + (self.group0()[3] * self.group0()[1])),
                (-(self.group0()[2] * self.group0()[3]) + (self.group0()[3] * self.group0()[2])),
                (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
            ]),
            // e23, e31, e12
            Simd32x3::from([
                ((self.group0()[1] * self.group0()[2]) - (self.group0()[2] * self.group0()[1])),
                (-(self.group0()[0] * self.group0()[2]) + (self.group0()[2] * self.group0()[0])),
                ((self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            ]),
        );
        let subtraction = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
            // e23, e31, e12
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       31        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       37        0
    //  no simd       44       55        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            ((swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 1) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[1]]))
                + Simd32x4::from([
                    ((self.group1()[3] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group1()[3])
                        + (self.group0()[3] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group0()[3])),
                    ((self.group1()[3] * reverse.group1()[1])
                        + (self.group1()[1] * reverse.group1()[3])
                        + (self.group0()[3] * reverse.group0()[1])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group1()[3] * reverse.group1()[2])
                        + (self.group1()[2] * reverse.group1()[3])
                        + (self.group0()[3] * reverse.group0()[2])
                        + (self.group0()[2] * reverse.group0()[3])
                        + (self.group0()[1] * reverse.group1()[0])),
                    ((self.group1()[0] * reverse.group1()[0]) * -1.0),
                ])),
            // e23, e31, e12, e1234
            ((swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group0()[3], reverse.group0()[3], reverse.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group0()[2]]))
                + (swizzle!(reverse.group1(), 0, 1, 0, 3) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group1()[1], self.group0()[3]]))
                + Simd32x4::from([
                    (self.group1()[0] * reverse.group0()[3]),
                    (self.group1()[0] * reverse.group1()[2]),
                    (self.group0()[3] * reverse.group1()[2]),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[2])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[1], 2) + f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2)),
        );
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       70        0
    //    simd3        0        2        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       67       82        0
    //  no simd       94      116        0
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
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group1(), 3, 0, 3, 1) * Simd32x4::from([self.group0()[0], self.group0()[2], self.group0()[2], self.group1()[1]]))
                + (swizzle!(reverse.group1(), 2, 3, 1, 0) * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group1()[0]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1]) - (self.group0()[2] * reverse.group1()[1])),
                    (-(self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[1] * reverse.group0()[0]) - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group2()[2] * reverse.group0()[2])
                        + (self.group2()[1] * reverse.group0()[1])
                        + (self.group2()[0] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group2()[2])
                        + (self.group0()[0] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self.group2()[2] * reverse.group0()[1]) + (self.group2()[1] * reverse.group0()[2]) - (self.group1()[2] * reverse.group1()[1])
                    + (self.group1()[1] * reverse.group1()[2])
                    + (self.group0()[1] * reverse.group2()[2])
                    - (self.group0()[2] * reverse.group2()[1])),
                ((self.group2()[2] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[2]) + (self.group1()[2] * reverse.group1()[0])
                    - (self.group1()[0] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group2()[2])
                    + (self.group0()[2] * reverse.group2()[0])),
                (-(self.group2()[1] * reverse.group0()[0]) + (self.group2()[0] * reverse.group0()[1]) - (self.group1()[1] * reverse.group1()[0])
                    + (self.group1()[0] * reverse.group1()[1])
                    + (self.group0()[0] * reverse.group2()[1])
                    - (self.group0()[1] * reverse.group2()[0])),
                ((self.group2()[2] * reverse.group0()[2]) + (self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group2()[2])
                    - (self.group0()[0] * reverse.group2()[0])
                    - (self.group0()[1] * reverse.group2()[1])),
            ]),
            // e15, e25, e35, e1234
            ((swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group0()[2]]))
                + (swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group0()[2]]))
                + (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[1]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group1()[1]) - (self.group2()[0] * reverse.group1()[3]) - (self.group1()[2] * reverse.group2()[1])),
                    (-(self.group2()[1] * reverse.group1()[3]) - (self.group2()[0] * reverse.group1()[2]) - (self.group1()[0] * reverse.group2()[2])),
                    (-(self.group2()[2] * reverse.group1()[3]) - (self.group2()[1] * reverse.group1()[0]) - (self.group1()[1] * reverse.group2()[0])),
                    ((self.group1()[0] * reverse.group0()[0]) + (self.group0()[0] * reverse.group1()[0]) + (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(reverse.group1(), 0, 1, 2, 2) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[2]]))
                + (swizzle!(reverse.group1(), 3, 3, 3, 1) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[1]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) + (self.group2()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group2()[2])
                        + (self.group0()[2] * reverse.group2()[1])),
                    ((self.group2()[2] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group2()[2])
                        - (self.group0()[2] * reverse.group2()[0])),
                    (-(self.group2()[1] * reverse.group0()[0]) + (self.group2()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group2()[1])
                        + (self.group0()[1] * reverse.group2()[0])),
                    ((self.group2()[0] * reverse.group1()[0])
                        + (self.group1()[2] * reverse.group2()[2])
                        + (self.group1()[0] * reverse.group2()[0])
                        + (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group2()[2] * self.group0()[2]) + (self.group2()[1] * self.group0()[1]) + (self.group2()[0] * self.group0()[0]) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       74       87        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       74       90        0
    //  no simd       74       96        0
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
                (-(self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[2]) + (self.group0()[1] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[0])),
                (-(self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group1()[0])),
                ((self.group2()[2] * reverse.group0()[2])
                    + (self.group2()[1] * reverse.group0()[1])
                    + (self.group2()[0] * reverse.group0()[0])
                    + (self.group1()[2] * reverse.group1()[2])
                    + (self.group1()[1] * reverse.group1()[1])
                    + (self.group1()[0] * reverse.group1()[0])
                    + (self.group0()[2] * reverse.group2()[2])
                    + (self.group0()[0] * reverse.group2()[0])
                    + (self.group0()[1] * reverse.group2()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self.group2()[2] * reverse.group0()[1]) + (self.group2()[1] * reverse.group0()[2]) - (self.group1()[2] * reverse.group1()[1])
                    + (self.group1()[1] * reverse.group1()[2])
                    + (self.group0()[1] * reverse.group2()[2])
                    - (self.group0()[2] * reverse.group2()[1])),
                ((self.group2()[2] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[2]) + (self.group1()[2] * reverse.group1()[0])
                    - (self.group1()[0] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group2()[2])
                    + (self.group0()[2] * reverse.group2()[0])),
                (-(self.group2()[1] * reverse.group0()[0]) + (self.group2()[0] * reverse.group0()[1]) - (self.group1()[1] * reverse.group1()[0])
                    + (self.group1()[0] * reverse.group1()[1])
                    + (self.group0()[0] * reverse.group2()[1])
                    - (self.group0()[1] * reverse.group2()[0])),
                ((self.group2()[2] * reverse.group0()[2]) + (self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group2()[2])
                    - (self.group0()[0] * reverse.group2()[0])
                    - (self.group0()[1] * reverse.group2()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * reverse.group1()[1]) + (self.group2()[1] * reverse.group1()[2]) + (self.group1()[1] * reverse.group2()[2])
                    - (self.group1()[2] * reverse.group2()[1])),
                ((self.group2()[2] * reverse.group1()[0]) - (self.group2()[0] * reverse.group1()[2]) - (self.group1()[0] * reverse.group2()[2])
                    + (self.group1()[2] * reverse.group2()[0])),
                (-(self.group2()[1] * reverse.group1()[0]) + (self.group2()[0] * reverse.group1()[1]) + (self.group1()[0] * reverse.group2()[1])
                    - (self.group1()[1] * reverse.group2()[0])),
                ((self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-(self.group2()[2] * reverse.group0()[1]) + (self.group2()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group2()[2])
                    + (self.group0()[2] * reverse.group2()[1])),
                ((self.group2()[2] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group2()[2])
                    - (self.group0()[2] * reverse.group2()[0])),
                (-(self.group2()[1] * reverse.group0()[0]) + (self.group2()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group2()[1])
                    + (self.group0()[1] * reverse.group2()[0])),
                ((self.group2()[2] * reverse.group1()[2])
                    + (self.group2()[1] * reverse.group1()[1])
                    + (self.group2()[0] * reverse.group1()[0])
                    + (self.group1()[2] * reverse.group2()[2])
                    + (self.group1()[0] * reverse.group2()[0])
                    + (self.group1()[1] * reverse.group2()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group2()[2] * self.group0()[2])
                + (self.group2()[1] * self.group0()[1])
                + (self.group2()[0] * self.group0()[0])
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       13       16        0
    //    simd3        0        1        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       19       24        0
    //  no simd       37       47        0
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
            (-(swizzle!(reverse.group0(), 3, 1, 3, 3) * Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[1], self.group1()[2]]))
                + (swizzle!(reverse.group0(), 2, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (swizzle!(self.group0(), 0, 3, 3, 3) * Simd32x4::from([reverse.group0()[0], reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(self.group0(), 1, 1, 2, 0) * Simd32x4::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    (-(self.group1()[0] * reverse.group0()[3]) - (self.group0()[2] * reverse.group1()[1])),
                    (-(self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[1] * reverse.group0()[0]) - (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                ((self.group0()[1] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[1])),
                (-(self.group0()[0] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[0])),
                ((self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            ((swizzle!(reverse.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[2]]))
                + (swizzle!(reverse.group0(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2], 0.0]),
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
    //      f32       34       42        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       34       44        0
    //  no simd       34       48        0
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
                ((self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[0])),
                (-(self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[1])
                    + (self.group0()[1] * reverse.group1()[0])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[2]) + (self.group0()[1] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[0])),
                (-(self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group1()[0])),
                ((self.group1()[2] * reverse.group0()[2]) + (self.group1()[1] * reverse.group0()[1]) + (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group1()[2] * self.group0()[2])
                + (self.group1()[1] * self.group0()[1])
                + (self.group1()[0] * self.group0()[0])
                + (self.group0()[2] * self.group1()[2])
                + (self.group0()[0] * self.group1()[0])
                + (self.group0()[1] * self.group1()[1])),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       27        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       22       33        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[2]) + (self.group0()[1] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[0])),
                (-(self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group1()[0])),
                ((self.group1()[2] * reverse.group1()[2]) + (self.group1()[0] * reverse.group1()[0]) + (self.group1()[1] * reverse.group1()[1])),
            ]),
            // e23, e31, e12, e1234
            Simd32x4::from([
                ((self.group1()[1] * reverse.group1()[2]) - (self.group1()[2] * reverse.group1()[1])),
                (-(self.group1()[0] * reverse.group1()[2]) + (self.group1()[2] * reverse.group1()[0])),
                ((self.group1()[0] * reverse.group1()[1]) - (self.group1()[1] * reverse.group1()[0])),
                ((self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2)));
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       39        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       45       62        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((swizzle!(reverse.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[2]]))
                - (Simd32x4::from(self.group0()[3]) * reverse.group0())
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e23, e31, e12, e45
            ((swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[2]]))
                - (swizzle!(self.group0(), 2, 0, 1, 2) * Simd32x4::from([reverse.group1()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1]) + (self.group0()[1] * reverse.group1()[2])),
                    (-(self.group1()[0] * reverse.group0()[2]) + (self.group0()[2] * reverse.group1()[0])),
                    (-(self.group1()[1] * reverse.group0()[0]) + (self.group0()[0] * reverse.group1()[1])),
                    ((self.group1()[1] * reverse.group0()[1]) + (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group0()[3] * reverse.group1()[0]) - (self.group1()[0] * reverse.group0()[3])),
                ((self.group0()[3] * reverse.group1()[1]) - (self.group1()[1] * reverse.group0()[3])),
                ((self.group0()[3] * reverse.group1()[2]) - (self.group1()[2] * reverse.group0()[3])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-(self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[0])),
                (-(self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[1])
                    + (self.group0()[1] * reverse.group1()[0])),
                0.0,
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group1()[2] * self.group0()[2]) + (self.group1()[1] * self.group0()[1]) + (self.group1()[0] * self.group0()[0]) - f32::powi(self.group0()[3], 2)
                + (self.group0()[2] * self.group1()[2])
                + (self.group0()[0] * self.group1()[0])
                + (self.group0()[1] * self.group1()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([geometric_product.group2()[0], geometric_product.group2()[1], geometric_product.group2()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([geometric_product.group3()[0], geometric_product.group3()[1], geometric_product.group3()[2], 0.0]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       66        0
    //    simd3        0        1        0
    //    simd4       16       17        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      116      137        0
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
            (-(Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group1(), 2, 0, 3, 1) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[2], self.group1()[1]]))
                + (swizzle!(reverse.group1(), 3, 3, 1, 0) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group1()[0]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1]) - (self.group0()[2] * reverse.group1()[1]) - (self.group0()[0] * reverse.group2()[3])),
                    (-(self.group1()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse.group2()[3]) - (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[1] * reverse.group0()[0]) - (self.group0()[2] * reverse.group2()[3]) - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group2()[2] * reverse.group0()[2])
                        + (self.group2()[1] * reverse.group0()[1])
                        + (self.group2()[0] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group2()[2])
                        + (self.group0()[0] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((swizzle!(self.group2(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 3) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[3]]))
                - (swizzle!(reverse.group2(), 3, 3, 3, 2) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 0) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[0]]))
                + Simd32x4::from([
                    (-(self.group2()[3] * reverse.group1()[0]) - (self.group2()[2] * reverse.group0()[1]) - (self.group1()[2] * reverse.group1()[1])
                        + (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[3] * reverse.group1()[1]) - (self.group2()[0] * reverse.group0()[2]) - (self.group1()[0] * reverse.group1()[2])
                        + (self.group0()[2] * reverse.group2()[0])),
                    (-(self.group2()[3] * reverse.group1()[2]) - (self.group2()[1] * reverse.group0()[0]) - (self.group1()[1] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group2()[1])),
                    ((self.group2()[2] * reverse.group0()[2]) + (self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e15, e25, e35, e1234
            ((swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group0()[2]]))
                + (swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group0()[2]]))
                + (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[1]]))
                + Simd32x4::from([
                    (-(self.group2()[3] * reverse.group2()[0])
                        - (self.group2()[2] * reverse.group1()[1])
                        - (self.group2()[0] * reverse.group2()[3])
                        - (self.group2()[0] * reverse.group1()[3])
                        - (self.group1()[2] * reverse.group2()[1])),
                    (-(self.group2()[3] * reverse.group2()[1])
                        - (self.group2()[1] * reverse.group2()[3])
                        - (self.group2()[1] * reverse.group1()[3])
                        - (self.group2()[0] * reverse.group1()[2])
                        - (self.group1()[0] * reverse.group2()[2])),
                    (-(self.group2()[3] * reverse.group2()[2])
                        - (self.group2()[2] * reverse.group2()[3])
                        - (self.group2()[2] * reverse.group1()[3])
                        - (self.group2()[1] * reverse.group1()[0])
                        - (self.group1()[1] * reverse.group2()[0])),
                    ((self.group1()[0] * reverse.group0()[0]) + (self.group0()[0] * reverse.group1()[0]) + (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[1]]))
                + (swizzle!(reverse.group1(), 3, 3, 3, 0) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[0]]))
                + (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) - (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group2()[0])),
                    (-(self.group2()[1] * reverse.group0()[0]) - (self.group0()[0] * reverse.group2()[1])),
                    ((self.group1()[0] * reverse.group2()[0]) + (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group2()[3], 2) + (self.group2()[2] * self.group0()[2]) + (self.group2()[1] * self.group0()[1]) + (self.group2()[0] * self.group0()[0])
                - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       70       85        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       76       93        0
    //  no simd       94      115        0
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
            (-(Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[2])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group2()[3])
                        + (self.group0()[1] * reverse.group1()[2])),
                    ((self.group1()[2] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[2]) + (self.group0()[2] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group1()[2])
                        - (self.group0()[1] * reverse.group2()[3])),
                    (-(self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[1]) - (self.group0()[2] * reverse.group2()[3])
                        + (self.group0()[0] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group2()[2] * reverse.group0()[2])
                        + (self.group2()[1] * reverse.group0()[1])
                        + (self.group2()[0] * reverse.group0()[0])
                        + (self.group1()[2] * reverse.group1()[2])
                        + (self.group1()[1] * reverse.group1()[1])
                        + (self.group1()[0] * reverse.group1()[0])
                        + (self.group0()[2] * reverse.group2()[2])
                        + (self.group0()[0] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group2(), 3, 3, 3, 2) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 0) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[0]]))
                + Simd32x4::from([
                    (-(self.group2()[3] * reverse.group1()[0]) - (self.group2()[2] * reverse.group0()[1]) - (self.group1()[2] * reverse.group1()[1])
                        + (self.group1()[1] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[3] * reverse.group1()[1]) - (self.group2()[0] * reverse.group0()[2]) + (self.group1()[2] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[2])
                        + (self.group0()[2] * reverse.group2()[0])),
                    (-(self.group2()[3] * reverse.group1()[2]) - (self.group2()[1] * reverse.group0()[0]) - (self.group1()[1] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group2()[1])),
                    ((self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[3] * reverse.group2()[0]) - (self.group2()[2] * reverse.group1()[1]) + (self.group2()[1] * reverse.group1()[2])
                    - (self.group2()[0] * reverse.group2()[3])
                    + (self.group1()[1] * reverse.group2()[2])
                    - (self.group1()[2] * reverse.group2()[1])),
                (-(self.group2()[3] * reverse.group2()[1]) + (self.group2()[2] * reverse.group1()[0])
                    - (self.group2()[1] * reverse.group2()[3])
                    - (self.group2()[0] * reverse.group1()[2])
                    - (self.group1()[0] * reverse.group2()[2])
                    + (self.group1()[2] * reverse.group2()[0])),
                (-(self.group2()[3] * reverse.group2()[2]) - (self.group2()[2] * reverse.group2()[3]) - (self.group2()[1] * reverse.group1()[0])
                    + (self.group2()[0] * reverse.group1()[1])
                    + (self.group1()[0] * reverse.group2()[1])
                    - (self.group1()[1] * reverse.group2()[0])),
                ((self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            ((swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) - (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group2()[0])),
                    (-(self.group2()[1] * reverse.group0()[0]) - (self.group0()[0] * reverse.group2()[1])),
                    ((self.group2()[1] * reverse.group1()[1])
                        + (self.group2()[0] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group2()[0])
                        + (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group2()[3], 2)
                + (self.group2()[2] * self.group0()[2])
                + (self.group2()[1] * self.group0()[1])
                + (self.group2()[0] * self.group0()[0])
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       24       31        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       35        0
    //  no simd       36       46        0
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
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group0()[2] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[3]) + (self.group0()[1] * reverse.group0()[2])),
                    ((self.group0()[2] * reverse.group0()[0]) - (self.group0()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group0()[2] * reverse.group1()[3]) + (self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
                    ((self.group0()[2] * reverse.group0()[2]) + (self.group0()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group0()[1])),
                ])),
            // e15, e25, e35, e3215
            ((swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group0()[2]]))
                + (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[3] * reverse.group1()[0])
                        - (self.group1()[2] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group1()[3])
                        - (self.group0()[2] * reverse.group1()[1])),
                    (-(self.group1()[3] * reverse.group1()[1])
                        - (self.group1()[1] * reverse.group1()[3])
                        - (self.group1()[0] * reverse.group0()[2])
                        - (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[3] * reverse.group1()[2])
                        - (self.group1()[2] * reverse.group1()[3])
                        - (self.group1()[1] * reverse.group0()[0])
                        - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       26        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       28       35        0
    //  no simd       52       62        0
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
            (-(Simd32x4::from(self.group1()[3]) * swizzle!(reverse.group1(), 3, 0, 1, 2))
                - (swizzle!(reverse.group0(), 3, 1, 3, 3) * Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[1], self.group1()[2]]))
                + (swizzle!(reverse.group0(), 2, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (swizzle!(self.group0(), 0, 3, 3, 3) * Simd32x4::from([reverse.group0()[0], reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(self.group0(), 1, 1, 2, 0) * Simd32x4::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    (-(self.group1()[0] * reverse.group1()[3]) - (self.group1()[0] * reverse.group0()[3]) - (self.group0()[2] * reverse.group1()[1])),
                    (-(self.group1()[1] * reverse.group1()[3]) - (self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[2] * reverse.group1()[3]) - (self.group1()[1] * reverse.group0()[0]) - (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            ((swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group1()[3] * reverse.group0()[0]) - (self.group0()[2] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[3])),
                    (-(self.group1()[3] * reverse.group0()[1]) - (self.group0()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group1()[3] * reverse.group0()[2]) - (self.group0()[2] * reverse.group1()[3]) - (self.group0()[1] * reverse.group0()[0])),
                    (self.group1()[3] * reverse.group0()[3]),
                ])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(reverse.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[2]]))
                + (swizzle!(reverse.group0(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[3], 2) - f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for CircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       39        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       36       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (-(swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[3]]))
                + Simd32x4::from([
                    ((self.group1()[1] * reverse.group0()[2])
                        - (self.group0()[3] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group0()[3])
                        + (self.group0()[1] * reverse.group1()[2])),
                    ((self.group1()[2] * reverse.group0()[0]) - (self.group0()[3] * reverse.group0()[1]) + (self.group0()[2] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group1()[2])
                        - (self.group0()[1] * reverse.group0()[3])),
                    ((self.group1()[0] * reverse.group0()[1]) - (self.group0()[3] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[3])
                        + (self.group0()[0] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group1()[2] * reverse.group1()[2]) + (self.group1()[1] * reverse.group1()[1]) + (self.group1()[0] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e1234
            Simd32x4::from([
                (-(self.group1()[2] * reverse.group1()[1]) + (self.group1()[1] * reverse.group1()[2])
                    - (self.group0()[3] * reverse.group1()[0])
                    - (self.group1()[0] * reverse.group0()[3])),
                ((self.group1()[2] * reverse.group1()[0])
                    - (self.group1()[1] * reverse.group0()[3])
                    - (self.group0()[3] * reverse.group1()[1])
                    - (self.group1()[0] * reverse.group1()[2])),
                (-(self.group1()[2] * reverse.group0()[3]) - (self.group1()[1] * reverse.group1()[0]) - (self.group0()[3] * reverse.group1()[2])
                    + (self.group1()[0] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[1], 2) - f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2)),
        );
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       66       78        0
    //    simd3        0        2        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       73       88        0
    //  no simd       94      116        0
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
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[2]]))
                + (swizzle!(self.group1(), 2, 0, 1, 3) * Simd32x4::from([reverse.group0()[1], reverse.group0()[2], reverse.group0()[0], reverse.group1()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[1]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 0) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[0]]))
                + Simd32x4::from([
                    ((self.group0()[2] * reverse.group1()[1]) + (self.group0()[0] * reverse.group1()[3])),
                    ((self.group0()[0] * reverse.group1()[2]) + (self.group0()[1] * reverse.group1()[3])),
                    ((self.group0()[2] * reverse.group1()[3]) + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group2()[2] * reverse.group0()[2])
                        - (self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group2()[2])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                ((self.group2()[2] * reverse.group0()[1]) - (self.group2()[1] * reverse.group0()[2]) + (self.group1()[2] * reverse.group1()[1])
                    - (self.group1()[1] * reverse.group1()[2])
                    - (self.group0()[1] * reverse.group2()[2])
                    + (self.group0()[2] * reverse.group2()[1])),
                (-(self.group2()[2] * reverse.group0()[0]) + (self.group2()[0] * reverse.group0()[2]) - (self.group1()[2] * reverse.group1()[0])
                    + (self.group1()[0] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group2()[2])
                    - (self.group0()[2] * reverse.group2()[0])),
                ((self.group2()[1] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[1]) + (self.group1()[1] * reverse.group1()[0])
                    - (self.group1()[0] * reverse.group1()[1])
                    - (self.group0()[0] * reverse.group2()[1])
                    + (self.group0()[1] * reverse.group2()[0])),
                (-(self.group2()[2] * reverse.group0()[2]) - (self.group2()[1] * reverse.group0()[1]) - (self.group2()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group2()[2])
                    + (self.group0()[0] * reverse.group2()[0])
                    + (self.group0()[1] * reverse.group2()[1])),
            ]),
            // e15, e25, e35, e1234
            (-(swizzle!(reverse.group1(), 2, 0, 3, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[2], self.group0()[2]]))
                - (swizzle!(reverse.group1(), 3, 3, 1, 0) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[0], self.group0()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group1()[1]) + (self.group1()[3] * reverse.group2()[0]) + (self.group1()[2] * reverse.group2()[1])),
                    ((self.group2()[0] * reverse.group1()[2]) + (self.group1()[3] * reverse.group2()[1]) + (self.group1()[0] * reverse.group2()[2])),
                    ((self.group2()[1] * reverse.group1()[0]) + (self.group1()[3] * reverse.group2()[2]) + (self.group1()[1] * reverse.group2()[0])),
                    (-(self.group1()[1] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self.group2()[2] * reverse.group0()[1]) - (self.group2()[1] * reverse.group0()[2])
                    + (self.group1()[3] * reverse.group1()[0])
                    + (self.group1()[0] * reverse.group1()[3])
                    + (self.group0()[1] * reverse.group2()[2])
                    - (self.group0()[2] * reverse.group2()[1])),
                (-(self.group2()[2] * reverse.group0()[0])
                    + (self.group2()[0] * reverse.group0()[2])
                    + (self.group1()[3] * reverse.group1()[1])
                    + (self.group1()[1] * reverse.group1()[3])
                    - (self.group0()[0] * reverse.group2()[2])
                    + (self.group0()[2] * reverse.group2()[0])),
                ((self.group2()[1] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[1])
                    + (self.group1()[3] * reverse.group1()[2])
                    + (self.group1()[2] * reverse.group1()[3])
                    + (self.group0()[0] * reverse.group2()[1])
                    - (self.group0()[1] * reverse.group2()[0])),
                (-(self.group2()[2] * reverse.group1()[2])
                    - (self.group2()[1] * reverse.group1()[1])
                    - (self.group2()[0] * reverse.group1()[0])
                    - (self.group1()[2] * reverse.group2()[2])
                    - (self.group1()[0] * reverse.group2()[0])
                    - (self.group1()[1] * reverse.group2()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group2()[2] * self.group0()[2]) - (self.group2()[1] * self.group0()[1]) - (self.group2()[0] * self.group0()[0]) + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       39        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       45       62        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((self.group0() * Simd32x4::from(reverse.group0()[3]))
                - (swizzle!(reverse.group0(), 0, 1, 2, 2) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[2]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[2])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[2]]))
                + (swizzle!(self.group0(), 2, 0, 1, 2) * Simd32x4::from([reverse.group1()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group0()[1]) - (self.group0()[1] * reverse.group1()[2])),
                    ((self.group1()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group1()[0])),
                    ((self.group1()[1] * reverse.group0()[0]) - (self.group0()[0] * reverse.group1()[1])),
                    (-(self.group1()[1] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group0()[3] * reverse.group1()[0]) - (self.group1()[0] * reverse.group0()[3])),
                ((self.group0()[3] * reverse.group1()[1]) - (self.group1()[1] * reverse.group0()[3])),
                ((self.group0()[3] * reverse.group1()[2]) - (self.group1()[2] * reverse.group0()[3])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self.group1()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group0()[2]) + (self.group0()[1] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[0])),
                ((self.group1()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group1()[0])),
                0.0,
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group1()[2] * self.group0()[2]) - (self.group1()[1] * self.group0()[1]) - (self.group1()[0] * self.group0()[0]) + f32::powi(self.group0()[3], 2)
                - (self.group0()[2] * self.group1()[2])
                - (self.group0()[0] * self.group1()[0])
                - (self.group0()[1] * self.group1()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([geometric_product.group2()[0], geometric_product.group2()[1], geometric_product.group2()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([geometric_product.group3()[0], geometric_product.group3()[1], geometric_product.group3()[2], 0.0]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       24        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       34       47        0
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
            ((swizzle!(reverse.group0(), 3, 1, 2, 0) * Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group1()[1]]))
                - (swizzle!(reverse.group0(), 2, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group1()[1], self.group1()[2], self.group1()[2]]))
                - (swizzle!(reverse.group0(), 0, 3, 3, 1) * Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[1], self.group1()[0]]))
                - (swizzle!(self.group0(), 1, 1, 2, 0) * Simd32x4::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    ((self.group0()[3] * reverse.group1()[0]) + (self.group0()[2] * reverse.group1()[1])),
                    ((self.group0()[3] * reverse.group1()[1]) + (self.group0()[0] * reverse.group1()[2])),
                    ((self.group0()[3] * reverse.group1()[2]) + (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self.group0()[1] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[1])),
                ((self.group0()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[0])),
                (-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self.group0()[0] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[0])),
                ((self.group0()[1] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[1])),
                ((self.group0()[2] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[2])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2], 0.0]),
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
    //      f32       34       42        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       34       44        0
    //  no simd       34       48        0
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
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group0()[2]) + (self.group0()[1] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[0])),
                ((self.group1()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group1()[0])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                ((self.group1()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[0])),
                ((self.group1()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[1])
                    + (self.group0()[1] * reverse.group1()[0])),
                (-(self.group1()[2] * reverse.group0()[2]) - (self.group1()[1] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group1()[2] * self.group0()[2])
                - (self.group1()[1] * self.group0()[1])
                - (self.group1()[0] * self.group0()[0])
                - (self.group0()[2] * self.group1()[2])
                - (self.group0()[0] * self.group1()[0])
                - (self.group0()[1] * self.group1()[1])),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84       96        0
    //    simd3        0        1        0
    //    simd4       35       36        0
    // Totals...
    // yes simd      119      133        0
    //  no simd      224      243        0
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
            (-(swizzle!(self.group3(), 2, 1, 2, 2) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[3], reverse.group3()[2]]))
                + (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group2()[3]]))
                - (swizzle!(self.group3(), 0, 0, 1, 1) * Simd32x4::from([reverse.group2()[3], reverse.group0()[2], reverse.group0()[0], reverse.group3()[1]]))
                + (Simd32x4::from(self.group2()[3]) * reverse.group3())
                + (reverse.group1() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[3]]))
                - (swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[2]]))
                - (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[1]]))
                - (swizzle!(reverse.group3(), 2, 0, 1, 0) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 0) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[0]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group2()[3])
                        + (self.group0()[2] * reverse.group3()[1])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group1()[3])),
                    ((self.group1()[1] * reverse.group2()[3])
                        + (self.group1()[0] * reverse.group0()[2])
                        + (self.group0()[1] * reverse.group1()[3])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group3()[2])),
                    ((self.group1()[2] * reverse.group2()[3])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[3])
                        + (self.group0()[1] * reverse.group3()[0])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group2()[2] * reverse.group0()[2])
                        - (self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group2()[2])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group3()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group2(), 0, 1, 2, 2) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[2]]))
                - (swizzle!(self.group2(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group3()[3]]))
                + (swizzle!(reverse.group2(), 3, 3, 3, 0) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 1) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group3()[1]]))
                + (swizzle!(reverse.group2(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[1]]))
                + Simd32x4::from([
                    ((self.group3()[2] * reverse.group3()[1])
                        + (self.group3()[0] * reverse.group1()[3])
                        + (self.group2()[2] * reverse.group0()[1])
                        + (self.group1()[3] * reverse.group3()[0])
                        + (self.group1()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group3()[3])
                        - (self.group0()[1] * reverse.group2()[2])),
                    ((self.group3()[1] * reverse.group1()[3])
                        + (self.group3()[0] * reverse.group3()[2])
                        + (self.group2()[0] * reverse.group0()[2])
                        + (self.group1()[3] * reverse.group3()[1])
                        + (self.group1()[0] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group3()[3])),
                    ((self.group3()[2] * reverse.group1()[3])
                        + (self.group3()[1] * reverse.group3()[0])
                        + (self.group2()[1] * reverse.group0()[0])
                        + (self.group1()[3] * reverse.group3()[2])
                        + (self.group1()[1] * reverse.group1()[0])
                        + (self.group0()[2] * reverse.group3()[3])
                        - (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group3()[0] * reverse.group1()[0])
                        - (self.group2()[2] * reverse.group0()[2])
                        - (self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group3()[2])
                        - (self.group1()[1] * reverse.group3()[1])
                        - (self.group1()[0] * reverse.group3()[0])),
                ])),
            // e15, e25, e35, e1234
            (-(swizzle!(reverse.group3(), 0, 1, 2, 2) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[2]]))
                + (swizzle!(self.group3(), 3, 3, 3, 2) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group0()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2, 1) * Simd32x4::from([reverse.group2()[1], reverse.group3()[3], reverse.group3()[3], reverse.group0()[1]]))
                - (swizzle!(reverse.group2(), 2, 0, 1, 3) * Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group1()[3]]))
                + (swizzle!(self.group3(), 0, 0, 1, 0) * Simd32x4::from([reverse.group3()[3], reverse.group2()[2], reverse.group2()[0], reverse.group0()[0]]))
                - (swizzle!(reverse.group3(), 1, 2, 0, 1) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group0()[1]]))
                + (swizzle!(self.group2(), 2, 2, 1, 3) * Simd32x4::from([reverse.group1()[1], reverse.group3()[0], reverse.group1()[0], reverse.group1()[3]]))
                - (swizzle!(reverse.group1(), 2, 0, 3, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[2], self.group0()[2]]))
                - (swizzle!(reverse.group1(), 3, 3, 1, 1) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[0], self.group0()[1]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[2]]))
                + Simd32x4::from([
                    ((self.group2()[1] * reverse.group3()[2])
                        + (self.group1()[3] * reverse.group2()[0])
                        + (self.group1()[2] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group3()[3])),
                    ((self.group2()[0] * reverse.group1()[2])
                        + (self.group1()[3] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group2()[2])
                        + (self.group1()[1] * reverse.group3()[3])),
                    ((self.group2()[0] * reverse.group3()[1])
                        + (self.group1()[3] * reverse.group2()[2])
                        + (self.group1()[2] * reverse.group3()[3])
                        + (self.group1()[1] * reverse.group2()[0])),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group3()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * swizzle!(reverse.group1(), 2, 0, 1, 3))
                - (swizzle!(reverse.group2(), 0, 1, 2, 2) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group3()[2]]))
                + (swizzle!(self.group2(), 2, 1, 2, 2) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[3], reverse.group3()[2]]))
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(self.group2(), 0, 0, 1, 1) * Simd32x4::from([reverse.group2()[3], reverse.group0()[2], reverse.group0()[0], reverse.group3()[1]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group3()[3]]))
                + (swizzle!(reverse.group3(), 1, 2, 0, 0) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group2()[2]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[1]]))
                + Simd32x4::from([
                    ((self.group3()[3] * reverse.group0()[0]) + (self.group3()[2] * reverse.group1()[1]) + (self.group1()[0] * reverse.group1()[3])
                        - (self.group0()[0] * reverse.group3()[3])
                        + (self.group0()[1] * reverse.group2()[2])),
                    ((self.group3()[3] * reverse.group0()[1])
                        + (self.group3()[0] * reverse.group1()[2])
                        + (self.group1()[1] * reverse.group1()[3])
                        + (self.group0()[2] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group3()[3])),
                    ((self.group3()[3] * reverse.group0()[2]) + (self.group3()[1] * reverse.group1()[0]) + (self.group1()[2] * reverse.group1()[3])
                        - (self.group0()[2] * reverse.group3()[3])
                        + (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group3()[0] * reverse.group2()[0])
                        - (self.group2()[1] * reverse.group1()[1])
                        - (self.group2()[0] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group2()[0])
                        - (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group3()[3] * self.group2()[3]) - f32::powi(self.group3()[2], 2) - f32::powi(self.group3()[1], 2) - f32::powi(self.group3()[0], 2)
                + (self.group2()[3] * self.group3()[3])
                - (self.group2()[2] * self.group0()[2])
                - (self.group2()[1] * self.group0()[1])
                - (self.group2()[0] * self.group0()[0])
                + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       40       55        0
    //    simd4       25       26        0
    // Totals...
    // yes simd       65       81        0
    //  no simd      140      159        0
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
            (-(swizzle!(self.group2(), 2, 1, 2, 2) * Simd32x4::from([reverse.group0()[1], reverse.group1()[3], reverse.group1()[3], reverse.group2()[2]]))
                + (swizzle!(self.group2(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[3]]))
                - (swizzle!(self.group2(), 0, 0, 1, 1) * Simd32x4::from([reverse.group1()[3], reverse.group0()[2], reverse.group0()[0], reverse.group2()[1]]))
                + (Simd32x4::from(self.group1()[3]) * reverse.group2())
                - (swizzle!(reverse.group0(), 0, 1, 2, 2) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[2]]))
                + (swizzle!(self.group0(), 2, 0, 2, 3) * Simd32x4::from([reverse.group2()[1], reverse.group2()[2], reverse.group0()[3], reverse.group0()[3]]))
                - (swizzle!(reverse.group2(), 2, 0, 1, 0) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group2()[0]]))
                + Simd32x4::from([
                    (self.group0()[0] * reverse.group0()[3]),
                    (self.group0()[1] * reverse.group0()[3]),
                    (self.group0()[1] * reverse.group2()[0]),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[2])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                - (swizzle!(reverse.group2(), 2, 0, 1, 3) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group1()[3]]))
                + (swizzle!(reverse.group1(), 0, 1, 2, 2) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[2]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + (swizzle!(reverse.group1(), 3, 3, 3, 0) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]))
                + (swizzle!(self.group0(), 3, 3, 3, 1) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group1()[1]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group2()[1])
                        + (self.group2()[0] * reverse.group0()[3])
                        + (self.group1()[2] * reverse.group0()[1])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group2()[3])
                        - (self.group0()[1] * reverse.group1()[2])),
                    ((self.group2()[1] * reverse.group0()[3]) + (self.group2()[0] * reverse.group2()[2]) + (self.group1()[0] * reverse.group0()[2])
                        - (self.group0()[2] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group2()[3])),
                    ((self.group2()[2] * reverse.group0()[3])
                        + (self.group2()[1] * reverse.group2()[0])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group2()[3])
                        - (self.group0()[0] * reverse.group1()[1])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group1()[1] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            (-(swizzle!(reverse.group2(), 0, 1, 2, 2) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[2]]))
                + (swizzle!(self.group2(), 2, 1, 2, 2) * Simd32x4::from([reverse.group1()[1], reverse.group2()[3], reverse.group2()[3], reverse.group0()[2]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 3) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group0()[3]]))
                + (swizzle!(self.group2(), 0, 0, 1, 1) * Simd32x4::from([reverse.group2()[3], reverse.group1()[2], reverse.group1()[0], reverse.group0()[1]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 0) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]))
                + (swizzle!(self.group1(), 1, 2, 0, 3) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[3]]))
                + Simd32x4::from([
                    ((self.group0()[3] * reverse.group1()[0]) - (self.group1()[0] * reverse.group0()[3])),
                    (-(self.group1()[1] * reverse.group0()[3]) + (self.group0()[3] * reverse.group1()[1])),
                    (-(self.group1()[2] * reverse.group0()[3]) + (self.group0()[3] * reverse.group1()[2])),
                    ((self.group2()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(reverse.group1(), 0, 1, 2, 2) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[2]]))
                + (swizzle!(self.group1(), 2, 1, 2, 2) * Simd32x4::from([reverse.group0()[1], reverse.group1()[3], reverse.group1()[3], reverse.group2()[2]]))
                - (swizzle!(reverse.group0(), 2, 0, 1, 3) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[3]]))
                + (swizzle!(self.group1(), 0, 0, 1, 1) * Simd32x4::from([reverse.group1()[3], reverse.group0()[2], reverse.group0()[0], reverse.group2()[1]]))
                - (swizzle!(reverse.group1(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group2()[1]]))
                + (swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[3]]))
                + Simd32x4::from([
                    ((self.group2()[3] * reverse.group0()[0]) - (self.group0()[0] * reverse.group2()[3])),
                    ((self.group2()[3] * reverse.group0()[1]) - (self.group0()[1] * reverse.group2()[3])),
                    ((self.group2()[3] * reverse.group0()[2]) - (self.group0()[2] * reverse.group2()[3])),
                    (-(self.group2()[0] * reverse.group1()[0]) + (self.group1()[0] * reverse.group2()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group2()[3] * self.group1()[3]) - f32::powi(self.group2()[2], 2) - f32::powi(self.group2()[1], 2) - f32::powi(self.group2()[0], 2)
                + (self.group1()[3] * self.group2()[3])
                - (self.group1()[2] * self.group0()[2])
                - (self.group1()[1] * self.group0()[1])
                - (self.group1()[0] * self.group0()[0])
                + f32::powi(self.group0()[3], 2)
                - (self.group0()[2] * self.group1()[2])
                - (self.group0()[0] * self.group1()[0])
                - (self.group0()[1] * self.group1()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       49       53        0
    //    simd3        0        1        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       62       68        0
    //  no simd      101      112        0
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
            (-(swizzle!(self.group2(), 2, 3, 3, 3) * swizzle!(reverse.group2(), 2, 0, 1, 2))
                - (swizzle!(self.group2(), 1, 1, 2, 0) * Simd32x4::from([reverse.group2()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                - (swizzle!(reverse.group2(), 0, 1, 2, 0) * Simd32x4::from([self.group2()[0], self.group1()[2], self.group1()[0], self.group1()[1]]))
                + (swizzle!(reverse.group0(), 3, 0, 1, 2) * Simd32x4::from([self.group0()[3], self.group2()[3], self.group2()[3], self.group2()[3]]))
                - (swizzle!(reverse.group0(), 2, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group1()[1], self.group1()[2], self.group1()[2]]))
                - (swizzle!(reverse.group0(), 0, 3, 3, 1) * Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[1], self.group1()[0]]))
                - (swizzle!(self.group0(), 1, 1, 2, 0) * Simd32x4::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + Simd32x4::from([
                    0.0,
                    ((self.group2()[2] * reverse.group1()[1])
                        + (self.group2()[0] * reverse.group2()[3])
                        + (self.group1()[2] * reverse.group0()[1])
                        + (self.group1()[1] * reverse.group2()[2])
                        + (self.group0()[3] * reverse.group1()[0])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group2()[3])),
                    ((self.group2()[1] * reverse.group2()[3])
                        + (self.group2()[0] * reverse.group1()[2])
                        + (self.group1()[2] * reverse.group2()[0])
                        + (self.group1()[0] * reverse.group0()[2])
                        + (self.group0()[3] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group2()[3])),
                    ((self.group2()[2] * reverse.group2()[3])
                        + (self.group2()[1] * reverse.group1()[0])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group1()[0] * reverse.group2()[1])
                        + (self.group0()[3] * reverse.group1()[2])
                        + (self.group0()[2] * reverse.group2()[3])
                        + (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group0(), 2, 0, 1, 1) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group2()[1]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group2()[1])
                        + (self.group2()[0] * reverse.group0()[3])
                        + (self.group0()[3] * reverse.group2()[0])
                        + (self.group0()[2] * reverse.group0()[1])),
                    ((self.group2()[1] * reverse.group0()[3])
                        + (self.group2()[0] * reverse.group2()[2])
                        + (self.group0()[3] * reverse.group2()[1])
                        + (self.group0()[0] * reverse.group0()[2])),
                    ((self.group2()[2] * reverse.group0()[3])
                        + (self.group2()[1] * reverse.group2()[0])
                        + (self.group0()[3] * reverse.group2()[2])
                        + (self.group0()[1] * reverse.group0()[0])),
                    (-(self.group2()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group2()[2])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 1, 2, 0, 3) * swizzle!(reverse.group0(), 2, 0, 1, 3))
                + (Simd32x4::from(self.group0()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                + (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group0()[1]) + (self.group0()[0] * reverse.group0()[3])),
                    ((self.group2()[0] * reverse.group0()[2]) + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group2()[1] * reverse.group0()[0]) + (self.group0()[2] * reverse.group0()[3])),
                    (-(self.group2()[2] * reverse.group1()[2])
                        - (self.group2()[1] * reverse.group1()[1])
                        - (self.group2()[0] * reverse.group1()[0])
                        - (self.group1()[2] * reverse.group0()[2])
                        + (self.group1()[1] * reverse.group2()[1])
                        - (self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group2()[0])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group2()[2], 2) - f32::powi(self.group2()[1], 2) - f32::powi(self.group2()[0], 2) + f32::powi(self.group0()[3], 2)
                - f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
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
    //      f32       20       34        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       31       45        0
    //  no simd       64       78        0
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
            ((swizzle!(self.group1(), 3, 2, 1, 2) * Simd32x4::from([reverse.group0()[3], reverse.group0()[1], reverse.group1()[3], reverse.group1()[3]]))
                - (swizzle!(self.group1(), 2, 3, 3, 3) * Simd32x4::from([reverse.group0()[2], reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                - (swizzle!(self.group1(), 1, 1, 2, 0) * swizzle!(reverse.group0(), 1, 2, 0, 1))
                - (swizzle!(reverse.group0(), 0, 3, 3, 3) * Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (swizzle!(reverse.group1(), 3, 3, 0, 1) * Simd32x4::from([self.group0()[3], self.group1()[0], self.group0()[2], self.group0()[0]]))
                - (swizzle!(self.group0(), 2, 2, 0, 1) * swizzle!(reverse.group1(), 2, 1, 2, 0))
                + Simd32x4::from([
                    (-(self.group0()[0] * reverse.group1()[0]) - (self.group0()[1] * reverse.group1()[1])),
                    ((self.group0()[3] * reverse.group0()[0]) + (self.group0()[1] * reverse.group1()[2])),
                    ((self.group1()[0] * reverse.group0()[2]) + (self.group0()[3] * reverse.group0()[1])),
                    ((self.group1()[1] * reverse.group0()[0]) + (self.group0()[3] * reverse.group0()[2])),
                ])),
            // e23, e31, e12, e45
            ((reverse.group1() * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 3) * swizzle!(reverse.group0(), 2, 0, 1, 3))
                + (swizzle!(reverse.group1(), 3, 3, 3, 2) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]))
                + (swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[0]]))
                + (swizzle!(self.group0(), 2, 0, 2, 1) * Simd32x4::from([reverse.group1()[1], reverse.group1()[2], reverse.group0()[3], reverse.group1()[1]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group0()[1]) + (self.group0()[0] * reverse.group0()[3]) - (self.group0()[1] * reverse.group1()[2])),
                    ((self.group1()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group1()[0]) + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group1()[1] * reverse.group0()[0]) - (self.group0()[0] * reverse.group1()[1]) + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group1()[2] * reverse.group0()[2]) - (self.group1()[1] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group1()[3] * self.group0()[3]) - (self.group1()[2] * self.group0()[2]) - (self.group1()[1] * self.group0()[1]) - (self.group1()[0] * self.group0()[0])
                + (self.group0()[3] * self.group1()[3])
                - (self.group0()[2] * self.group1()[2])
                - (self.group0()[0] * self.group1()[0])
                - (self.group0()[1] * self.group1()[1])),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleInversionOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       20        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       24       28        0
    //  no simd       45       52        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e1234, e4235, e4315, e4125
            self.group1(),
        );
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (-(swizzle!(self.group1(), 3, 2, 3, 3) * Simd32x4::from([reverse.group0()[1], reverse.group1()[0], reverse.group1()[0], reverse.group1()[3]]))
                + (swizzle!(reverse.group0(), 2, 0, 1, 3) * Simd32x4::from([self.group1()[2], self.group1()[3], self.group1()[1], self.group0()[3]]))
                - (swizzle!(self.group1(), 1, 1, 2, 2) * Simd32x4::from([reverse.group1()[0], reverse.group0()[2], reverse.group0()[0], reverse.group1()[2]]))
                - (swizzle!(reverse.group1(), 3, 1, 2, 1) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[1]]))
                + Simd32x4::from([
                    ((self.group1()[0] * reverse.group1()[1]) - (self.group0()[3] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group0()[3])),
                    ((self.group1()[0] * reverse.group1()[2]) - (self.group0()[3] * reverse.group0()[1])
                        + (self.group0()[0] * reverse.group1()[3])
                        + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group1()[0] * reverse.group1()[3]) - (self.group0()[3] * reverse.group0()[2])
                        + (self.group0()[2] * reverse.group0()[3])
                        + (self.group0()[1] * reverse.group1()[1])),
                    0.0,
                ])),
            // e23, e31, e12, e1234
            ((swizzle!(self.group1(), 3, 2, 3, 3) * Simd32x4::from([reverse.group1()[2], reverse.group0()[3], reverse.group0()[3], reverse.group0()[2]]))
                - (swizzle!(reverse.group1(), 3, 1, 2, 0) * Simd32x4::from([self.group1()[2], self.group1()[3], self.group1()[1], self.group0()[3]]))
                + (swizzle!(self.group1(), 1, 1, 2, 2) * Simd32x4::from([reverse.group0()[3], reverse.group1()[3], reverse.group1()[1], reverse.group0()[1]]))
                + Simd32x4::from([
                    (self.group0()[3] * reverse.group1()[1]),
                    (self.group0()[3] * reverse.group1()[2]),
                    (self.group0()[3] * reverse.group1()[3]),
                    ((self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[3])
                        - (self.group0()[2] * reverse.group1()[3])
                        - (self.group0()[0] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group1()[2])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[3], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[1], 2)),
        );
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       87        0
    //    simd3        0        1        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       80      100        0
    //  no simd      116      138        0
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
            ((Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group0()[3]]))
                - (swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[2]]))
                + (Simd32x4::from(reverse.group2()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group0()[1]) + (self.group0()[2] * reverse.group1()[1])),
                    ((self.group1()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group1()[2])),
                    ((self.group1()[1] * reverse.group0()[0]) + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group1()[2])
                        - (self.group1()[1] * reverse.group1()[1])
                        - (self.group1()[0] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((reverse.group2() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[3]]))
                - (swizzle!(self.group2(), 1, 2, 0, 3) * swizzle!(reverse.group0(), 2, 0, 1, 3))
                + (swizzle!(reverse.group2(), 3, 3, 3, 2) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[2]]))
                + (swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[0]]))
                + (swizzle!(self.group0(), 2, 0, 2, 1) * Simd32x4::from([reverse.group2()[1], reverse.group2()[2], reverse.group0()[3], reverse.group2()[1]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group0()[1]) + (self.group1()[2] * reverse.group1()[1]) - (self.group1()[1] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group0()[3])
                        - (self.group0()[1] * reverse.group2()[2])),
                    ((self.group2()[0] * reverse.group0()[2]) - (self.group1()[2] * reverse.group1()[0]) + (self.group1()[0] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group2()[1] * reverse.group0()[0]) + (self.group1()[1] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group2()[1])
                        + (self.group0()[1] * reverse.group2()[0])),
                    (-(self.group2()[2] * reverse.group0()[2]) - (self.group2()[1] * reverse.group0()[1]) - (self.group2()[0] * reverse.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group2()[2] * reverse.group1()[1]) - (self.group2()[1] * reverse.group1()[2]) + (self.group1()[2] * reverse.group2()[1])
                    - (self.group1()[1] * reverse.group2()[2])
                    + (self.group0()[3] * reverse.group1()[0])
                    + (self.group1()[0] * reverse.group0()[3])),
                (-(self.group2()[2] * reverse.group1()[0]) + (self.group2()[0] * reverse.group1()[2]) - (self.group1()[2] * reverse.group2()[0])
                    + (self.group1()[1] * reverse.group0()[3])
                    + (self.group0()[3] * reverse.group1()[1])
                    + (self.group1()[0] * reverse.group2()[2])),
                ((self.group2()[1] * reverse.group1()[0]) - (self.group2()[0] * reverse.group1()[1])
                    + (self.group1()[2] * reverse.group0()[3])
                    + (self.group1()[1] * reverse.group2()[0])
                    + (self.group0()[3] * reverse.group1()[2])
                    - (self.group1()[0] * reverse.group2()[1])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group1()[2]]))
                - (swizzle!(self.group2(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[1]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group0()[1]) + (self.group2()[0] * reverse.group2()[3]) + (self.group0()[3] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group0()[3])
                        + (self.group0()[1] * reverse.group2()[2])),
                    ((self.group2()[1] * reverse.group2()[3])
                        + (self.group2()[0] * reverse.group0()[2])
                        + (self.group0()[3] * reverse.group0()[1])
                        + (self.group0()[2] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group0()[3])),
                    ((self.group2()[2] * reverse.group2()[3]) + (self.group2()[1] * reverse.group0()[0]) + (self.group0()[3] * reverse.group0()[2])
                        - (self.group0()[2] * reverse.group0()[3])
                        + (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group2()[0] * reverse.group1()[0]) - (self.group1()[0] * reverse.group2()[0]) - (self.group1()[1] * reverse.group2()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group2()[3] * self.group0()[3])
                - (self.group2()[2] * self.group0()[2])
                - (self.group2()[1] * self.group0()[1])
                - (self.group2()[0] * self.group0()[0])
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                + (self.group0()[3] * self.group2()[3])
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       11        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group0()[0] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[0])),
                ((self.group0()[1] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[1])),
                ((self.group0()[2] * reverse.group0()[3]) - (self.group0()[3] * reverse.group0()[2])),
                (self.group0()[3] * reverse.group0()[3]),
            ]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[3], 2));
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DipoleOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       74       87        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       74       90        0
    //  no simd       74       96        0
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
                ((self.group1()[2] * reverse.group0()[1]) - (self.group1()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[1])),
                (-(self.group1()[2] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[2]) + (self.group0()[0] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[0])),
                ((self.group1()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[1])
                    + (self.group0()[1] * reverse.group1()[0])),
                (-(self.group2()[2] * reverse.group0()[2])
                    - (self.group2()[1] * reverse.group0()[1])
                    - (self.group2()[0] * reverse.group0()[0])
                    - (self.group1()[2] * reverse.group1()[2])
                    - (self.group1()[1] * reverse.group1()[1])
                    - (self.group1()[0] * reverse.group1()[0])
                    - (self.group0()[2] * reverse.group2()[2])
                    - (self.group0()[0] * reverse.group2()[0])
                    - (self.group0()[1] * reverse.group2()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                ((self.group2()[2] * reverse.group0()[1]) - (self.group2()[1] * reverse.group0()[2]) + (self.group1()[2] * reverse.group1()[1])
                    - (self.group1()[1] * reverse.group1()[2])
                    - (self.group0()[1] * reverse.group2()[2])
                    + (self.group0()[2] * reverse.group2()[1])),
                (-(self.group2()[2] * reverse.group0()[0]) + (self.group2()[0] * reverse.group0()[2]) - (self.group1()[2] * reverse.group1()[0])
                    + (self.group1()[0] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group2()[2])
                    - (self.group0()[2] * reverse.group2()[0])),
                ((self.group2()[1] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[1]) + (self.group1()[1] * reverse.group1()[0])
                    - (self.group1()[0] * reverse.group1()[1])
                    - (self.group0()[0] * reverse.group2()[1])
                    + (self.group0()[1] * reverse.group2()[0])),
                (-(self.group2()[2] * reverse.group0()[2]) - (self.group2()[1] * reverse.group0()[1]) - (self.group2()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group2()[2])
                    + (self.group0()[0] * reverse.group2()[0])
                    + (self.group0()[1] * reverse.group2()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group2()[2] * reverse.group1()[1]) - (self.group2()[1] * reverse.group1()[2]) - (self.group1()[1] * reverse.group2()[2])
                    + (self.group1()[2] * reverse.group2()[1])),
                (-(self.group2()[2] * reverse.group1()[0]) + (self.group2()[0] * reverse.group1()[2]) + (self.group1()[0] * reverse.group2()[2])
                    - (self.group1()[2] * reverse.group2()[0])),
                ((self.group2()[1] * reverse.group1()[0]) - (self.group2()[0] * reverse.group1()[1]) - (self.group1()[0] * reverse.group2()[1])
                    + (self.group1()[1] * reverse.group2()[0])),
                (-(self.group1()[2] * reverse.group0()[2])
                    - (self.group1()[1] * reverse.group0()[1])
                    - (self.group1()[0] * reverse.group0()[0])
                    - (self.group0()[2] * reverse.group1()[2])
                    - (self.group0()[0] * reverse.group1()[0])
                    - (self.group0()[1] * reverse.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self.group2()[2] * reverse.group0()[1]) - (self.group2()[1] * reverse.group0()[2]) + (self.group0()[1] * reverse.group2()[2])
                    - (self.group0()[2] * reverse.group2()[1])),
                (-(self.group2()[2] * reverse.group0()[0]) + (self.group2()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group2()[2])
                    + (self.group0()[2] * reverse.group2()[0])),
                ((self.group2()[1] * reverse.group0()[0]) - (self.group2()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group2()[1])
                    - (self.group0()[1] * reverse.group2()[0])),
                (-(self.group2()[2] * reverse.group1()[2])
                    - (self.group2()[1] * reverse.group1()[1])
                    - (self.group2()[0] * reverse.group1()[0])
                    - (self.group1()[2] * reverse.group2()[2])
                    - (self.group1()[0] * reverse.group2()[0])
                    - (self.group1()[1] * reverse.group2()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group2()[2] * self.group0()[2])
                - (self.group2()[1] * self.group0()[1])
                - (self.group2()[0] * self.group0()[0])
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
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
        let geometric_product = Scalar::from_groups(/* scalar */ (self[e45] * reverse[e45]));
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2));
        let subtraction = Scalar::from_groups(/* scalar */ (geometric_product[scalar] - scalar_product[scalar]));
        return subtraction;
    }
}
impl ConstraintViolation for FlatPoint {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       11        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (-(self.group0()[0] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[0])),
                (-(self.group0()[1] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[1])),
                (-(self.group0()[2] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[2])),
                (self.group0()[3] * reverse.group0()[3]),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[3], 2));
        let subtraction = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
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
impl ConstraintViolation for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       20       25        0
    //  no simd       44       52        0
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
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group1(), 2, 0, 1, 2))
                + (Simd32x4::from(self.group0()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group0()[3]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group1()[1]) + (self.group1()[0] * reverse.group0()[3])),
                    ((self.group1()[1] * reverse.group0()[3]) + (self.group1()[0] * reverse.group1()[2])),
                    ((self.group1()[2] * reverse.group0()[3]) + (self.group1()[1] * reverse.group1()[0])),
                    (-(self.group1()[1] * reverse.group1()[1]) - (self.group1()[0] * reverse.group1()[0])),
                ])),
            // e15, e25, e35, e3215
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
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[1], 2) + f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[0], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for FlectorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       16       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
        let geometric_product = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            ((swizzle!(self.group0(), 3, 2, 3, 0) * swizzle!(reverse.group0(), 2, 0, 0, 0)) - (swizzle!(self.group0(), 2, 3, 1, 3) * swizzle!(reverse.group0(), 3, 1, 2, 3))
                + Simd32x4::from([
                    ((self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
                    ((self.group0()[0] * reverse.group0()[2]) + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group0()[2] * reverse.group0()[1]) + (self.group0()[0] * reverse.group0()[3])),
                    (-(self.group0()[2] * reverse.group0()[2]) - (self.group0()[1] * reverse.group0()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for Line {
    type Output = AntiMotor;
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
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self.group0()[1] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[1])),
                (-(self.group0()[0] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[0])),
                ((self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
                ((self.group0()[2] * reverse.group0()[2]) + (self.group0()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group0()[1])),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-(self.group1()[2] * reverse.group0()[1]) + (self.group1()[1] * reverse.group0()[2]) + (self.group0()[1] * reverse.group1()[2])
                    - (self.group0()[2] * reverse.group1()[1])),
                ((self.group1()[2] * reverse.group0()[0]) - (self.group1()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group1()[2])
                    + (self.group0()[2] * reverse.group1()[0])),
                (-(self.group1()[1] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[1])
                    - (self.group0()[1] * reverse.group1()[0])),
                ((self.group1()[2] * reverse.group0()[2])
                    + (self.group1()[1] * reverse.group0()[1])
                    + (self.group1()[0] * reverse.group0()[0])
                    + (self.group0()[2] * reverse.group1()[2])
                    + (self.group0()[0] * reverse.group1()[0])
                    + (self.group0()[1] * reverse.group1()[1])),
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)));
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for LineOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       10        0
    //  no simd        8       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_product = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            ((self.group0()[1] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[1])),
            (-(self.group0()[0] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[0])),
            ((self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
            ((self.group0()[2] * reverse.group0()[2]) + (self.group0()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group0()[1])),
        ]));
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)));
        let subtraction = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiMotor;
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
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (-(Simd32x4::from(self.group0()[3]) * reverse.group0())
                + (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group0()[2] * reverse.group0()[1]) - (self.group0()[0] * reverse.group0()[3])),
                    (-(self.group0()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group0()[2] * reverse.group0()[3]) - (self.group0()[1] * reverse.group0()[0])),
                    ((self.group0()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group0()[1])),
                ])),
            // e15, e25, e35, e3215
            (-(Simd32x4::from(self.group1()[3]) * reverse.group0()) + (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                - (Simd32x4::from(self.group0()[3]) * reverse.group1())
                + (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(reverse.group1(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[3])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group1()[3])),
                    (-(self.group1()[1] * reverse.group0()[3])
                        - (self.group1()[0] * reverse.group0()[2])
                        - (self.group0()[0] * reverse.group1()[2])
                        - (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group1()[2] * reverse.group0()[3])
                        - (self.group1()[1] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[3])
                        - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       13        0
    //  no simd       16       19        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            (-(Simd32x4::from(self.group0()[3]) * reverse.group0())
                + (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group0()[2] * reverse.group0()[1]) - (self.group0()[0] * reverse.group0()[3])),
                    (-(self.group0()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group0()[2] * reverse.group0()[3]) - (self.group0()[1] * reverse.group0()[0])),
                    ((self.group0()[0] * reverse.group0()[0]) + (self.group0()[1] * reverse.group0()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      320      330        0
    //    simd2       16       16        0
    //    simd3      116      122        0
    //    simd4       81       83        0
    // Totals...
    // yes simd      533      551        0
    //  no simd     1024     1060        0
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
            ((Simd32x2::from(self[e45]) * Simd32x2::from([reverse.group9()[0], reverse.group1()[3]]))
                + (Simd32x2::from(self.group9()[0]) * Simd32x2::from([reverse[e45], reverse[e1]]))
                - (Simd32x2::from(self.group6()[3]) * Simd32x2::from([reverse.group6()[3], reverse.group3()[3]]))
                - (Simd32x2::from(reverse.group5()[2]) * Simd32x2::from([self.group5()[2], self.group6()[2]]))
                - (Simd32x2::from(reverse.group5()[1]) * Simd32x2::from([self.group5()[1], self.group6()[1]]))
                - (Simd32x2::from(reverse.group5()[0]) * Simd32x2::from([self.group5()[0], self.group6()[0]]))
                - (Simd32x2::from(reverse.group3()[2]) * Simd32x2::from([self.group4()[2], self.group8()[2]]))
                - (Simd32x2::from(reverse.group3()[1]) * Simd32x2::from([self.group4()[1], self.group8()[1]]))
                - (Simd32x2::from(reverse.group3()[0]) * Simd32x2::from([self.group4()[0], self.group8()[0]]))
                - (Simd32x2::from(reverse.group4()[2]) * Simd32x2::from([self.group3()[2], self.group7()[2]]))
                - (Simd32x2::from(reverse.group4()[1]) * Simd32x2::from([self.group3()[1], self.group7()[1]]))
                - (Simd32x2::from(reverse.group4()[0]) * Simd32x2::from([self.group3()[0], self.group7()[0]]))
                + (Simd32x2::from(reverse.group1()[2]) * Simd32x2::from([self.group1()[2], self.group9()[3]]))
                + (Simd32x2::from(reverse.group1()[1]) * Simd32x2::from([self.group1()[1], self.group9()[2]]))
                + (Simd32x2::from(reverse.group1()[0]) * Simd32x2::from([self.group1()[0], self.group9()[1]]))
                + (Simd32x2::from(self.group0()[0]) * reverse.group0())
                + Simd32x2::from([
                    (-(self.group9()[3] * reverse.group9()[3]) - (self.group9()[2] * reverse.group9()[2]) - (self.group9()[1] * reverse.group9()[1])
                        + (self.group8()[2] * reverse.group7()[2])
                        + (self.group8()[1] * reverse.group7()[1])
                        + (self.group8()[0] * reverse.group7()[0])
                        + (self.group7()[2] * reverse.group8()[2])
                        + (self.group7()[1] * reverse.group8()[1])
                        + (self.group7()[0] * reverse.group8()[0])
                        + (self.group6()[2] * reverse.group6()[2])
                        + (self.group6()[1] * reverse.group6()[1])
                        + (self.group6()[0] * reverse.group6()[0])
                        + (self.group3()[3] * reverse.group3()[3])
                        - (self[e1] * reverse.group1()[3])
                        - (self.group1()[3] * reverse[e1])
                        - (self.group0()[1] * reverse.group0()[1])),
                    (-(self.group5()[2] * reverse.group6()[2])
                        - (self.group5()[1] * reverse.group6()[1])
                        - (self.group5()[0] * reverse.group6()[0])
                        - (self.group4()[2] * reverse.group7()[2])
                        - (self.group4()[1] * reverse.group7()[1])
                        - (self.group4()[0] * reverse.group7()[0])
                        - (self.group3()[3] * reverse.group6()[3])
                        - (self.group3()[2] * reverse.group8()[2])
                        - (self.group3()[1] * reverse.group8()[1])
                        - (self.group3()[0] * reverse.group8()[0])
                        + (self[e1] * reverse.group9()[0])
                        + (self.group1()[3] * reverse[e45])
                        + (self.group1()[2] * reverse.group9()[3])
                        + (self.group1()[1] * reverse.group9()[2])
                        + (self.group1()[0] * reverse.group9()[1])
                        + (self.group0()[1] * reverse.group0()[0])),
                ])),
            // e1, e2, e3, e4
            (-(swizzle!(self.group9(), 3, 2, 3, 0) * Simd32x4::from([reverse.group6()[1], reverse.group0()[1], reverse.group0()[1], reverse.group6()[3]]))
                + (swizzle!(self.group9(), 2, 3, 1, 3) * Simd32x4::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1], reverse.group7()[2]]))
                + (swizzle!(self.group9(), 0, 0, 0, 2) * Simd32x4::from([reverse.group8()[0], reverse.group8()[1], reverse.group8()[2], reverse.group7()[1]]))
                - (swizzle!(reverse.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self.group6()[2]]))
                + (swizzle!(reverse.group3(), 2, 0, 1, 3) * Simd32x4::from([self.group8()[1], self.group8()[2], self.group8()[0], self.group1()[3]]))
                - (swizzle!(reverse.group9(), 0, 0, 0, 3) * Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], self.group7()[2]]))
                + (Simd32x4::from(self.group6()[3]) * Simd32x4::from([reverse.group5()[0], reverse.group5()[1], reverse.group5()[2], reverse.group9()[0]]))
                - (swizzle!(self.group6(), 2, 1, 2, 1) * Simd32x4::from([reverse.group9()[2], reverse.group3()[3], reverse.group3()[3], reverse.group3()[1]]))
                + (swizzle!(reverse.group9(), 3, 1, 2, 0) * Simd32x4::from([self.group6()[1], self.group6()[2], self.group6()[0], self.group0()[1]]))
                - (swizzle!(self.group6(), 0, 0, 1, 0) * Simd32x4::from([reverse.group3()[3], reverse.group9()[3], reverse.group9()[1], reverse.group3()[0]]))
                + (swizzle!(reverse.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group3()[2]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 3) * Simd32x4::from([self.group5()[1], self.group5()[2], self.group5()[0], self.group3()[3]]))
                - (swizzle!(reverse.group6(), 0, 2, 0, 2) * Simd32x4::from([self.group3()[3], self.group9()[1], self.group9()[2], self.group3()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2, 1) * Simd32x4::from([reverse.group8()[1], reverse[e1], reverse[e1], reverse.group1()[1]]))
                - (swizzle!(self.group3(), 1, 3, 3, 1) * Simd32x4::from([reverse.group8()[2], reverse.group6()[1], reverse.group6()[2], reverse.group6()[1]]))
                + (swizzle!(self.group3(), 0, 0, 1, 0) * Simd32x4::from([reverse[e1], reverse.group8()[2], reverse.group8()[0], reverse.group1()[0]]))
                - (swizzle!(reverse.group3(), 0, 1, 2, 2) * Simd32x4::from([self[e1], self[e1], self[e1], self.group1()[2]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group4()[0], reverse.group4()[1], reverse.group4()[2], reverse.group0()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group5()[2], reverse.group5()[0], reverse.group5()[1], reverse.group3()[1]]))
                + (Simd32x4::from(self.group0()[0]) * reverse.group1())
                - (swizzle!(reverse.group9(), 1, 2, 3, 2) * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group7()[1]]))
                + Simd32x4::from([
                    (-(self[e45] * reverse.group7()[0]) - (self.group9()[1] * reverse.group0()[1]) + (self.group7()[2] * reverse.group4()[1])
                        - (self.group7()[1] * reverse.group4()[2])
                        + (self.group7()[0] * reverse[e45])
                        + (self.group5()[0] * reverse.group6()[3])
                        - (self.group4()[2] * reverse.group7()[1])
                        + (self.group4()[1] * reverse.group7()[2])
                        - (self.group4()[0] * reverse.group1()[3])
                        + (self.group1()[2] * reverse.group5()[1])
                        + (self.group1()[0] * reverse.group0()[0])),
                    (-(self[e45] * reverse.group7()[1]) - (self.group7()[2] * reverse.group4()[0])
                        + (self.group7()[1] * reverse[e45])
                        + (self.group7()[0] * reverse.group4()[2])
                        + (self.group5()[1] * reverse.group6()[3])
                        + (self.group4()[2] * reverse.group7()[0])
                        - (self.group4()[1] * reverse.group1()[3])
                        - (self.group4()[0] * reverse.group7()[2])
                        - (self.group3()[2] * reverse.group8()[0])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group1()[0] * reverse.group5()[2])),
                    (-(self[e45] * reverse.group7()[2]) + (self.group7()[2] * reverse[e45]) + (self.group7()[1] * reverse.group4()[0]) - (self.group7()[0] * reverse.group4()[1])
                        + (self.group5()[2] * reverse.group6()[3])
                        - (self.group4()[2] * reverse.group1()[3])
                        - (self.group4()[1] * reverse.group7()[0])
                        + (self.group4()[0] * reverse.group7()[1])
                        - (self.group3()[0] * reverse.group8()[1])
                        + (self.group1()[2] * reverse.group0()[0])
                        + (self.group1()[1] * reverse.group5()[0])),
                    ((self.group9()[1] * reverse.group7()[0]) + (self.group9()[0] * reverse.group0()[1])
                        - (self.group7()[2] * reverse.group5()[2])
                        - (self.group7()[1] * reverse.group5()[1])
                        - (self.group7()[0] * reverse.group9()[1])
                        - (self.group7()[0] * reverse.group5()[0])
                        - (self.group5()[2] * reverse.group7()[2])
                        - (self.group5()[1] * reverse.group7()[1])
                        - (self.group5()[0] * reverse.group7()[0])
                        - (self.group3()[0] * reverse.group6()[0])
                        - (self.group1()[0] * reverse.group3()[0])),
                ])),
            // e5
            ((self[e45] * reverse.group6()[3]) + (self[e45] * reverse.group0()[1])
                - (self.group9()[3] * reverse.group8()[2])
                - (self.group9()[2] * reverse.group8()[1])
                - (self.group9()[1] * reverse.group8()[0])
                + (self.group8()[2] * reverse.group9()[3])
                - (self.group8()[2] * reverse.group5()[2])
                + (self.group8()[1] * reverse.group9()[2])
                - (self.group8()[1] * reverse.group5()[1])
                + (self.group8()[0] * reverse.group9()[1])
                - (self.group8()[0] * reverse.group5()[0])
                - (self.group6()[3] * reverse[e45])
                - (self.group6()[2] * reverse.group4()[2])
                - (self.group6()[1] * reverse.group4()[1])
                - (self.group6()[0] * reverse.group4()[0])
                - (self.group5()[2] * reverse.group8()[2])
                - (self.group5()[1] * reverse.group8()[1])
                - (self.group5()[0] * reverse.group8()[0])
                - (self.group4()[2] * reverse.group6()[2])
                - (self.group4()[2] * reverse.group1()[2])
                - (self.group4()[1] * reverse.group6()[1])
                - (self.group4()[1] * reverse.group1()[1])
                - (self.group4()[0] * reverse.group6()[0])
                - (self.group4()[0] * reverse.group1()[0])
                + (self.group3()[3] * reverse[e1])
                - (self[e1] * reverse.group3()[3])
                + (self[e1] * reverse.group0()[0])
                + (self.group1()[2] * reverse.group4()[2])
                + (self.group1()[1] * reverse.group4()[1])
                + (self.group1()[0] * reverse.group4()[0])
                + (self.group0()[0] * reverse[e1])
                + (self.group0()[1] * reverse[e45])),
            // e41, e42, e43, e45
            (-(swizzle!(self.group9(), 3, 2, 3, 3) * Simd32x4::from([reverse.group3()[1], reverse.group9()[0], reverse.group9()[0], reverse.group5()[2]]))
                + (swizzle!(reverse.group3(), 2, 0, 1, 3) * Simd32x4::from([self.group9()[2], self.group9()[3], self.group9()[1], self.group0()[0]]))
                - (swizzle!(self.group9(), 1, 1, 2, 2) * Simd32x4::from([reverse.group9()[0], reverse.group3()[2], reverse.group3()[0], reverse.group5()[1]]))
                + (swizzle!(reverse.group9(), 1, 2, 3, 0) * Simd32x4::from([self.group9()[0], self.group9()[0], self.group9()[0], self[e45]]))
                - (Simd32x4::from(self.group7()[2]) * Simd32x4::from([reverse.group6()[1], reverse.group1()[0], reverse.group0()[1], reverse.group8()[2]]))
                + (swizzle!(reverse.group6(), 2, 0, 3, 3) * Simd32x4::from([self.group7()[1], self.group7()[2], self.group7()[2], self.group0()[1]]))
                - (Simd32x4::from(self.group7()[1]) * Simd32x4::from([reverse.group1()[2], reverse.group0()[1], reverse.group6()[0], reverse.group8()[1]]))
                - (Simd32x4::from(self.group7()[0]) * Simd32x4::from([reverse.group0()[1], reverse.group6()[2], reverse.group1()[1], reverse.group8()[0]]))
                - (swizzle!(self.group6(), 3, 3, 3, 2) * Simd32x4::from([reverse.group7()[0], reverse.group7()[1], reverse.group7()[2], reverse.group1()[2]]))
                - (swizzle!(self.group6(), 2, 1, 2, 1) * Simd32x4::from([reverse.group7()[1], reverse.group1()[3], reverse.group1()[3], reverse.group1()[1]]))
                + (swizzle!(self.group6(), 1, 2, 0, 3) * Simd32x4::from([reverse.group7()[2], reverse.group7()[0], reverse.group7()[1], reverse.group0()[1]]))
                - (swizzle!(self.group6(), 0, 0, 1, 0) * Simd32x4::from([reverse.group1()[3], reverse.group7()[2], reverse.group7()[0], reverse.group1()[0]]))
                - (swizzle!(reverse.group3(), 2, 0, 1, 2) * Simd32x4::from([self.group5()[1], self.group5()[2], self.group5()[0], self.group4()[2]]))
                - (swizzle!(reverse.group3(), 0, 1, 2, 1) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group4()[1]]))
                + (swizzle!(self.group3(), 2, 1, 2, 3) * Simd32x4::from([reverse.group9()[2], reverse.group3()[3], reverse.group3()[3], reverse.group0()[0]]))
                + (swizzle!(self.group3(), 2, 1, 2, 2) * Simd32x4::from([reverse.group5()[1], reverse.group0()[0], reverse.group0()[0], reverse.group4()[2]]))
                - (swizzle!(reverse.group9(), 3, 1, 2, 3) * Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group5()[2]]))
                + (swizzle!(self.group3(), 0, 0, 1, 1) * Simd32x4::from([reverse.group3()[3], reverse.group9()[3], reverse.group9()[1], reverse.group4()[1]]))
                + (swizzle!(self.group3(), 0, 0, 1, 0) * Simd32x4::from([reverse.group0()[0], reverse.group5()[2], reverse.group5()[0], reverse.group4()[0]]))
                - (swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(reverse.group6(), 0, 1, 2, 2))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse[e1]]))
                - (swizzle!(self.group1(), 2, 1, 2, 1) * Simd32x4::from([reverse.group7()[1], reverse.group1()[3], reverse.group1()[3], reverse.group6()[1]]))
                - (swizzle!(self.group1(), 0, 0, 1, 0) * Simd32x4::from([reverse.group1()[3], reverse.group7()[2], reverse.group7()[0], reverse.group6()[0]]))
                + Simd32x4::from([
                    ((self.group9()[0] * reverse.group5()[0])
                        + (self.group7()[2] * reverse.group1()[1])
                        + (self.group7()[0] * reverse.group6()[3])
                        + (self.group5()[2] * reverse.group3()[1])
                        + (self.group5()[0] * reverse.group9()[0])
                        - (self.group3()[1] * reverse.group5()[2])
                        + (self.group1()[1] * reverse.group7()[2])
                        + (self.group0()[0] * reverse.group3()[0])
                        - (self.group0()[1] * reverse.group7()[0])),
                    ((self.group9()[0] * reverse.group5()[1])
                        + (self.group7()[1] * reverse.group6()[3])
                        + (self.group7()[0] * reverse.group1()[2])
                        + (self.group5()[1] * reverse.group9()[0])
                        + (self.group5()[0] * reverse.group3()[2])
                        - (self.group3()[2] * reverse.group5()[0])
                        + (self.group1()[2] * reverse.group7()[0])
                        + (self.group0()[0] * reverse.group3()[1])
                        - (self.group0()[1] * reverse.group7()[1])),
                    ((self.group9()[0] * reverse.group5()[2])
                        + (self.group7()[1] * reverse.group1()[0])
                        + (self.group7()[0] * reverse.group6()[1])
                        + (self.group5()[2] * reverse.group9()[0])
                        + (self.group5()[1] * reverse.group3()[0])
                        - (self.group3()[0] * reverse.group5()[1])
                        + (self.group1()[0] * reverse.group7()[1])
                        + (self.group0()[0] * reverse.group3()[2])
                        - (self.group0()[1] * reverse.group7()[2])),
                    (-(self.group9()[1] * reverse.group5()[0]) - (self.group9()[0] * reverse[e45])
                        + (self.group8()[2] * reverse.group7()[2])
                        + (self.group8()[1] * reverse.group7()[1])
                        + (self.group8()[0] * reverse.group7()[0])
                        - (self.group5()[1] * reverse.group9()[2])
                        - (self.group5()[0] * reverse.group9()[1])
                        - (self.group4()[0] * reverse.group3()[0])
                        - (self[e1] * reverse.group1()[3])),
                ])),
            // e15, e25, e35
            (-(Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group9()[1], reverse.group9()[2], reverse.group9()[3]]))
                + (Simd32x3::from(self[e45]) * reverse.group5())
                + (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group9()[3], self.group9()[1], self.group9()[2]]))
                - (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group9()[2], self.group9()[3], self.group9()[1]]))
                + (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                - (swizzle!(self.group8(), 2, 1, 2) * Simd32x3::from([reverse.group6()[1], reverse.group6()[3], reverse.group6()[3]]))
                - (swizzle!(self.group8(), 2, 1, 2) * Simd32x3::from([reverse.group1()[1], reverse.group0()[1], reverse.group0()[1]]))
                + (swizzle!(self.group8(), 1, 2, 0) * Simd32x3::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1]]))
                + (swizzle!(self.group8(), 1, 2, 0) * Simd32x3::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                - (swizzle!(self.group8(), 0, 0, 1) * Simd32x3::from([reverse.group6()[3], reverse.group6()[2], reverse.group6()[0]]))
                - (swizzle!(self.group8(), 0, 0, 1) * Simd32x3::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0]]))
                + (Simd32x3::from(self.group6()[3]) * reverse.group8())
                - (swizzle!(reverse.group8(), 1, 2, 0) * Simd32x3::from([self.group6()[2], self.group6()[0], self.group6()[1]]))
                + (swizzle!(reverse.group8(), 2, 0, 1) * Simd32x3::from([self.group6()[1], self.group6()[2], self.group6()[0]]))
                - (Simd32x3::from(reverse[e1]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (swizzle!(self.group5(), 2, 1, 2) * Simd32x3::from([reverse.group4()[1], reverse[e45], reverse[e45]]))
                - (swizzle!(self.group5(), 1, 2, 0) * swizzle!(reverse.group4(), 2, 0, 1))
                + (swizzle!(self.group5(), 0, 0, 1) * Simd32x3::from([reverse[e45], reverse.group4()[2], reverse.group4()[0]]))
                - (Simd32x3::from(self.group4()[2]) * Simd32x3::from([reverse.group9()[2], reverse.group5()[0], reverse.group3()[3]]))
                + (Simd32x3::from(self.group4()[2]) * Simd32x3::from([reverse.group5()[1], reverse.group9()[1], reverse.group0()[0]]))
                + (Simd32x3::from(self.group4()[1]) * Simd32x3::from([reverse.group9()[3], reverse.group0()[0], reverse.group5()[0]]))
                - (Simd32x3::from(self.group4()[1]) * Simd32x3::from([reverse.group5()[2], reverse.group3()[3], reverse.group9()[1]]))
                - (Simd32x3::from(self.group4()[0]) * Simd32x3::from([reverse.group3()[3], reverse.group9()[3], reverse.group5()[1]]))
                + (Simd32x3::from(self.group4()[0]) * Simd32x3::from([reverse.group0()[0], reverse.group5()[2], reverse.group9()[2]]))
                + (Simd32x3::from(self.group3()[3]) * reverse.group4())
                - (Simd32x3::from(self[e1]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                - (Simd32x3::from(self[e1]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group8(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))
                - (swizzle!(reverse.group8(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (Simd32x3::from(reverse[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group4())
                - (Simd32x3::from(self.group0()[1]) * reverse.group8())),
            // e23, e31, e12
            ((Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2]]))
                + (Simd32x3::from(reverse.group3()[3]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                + (Simd32x3::from(self.group9()[0]) * reverse.group4())
                - (swizzle!(self.group8(), 2, 1, 2) * Simd32x3::from([reverse.group7()[1], reverse.group1()[3], reverse.group1()[3]]))
                + (swizzle!(self.group8(), 1, 2, 0) * swizzle!(reverse.group7(), 2, 0, 1))
                - (swizzle!(self.group8(), 0, 0, 1) * Simd32x3::from([reverse.group1()[3], reverse.group7()[2], reverse.group7()[0]]))
                - (swizzle!(self.group7(), 2, 1, 2) * Simd32x3::from([reverse.group8()[1], reverse[e1], reverse[e1]]))
                + (swizzle!(self.group7(), 1, 2, 0) * swizzle!(reverse.group8(), 2, 0, 1))
                - (swizzle!(self.group7(), 0, 0, 1) * Simd32x3::from([reverse[e1], reverse.group8()[2], reverse.group8()[0]]))
                - (Simd32x3::from(self.group6()[3]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                - (Simd32x3::from(reverse.group0()[1]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (swizzle!(self.group5(), 2, 1, 2) * Simd32x3::from([reverse.group5()[1], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group5(), 1, 2, 0) * swizzle!(reverse.group5(), 2, 0, 1))
                + (swizzle!(self.group5(), 0, 0, 1) * Simd32x3::from([reverse.group0()[0], reverse.group5()[2], reverse.group5()[0]]))
                + (swizzle!(self.group4(), 2, 1, 2) * Simd32x3::from([reverse.group3()[1], reverse.group9()[0], reverse.group9()[0]]))
                - (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1]]))
                + (swizzle!(self.group4(), 0, 0, 1) * Simd32x3::from([reverse.group9()[0], reverse.group3()[2], reverse.group3()[0]]))
                + (Simd32x3::from(self.group3()[3]) * Simd32x3::from([reverse.group9()[1], reverse.group9()[2], reverse.group9()[3]]))
                + (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]))
                - (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]))
                + (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                - (Simd32x3::from(self[e1]) * reverse.group7())
                - (Simd32x3::from(self.group1()[3]) * reverse.group8())
                - (Simd32x3::from(reverse.group6()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group5())
                - (Simd32x3::from(self.group0()[1]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                + Simd32x3::from([
                    ((self.group9()[3] * reverse.group9()[2]) - (self.group9()[2] * reverse.group9()[3]) - (self.group6()[2] * reverse.group6()[1])
                        + (self.group6()[1] * reverse.group6()[2])
                        - (self.group1()[2] * reverse.group1()[1])
                        + (self.group1()[1] * reverse.group1()[2])),
                    (-(self.group9()[3] * reverse.group9()[1]) + (self.group9()[1] * reverse.group9()[3]) + (self.group6()[2] * reverse.group6()[0])
                        - (self.group6()[0] * reverse.group6()[2])
                        + (self.group1()[2] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[2])),
                    ((self.group9()[2] * reverse.group9()[1]) - (self.group9()[1] * reverse.group9()[2]) - (self.group6()[1] * reverse.group6()[0])
                        + (self.group6()[0] * reverse.group6()[1])
                        - (self.group1()[1] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group1()[1])),
                ])),
            // e415, e425, e435, e321
            (-(swizzle!(reverse.group1(), 1, 2, 0, 3) * Simd32x4::from([self.group9()[3], self.group9()[1], self.group9()[2], self[e45]]))
                + (swizzle!(self.group9(), 2, 3, 1, 3) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group6()[2]]))
                + (swizzle!(self.group9(), 0, 0, 0, 2) * Simd32x4::from([reverse.group8()[0], reverse.group8()[1], reverse.group8()[2], reverse.group6()[1]]))
                + (swizzle!(reverse.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self.group8()[2]]))
                - (swizzle!(reverse.group3(), 2, 0, 1, 3) * Simd32x4::from([self.group8()[1], self.group8()[2], self.group8()[0], self.group0()[1]]))
                + (swizzle!(reverse.group9(), 0, 0, 0, 3) * Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], self.group6()[2]]))
                + (Simd32x4::from(reverse[e45]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group1()[3]]))
                - (swizzle!(reverse.group9(), 1, 2, 3, 0) * Simd32x4::from([self.group6()[3], self.group6()[3], self.group6()[3], self[e1]]))
                + (swizzle!(self.group6(), 2, 1, 2, 3) * Simd32x4::from([reverse.group5()[1], reverse.group0()[0], reverse.group0()[0], reverse.group0()[0]]))
                + (swizzle!(self.group6(), 0, 0, 1, 1) * Simd32x4::from([reverse.group0()[0], reverse.group5()[2], reverse.group5()[0], reverse.group9()[2]]))
                + (swizzle!(reverse.group6(), 1, 2, 0, 0) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group9()[1]]))
                - (swizzle!(reverse.group1(), 0, 1, 2, 2) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group5()[2]]))
                - (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([reverse.group8()[2], reverse.group8()[0], reverse.group8()[1], reverse.group0()[1]]))
                + (Simd32x4::from(reverse[e1]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group9()[0]]))
                + (swizzle!(reverse.group3(), 0, 1, 2, 1) * Simd32x4::from([self[e1], self[e1], self[e1], self.group8()[1]]))
                - (swizzle!(self.group1(), 2, 1, 2, 2) * Simd32x4::from([reverse.group9()[2], reverse.group3()[3], reverse.group3()[3], reverse.group5()[2]]))
                + (swizzle!(reverse.group9(), 3, 1, 2, 1) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group6()[0]]))
                - (swizzle!(self.group1(), 0, 0, 1, 1) * Simd32x4::from([reverse.group3()[3], reverse.group9()[3], reverse.group9()[1], reverse.group5()[1]]))
                + (Simd32x4::from(self.group0()[0]) * reverse.group6())
                + Simd32x4::from([
                    ((self[e45] * reverse.group7()[0]) - (self.group9()[1] * reverse.group6()[3]) + (self.group7()[2] * reverse.group4()[1])
                        - (self.group7()[1] * reverse.group4()[2])
                        - (self.group6()[1] * reverse.group5()[2])
                        - (self.group5()[1] * reverse.group6()[2])
                        + (self.group5()[0] * reverse.group0()[1])
                        + (self.group4()[2] * reverse.group7()[1])
                        - (self.group4()[1] * reverse.group7()[2])
                        + (self.group4()[0] * reverse.group1()[3])
                        + (self.group3()[2] * reverse.group8()[1])
                        + (self.group1()[3] * reverse.group4()[0])
                        + (self.group0()[1] * reverse.group5()[0])),
                    ((self[e45] * reverse.group7()[1]) - (self.group9()[2] * reverse.group6()[3]) - (self.group7()[2] * reverse.group4()[0])
                        + (self.group7()[0] * reverse.group4()[2])
                        - (self.group6()[2] * reverse.group5()[0])
                        - (self.group5()[2] * reverse.group6()[0])
                        + (self.group5()[1] * reverse.group0()[1])
                        - (self.group4()[2] * reverse.group7()[0])
                        + (self.group4()[1] * reverse.group1()[3])
                        + (self.group4()[0] * reverse.group7()[2])
                        + (self.group3()[0] * reverse.group8()[2])
                        + (self.group1()[3] * reverse.group4()[1])
                        + (self.group0()[1] * reverse.group5()[1])),
                    ((self[e45] * reverse.group7()[2]) - (self.group9()[3] * reverse.group6()[3]) + (self.group7()[1] * reverse.group4()[0])
                        - (self.group7()[0] * reverse.group4()[1])
                        - (self.group6()[0] * reverse.group5()[1])
                        + (self.group5()[2] * reverse.group0()[1])
                        - (self.group5()[0] * reverse.group6()[1])
                        + (self.group4()[2] * reverse.group1()[3])
                        + (self.group4()[1] * reverse.group7()[0])
                        - (self.group4()[0] * reverse.group7()[1])
                        + (self.group3()[1] * reverse.group8()[0])
                        + (self.group1()[3] * reverse.group4()[2])
                        + (self.group0()[1] * reverse.group5()[2])),
                    ((self.group8()[0] * reverse.group3()[0])
                        - (self.group7()[2] * reverse.group4()[2])
                        - (self.group7()[1] * reverse.group4()[1])
                        - (self.group7()[0] * reverse.group4()[0])
                        - (self.group5()[1] * reverse.group1()[1])
                        - (self.group5()[0] * reverse.group1()[0])
                        + (self.group4()[2] * reverse.group7()[2])
                        + (self.group4()[1] * reverse.group7()[1])
                        + (self.group4()[0] * reverse.group7()[0])
                        - (self.group3()[2] * reverse.group8()[2])
                        - (self.group3()[1] * reverse.group8()[1])
                        - (self.group3()[0] * reverse.group8()[0])
                        - (self.group1()[0] * reverse.group5()[0])),
                ])),
            // e423, e431, e412
            (-(swizzle!(reverse.group7(), 1, 2, 0) * Simd32x3::from([self.group9()[3], self.group9()[1], self.group9()[2]]))
                + (swizzle!(reverse.group7(), 2, 0, 1) * Simd32x3::from([self.group9()[2], self.group9()[3], self.group9()[1]]))
                - (Simd32x3::from(reverse.group1()[3]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                + (Simd32x3::from(self.group9()[0]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                - (Simd32x3::from(self.group9()[0]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(self.group7(), 2, 1, 2) * Simd32x3::from([reverse.group9()[2], reverse.group3()[3], reverse.group3()[3]]))
                + (swizzle!(self.group7(), 2, 1, 2) * Simd32x3::from([reverse.group5()[1], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group7(), 1, 2, 0) * Simd32x3::from([reverse.group9()[3], reverse.group9()[1], reverse.group9()[2]]))
                - (swizzle!(self.group7(), 1, 2, 0) * swizzle!(reverse.group5(), 2, 0, 1))
                + (swizzle!(self.group7(), 0, 0, 1) * Simd32x3::from([reverse.group3()[3], reverse.group9()[3], reverse.group9()[1]]))
                + (swizzle!(self.group7(), 0, 0, 1) * Simd32x3::from([reverse.group0()[0], reverse.group5()[2], reverse.group5()[0]]))
                + (Simd32x3::from(self.group6()[3]) * Simd32x3::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2]]))
                + (Simd32x3::from(reverse.group9()[0]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (swizzle!(self.group5(), 2, 1, 2) * Simd32x3::from([reverse.group7()[1], reverse.group1()[3], reverse.group1()[3]]))
                - (swizzle!(self.group5(), 1, 2, 0) * swizzle!(reverse.group7(), 2, 0, 1))
                + (swizzle!(self.group5(), 0, 0, 1) * Simd32x3::from([reverse.group1()[3], reverse.group7()[2], reverse.group7()[0]]))
                - (Simd32x3::from(self.group3()[3]) * reverse.group7())
                + (Simd32x3::from(self.group3()[2]) * Simd32x3::from([reverse.group6()[1], reverse.group1()[0], reverse.group0()[1]]))
                - (Simd32x3::from(self.group3()[2]) * Simd32x3::from([reverse.group1()[1], reverse.group6()[0], reverse.group6()[3]]))
                - (Simd32x3::from(self.group3()[1]) * Simd32x3::from([reverse.group6()[2], reverse.group6()[3], reverse.group1()[0]]))
                + (Simd32x3::from(self.group3()[1]) * Simd32x3::from([reverse.group1()[2], reverse.group0()[1], reverse.group6()[0]]))
                - (Simd32x3::from(self.group3()[0]) * Simd32x3::from([reverse.group6()[3], reverse.group1()[2], reverse.group6()[1]]))
                + (Simd32x3::from(self.group3()[0]) * Simd32x3::from([reverse.group0()[1], reverse.group6()[2], reverse.group1()[1]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([reverse.group9()[1], reverse.group9()[2], reverse.group9()[3]]))
                + (Simd32x3::from(self.group1()[3]) * reverse.group5())
                + (Simd32x3::from(reverse.group9()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group7())
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2]]))
                + Simd32x3::from([
                    ((self.group6()[2] * reverse.group3()[1]) - (self.group6()[1] * reverse.group3()[2]) + (self.group1()[2] * reverse.group3()[1])
                        - (self.group1()[1] * reverse.group3()[2])),
                    (-(self.group6()[2] * reverse.group3()[0]) + (self.group6()[0] * reverse.group3()[2]) - (self.group1()[2] * reverse.group3()[0])
                        + (self.group1()[0] * reverse.group3()[2])),
                    ((self.group6()[1] * reverse.group3()[0]) - (self.group6()[0] * reverse.group3()[1]) + (self.group1()[1] * reverse.group3()[0])
                        - (self.group1()[0] * reverse.group3()[1])),
                ])),
            // e235, e315, e125
            ((Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                + (Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group8(), 1, 2, 0) * Simd32x3::from([self.group9()[3], self.group9()[1], self.group9()[2]]))
                - (swizzle!(reverse.group8(), 2, 0, 1) * Simd32x3::from([self.group9()[2], self.group9()[3], self.group9()[1]]))
                + (Simd32x3::from(reverse[e1]) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[3]]))
                - (Simd32x3::from(self.group8()[2]) * Simd32x3::from([reverse.group9()[2], reverse.group5()[0], reverse.group3()[3]]))
                + (Simd32x3::from(self.group8()[2]) * Simd32x3::from([reverse.group5()[1], reverse.group9()[1], reverse.group0()[0]]))
                + (Simd32x3::from(self.group8()[1]) * Simd32x3::from([reverse.group9()[3], reverse.group0()[0], reverse.group5()[0]]))
                - (Simd32x3::from(self.group8()[1]) * Simd32x3::from([reverse.group5()[2], reverse.group3()[3], reverse.group9()[1]]))
                - (Simd32x3::from(self.group8()[0]) * Simd32x3::from([reverse.group3()[3], reverse.group9()[3], reverse.group5()[1]]))
                + (Simd32x3::from(self.group8()[0]) * Simd32x3::from([reverse.group0()[0], reverse.group5()[2], reverse.group9()[2]]))
                - (Simd32x3::from(self.group6()[3]) * reverse.group4())
                + (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group6()[2], self.group6()[0], self.group6()[1]]))
                - (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group6()[1], self.group6()[2], self.group6()[0]]))
                + (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (swizzle!(self.group5(), 2, 1, 2) * Simd32x3::from([reverse.group8()[1], reverse[e1], reverse[e1]]))
                - (swizzle!(self.group5(), 1, 2, 0) * swizzle!(reverse.group8(), 2, 0, 1))
                + (swizzle!(self.group5(), 0, 0, 1) * Simd32x3::from([reverse[e1], reverse.group8()[2], reverse.group8()[0]]))
                + (swizzle!(self.group4(), 2, 1, 2) * Simd32x3::from([reverse.group6()[1], reverse.group6()[3], reverse.group6()[3]]))
                + (swizzle!(self.group4(), 2, 1, 2) * Simd32x3::from([reverse.group1()[1], reverse.group0()[1], reverse.group0()[1]]))
                - (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1]]))
                - (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + (swizzle!(self.group4(), 0, 0, 1) * Simd32x3::from([reverse.group6()[3], reverse.group6()[2], reverse.group6()[0]]))
                + (swizzle!(self.group4(), 0, 0, 1) * Simd32x3::from([reverse.group0()[1], reverse.group1()[2], reverse.group1()[0]]))
                + (Simd32x3::from(self.group3()[3]) * reverse.group8())
                - (Simd32x3::from(self[e1]) * Simd32x3::from([reverse.group9()[1], reverse.group9()[2], reverse.group9()[3]]))
                + (Simd32x3::from(self[e1]) * reverse.group5())
                - (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))
                + (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                - (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group8())
                + (Simd32x3::from(self.group0()[1]) * reverse.group4())),
            // e1234, e4235, e4315, e4125
            ((swizzle!(reverse.group3(), 2, 0, 1, 2) * Simd32x4::from([self.group9()[3], self[e45], self[e45], self[e45]]))
                + (swizzle!(self.group9(), 2, 3, 2, 3) * Simd32x4::from([reverse.group3()[1], reverse.group5()[1], reverse.group0()[0], reverse.group0()[0]]))
                + (swizzle!(self.group9(), 1, 1, 1, 2) * Simd32x4::from([reverse.group3()[0], reverse.group0()[0], reverse.group5()[2], reverse.group5()[0]]))
                + (Simd32x4::from(reverse.group3()[3]) * Simd32x4::from([self.group9()[0], self.group5()[0], self.group5()[1], self.group5()[2]]))
                + (swizzle!(reverse.group6(), 2, 0, 1, 2) * Simd32x4::from([self.group7()[2], self.group6()[3], self.group6()[3], self.group6()[3]]))
                - (swizzle!(reverse.group1(), 2, 3, 3, 3) * Simd32x4::from([self.group7()[2], self.group8()[0], self.group8()[1], self.group8()[2]]))
                + (swizzle!(reverse.group6(), 1, 3, 3, 3) * Simd32x4::from([self.group7()[1], self.group6()[0], self.group6()[1], self.group6()[2]]))
                - (swizzle!(reverse.group1(), 1, 2, 0, 1) * Simd32x4::from([self.group7()[1], self.group6()[1], self.group6()[2], self.group6()[0]]))
                + (swizzle!(reverse.group6(), 0, 1, 2, 0) * Simd32x4::from([self.group7()[0], self.group1()[2], self.group1()[0], self.group1()[1]]))
                + (swizzle!(self.group6(), 2, 2, 0, 1) * Simd32x4::from([reverse.group7()[2], reverse.group1()[1], reverse.group1()[2], reverse.group1()[0]]))
                - (swizzle!(reverse.group3(), 2, 2, 0, 1) * Simd32x4::from([self.group5()[2], self.group4()[1], self.group4()[2], self.group4()[0]]))
                - (swizzle!(reverse.group9(), 0, 3, 1, 2) * Simd32x4::from([self.group3()[3], self.group5()[1], self.group5()[2], self.group5()[0]]))
                - (swizzle!(self.group3(), 2, 2, 1, 2) * Simd32x4::from([reverse.group9()[3], reverse.group4()[1], reverse[e45], reverse[e45]]))
                - (swizzle!(self.group3(), 2, 0, 0, 1) * Simd32x4::from([reverse.group5()[2], reverse[e45], reverse.group4()[2], reverse.group4()[0]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group6()[3], reverse.group8()[0], reverse.group8()[1], reverse.group8()[2]]))
                - (swizzle!(self.group1(), 3, 1, 2, 0) * Simd32x4::from([reverse.group0()[1], reverse.group6()[2], reverse.group6()[0], reverse.group6()[1]]))
                + (swizzle!(self.group1(), 2, 0, 1, 2) * Simd32x4::from([reverse.group7()[2], reverse.group0()[1], reverse.group0()[1], reverse.group0()[1]]))
                + (swizzle!(reverse.group9(), 0, 2, 3, 1) * Simd32x4::from([self.group0()[0], self.group5()[2], self.group5()[0], self.group5()[1]]))
                + Simd32x4::from([
                    ((self.group9()[0] * reverse.group0()[0]) - (self.group7()[0] * reverse.group1()[0]) - (self.group6()[3] * reverse.group1()[3])
                        + (self.group6()[1] * reverse.group7()[1])
                        + (self.group6()[0] * reverse.group7()[0])
                        - (self.group5()[1] * reverse.group3()[1])
                        - (self.group5()[0] * reverse.group3()[0])
                        - (self.group3()[1] * reverse.group9()[2])
                        - (self.group3()[1] * reverse.group5()[1])
                        - (self.group3()[0] * reverse.group9()[1])
                        - (self.group3()[0] * reverse.group5()[0])
                        + (self.group1()[1] * reverse.group7()[1])
                        + (self.group1()[0] * reverse.group7()[0])
                        - (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group9()[2] * reverse.group5()[2]) - (self.group9()[0] * reverse.group4()[0]) - (self.group8()[2] * reverse.group7()[1])
                        + (self.group8()[1] * reverse.group7()[2])
                        + (self.group7()[2] * reverse.group8()[1])
                        - (self.group7()[1] * reverse.group8()[2])
                        + (self.group7()[0] * reverse[e1])
                        + (self.group4()[2] * reverse.group3()[1])
                        + (self.group4()[0] * reverse.group9()[0])
                        + (self.group3()[3] * reverse.group5()[0])
                        + (self.group3()[1] * reverse.group4()[2])
                        - (self[e1] * reverse.group7()[0])
                        + (self.group0()[0] * reverse.group9()[1])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group9()[3] * reverse.group5()[0]) - (self.group9()[0] * reverse.group4()[1]) + (self.group8()[2] * reverse.group7()[0])
                        - (self.group8()[0] * reverse.group7()[2])
                        - (self.group7()[2] * reverse.group8()[0])
                        + (self.group7()[1] * reverse[e1])
                        + (self.group7()[0] * reverse.group8()[2])
                        + (self.group4()[1] * reverse.group9()[0])
                        + (self.group4()[0] * reverse.group3()[2])
                        + (self.group3()[3] * reverse.group5()[1])
                        + (self.group3()[2] * reverse.group4()[0])
                        - (self[e1] * reverse.group7()[1])
                        + (self.group0()[0] * reverse.group9()[2])
                        + (self.group0()[1] * reverse.group1()[1])),
                    (-(self.group9()[1] * reverse.group5()[1]) - (self.group9()[0] * reverse.group4()[2]) - (self.group8()[1] * reverse.group7()[0])
                        + (self.group8()[0] * reverse.group7()[1])
                        + (self.group7()[2] * reverse[e1])
                        + (self.group7()[1] * reverse.group8()[0])
                        - (self.group7()[0] * reverse.group8()[1])
                        + (self.group4()[2] * reverse.group9()[0])
                        + (self.group4()[1] * reverse.group3()[0])
                        + (self.group3()[3] * reverse.group5()[2])
                        + (self.group3()[0] * reverse.group4()[1])
                        - (self[e1] * reverse.group7()[2])
                        + (self.group0()[0] * reverse.group9()[3])
                        + (self.group0()[1] * reverse.group1()[2])),
                ])),
            // e3215
            (-(self[e45] * reverse.group3()[3]) + (self[e45] * reverse.group0()[0])
                - (self.group9()[3] * reverse.group4()[2])
                - (self.group9()[2] * reverse.group4()[1])
                - (self.group9()[1] * reverse.group4()[0])
                + (self.group8()[2] * reverse.group6()[2])
                + (self.group8()[2] * reverse.group1()[2])
                + (self.group8()[1] * reverse.group6()[1])
                + (self.group8()[1] * reverse.group1()[1])
                + (self.group8()[0] * reverse.group6()[0])
                + (self.group8()[0] * reverse.group1()[0])
                + (self.group6()[3] * reverse[e1])
                + (self.group6()[2] * reverse.group8()[2])
                + (self.group6()[1] * reverse.group8()[1])
                + (self.group6()[0] * reverse.group8()[0])
                - (self.group5()[2] * reverse.group4()[2])
                - (self.group5()[1] * reverse.group4()[1])
                - (self.group5()[0] * reverse.group4()[0])
                + (self.group4()[2] * reverse.group9()[3])
                - (self.group4()[2] * reverse.group5()[2])
                + (self.group4()[1] * reverse.group9()[2])
                - (self.group4()[1] * reverse.group5()[1])
                + (self.group4()[0] * reverse.group9()[1])
                - (self.group4()[0] * reverse.group5()[0])
                + (self.group3()[3] * reverse[e45])
                - (self[e1] * reverse.group6()[3])
                - (self[e1] * reverse.group0()[1])
                - (self.group1()[2] * reverse.group8()[2])
                - (self.group1()[1] * reverse.group8()[1])
                - (self.group1()[0] * reverse.group8()[0])
                + (self.group0()[0] * reverse[e45])
                - (self.group0()[1] * reverse[e1])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self[e45] * self.group9()[0]) - f32::powi(self.group9()[3], 2) - f32::powi(self.group9()[2], 2) - f32::powi(self.group9()[1], 2)
                + (self.group9()[0] * self[e45])
                + (self.group8()[2] * self.group7()[2])
                + (self.group8()[1] * self.group7()[1])
                + (self.group8()[0] * self.group7()[0])
                + (self.group7()[2] * self.group8()[2])
                + (self.group7()[1] * self.group8()[1])
                + (self.group7()[0] * self.group8()[0])
                - f32::powi(self.group6()[3], 2)
                + f32::powi(self.group6()[2], 2)
                + f32::powi(self.group6()[1], 2)
                + f32::powi(self.group6()[0], 2)
                - f32::powi(self.group5()[2], 2)
                - f32::powi(self.group5()[1], 2)
                - f32::powi(self.group5()[0], 2)
                - (self.group4()[2] * self.group3()[2])
                - (self.group4()[1] * self.group3()[1])
                - (self.group4()[0] * self.group3()[0])
                + f32::powi(self.group3()[3], 2)
                - (self.group3()[2] * self.group4()[2])
                - (self.group3()[1] * self.group4()[1])
                - (self.group3()[0] * self.group4()[0])
                - (self[e1] * self.group1()[3])
                - (self.group1()[3] * self[e1])
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(geometric_product.group0()[0] - scalar_product[scalar]), geometric_product.group0()[1]]),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e5
            geometric_product[e1],
            // e41, e42, e43, e45
            geometric_product.group3(),
            // e15, e25, e35
            geometric_product.group4(),
            // e23, e31, e12
            geometric_product.group5(),
            // e415, e425, e435, e321
            geometric_product.group6(),
            // e423, e431, e412
            geometric_product.group7(),
            // e235, e315, e125
            geometric_product.group8(),
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
    //      f32        8        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       16       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircle::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            ((swizzle!(self.group0(), 2, 0, 1, 2) * swizzle!(reverse.group0(), 2, 3, 3, 3))
                + (swizzle!(self.group0(), 0, 3, 3, 3) * swizzle!(reverse.group0(), 0, 0, 1, 2))
                + Simd32x4::from([(-(self.group0()[3] * reverse.group0()[3]) + (self.group0()[1] * reverse.group0()[1])), 0.0, 0.0, 0.0])),
            // e23, e31, e12, e45
            Simd32x4::from([
                ((self.group0()[1] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[1])),
                (-(self.group0()[0] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[0])),
                ((self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
                0.0,
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
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
            Simd32x4::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2], 0.0]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryCircleRotor {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       16       17        0
    //  no simd       25       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e12345 */ self[e425]);
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            ((swizzle!(self.group0(), 2, 0, 1, 2) * swizzle!(reverse.group0(), 2, 3, 3, 3))
                + (swizzle!(self.group0(), 0, 3, 3, 3) * swizzle!(reverse.group0(), 0, 0, 1, 2))
                + Simd32x4::from([
                    (-(self[e425] * reverse[e425]) - (self.group0()[3] * reverse.group0()[3]) + (self.group0()[1] * reverse.group0()[1])),
                    0.0,
                    0.0,
                    0.0,
                ])),
            // e23, e31, e12, e45
            ((swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse[e425]]))
                + Simd32x4::from([
                    (-(self[e425] * reverse.group0()[0]) - (self.group0()[2] * reverse.group0()[1]) - (self.group0()[0] * reverse[e425])),
                    (-(self[e425] * reverse.group0()[1]) - (self.group0()[0] * reverse.group0()[2]) - (self.group0()[1] * reverse[e425])),
                    (-(self[e425] * reverse.group0()[2]) - (self.group0()[2] * reverse[e425]) - (self.group0()[1] * reverse.group0()[0])),
                    (self[e425] * reverse.group0()[3]),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self[e425], 2) - f32::powi(self.group0()[3], 2) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryDipole {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       13       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            ((swizzle!(self.group0(), 3, 0, 1, 2) * Simd32x4::from(reverse.group0()[3]))
                + Simd32x4::from([
                    (-(self.group0()[2] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
                    (self.group0()[3] * reverse.group0()[0]),
                    (self.group0()[3] * reverse.group0()[1]),
                    (self.group0()[3] * reverse.group0()[2]),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self.group0()[1] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[1])),
                ((self.group0()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[0])),
                (-(self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
                0.0,
            ]),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
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
            Simd32x4::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2], 0.0]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       33        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       36       38        0
    //  no simd       48       53        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)), /* e4235, e4315, e4125 */ self.group1());
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            ((swizzle!(reverse.group0(), 3, 1, 2, 0) * Simd32x4::from([self.group0()[3], self.group1()[2], self.group1()[0], self.group1()[1]]))
                - (swizzle!(reverse.group0(), 2, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group1()[1], self.group1()[2], self.group1()[0]]))
                - (swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([reverse.group0()[0], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group1()[2])
                        - (self.group1()[1] * reverse.group1()[1])
                        - (self.group1()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group0()[1])),
                    ((self.group0()[3] * reverse.group0()[0]) + (self.group0()[2] * reverse.group1()[1]) + (self.group0()[0] * reverse.group0()[3])),
                    ((self.group0()[3] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[2]) + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group0()[3] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[3]) + (self.group0()[1] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * reverse.group1()[1]) - (self.group1()[1] * reverse.group1()[2])
                        + (self.group1()[0] * reverse.group0()[3])
                        + (self.group0()[3] * reverse.group1()[0])
                        + (self.group0()[2] * reverse.group0()[1])),
                    (-(self.group1()[2] * reverse.group1()[0])
                        + (self.group1()[1] * reverse.group0()[3])
                        + (self.group1()[0] * reverse.group1()[2])
                        + (self.group0()[3] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group0()[2])),
                    ((self.group1()[2] * reverse.group0()[3]) + (self.group1()[1] * reverse.group1()[0]) - (self.group1()[0] * reverse.group1()[1])
                        + (self.group0()[3] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group0()[0])),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[2])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[0], 2) + f32::powi(self.group0()[3], 2)
                - f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryQuadNum {
    type Output = AntiMysteryQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryQuadNum::from_groups(/* e321, e12345 */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_product = AntiMysteryQuadNum::from_groups(/* e45, scalar */ Simd32x2::from([
            ((self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
            (-(self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
        ]));
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)));
        let subtraction = AntiMysteryQuadNum::from_groups(
            // e45, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4       14       17        0
    // Totals...
    // yes simd       22       17        0
    //  no simd       64       68        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ (self.group1() * Simd32x4::from(-1.0)));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            (-(swizzle!(self.group1(), 3, 1, 2, 0) * Simd32x4::from([reverse.group1()[3], reverse.group0()[3], reverse.group0()[1], reverse.group0()[2]]))
                + (swizzle!(self.group1(), 2, 3, 3, 3) * swizzle!(reverse.group1(), 2, 0, 1, 2))
                + (swizzle!(self.group1(), 1, 2, 1, 2) * Simd32x4::from([reverse.group1()[1], reverse.group0()[2], reverse.group1()[3], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 0, 0, 0, 1) * Simd32x4::from([reverse.group1()[0], reverse.group1()[3], reverse.group0()[3], reverse.group0()[1]]))
                + (swizzle!(self.group0(), 3, 3, 2, 3) * Simd32x4::from([reverse.group0()[3], reverse.group1()[1], reverse.group0()[0], reverse.group0()[0]]))
                + (swizzle!(self.group0(), 2, 0, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[1], reverse.group0()[2], reverse.group1()[0]]))
                - (swizzle!(self.group0(), 0, 2, 3, 1) * Simd32x4::from([reverse.group0()[0], reverse.group1()[2], reverse.group1()[0], reverse.group1()[1]]))
                + (swizzle!(self.group0(), 1, 1, 1, 0) * Simd32x4::from([reverse.group0()[1], reverse.group0()[0], reverse.group1()[2], reverse.group0()[3]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(reverse.group0(), 1, 2, 3, 3))
                - (swizzle!(self.group1(), 2, 1, 2, 1) * Simd32x4::from([reverse.group1()[1], reverse.group0()[0], reverse.group0()[0], reverse.group0()[2]]))
                + (swizzle!(self.group1(), 1, 2, 0, 3) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group0()[0]]))
                - (swizzle!(self.group1(), 0, 0, 1, 0) * Simd32x4::from([reverse.group0()[0], reverse.group1()[2], reverse.group1()[0], reverse.group0()[1]]))
                - (swizzle!(self.group0(), 3, 2, 3, 3) * Simd32x4::from([reverse.group0()[2], reverse.group1()[3], reverse.group1()[3], reverse.group1()[2]]))
                + (swizzle!(self.group0(), 2, 3, 1, 0) * Simd32x4::from([reverse.group0()[3], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                - (swizzle!(self.group0(), 0, 0, 2, 2) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group0()[1], reverse.group1()[1]]))
                - (swizzle!(self.group0(), 1, 1, 0, 1) * Simd32x4::from([reverse.group1()[3], reverse.group0()[3], reverse.group1()[2], reverse.group1()[0]]))),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[3], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group0()[3], 2)
                + f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       32        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       40       41        0
    //  no simd       64       68        0
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
            ((Simd32x4::from(self.group1()[3]) * swizzle!(reverse.group1(), 3, 0, 1, 2))
                - (swizzle!(self.group1(), 2, 1, 2, 0) * Simd32x4::from([reverse.group1()[2], reverse.group0()[3], reverse.group0()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group1(), 1, 2, 0, 1) * Simd32x4::from([self.group1()[1], self.group0()[2], self.group0()[3], self.group0()[1]]))
                + (swizzle!(reverse.group0(), 0, 2, 3, 1) * Simd32x4::from([self.group0()[0], self.group1()[2], self.group1()[0], self.group1()[1]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * reverse.group1()[0])
                        - (self.group0()[3] * reverse.group0()[3])
                        - (self.group0()[2] * reverse.group0()[2])
                        - (self.group0()[1] * reverse.group0()[1])),
                    ((self.group1()[0] * reverse.group1()[3])
                        + (self.group0()[3] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group0()[1])
                        + (self.group0()[1] * reverse.group0()[0])),
                    ((self.group1()[1] * reverse.group1()[3])
                        + (self.group0()[2] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group0()[2])
                        + (self.group0()[1] * reverse.group1()[2])),
                    ((self.group1()[2] * reverse.group1()[3])
                        + (self.group0()[3] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group0()[3])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group1()[3]) * swizzle!(reverse.group0(), 1, 2, 3, 0))
                + (swizzle!(reverse.group1(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group0()[3]]))
                - (swizzle!(reverse.group0(), 3, 1, 2, 2) * Simd32x4::from([self.group0()[2], self.group0()[3], self.group0()[1], self.group1()[1]]))
                + Simd32x4::from([
                    ((self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[3] * reverse.group0()[2])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[3])),
                    ((self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[3])
                        + (self.group0()[0] * reverse.group1()[1])
                        + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group1()[2] * reverse.group0()[0])
                        + (self.group0()[3] * reverse.group1()[3])
                        + (self.group0()[2] * reverse.group0()[1])
                        + (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[0] * reverse.group0()[1])
                        - (self.group0()[3] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group1()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[3], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                - f32::powi(self.group0()[3], 2)
                - f32::powi(self.group0()[2], 2)
                + f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for Plane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from([
                (-(self.group0()[1] * self.group0()[2]) + (self.group0()[2] * self.group0()[1])),
                ((self.group0()[0] * self.group0()[2]) - (self.group0()[2] * self.group0()[0])),
                (-(self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                ((self.group0()[0] * self.group0()[3]) - (self.group0()[3] * self.group0()[0])),
                ((self.group0()[1] * self.group0()[3]) - (self.group0()[3] * self.group0()[1])),
                ((self.group0()[2] * self.group0()[3]) - (self.group0()[3] * self.group0()[2])),
                (-f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
            ]),
        );
        let subtraction = AntiLine::from_groups(
            // e23, e31, e12
            geometric_product.group0(),
            // e15, e25, e35
            Simd32x3::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for PlaneOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            (-(self.group0()[1] * self.group0()[2]) + (self.group0()[2] * self.group0()[1])),
            ((self.group0()[0] * self.group0()[2]) - (self.group0()[2] * self.group0()[0])),
            (-(self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            (-f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
        ]));
        let subtraction = AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for QuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       16        0
    //  no simd       16       19        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = QuadNum::from_groups(
            // e4, e5, e321, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0), self.group0()[3]]),
        );
        let geometric_product = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            (-(swizzle!(self.group0(), 3, 3, 1, 3) * swizzle!(reverse.group0(), 0, 1, 0, 3))
                + Simd32x4::from([
                    (-(self.group0()[2] * reverse.group0()[0]) + (self.group0()[0] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[3])),
                    ((self.group0()[2] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[2]) - (self.group0()[1] * reverse.group0()[3])),
                    ((self.group0()[3] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[3]) + (self.group0()[0] * reverse.group0()[1])),
                    (-(self.group0()[2] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[3], 2) - f32::powi(self.group0()[2], 2) - (self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
        );
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for QuadNumAligningOrigin {
    type Output = AntiQuadNumOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        8        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (-(self.group0()[0] * self.group0()[2]) - (self.group0()[2] * self.group0()[0])),
            (-(self.group0()[1] * self.group0()[2]) - (self.group0()[2] * self.group0()[1])),
            ((self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            (-f32::powi(self.group0()[2], 2) - (self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
        ]));
        let subtraction = AntiQuadNumOrthogonalOrigin::from_groups(
            // e1234, e3215, e45
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiQuadNumAligningOriginAtInfinity::from_groups(/* e3215, scalar */ Simd32x2::from([
            (-(self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            (f32::powi(self.group0()[1], 2) * -1.0),
        ]));
        let subtraction = Horizon::from_groups(/* e3215 */ geometric_product.group0()[0]);
        return subtraction;
    }
}
impl ConstraintViolation for QuadNumAtInfinity {
    type Output = AntiQuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        9        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = QuadNumAtInfinity::from_groups(/* e5, e321, e12345 */ Simd32x3::from([self.group0()[0], (self.group0()[1] * -1.0), self.group0()[2]]));
        let geometric_product = AntiQuadNumAtInfinity::from_groups(/* e3215, e45, scalar */ Simd32x3::from([
            (-(self.group0()[2] * reverse.group0()[0]) + (self.group0()[1] * reverse.group0()[0])
                - (self.group0()[0] * reverse.group0()[1])
                - (self.group0()[0] * reverse.group0()[2])),
            ((self.group0()[1] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[1])),
            (-(self.group0()[1] * reverse.group0()[1]) - (self.group0()[2] * reverse.group0()[2])),
        ]));
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        let subtraction = AntiQuadNumAtInfinity::from_groups(/* e3215, e45, scalar */ Simd32x3::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            (geometric_product.group0()[2] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for QuadNumOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiQuadNumOnOrigin::from_groups(/* e1234, scalar */ Simd32x2::from([
            (-(self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            (f32::powi(self.group0()[1], 2) * -1.0),
        ]));
        let subtraction = NullSphereAtOrigin::from_groups(/* e1234 */ geometric_product.group0()[0]);
        return subtraction;
    }
}
impl ConstraintViolation for QuadNumOrthogonalOrigin {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = QuadNumOrthogonalOrigin::from_groups(/* e4, e5, e321 */ Simd32x3::from([self.group0()[0], self.group0()[1], (self.group0()[2] * -1.0)]));
        let geometric_product = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            ((self.group0()[0] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[0])),
            (-(self.group0()[1] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[1])),
            ((self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
            (-(self.group0()[2] * reverse.group0()[2]) - (self.group0()[0] * reverse.group0()[1]) - (self.group0()[1] * reverse.group0()[0])),
        ]));
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group0()[2], 2) - (self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
        );
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            geometric_product.group0()[0],
            geometric_product.group0()[1],
            geometric_product.group0()[2],
            (geometric_product.group0()[3] - scalar_product[scalar]),
        ]));
        return subtraction;
    }
}
impl ConstraintViolation for RoundPoint {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        6        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       10        9        0
    //  no simd       16       18        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(-0.0),
            // e23, e31, e12, e45
            ((swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e2]]))
                - (swizzle!(self.group0(), 2, 0, 1, 3) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self[e2]]))),
            // e15, e25, e35, scalar
            (-(Simd32x4::from(self[e2]) * self.group0())
                + Simd32x4::from([
                    (self.group0()[0] * self[e2]),
                    (self.group0()[1] * self[e2]),
                    (self.group0()[2] * self[e2]),
                    (-(self.group0()[3] * self[e2]) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self[e2] * self.group0()[3]) - (self.group0()[3] * self[e2]) + f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([
                geometric_product.group2()[0],
                geometric_product.group2()[1],
                geometric_product.group2()[2],
                (geometric_product.group2()[3] - scalar_product[scalar]),
            ]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for RoundPointAtOrigin {
    type Output = AntiMysteryQuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd2        1        3        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        4        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiMysteryQuadNum::from_groups(
            // e45, scalar
            (-(swizzle!(self.group0(), 1, 0) * self.group0()) + (self.group0() * swizzle!(self.group0(), 1, 0) * Simd32x2::from([1.0, -1.0]))),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (-(self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])));
        let subtraction = AntiMysteryQuadNum::from_groups(
            // e45, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
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
impl ConstraintViolation for Sphere {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        9        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       10       12        0
    //  no simd       16       21        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(-0.0),
            // e23, e31, e12, e45
            (-(swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e4315]]))
                + (swizzle!(self.group0(), 2, 0, 1, 3) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self[e4315]]))),
            // e15, e25, e35, scalar
            ((self.group0() * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self[e4315]]))
                + Simd32x4::from([
                    ((self.group0()[3] * self.group0()[0]) * -1.0),
                    ((self.group0()[3] * self.group0()[1]) * -1.0),
                    ((self.group0()[3] * self.group0()[2]) * -1.0),
                    ((self.group0()[3] * self[e4315]) - f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self[e4315] * self.group0()[3]) + (self.group0()[3] * self[e4315])
                - f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[0], 2)
                - f32::powi(self.group0()[1], 2)),
        );
        let subtraction = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([
                geometric_product.group2()[0],
                geometric_product.group2()[1],
                geometric_product.group2()[2],
                (geometric_product.group2()[3] - scalar_product[scalar]),
            ]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for SphereAtOrigin {
    type Output = AntiMysteryQuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd2        1        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        4        6        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiMysteryQuadNum::from_groups(
            // e45, scalar
            (Simd32x2::from((self.group0()[0] * self.group0()[1])) + (Simd32x2::from((self.group0()[1] * self.group0()[0])) * Simd32x2::from([-1.0, 1.0]))),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ ((self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])));
        let subtraction = AntiMysteryQuadNum::from_groups(
            // e45, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for SphereOnOrigin {
    type Output = AntiCircleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group0()[0] * self.group0()[3]) + (self.group0()[3] * self.group0()[0])),
                (-(self.group0()[1] * self.group0()[3]) + (self.group0()[3] * self.group0()[1])),
                (-(self.group0()[2] * self.group0()[3]) + (self.group0()[3] * self.group0()[2])),
                (-f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
            ]),
            // e23, e31, e12
            Simd32x3::from([
                (-(self.group0()[1] * self.group0()[2]) + (self.group0()[2] * self.group0()[1])),
                ((self.group0()[0] * self.group0()[2]) - (self.group0()[2] * self.group0()[0])),
                (-(self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            ]),
        );
        let subtraction = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
            // e23, e31, e12
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       90        0
    //    simd4       45       46        0
    // Totals...
    // yes simd      121      136        0
    //  no simd      256      274        0
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
            ((swizzle!(self.group3(), 3, 3, 3, 2) * swizzle!(reverse.group3(), 0, 1, 2, 2))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[3]]))
                - (swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[3]]))
                + (swizzle!(self.group3(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group3()[1]]))
                - (Simd32x4::from(reverse.group3()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[2]]))
                + (Simd32x4::from(self.group0()[2]) * Simd32x4::from([reverse.group3()[1], reverse.group1()[0], reverse.group1()[3], reverse.group2()[2]]))
                + (swizzle!(self.group0(), 1, 1, 1, 0) * Simd32x4::from([reverse.group1()[2], reverse.group1()[3], reverse.group3()[0], reverse.group2()[0]]))
                + (swizzle!(self.group0(), 0, 0, 0, 1) * Simd32x4::from([reverse.group1()[3], reverse.group3()[2], reverse.group1()[1], reverse.group2()[1]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group3()[3])
                        - (self.group0()[3] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group3()[2])
                        - (self.group0()[0] * reverse.group0()[3])),
                    (-(self.group1()[1] * reverse.group3()[3])
                        - (self.group1()[0] * reverse.group0()[2])
                        - (self.group0()[3] * reverse.group0()[1])
                        - (self.group0()[2] * reverse.group3()[0])
                        - (self.group0()[1] * reverse.group0()[3])
                        - (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group1()[2] * reverse.group3()[3])
                        - (self.group1()[1] * reverse.group0()[0])
                        - (self.group0()[3] * reverse.group0()[2])
                        - (self.group0()[2] * reverse.group0()[3])
                        - (self.group0()[1] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group3()[1])),
                    ((self.group3()[0] * reverse.group3()[0])
                        + (self.group2()[1] * reverse.group0()[1])
                        + (self.group2()[0] * reverse.group0()[0])
                        + (self.group1()[2] * reverse.group1()[2])
                        + (self.group1()[1] * reverse.group1()[1])
                        + (self.group1()[0] * reverse.group1()[0])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group3(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group1()[2]]))
                - (swizzle!(self.group3(), 2, 1, 2, 1) * Simd32x4::from([reverse.group3()[1], reverse.group1()[3], reverse.group1()[3], reverse.group1()[1]]))
                + (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group2()[3]]))
                - (swizzle!(self.group3(), 0, 0, 1, 0) * Simd32x4::from([reverse.group1()[3], reverse.group3()[2], reverse.group3()[0], reverse.group1()[0]]))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group3()[3]]))
                + (swizzle!(self.group2(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                - (swizzle!(reverse.group3(), 3, 3, 3, 2) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[2]]))
                - (swizzle!(self.group1(), 3, 3, 3, 1) * swizzle!(reverse.group3(), 0, 1, 2, 1))
                - (swizzle!(self.group1(), 2, 1, 2, 0) * Simd32x4::from([reverse.group1()[1], reverse.group0()[3], reverse.group0()[3], reverse.group3()[0]]))
                + (swizzle!(self.group1(), 1, 2, 0, 3) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group0()[3]]))
                - (swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[2]]))
                - (swizzle!(self.group0(), 2, 0, 2, 0) * swizzle!(reverse.group2(), 1, 2, 3, 0))
                - (swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(reverse.group2(), 3, 3, 0, 1))
                + (swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[3])),
                    (-(self.group2()[0] * reverse.group0()[2]) - (self.group1()[0] * reverse.group1()[2])),
                    (-(self.group2()[1] * reverse.group0()[0]) - (self.group1()[1] * reverse.group1()[0])),
                    ((self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            ((swizzle!(self.group3(), 2, 1, 2, 3) * Simd32x4::from([reverse.group2()[1], reverse.group2()[3], reverse.group2()[3], reverse.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[3]]))
                + (swizzle!(self.group3(), 0, 0, 1, 2) * Simd32x4::from([reverse.group2()[3], reverse.group2()[2], reverse.group2()[0], reverse.group0()[2]]))
                - (reverse.group3() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[3]]))
                - (swizzle!(reverse.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group0()[3]]))
                + (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group0()[2]]))
                + (swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group0()[2]]))
                + (swizzle!(self.group1(), 1, 2, 0, 1) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[1]]))
                - (swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group3()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[3] * reverse.group1()[0]) - (self.group2()[2] * reverse.group1()[1]) + (self.group2()[1] * reverse.group3()[2])
                        - (self.group2()[0] * reverse.group1()[3])
                        - (self.group2()[0] * reverse.group0()[3])
                        - (self.group1()[2] * reverse.group2()[1])
                        - (self.group1()[0] * reverse.group2()[3])),
                    (-(self.group2()[3] * reverse.group1()[1]) + (self.group2()[2] * reverse.group3()[0])
                        - (self.group2()[1] * reverse.group1()[3])
                        - (self.group2()[1] * reverse.group0()[3])
                        - (self.group2()[0] * reverse.group1()[2])
                        - (self.group1()[1] * reverse.group2()[3])
                        - (self.group1()[0] * reverse.group2()[2])),
                    (-(self.group2()[3] * reverse.group1()[2])
                        - (self.group2()[2] * reverse.group1()[3])
                        - (self.group2()[2] * reverse.group0()[3])
                        - (self.group2()[1] * reverse.group1()[0])
                        + (self.group2()[0] * reverse.group3()[1])
                        - (self.group1()[2] * reverse.group2()[3])
                        - (self.group1()[1] * reverse.group2()[0])),
                    ((self.group3()[1] * reverse.group0()[1]) + (self.group3()[0] * reverse.group0()[0]) + (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[1] * reverse.group3()[1])
                        + (self.group0()[1] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group3()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            ((reverse.group2() * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]))
                + (swizzle!(reverse.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[2]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[2]]))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                - (swizzle!(self.group2(), 2, 1, 2, 3) * Simd32x4::from([reverse.group0()[1], reverse.group3()[3], reverse.group3()[3], reverse.group0()[3]]))
                + (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group3()[2]]))
                + (swizzle!(reverse.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[1]]))
                + (swizzle!(self.group1(), 2, 1, 2, 2) * Simd32x4::from([reverse.group3()[1], reverse.group1()[3], reverse.group1()[3], reverse.group2()[2]]))
                + (swizzle!(self.group1(), 0, 0, 1, 1) * Simd32x4::from([reverse.group1()[3], reverse.group3()[2], reverse.group3()[0], reverse.group2()[1]]))
                + (swizzle!(reverse.group3(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group2()[1]]))
                + (swizzle!(reverse.group2(), 1, 2, 3, 0) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[2], self.group1()[0]]))
                - (swizzle!(reverse.group2(), 2, 0, 1, 1) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[1]]))
                + Simd32x4::from([
                    ((self.group3()[0] * reverse.group0()[3]) - (self.group2()[0] * reverse.group3()[3]) - (self.group1()[1] * reverse.group3()[2])
                        + (self.group0()[0] * reverse.group2()[3])),
                    ((self.group3()[1] * reverse.group0()[3]) - (self.group2()[0] * reverse.group0()[2]) - (self.group1()[2] * reverse.group3()[0])
                        + (self.group0()[1] * reverse.group2()[3])),
                    ((self.group3()[2] * reverse.group0()[3]) - (self.group2()[1] * reverse.group0()[0]) - (self.group1()[0] * reverse.group3()[1])
                        + (self.group0()[1] * reverse.group2()[0])),
                    (-(self.group3()[0] * reverse.group2()[0]) + (self.group2()[0] * reverse.group3()[0]) + (self.group2()[0] * reverse.group1()[0])
                        - (self.group0()[3] * reverse.group2()[3])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group3()[3] * self.group2()[3]) + f32::powi(self.group3()[2], 2) + f32::powi(self.group3()[1], 2) + f32::powi(self.group3()[0], 2)
                - (self.group2()[3] * self.group3()[3])
                + (self.group2()[2] * self.group0()[2])
                + (self.group2()[1] * self.group0()[1])
                + (self.group2()[0] * self.group0()[0])
                - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                - f32::powi(self.group0()[3], 2)
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       56       77        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       77       98        0
    //  no simd      140      161        0
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
            (-(reverse.group1() * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[3]]))
                - (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group0()[1], reverse.group1()[3], reverse.group1()[3], reverse.group2()[3]]))
                + (swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[2]]))
                - (swizzle!(reverse.group0(), 0, 2, 0, 3) * Simd32x4::from([self.group0()[3], self.group1()[0], self.group1()[1], self.group0()[3]]))
                + (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * reverse.group1()[3]) - (self.group0()[2] * reverse.group1()[1]) - (self.group0()[0] * reverse.group0()[3])),
                    (-(self.group0()[3] * reverse.group0()[1]) - (self.group0()[0] * reverse.group1()[2]) - (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group0()[3] * reverse.group0()[2]) - (self.group0()[2] * reverse.group0()[3]) - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group2()[1] * reverse.group0()[1])
                        + (self.group2()[0] * reverse.group0()[0])
                        + (self.group1()[1] * reverse.group1()[1])
                        + (self.group1()[0] * reverse.group1()[0])
                        + (self.group0()[2] * reverse.group2()[2])
                        + (self.group0()[0] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(Simd32x4::from(self.group2()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                + (swizzle!(self.group2(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                - (swizzle!(reverse.group2(), 0, 1, 2, 2) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[2]]))
                + (swizzle!(self.group1(), 1, 2, 0, 3) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group2()[3]]))
                - (swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[0]]))
                - (swizzle!(self.group0(), 2, 0, 2, 1) * swizzle!(reverse.group2(), 1, 2, 3, 1))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group1()[3])
                        - (self.group1()[2] * reverse.group1()[1])
                        - (self.group1()[0] * reverse.group0()[3])
                        - (self.group0()[0] * reverse.group2()[3])
                        + (self.group0()[1] * reverse.group2()[2])),
                    (-(self.group2()[1] * reverse.group1()[3])
                        - (self.group2()[0] * reverse.group0()[2])
                        - (self.group1()[1] * reverse.group0()[3])
                        - (self.group1()[0] * reverse.group1()[2])
                        + (self.group0()[2] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[3])),
                    (-(self.group2()[2] * reverse.group1()[3])
                        - (self.group2()[1] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group0()[3])
                        - (self.group1()[1] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group2()[1])
                        - (self.group0()[1] * reverse.group2()[0])),
                    ((self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            (-(reverse.group1() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[3]]))
                + (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group0()[2]]))
                - (Simd32x4::from(reverse.group0()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group1()[1])
                        - (self.group1()[2] * reverse.group2()[1])
                        - (self.group0()[3] * reverse.group2()[0])
                        - (self.group1()[0] * reverse.group2()[3])),
                    (-(self.group2()[0] * reverse.group1()[2])
                        - (self.group1()[1] * reverse.group2()[3])
                        - (self.group0()[3] * reverse.group2()[1])
                        - (self.group1()[0] * reverse.group2()[2])),
                    (-(self.group2()[1] * reverse.group1()[0])
                        - (self.group1()[2] * reverse.group2()[3])
                        - (self.group1()[1] * reverse.group2()[0])
                        - (self.group0()[3] * reverse.group2()[2])),
                    ((self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self.group2()[3]) * reverse.group0())
                + (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(reverse.group2(), 0, 1, 2, 2))
                + (swizzle!(reverse.group2(), 1, 2, 3, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[2], self.group1()[1]]))
                + (swizzle!(reverse.group2(), 3, 3, 0, 0) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[1], self.group1()[0]]))
                - (swizzle!(self.group0(), 1, 2, 0, 3) * swizzle!(reverse.group2(), 2, 0, 1, 3))
                + Simd32x4::from([
                    (-(self.group2()[2] * reverse.group0()[1]) - (self.group2()[0] * reverse.group1()[3])),
                    (-(self.group2()[1] * reverse.group1()[3]) - (self.group2()[0] * reverse.group0()[2])),
                    (-(self.group2()[2] * reverse.group1()[3]) - (self.group2()[1] * reverse.group0()[0])),
                    ((self.group2()[1] * reverse.group1()[1]) + (self.group2()[0] * reverse.group1()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group2()[3] * self.group1()[3]) + (self.group2()[2] * self.group0()[2]) + (self.group2()[1] * self.group0()[1]) + (self.group2()[0] * self.group0()[0])
                - (self.group1()[3] * self.group2()[3])
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                - f32::powi(self.group0()[3], 2)
                + (self.group0()[2] * self.group2()[2])
                + (self.group0()[0] * self.group2()[0])
                + (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       36       35        0
    //    simd4       23       25        0
    // Totals...
    // yes simd       59       60        0
    //  no simd      128      135        0
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
            (-(swizzle!(reverse.group1(), 3, 0, 1, 2) * Simd32x4::from([self.group1()[3], self.group2()[3], self.group2()[3], self.group2()[3]]))
                + (swizzle!(reverse.group1(), 2, 2, 0, 1) * Simd32x4::from([self.group1()[2], self.group2()[1], self.group2()[2], self.group2()[0]]))
                + (swizzle!(self.group1(), 1, 3, 3, 3) * Simd32x4::from([reverse.group1()[1], reverse.group2()[0], reverse.group2()[1], reverse.group2()[2]]))
                + (swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([reverse.group1()[0], reverse.group2()[2], reverse.group2()[0], reverse.group2()[1]]))
                + (swizzle!(reverse.group0(), 3, 3, 1, 2) * Simd32x4::from([self.group0()[3], self.group2()[1], self.group2()[2], self.group2()[0]]))
                + (swizzle!(self.group0(), 2, 3, 2, 3) * Simd32x4::from([reverse.group0()[2], reverse.group2()[1], reverse.group2()[3], reverse.group2()[3]]))
                - (reverse.group0() * Simd32x4::from([self.group0()[0], self.group2()[3], self.group2()[3], self.group2()[3]]))
                + (swizzle!(self.group0(), 1, 1, 1, 2) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[2], reverse.group2()[0]]))
                + Simd32x4::from([
                    0.0,
                    (-(self.group2()[2] * reverse.group1()[1])
                        - (self.group2()[2] * reverse.group0()[2])
                        - (self.group2()[0] * reverse.group1()[3])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group2()[1])
                        - (self.group1()[0] * reverse.group2()[3])
                        - (self.group0()[2] * reverse.group2()[2])
                        - (self.group0()[0] * reverse.group2()[0])),
                    (-(self.group2()[1] * reverse.group1()[3])
                        - (self.group2()[1] * reverse.group0()[0])
                        - (self.group2()[0] * reverse.group1()[2])
                        - (self.group2()[0] * reverse.group0()[3])
                        - (self.group1()[1] * reverse.group2()[3])
                        - (self.group1()[0] * reverse.group2()[2])
                        - (self.group0()[3] * reverse.group2()[0])
                        - (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group2()[2] * reverse.group1()[3])
                        - (self.group2()[2] * reverse.group0()[0])
                        - (self.group2()[1] * reverse.group1()[0])
                        - (self.group2()[1] * reverse.group0()[1])
                        - (self.group1()[2] * reverse.group2()[3])
                        - (self.group1()[1] * reverse.group2()[0])
                        - (self.group0()[0] * reverse.group2()[2])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(reverse.group0(), 1, 2, 3, 3))
                - (swizzle!(self.group1(), 2, 1, 2, 1) * Simd32x4::from([reverse.group1()[1], reverse.group0()[0], reverse.group0()[0], reverse.group0()[2]]))
                + (swizzle!(self.group1(), 1, 2, 0, 3) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group0()[0]]))
                - (swizzle!(self.group1(), 0, 0, 1, 0) * Simd32x4::from([reverse.group0()[0], reverse.group1()[2], reverse.group1()[0], reverse.group0()[1]]))
                - (swizzle!(self.group0(), 3, 2, 3, 3) * Simd32x4::from([reverse.group0()[2], reverse.group1()[3], reverse.group1()[3], reverse.group1()[2]]))
                + (swizzle!(self.group0(), 2, 3, 1, 0) * Simd32x4::from([reverse.group0()[3], reverse.group0()[1], reverse.group0()[2], reverse.group1()[3]]))
                - (swizzle!(self.group0(), 0, 0, 2, 2) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group0()[1], reverse.group1()[1]]))
                - (swizzle!(self.group0(), 1, 1, 0, 1) * Simd32x4::from([reverse.group1()[3], reverse.group0()[3], reverse.group1()[2], reverse.group1()[0]]))),
            // e4235, e4315, e4125, e3215
            ((swizzle!(reverse.group1(), 0, 1, 2, 2) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[2]]))
                + (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group0()[2], reverse.group1()[3], reverse.group1()[3], reverse.group2()[3]]))
                - (swizzle!(reverse.group0(), 3, 1, 2, 0) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[3]]))
                + (swizzle!(self.group1(), 0, 0, 1, 2) * Simd32x4::from([reverse.group1()[3], reverse.group0()[3], reverse.group0()[1], reverse.group2()[2]]))
                + (swizzle!(reverse.group1(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[3], self.group0()[1], self.group0()[2], self.group2()[1]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 3) * Simd32x4::from([self.group0()[2], self.group0()[3], self.group0()[1], self.group2()[3]]))
                + (swizzle!(reverse.group0(), 1, 0, 0, 3) * Simd32x4::from([self.group0()[0], self.group0()[2], self.group0()[3], self.group2()[2]]))
                + (swizzle!(reverse.group0(), 0, 2, 3, 2) * Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group2()[1]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group2()[0] * reverse.group1()[0])
                        + (self.group2()[0] * reverse.group0()[1])
                        + (self.group1()[1] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group2()[0])
                        - (self.group0()[3] * reverse.group2()[2])
                        - (self.group0()[2] * reverse.group2()[1])
                        - (self.group0()[0] * reverse.group2()[3])
                        - (self.group0()[1] * reverse.group2()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group1()[3], 2)
                + f32::powi(self.group1()[2], 2)
                + f32::powi(self.group1()[1], 2)
                + f32::powi(self.group1()[0], 2)
                + f32::powi(self.group0()[3], 2)
                + f32::powi(self.group0()[2], 2)
                - f32::powi(self.group0()[0], 2)
                + f32::powi(self.group0()[1], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
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
    //      f32       16       30        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       28       42        0
    //  no simd       64       78        0
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
            (-(Simd32x4::from(self.group1()[3]) * swizzle!(reverse.group0(), 3, 0, 1, 2)) + (swizzle!(self.group1(), 2, 1, 2, 0) * swizzle!(reverse.group0(), 2, 2, 0, 1))
                - (swizzle!(self.group0(), 3, 1, 2, 0) * swizzle!(reverse.group1(), 3, 2, 0, 1))
                + (swizzle!(self.group0(), 2, 3, 3, 3) * swizzle!(reverse.group1(), 2, 0, 1, 2))
                + (swizzle!(self.group0(), 0, 2, 0, 2) * reverse.group1())
                + (swizzle!(self.group0(), 1, 0, 1, 1) * swizzle!(reverse.group1(), 1, 3, 3, 0))
                + Simd32x4::from([
                    ((self.group1()[1] * reverse.group0()[1]) + (self.group1()[0] * reverse.group0()[0])),
                    (-(self.group1()[2] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[3])),
                    (-(self.group1()[1] * reverse.group0()[3]) - (self.group1()[0] * reverse.group0()[2])),
                    (-(self.group1()[2] * reverse.group0()[3]) - (self.group1()[1] * reverse.group0()[0])),
                ])),
            // e23, e31, e12, e45
            (-(Simd32x4::from(self.group1()[3]) * reverse.group0()) + (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                - (swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(reverse.group1(), 0, 1, 2, 2))
                - (swizzle!(self.group0(), 2, 0, 2, 0) * swizzle!(reverse.group1(), 1, 2, 3, 0))
                - (swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(reverse.group1(), 3, 3, 0, 1))
                + (swizzle!(self.group0(), 1, 2, 0, 3) * swizzle!(reverse.group1(), 2, 0, 1, 3))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1]) - (self.group1()[0] * reverse.group0()[3])),
                    (-(self.group1()[1] * reverse.group0()[3]) - (self.group1()[0] * reverse.group0()[2])),
                    (-(self.group1()[2] * reverse.group0()[3]) - (self.group1()[1] * reverse.group0()[0])),
                    ((self.group1()[1] * reverse.group0()[1]) + (self.group1()[0] * reverse.group0()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group1()[3] * self.group0()[3]) + (self.group1()[2] * self.group0()[2]) + (self.group1()[1] * self.group0()[1]) + (self.group1()[0] * self.group0()[0])
                - (self.group0()[3] * self.group1()[3])
                + (self.group0()[2] * self.group1()[2])
                + (self.group0()[0] * self.group1()[0])
                + (self.group0()[1] * self.group1()[1])),
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
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       44       57        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_product = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (-(swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group1(), 2, 0, 1, 1) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[1]]))
                + Simd32x4::from([
                    (-(self.group1()[3] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[3])
                        - (self.group0()[3] * reverse.group0()[0])
                        - (self.group0()[2] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group0()[3])),
                    (-(self.group1()[3] * reverse.group1()[1])
                        - (self.group1()[1] * reverse.group1()[3])
                        - (self.group0()[3] * reverse.group0()[1])
                        - (self.group0()[0] * reverse.group1()[2])
                        - (self.group0()[1] * reverse.group0()[3])),
                    (-(self.group1()[3] * reverse.group1()[2])
                        - (self.group1()[2] * reverse.group1()[3])
                        - (self.group0()[3] * reverse.group0()[2])
                        - (self.group0()[2] * reverse.group0()[3])
                        - (self.group0()[1] * reverse.group1()[0])),
                    (self.group1()[0] * reverse.group1()[0]),
                ])),
            // e23, e31, e12, e1234
            (-(swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group0()[3], reverse.group0()[3], reverse.group0()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group1(), 0, 1, 0, 3) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group1()[1], self.group0()[3]]))
                + Simd32x4::from([
                    ((self.group1()[0] * reverse.group0()[3]) * -1.0),
                    ((self.group1()[0] * reverse.group1()[2]) * -1.0),
                    ((self.group0()[3] * reverse.group1()[2]) * -1.0),
                    ((self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[1], 2) - f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[0], 2)),
        );
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e1234
            geometric_product.group1(),
        );
        return subtraction;
    }
}
impl ConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       74        0
    //    simd4       21       22        0
    // Totals...
    // yes simd       77       96        0
    //  no simd      140      162        0
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
            ((swizzle!(self.group2(), 3, 3, 3, 2) * swizzle!(reverse.group2(), 0, 1, 2, 2))
                - (swizzle!(self.group2(), 2, 1, 2, 3) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[3], reverse.group1()[3]]))
                + (swizzle!(self.group2(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group2()[1]]))
                - (swizzle!(reverse.group2(), 3, 0, 1, 3) * Simd32x4::from([self.group2()[0], self.group0()[2], self.group0()[0], self.group1()[3]]))
                - (swizzle!(reverse.group0(), 0, 2, 0, 3) * Simd32x4::from([self.group0()[3], self.group2()[0], self.group2()[1], self.group0()[3]]))
                + (swizzle!(self.group0(), 2, 0, 2, 2) * Simd32x4::from([reverse.group2()[1], reverse.group2()[2], reverse.group0()[3], reverse.group1()[2]]))
                + (swizzle!(self.group0(), 0, 1, 1, 0) * Simd32x4::from([reverse.group0()[3], reverse.group0()[3], reverse.group2()[0], reverse.group1()[0]]))
                + Simd32x4::from([
                    ((self.group0()[1] * reverse.group2()[2]) * -1.0),
                    ((self.group0()[3] * reverse.group0()[1]) * -1.0),
                    ((self.group0()[3] * reverse.group0()[2]) * -1.0),
                    ((self.group2()[0] * reverse.group2()[0])
                        + (self.group1()[2] * reverse.group0()[2])
                        + (self.group1()[1] * reverse.group0()[1])
                        + (self.group1()[0] * reverse.group0()[0])
                        + (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(reverse.group1(), 0, 1, 2, 2) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[2]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 3) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[3]]))
                + (swizzle!(self.group2(), 1, 2, 0, 3) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(reverse.group0(), 2, 0, 1, 2))
                - (swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group1()[0]]))
                - (swizzle!(self.group0(), 2, 0, 2, 1) * swizzle!(reverse.group1(), 1, 2, 3, 1))
                + Simd32x4::from([
                    (-(self.group2()[0] * reverse.group0()[3])
                        - (self.group1()[3] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group2()[3])
                        - (self.group0()[0] * reverse.group1()[3])
                        + (self.group0()[1] * reverse.group1()[2])),
                    (-(self.group2()[1] * reverse.group0()[3])
                        - (self.group1()[3] * reverse.group0()[1])
                        - (self.group1()[1] * reverse.group2()[3])
                        - (self.group1()[0] * reverse.group0()[2])
                        + (self.group0()[2] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group2()[2] * reverse.group0()[3])
                        - (self.group1()[3] * reverse.group0()[2])
                        - (self.group1()[2] * reverse.group2()[3])
                        - (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group1()[1])
                        - (self.group0()[1] * reverse.group1()[0])),
                    ((self.group1()[1] * reverse.group0()[1]) + (self.group1()[0] * reverse.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            ((swizzle!(self.group2(), 2, 1, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group1()[3], reverse.group1()[3], reverse.group0()[3]]))
                + (swizzle!(self.group2(), 0, 0, 1, 2) * Simd32x4::from([reverse.group1()[3], reverse.group1()[2], reverse.group1()[0], reverse.group0()[2]]))
                - (reverse.group2() * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[3]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[1] * reverse.group1()[2]) + (self.group1()[1] * reverse.group2()[2]) + (self.group0()[3] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group0()[3])),
                    (-(self.group2()[2] * reverse.group1()[0]) + (self.group1()[2] * reverse.group2()[0]) - (self.group1()[1] * reverse.group0()[3])
                        + (self.group0()[3] * reverse.group1()[1])),
                    (-(self.group2()[0] * reverse.group1()[1]) - (self.group1()[2] * reverse.group0()[3])
                        + (self.group0()[3] * reverse.group1()[2])
                        + (self.group1()[0] * reverse.group2()[1])),
                    ((self.group2()[1] * reverse.group0()[1]) + (self.group2()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((reverse.group1() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[3]])) - (Simd32x4::from(self.group1()[3]) * reverse.group0())
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group2()[2]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group2()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * reverse.group0()[1]) - (self.group1()[0] * reverse.group2()[3])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group1()[3])),
                    (-(self.group1()[1] * reverse.group2()[3]) - (self.group1()[0] * reverse.group0()[2])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[1] * reverse.group1()[3])),
                    (-(self.group1()[2] * reverse.group2()[3]) - (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[3])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group2()[1] * reverse.group1()[1]) - (self.group2()[0] * reverse.group1()[0])
                        + (self.group1()[1] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group2()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-(self.group2()[3] * self.group1()[3]) + f32::powi(self.group2()[2], 2) + f32::powi(self.group2()[1], 2) + f32::powi(self.group2()[0], 2)
                - (self.group1()[3] * self.group2()[3])
                + (self.group1()[2] * self.group0()[2])
                + (self.group1()[1] * self.group0()[1])
                + (self.group1()[0] * self.group0()[0])
                - f32::powi(self.group0()[3], 2)
                + (self.group0()[2] * self.group1()[2])
                + (self.group0()[0] * self.group1()[0])
                + (self.group0()[1] * self.group1()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       88      102        0
    //    simd4       42       43        0
    // Totals...
    // yes simd      130      145        0
    //  no simd      256      274        0
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
            (-(swizzle!(self.group3(), 2, 1, 2, 2) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[3], reverse.group3()[2]]))
                + (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group2()[3]]))
                - (swizzle!(self.group3(), 0, 0, 1, 1) * Simd32x4::from([reverse.group2()[3], reverse.group0()[2], reverse.group0()[0], reverse.group3()[1]]))
                + (Simd32x4::from(self.group2()[3]) * reverse.group3())
                + (reverse.group1() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[3]]))
                - (swizzle!(reverse.group0(), 0, 1, 2, 2) * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group2()[2]]))
                + (swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[3]]))
                - (swizzle!(reverse.group0(), 2, 0, 1, 1) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[1]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group1()[1], reverse.group2()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0, 0) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group3()[1], reverse.group2()[0]]))
                + Simd32x4::from([
                    ((self.group1()[0] * reverse.group2()[3])
                        + (self.group0()[3] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group3()[1])
                        + (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group0()[3])
                        + (self.group0()[0] * reverse.group1()[3])),
                    ((self.group1()[1] * reverse.group2()[3])
                        + (self.group0()[3] * reverse.group0()[1])
                        + (self.group0()[1] * reverse.group1()[3])
                        + (self.group0()[1] * reverse.group0()[3])
                        + (self.group0()[0] * reverse.group1()[2])
                        + (self.group0()[0] * reverse.group3()[2])),
                    ((self.group1()[2] * reverse.group2()[3])
                        + (self.group0()[3] * reverse.group0()[2])
                        + (self.group0()[2] * reverse.group1()[3])
                        + (self.group0()[2] * reverse.group0()[3])
                        + (self.group0()[1] * reverse.group3()[0])
                        + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group3()[0] * reverse.group3()[0])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group1()[2])
                        - (self.group1()[1] * reverse.group1()[1])
                        - (self.group1()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group3()[3]) * Simd32x4::from([reverse.group0()[0], reverse.group0()[1], reverse.group0()[2], reverse.group2()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group1()[2]]))
                + (Simd32x4::from(reverse.group1()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[3]]))
                + (swizzle!(reverse.group2(), 0, 1, 2, 2) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[2]]))
                + (swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[3]]))
                - (swizzle!(self.group2(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group3()[3]]))
                + (swizzle!(reverse.group2(), 3, 3, 3, 0) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 1) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group3()[1]]))
                + (swizzle!(self.group0(), 3, 3, 3, 1) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[1]]))
                + Simd32x4::from([
                    ((self.group3()[2] * reverse.group3()[1])
                        + (self.group1()[3] * reverse.group3()[0])
                        + (self.group1()[2] * reverse.group1()[1])
                        + (self.group1()[0] * reverse.group0()[3])
                        + (self.group0()[2] * reverse.group2()[1])
                        + (self.group0()[0] * reverse.group3()[3])
                        - (self.group0()[1] * reverse.group2()[2])),
                    ((self.group3()[0] * reverse.group3()[2])
                        + (self.group1()[3] * reverse.group3()[1])
                        + (self.group1()[1] * reverse.group0()[3])
                        + (self.group1()[0] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group2()[0])
                        + (self.group0()[0] * reverse.group2()[2])
                        + (self.group0()[1] * reverse.group3()[3])),
                    ((self.group3()[1] * reverse.group3()[0])
                        + (self.group1()[3] * reverse.group3()[2])
                        + (self.group1()[2] * reverse.group0()[3])
                        + (self.group1()[1] * reverse.group1()[0])
                        + (self.group0()[2] * reverse.group3()[3])
                        - (self.group0()[0] * reverse.group2()[1])
                        + (self.group0()[1] * reverse.group2()[0])),
                    (-(self.group3()[0] * reverse.group1()[0])
                        - (self.group2()[2] * reverse.group0()[2])
                        - (self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group1()[2] * reverse.group3()[2])
                        - (self.group1()[1] * reverse.group3()[1])
                        - (self.group1()[0] * reverse.group3()[0])),
                ])),
            // e15, e25, e35, e1234
            (-(swizzle!(reverse.group3(), 0, 1, 2, 2) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[2]]))
                + (swizzle!(self.group3(), 3, 3, 3, 2) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group0()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2, 1) * Simd32x4::from([reverse.group2()[1], reverse.group3()[3], reverse.group3()[3], reverse.group0()[1]]))
                - (swizzle!(reverse.group2(), 2, 0, 1, 3) * Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group1()[3]]))
                + (swizzle!(self.group3(), 0, 0, 1, 0) * Simd32x4::from([reverse.group3()[3], reverse.group2()[2], reverse.group2()[0], reverse.group0()[0]]))
                - (swizzle!(reverse.group3(), 1, 2, 0, 1) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group0()[1]]))
                + (swizzle!(self.group2(), 2, 2, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group3()[0], reverse.group0()[3], reverse.group1()[3]]))
                + (swizzle!(self.group2(), 1, 1, 1, 3) * Simd32x4::from([reverse.group3()[2], reverse.group0()[3], reverse.group1()[0], reverse.group0()[3]]))
                - (swizzle!(reverse.group1(), 2, 0, 3, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[2], self.group0()[2]]))
                - (swizzle!(reverse.group1(), 3, 3, 1, 1) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[0], self.group0()[1]]))
                + (reverse.group2() * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[2]]))
                + Simd32x4::from([
                    ((self.group2()[0] * reverse.group0()[3])
                        + (self.group1()[2] * reverse.group2()[1])
                        + (self.group0()[3] * reverse.group2()[0])
                        + (self.group1()[0] * reverse.group3()[3])),
                    ((self.group2()[0] * reverse.group1()[2])
                        + (self.group1()[1] * reverse.group3()[3])
                        + (self.group0()[3] * reverse.group2()[1])
                        + (self.group1()[0] * reverse.group2()[2])),
                    ((self.group2()[0] * reverse.group3()[1])
                        + (self.group1()[2] * reverse.group3()[3])
                        + (self.group1()[1] * reverse.group2()[0])
                        + (self.group0()[3] * reverse.group2()[2])),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[0] * reverse.group3()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(self.group3()[3]) * reverse.group0())
                - (swizzle!(self.group3(), 1, 2, 0, 3) * swizzle!(reverse.group1(), 2, 0, 1, 3))
                - (swizzle!(reverse.group2(), 0, 1, 2, 2) * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group3()[2]]))
                + (swizzle!(self.group2(), 2, 1, 2, 2) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[3], reverse.group3()[2]]))
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[2]]))
                + (swizzle!(self.group2(), 0, 0, 1, 1) * Simd32x4::from([reverse.group2()[3], reverse.group0()[2], reverse.group0()[0], reverse.group3()[1]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group3()[3]]))
                + (swizzle!(reverse.group3(), 1, 2, 0, 0) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group2()[2]]))
                + (Simd32x4::from(self.group0()[3]) * reverse.group3())
                - (swizzle!(reverse.group2(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[1]]))
                + Simd32x4::from([
                    ((self.group3()[2] * reverse.group1()[1]) + (self.group3()[0] * reverse.group0()[3]) + (self.group1()[0] * reverse.group1()[3])
                        - (self.group0()[0] * reverse.group3()[3])
                        + (self.group0()[1] * reverse.group2()[2])),
                    ((self.group3()[1] * reverse.group0()[3])
                        + (self.group3()[0] * reverse.group1()[2])
                        + (self.group1()[1] * reverse.group1()[3])
                        + (self.group0()[2] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group3()[3])),
                    ((self.group3()[2] * reverse.group0()[3]) + (self.group3()[1] * reverse.group1()[0]) + (self.group1()[2] * reverse.group1()[3])
                        - (self.group0()[2] * reverse.group3()[3])
                        + (self.group0()[0] * reverse.group2()[1])),
                    (-(self.group3()[0] * reverse.group2()[0])
                        - (self.group2()[1] * reverse.group1()[1])
                        - (self.group2()[0] * reverse.group1()[0])
                        - (self.group1()[1] * reverse.group2()[1])
                        - (self.group1()[0] * reverse.group2()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group3()[3] * self.group2()[3]) - f32::powi(self.group3()[2], 2) - f32::powi(self.group3()[1], 2) - f32::powi(self.group3()[0], 2)
                + (self.group2()[3] * self.group3()[3])
                - (self.group2()[2] * self.group0()[2])
                - (self.group2()[1] * self.group0()[1])
                - (self.group2()[0] * self.group0()[0])
                + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                + f32::powi(self.group0()[3], 2)
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
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
    //      f32       53       59        0
    //    simd4       18       19        0
    // Totals...
    // yes simd       71       78        0
    //  no simd      125      135        0
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
            (-(swizzle!(self.group2(), 2, 3, 3, 3) * swizzle!(reverse.group2(), 2, 0, 1, 2))
                - (swizzle!(self.group2(), 1, 1, 2, 0) * Simd32x4::from([reverse.group2()[1], reverse.group0()[3], reverse.group0()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group2(), 0, 1, 2, 0) * Simd32x4::from([self.group2()[0], self.group0()[3], self.group0()[1], self.group0()[2]]))
                + (swizzle!(reverse.group1(), 3, 0, 1, 2) * Simd32x4::from([self.group1()[3], self.group2()[3], self.group2()[3], self.group2()[3]]))
                - (swizzle!(self.group1(), 2, 1, 2, 0) * Simd32x4::from([reverse.group1()[2], reverse.group0()[3], reverse.group0()[1], reverse.group0()[2]]))
                - (swizzle!(reverse.group1(), 1, 2, 0, 3) * Simd32x4::from([self.group1()[1], self.group0()[2], self.group0()[3], self.group0()[3]]))
                + (swizzle!(reverse.group0(), 0, 2, 3, 1) * Simd32x4::from([self.group0()[0], self.group2()[2], self.group2()[0], self.group2()[1]]))
                - (swizzle!(reverse.group1(), 0, 3, 3, 1) * Simd32x4::from([self.group1()[0], self.group0()[1], self.group0()[2], self.group0()[1]]))
                + Simd32x4::from([
                    0.0,
                    ((self.group2()[0] * reverse.group2()[3])
                        + (self.group1()[3] * reverse.group0()[1])
                        + (self.group1()[2] * reverse.group0()[2])
                        + (self.group1()[0] * reverse.group2()[3])
                        + (self.group0()[3] * reverse.group1()[1])
                        + (self.group0()[2] * reverse.group2()[2])
                        + (self.group0()[0] * reverse.group0()[1])
                        + (self.group0()[1] * reverse.group0()[0])),
                    ((self.group2()[1] * reverse.group2()[3])
                        + (self.group1()[3] * reverse.group0()[2])
                        + (self.group1()[1] * reverse.group2()[3])
                        + (self.group1()[0] * reverse.group0()[3])
                        + (self.group0()[3] * reverse.group2()[0])
                        + (self.group0()[2] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group0()[2])
                        + (self.group0()[1] * reverse.group1()[2])),
                    ((self.group2()[2] * reverse.group2()[3])
                        + (self.group1()[3] * reverse.group0()[3])
                        + (self.group1()[2] * reverse.group2()[3])
                        + (self.group1()[1] * reverse.group0()[1])
                        + (self.group0()[3] * reverse.group0()[0])
                        + (self.group0()[2] * reverse.group1()[0])
                        + (self.group0()[1] * reverse.group2()[1])
                        + (self.group0()[0] * reverse.group0()[3])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group1()[2]]))
                + (Simd32x4::from(reverse.group1()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group0()[0]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 1) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[1]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group2()[1])
                        + (self.group1()[2] * reverse.group1()[1])
                        + (self.group0()[0] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group0()[0])),
                    ((self.group2()[0] * reverse.group2()[2])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group0()[0] * reverse.group1()[1])
                        + (self.group1()[0] * reverse.group1()[2])),
                    ((self.group2()[1] * reverse.group2()[0])
                        + (self.group1()[2] * reverse.group0()[0])
                        + (self.group1()[1] * reverse.group1()[0])
                        + (self.group0()[0] * reverse.group1()[2])),
                    (-(self.group2()[0] * reverse.group1()[0])
                        - (self.group1()[2] * reverse.group2()[2])
                        - (self.group1()[1] * reverse.group2()[1])
                        - (self.group1()[0] * reverse.group2()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(self.group2(), 2, 1, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group0()[0], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group2(), 1, 2, 0, 3) * swizzle!(reverse.group1(), 2, 0, 1, 3))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[3]]))
                + (swizzle!(reverse.group2(), 1, 1, 0, 2) * Simd32x4::from([self.group1()[2], self.group0()[0], self.group1()[1], self.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[3]]))
                + (swizzle!(reverse.group2(), 0, 2, 2, 1) * Simd32x4::from([self.group0()[0], self.group1()[0], self.group0()[0], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group2()[0] * reverse.group0()[0]) + (self.group1()[0] * reverse.group1()[3])),
                    ((self.group2()[0] * reverse.group1()[2]) + (self.group1()[1] * reverse.group1()[3])),
                    ((self.group2()[1] * reverse.group1()[0]) + (self.group1()[2] * reverse.group1()[3])),
                    (-(self.group2()[2] * reverse.group0()[3])
                        - (self.group2()[1] * reverse.group0()[2])
                        - (self.group2()[0] * reverse.group0()[1])
                        - (self.group1()[1] * reverse.group0()[2])
                        - (self.group1()[0] * reverse.group0()[1])
                        - (self.group0()[3] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group1()[1])
                        + (self.group0()[1] * reverse.group2()[0])
                        + (self.group0()[0] * reverse.group2()[3])
                        - (self.group0()[1] * reverse.group1()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            (-f32::powi(self.group2()[2], 2) - f32::powi(self.group2()[1], 2) - f32::powi(self.group2()[0], 2) + f32::powi(self.group1()[3], 2)
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                + f32::powi(self.group0()[0], 2)
                - f32::powi(self.group1()[0], 2)),
        );
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (geometric_product.group0()[0] - scalar_product[scalar]),
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                geometric_product.group0()[3],
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
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
    //      f32       60       81        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       80      101        0
    //  no simd      140      161        0
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
            ((Simd32x4::from(self.group2()[3]) * reverse.group1())
                + (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group0()[1], reverse.group2()[3], reverse.group2()[3], reverse.group2()[3]]))
                - (swizzle!(reverse.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group2()[2]]))
                + (swizzle!(reverse.group0(), 0, 2, 0, 3) * Simd32x4::from([self.group0()[3], self.group1()[0], self.group1()[1], self.group0()[3]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * reverse.group2()[3]) + (self.group0()[2] * reverse.group1()[1]) + (self.group0()[0] * reverse.group0()[3])),
                    ((self.group0()[3] * reverse.group0()[1]) + (self.group0()[0] * reverse.group1()[2]) + (self.group0()[1] * reverse.group0()[3])),
                    ((self.group0()[3] * reverse.group0()[2]) + (self.group0()[2] * reverse.group0()[3]) + (self.group0()[1] * reverse.group1()[0])),
                    (-(self.group2()[1] * reverse.group0()[1])
                        - (self.group2()[0] * reverse.group0()[0])
                        - (self.group1()[1] * reverse.group1()[1])
                        - (self.group1()[0] * reverse.group1()[0])
                        - (self.group0()[2] * reverse.group2()[2])
                        - (self.group0()[0] * reverse.group2()[0])
                        - (self.group0()[1] * reverse.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((reverse.group2() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[3]]))
                - (swizzle!(self.group2(), 1, 2, 0, 3) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[3]]))
                + (swizzle!(reverse.group2(), 3, 3, 3, 2) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[2]]))
                + (swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2], reverse.group2()[0]]))
                + (swizzle!(self.group0(), 2, 0, 2, 1) * Simd32x4::from([reverse.group2()[1], reverse.group2()[2], reverse.group1()[3], reverse.group2()[1]]))
                + Simd32x4::from([
                    ((self.group2()[2] * reverse.group0()[1]) + (self.group1()[3] * reverse.group0()[0]) + (self.group1()[2] * reverse.group1()[1])
                        - (self.group1()[1] * reverse.group1()[2])
                        + (self.group1()[0] * reverse.group0()[3])
                        + (self.group0()[0] * reverse.group1()[3])
                        - (self.group0()[1] * reverse.group2()[2])),
                    ((self.group2()[0] * reverse.group0()[2]) + (self.group1()[3] * reverse.group0()[1]) - (self.group1()[2] * reverse.group1()[0])
                        + (self.group1()[1] * reverse.group0()[3])
                        + (self.group1()[0] * reverse.group1()[2])
                        - (self.group0()[2] * reverse.group2()[0])
                        + (self.group0()[1] * reverse.group1()[3])),
                    ((self.group2()[1] * reverse.group0()[0])
                        + (self.group1()[3] * reverse.group0()[2])
                        + (self.group1()[2] * reverse.group0()[3])
                        + (self.group1()[1] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[1])
                        - (self.group0()[0] * reverse.group2()[1])
                        + (self.group0()[1] * reverse.group2()[0])),
                    (-(self.group2()[2] * reverse.group0()[2]) - (self.group2()[1] * reverse.group0()[1]) - (self.group2()[0] * reverse.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            ((swizzle!(self.group2(), 2, 1, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group0()[3], reverse.group0()[3], reverse.group0()[3]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group2()[1], self.group2()[2], self.group2()[0], self.group0()[2]]))
                + (swizzle!(reverse.group2(), 1, 1, 0, 3) * Simd32x4::from([self.group1()[2], self.group0()[3], self.group1()[1], self.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group0()[2]]))
                + Simd32x4::from([
                    ((self.group2()[0] * reverse.group0()[3])
                        + (self.group1()[3] * reverse.group1()[0])
                        + (self.group0()[3] * reverse.group2()[0])
                        + (self.group1()[0] * reverse.group1()[3])),
                    ((self.group2()[0] * reverse.group1()[2])
                        + (self.group1()[3] * reverse.group1()[1])
                        + (self.group1()[1] * reverse.group1()[3])
                        + (self.group1()[0] * reverse.group2()[2])),
                    ((self.group2()[1] * reverse.group1()[0])
                        + (self.group1()[3] * reverse.group1()[2])
                        + (self.group1()[2] * reverse.group1()[3])
                        + (self.group0()[3] * reverse.group2()[2])),
                    (-(self.group1()[1] * reverse.group0()[1])
                        - (self.group1()[0] * reverse.group0()[0])
                        - (self.group0()[0] * reverse.group1()[0])
                        - (self.group0()[1] * reverse.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 3, 3, 3, 2) * Simd32x4::from([reverse.group2()[0], reverse.group2()[1], reverse.group2()[2], reverse.group1()[2]]))
                + (swizzle!(reverse.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[3]]))
                - (swizzle!(self.group2(), 1, 2, 0, 1) * Simd32x4::from([reverse.group0()[2], reverse.group0()[0], reverse.group0()[1], reverse.group1()[1]]))
                - (swizzle!(reverse.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                - (swizzle!(reverse.group1(), 3, 3, 3, 0) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]))
                + (swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([reverse.group2()[2], reverse.group2()[0], reverse.group2()[1], reverse.group1()[3]]))
                + Simd32x4::from([
                    ((self.group2()[0] * reverse.group2()[3]) + (self.group1()[3] * reverse.group0()[0])),
                    ((self.group2()[1] * reverse.group2()[3]) + (self.group1()[3] * reverse.group0()[1])),
                    ((self.group2()[2] * reverse.group2()[3]) + (self.group1()[3] * reverse.group0()[2])),
                    (-(self.group1()[1] * reverse.group2()[1]) - (self.group1()[0] * reverse.group2()[0])),
                ])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self.group2()[3] * self.group1()[3]) - (self.group2()[2] * self.group0()[2]) - (self.group2()[1] * self.group0()[1]) - (self.group2()[0] * self.group0()[0])
                + (self.group1()[3] * self.group2()[3])
                - f32::powi(self.group1()[2], 2)
                - f32::powi(self.group1()[1], 2)
                - f32::powi(self.group1()[0], 2)
                + f32::powi(self.group0()[3], 2)
                - (self.group0()[2] * self.group2()[2])
                - (self.group0()[0] * self.group2()[0])
                - (self.group0()[1] * self.group2()[1])),
        );
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                geometric_product.group0()[0],
                geometric_product.group0()[1],
                geometric_product.group0()[2],
                (geometric_product.group0()[3] - scalar_product[scalar]),
            ]),
            // e23, e31, e12, e45
            geometric_product.group1(),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
        return subtraction;
    }
}
