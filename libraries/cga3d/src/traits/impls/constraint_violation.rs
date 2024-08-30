// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 33
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        10      12       0
//  Average:        46      53       0
//  Maximum:       567     585       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        16      21       0
//  Average:        82      92       0
//  Maximum:      1024    1060       0
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
impl ConstraintViolation for AntiDualNum321 {
    type Output = AntiDualNum321;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd2        1        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        4        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDualNum321::from_groups(/* e45, scalar */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_product = AntiDualNum321::from_groups(
            // e45, scalar
            ((Simd32x2::from(self.group0()[0]) * swizzle!(reverse.group0(), 1, 0)) + (Simd32x2::from(self.group0()[1]) * reverse.group0())),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)));
        let subtraction = AntiDualNum321::from_groups(
            // e45, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for AntiDualNum4 {
    type Output = AntiDualNum4;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiDualNum4::from_groups(/* e1234, scalar */ Simd32x2::from([
            ((self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            f32::powi(self.group0()[1], 2),
        ]));
        let subtraction = AntiDualNum4::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product.group0()[0], 0.0]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiDualNum5 {
    type Output = AntiDualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiDualNum5::from_groups(/* e3215, scalar */ Simd32x2::from([
            ((self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
            f32::powi(self.group0()[1], 2),
        ]));
        let subtraction = AntiDualNum5::from_groups(/* e3215, scalar */ Simd32x2::from([geometric_product.group0()[0], 0.0]));
        return subtraction;
    }
}
impl ConstraintViolation for AntiFlatPoint {
    type Output = AntiMotor;
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
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * reverse.group0()[3] * -1.0)]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-(self.group0()[0] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[0])),
                (-(self.group0()[1] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[1])),
                (-(self.group0()[2] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[2])),
                0.0,
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ (f32::powi(self.group0()[3], 2) * -1.0));
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2], 0.0]),
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
impl ConstraintViolation for AntiPlane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self.group0()[1] * self.group0()[2]) - (self.group0()[2] * self.group0()[1])),
                (-(self.group0()[0] * self.group0()[2]) + (self.group0()[2] * self.group0()[0])),
                ((self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
                (f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                ((self.group0()[0] * self.group0()[3]) - (self.group0()[3] * self.group0()[0])),
                ((self.group0()[1] * self.group0()[3]) - (self.group0()[3] * self.group0()[1])),
                ((self.group0()[2] * self.group0()[3]) - (self.group0()[3] * self.group0()[2])),
                0.0,
            ]),
        );
        let subtraction = AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
            // e15, e25, e35
            Simd32x3::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2]]),
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
impl ConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for AntiTripleNum {
    type Output = AntiQuadNum;
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
        let subtraction = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2], 0.0]),
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
impl ConstraintViolation for DualNum321 {
    type Output = AntiDualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DualNum321::from_groups(/* e321, e12345 */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
        let geometric_product = AntiDualNum321::from_groups(/* e45, scalar */ Simd32x2::from([
            ((self.group0()[0] * reverse.group0()[1]) + (self.group0()[1] * reverse.group0()[0])),
            (-(self.group0()[0] * reverse.group0()[0]) - (self.group0()[1] * reverse.group0()[1])),
        ]));
        let scalar_product = Scalar::from_groups(/* scalar */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)));
        let subtraction = AntiDualNum321::from_groups(
            // e45, scalar
            Simd32x2::from([geometric_product.group0()[0], (geometric_product.group0()[1] - scalar_product[scalar])]),
        );
        return subtraction;
    }
}
impl ConstraintViolation for DualNum4 {
    type Output = AntiDualNum4;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiDualNum4::from_groups(/* e1234, scalar */ Simd32x2::from([
            (-(self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            (f32::powi(self.group0()[1], 2) * -1.0),
        ]));
        let subtraction = AntiDualNum4::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product.group0()[0], 0.0]));
        return subtraction;
    }
}
impl ConstraintViolation for DualNum5 {
    type Output = AntiDualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = AntiDualNum5::from_groups(/* e3215, scalar */ Simd32x2::from([
            (-(self.group0()[0] * self.group0()[1]) - (self.group0()[1] * self.group0()[0])),
            (f32::powi(self.group0()[1], 2) * -1.0),
        ]));
        let subtraction = AntiDualNum5::from_groups(/* e3215, scalar */ Simd32x2::from([geometric_product.group0()[0], 0.0]));
        return subtraction;
    }
}
impl ConstraintViolation for FlatPoint {
    type Output = AntiMotor;
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
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * reverse.group0()[3])]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-(self.group0()[0] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[0])),
                (-(self.group0()[1] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[1])),
                (-(self.group0()[2] * reverse.group0()[3]) + (self.group0()[3] * reverse.group0()[2])),
                0.0,
            ]),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[3], 2));
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_product.group0()[3] - scalar_product[scalar])]),
            // e15, e25, e35, e3215
            Simd32x4::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2], 0.0]),
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
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      366      376        0
    //    simd2       16       16        0
    //    simd3      114      120        0
    //    simd4       71       73        0
    // Totals...
    // yes simd      567      585        0
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
            ((Simd32x2::from(self[e45]) * Simd32x2::from([reverse.group9()[3], reverse[e1]]))
                + (Simd32x2::from(self.group9()[3]) * Simd32x2::from([reverse[e45], reverse.group1()[3]]))
                - (Simd32x2::from(self.group6()[3]) * Simd32x2::from([reverse.group6()[3], reverse.group3()[3]]))
                - (Simd32x2::from(reverse.group5()[2]) * Simd32x2::from([self.group5()[2], self.group6()[2]]))
                - (Simd32x2::from(reverse.group5()[1]) * Simd32x2::from([self.group5()[1], self.group6()[1]]))
                - (Simd32x2::from(reverse.group5()[0]) * Simd32x2::from([self.group5()[0], self.group6()[0]]))
                - (Simd32x2::from(reverse.group3()[2]) * Simd32x2::from([self.group4()[2], self.group7()[2]]))
                - (Simd32x2::from(reverse.group3()[1]) * Simd32x2::from([self.group4()[1], self.group7()[1]]))
                - (Simd32x2::from(reverse.group3()[0]) * Simd32x2::from([self.group4()[0], self.group7()[0]]))
                - (Simd32x2::from(reverse.group4()[2]) * Simd32x2::from([self.group3()[2], self.group8()[2]]))
                - (Simd32x2::from(reverse.group4()[1]) * Simd32x2::from([self.group3()[1], self.group8()[1]]))
                - (Simd32x2::from(reverse.group4()[0]) * Simd32x2::from([self.group3()[0], self.group8()[0]]))
                + (Simd32x2::from(reverse.group1()[2]) * Simd32x2::from([self.group1()[2], self.group9()[2]]))
                + (Simd32x2::from(reverse.group1()[1]) * Simd32x2::from([self.group1()[1], self.group9()[1]]))
                + (Simd32x2::from(reverse.group1()[0]) * Simd32x2::from([self.group1()[0], self.group9()[0]]))
                + (Simd32x2::from(self.group0()[0]) * reverse.group0())
                + Simd32x2::from([
                    (-(self.group9()[2] * reverse.group9()[2]) - (self.group9()[1] * reverse.group9()[1]) - (self.group9()[0] * reverse.group9()[0])
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
                        - (self.group4()[2] * reverse.group8()[2])
                        - (self.group4()[1] * reverse.group8()[1])
                        - (self.group4()[0] * reverse.group8()[0])
                        - (self.group3()[3] * reverse.group6()[3])
                        - (self.group3()[2] * reverse.group7()[2])
                        - (self.group3()[1] * reverse.group7()[1])
                        - (self.group3()[0] * reverse.group7()[0])
                        + (self[e1] * reverse[e45])
                        + (self.group1()[3] * reverse.group9()[3])
                        + (self.group1()[2] * reverse.group9()[2])
                        + (self.group1()[1] * reverse.group9()[1])
                        + (self.group1()[0] * reverse.group9()[0])
                        + (self.group0()[1] * reverse.group0()[0])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e45]) * Simd32x4::from([reverse.group8()[0], reverse.group8()[1], reverse.group8()[2], reverse.group0()[1]]))
                - (swizzle!(reverse.group6(), 1, 2, 0, 3) * Simd32x4::from([self.group9()[2], self.group9()[0], self.group9()[1], self[e45]]))
                + (swizzle!(self.group9(), 1, 2, 0, 2) * Simd32x4::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1], reverse.group7()[2]]))
                + (swizzle!(reverse.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group1()[3]]))
                + (Simd32x4::from(self.group6()[3]) * Simd32x4::from([reverse.group5()[0], reverse.group5()[1], reverse.group5()[2], reverse[e45]]))
                - (swizzle!(self.group6(), 2, 1, 2, 2) * Simd32x4::from([reverse.group9()[1], reverse.group3()[3], reverse.group3()[3], reverse.group4()[2]]))
                - (swizzle!(self.group6(), 0, 0, 1, 1) * Simd32x4::from([reverse.group3()[3], reverse.group9()[2], reverse.group9()[0], reverse.group4()[1]]))
                + (swizzle!(reverse.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group4()[2]]))
                - (swizzle!(reverse.group1(), 2, 0, 1, 3) * Simd32x4::from([self.group5()[1], self.group5()[2], self.group5()[0], self.group3()[3]]))
                - (swizzle!(reverse.group6(), 0, 1, 2, 2) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group4()[2]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2], reverse.group0()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group5()[2], reverse.group5()[0], reverse.group5()[1], reverse.group4()[2]]))
                + (swizzle!(reverse.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group4()[1]]))
                - (swizzle!(reverse.group9(), 0, 1, 2, 2) * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group7()[2]]))
                + Simd32x4::from([
                    (-(self.group9()[3] * reverse.group7()[0]) - (self.group9()[0] * reverse.group0()[1]) - (self.group8()[2] * reverse.group4()[1])
                        + (self.group8()[1] * reverse.group4()[2])
                        - (self.group8()[0] * reverse[e45])
                        - (self.group7()[1] * reverse.group3()[2])
                        + (self.group7()[0] * reverse.group9()[3])
                        + (self.group6()[1] * reverse.group9()[2])
                        + (self.group5()[0] * reverse.group6()[3])
                        + (self.group4()[2] * reverse.group8()[1])
                        - (self.group4()[1] * reverse.group8()[2])
                        + (self.group4()[0] * reverse[e1])
                        - (self.group3()[2] * reverse.group7()[1])
                        + (self.group3()[1] * reverse.group7()[2])
                        - (self.group3()[0] * reverse.group1()[3])
                        - (self[e1] * reverse.group4()[0])
                        + (self.group1()[2] * reverse.group5()[1])
                        + (self.group1()[0] * reverse.group0()[0])),
                    (-(self.group9()[3] * reverse.group7()[1]) - (self.group9()[1] * reverse.group0()[1]) + (self.group8()[2] * reverse.group4()[0])
                        - (self.group8()[1] * reverse[e45])
                        - (self.group8()[0] * reverse.group4()[2])
                        - (self.group7()[2] * reverse.group3()[0])
                        + (self.group7()[1] * reverse.group9()[3])
                        + (self.group6()[2] * reverse.group9()[0])
                        + (self.group5()[1] * reverse.group6()[3])
                        - (self.group4()[2] * reverse.group8()[0])
                        + (self.group4()[1] * reverse[e1])
                        + (self.group4()[0] * reverse.group8()[2])
                        + (self.group3()[2] * reverse.group7()[0])
                        - (self.group3()[1] * reverse.group1()[3])
                        - (self.group3()[0] * reverse.group7()[2])
                        - (self[e1] * reverse.group4()[1])
                        + (self.group1()[1] * reverse.group0()[0])
                        + (self.group1()[0] * reverse.group5()[2])),
                    (-(self.group9()[3] * reverse.group7()[2])
                        - (self.group9()[2] * reverse.group0()[1])
                        - (self.group8()[2] * reverse[e45])
                        - (self.group8()[1] * reverse.group4()[0])
                        + (self.group8()[0] * reverse.group4()[1])
                        + (self.group7()[2] * reverse.group9()[3])
                        - (self.group7()[0] * reverse.group3()[1])
                        + (self.group6()[0] * reverse.group9()[1])
                        + (self.group5()[2] * reverse.group6()[3])
                        + (self.group4()[2] * reverse[e1])
                        + (self.group4()[1] * reverse.group8()[0])
                        - (self.group4()[0] * reverse.group8()[1])
                        - (self.group3()[2] * reverse.group1()[3])
                        - (self.group3()[1] * reverse.group7()[0])
                        + (self.group3()[0] * reverse.group7()[1])
                        - (self[e1] * reverse.group4()[2])
                        + (self.group1()[2] * reverse.group0()[0])
                        + (self.group1()[1] * reverse.group5()[0])),
                    ((self.group9()[1] * reverse.group7()[1]) + (self.group9()[0] * reverse.group7()[0])
                        - (self.group7()[2] * reverse.group5()[2])
                        - (self.group7()[1] * reverse.group9()[1])
                        - (self.group7()[1] * reverse.group5()[1])
                        - (self.group7()[0] * reverse.group9()[0])
                        - (self.group7()[0] * reverse.group5()[0])
                        - (self.group6()[0] * reverse.group4()[0])
                        - (self.group5()[2] * reverse.group7()[2])
                        - (self.group5()[1] * reverse.group7()[1])
                        - (self.group5()[0] * reverse.group7()[0])
                        - (self.group4()[1] * reverse.group6()[1])
                        - (self.group4()[0] * reverse.group6()[0])
                        + (self.group4()[0] * reverse.group1()[0])
                        - (self.group1()[1] * reverse.group4()[1])
                        - (self.group1()[0] * reverse.group4()[0])
                        + (self.group0()[0] * reverse.group1()[3])
                        + (self.group0()[1] * reverse[e45])),
                ])),
            // e5
            ((self.group9()[3] * reverse.group6()[3]) + (self.group9()[3] * reverse.group0()[1])
                - (self.group9()[2] * reverse.group8()[2])
                - (self.group9()[1] * reverse.group8()[1])
                - (self.group9()[0] * reverse.group8()[0])
                + (self.group8()[2] * reverse.group9()[2])
                - (self.group8()[2] * reverse.group5()[2])
                + (self.group8()[1] * reverse.group9()[1])
                - (self.group8()[1] * reverse.group5()[1])
                + (self.group8()[0] * reverse.group9()[0])
                - (self.group8()[0] * reverse.group5()[0])
                - (self.group6()[3] * reverse.group9()[3])
                - (self.group6()[2] * reverse.group3()[2])
                - (self.group6()[1] * reverse.group3()[1])
                - (self.group6()[0] * reverse.group3()[0])
                - (self.group5()[2] * reverse.group8()[2])
                - (self.group5()[1] * reverse.group8()[1])
                - (self.group5()[0] * reverse.group8()[0])
                + (self.group3()[3] * reverse[e1])
                - (self.group3()[2] * reverse.group6()[2])
                - (self.group3()[2] * reverse.group1()[2])
                - (self.group3()[1] * reverse.group6()[1])
                - (self.group3()[1] * reverse.group1()[1])
                - (self.group3()[0] * reverse.group6()[0])
                - (self.group3()[0] * reverse.group1()[0])
                - (self[e1] * reverse.group3()[3])
                + (self[e1] * reverse.group0()[0])
                + (self.group1()[2] * reverse.group3()[2])
                + (self.group1()[1] * reverse.group3()[1])
                + (self.group1()[0] * reverse.group3()[0])
                + (self.group0()[0] * reverse[e1])
                + (self.group0()[1] * reverse.group9()[3])),
            // e15, e25, e35, e45
            (-(reverse.group9() * Simd32x4::from([self.group9()[3], self.group9()[3], self.group9()[3], self[e45]]))
                + (Simd32x4::from(self.group9()[3]) * Simd32x4::from([reverse.group5()[0], reverse.group5()[1], reverse.group5()[2], reverse[e45]]))
                + (swizzle!(reverse.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group9()[2], self.group9()[0], self.group9()[1], self.group4()[2]]))
                - (swizzle!(self.group9(), 1, 2, 0, 2) * Simd32x4::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1], reverse.group5()[2]]))
                - (swizzle!(reverse.group6(), 1, 3, 3, 2) * Simd32x4::from([self.group8()[2], self.group8()[1], self.group8()[2], self.group1()[2]]))
                - (swizzle!(reverse.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self.group6()[2]]))
                + (swizzle!(reverse.group6(), 2, 0, 1, 3) * Simd32x4::from([self.group8()[1], self.group8()[2], self.group8()[0], self.group0()[1]]))
                - (swizzle!(reverse.group6(), 3, 2, 0, 1) * Simd32x4::from([self.group8()[0], self.group8()[0], self.group8()[1], self.group1()[1]]))
                + (Simd32x4::from(self.group6()[3]) * Simd32x4::from([reverse.group8()[0], reverse.group8()[1], reverse.group8()[2], reverse.group0()[1]]))
                - (swizzle!(self.group6(), 2, 1, 2, 1) * Simd32x4::from([reverse.group8()[1], reverse[e1], reverse[e1], reverse.group1()[1]]))
                - (swizzle!(self.group6(), 0, 0, 1, 0) * Simd32x4::from([reverse[e1], reverse.group8()[2], reverse.group8()[0], reverse.group1()[0]]))
                + (swizzle!(reverse.group3(), 1, 2, 0, 1) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group4()[1]]))
                + (swizzle!(reverse.group3(), 0, 1, 2, 0) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group4()[0]]))
                - (Simd32x4::from(self.group3()[2]) * Simd32x4::from([reverse.group9()[1], reverse.group5()[0], reverse.group3()[3], reverse.group4()[2]]))
                + (swizzle!(self.group3(), 2, 2, 2, 3) * Simd32x4::from([reverse.group5()[1], reverse.group9()[0], reverse.group0()[0], reverse.group0()[0]]))
                - (Simd32x4::from(self.group3()[1]) * Simd32x4::from([reverse.group5()[2], reverse.group3()[3], reverse.group9()[0], reverse.group4()[1]]))
                - (Simd32x4::from(self.group3()[0]) * Simd32x4::from([reverse.group3()[3], reverse.group9()[2], reverse.group5()[1], reverse.group4()[0]]))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2], reverse.group1()[3]]))
                + (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([reverse.group8()[1], reverse[e1], reverse[e1], reverse[e1]]))
                - (swizzle!(self.group1(), 1, 2, 0, 0) * Simd32x4::from([reverse.group8()[2], reverse.group8()[0], reverse.group8()[1], reverse.group6()[0]]))
                + (Simd32x4::from(self.group0()[0]) * reverse.group3())
                + Simd32x4::from([
                    ((self.group9()[0] * reverse.group9()[3]) + (self.group8()[1] * reverse.group1()[2]) - (self.group8()[0] * reverse.group0()[1])
                        + (self.group6()[1] * reverse.group8()[2])
                        - (self.group5()[1] * reverse.group3()[2])
                        + (self.group5()[0] * reverse.group9()[3])
                        + (self.group3()[1] * reverse.group9()[2])
                        + (self.group3()[0] * reverse.group0()[0])
                        - (self[e1] * reverse.group1()[0])
                        + (self.group1()[0] * reverse[e1])
                        - (self.group0()[1] * reverse.group8()[0])),
                    ((self.group9()[1] * reverse.group9()[3]) + (self.group8()[2] * reverse.group1()[0]) - (self.group8()[1] * reverse.group0()[1])
                        + (self.group6()[2] * reverse.group8()[0])
                        - (self.group5()[2] * reverse.group3()[0])
                        + (self.group5()[1] * reverse.group9()[3])
                        + (self.group3()[1] * reverse.group0()[0])
                        + (self.group3()[0] * reverse.group5()[2])
                        - (self[e1] * reverse.group1()[1])
                        + (self.group1()[0] * reverse.group8()[2])
                        - (self.group0()[1] * reverse.group8()[1])),
                    ((self.group9()[2] * reverse.group9()[3]) - (self.group8()[2] * reverse.group0()[1])
                        + (self.group8()[0] * reverse.group1()[1])
                        + (self.group6()[0] * reverse.group8()[1])
                        + (self.group5()[2] * reverse.group9()[3])
                        - (self.group5()[0] * reverse.group3()[1])
                        + (self.group3()[1] * reverse.group5()[0])
                        + (self.group3()[0] * reverse.group9()[1])
                        - (self[e1] * reverse.group1()[2])
                        + (self.group1()[1] * reverse.group8()[0])
                        - (self.group0()[1] * reverse.group8()[2])),
                    (-(self.group9()[1] * reverse.group5()[1]) - (self.group9()[0] * reverse.group5()[0])
                        + (self.group8()[2] * reverse.group7()[2])
                        + (self.group8()[1] * reverse.group7()[1])
                        + (self.group8()[0] * reverse.group7()[0])
                        - (self.group7()[2] * reverse.group8()[2])
                        - (self.group7()[1] * reverse.group8()[1])
                        - (self.group7()[0] * reverse.group8()[0])
                        - (self.group5()[2] * reverse.group9()[2])
                        - (self.group5()[1] * reverse.group9()[1])
                        - (self.group5()[0] * reverse.group9()[0])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group9()[0], reverse.group9()[1], reverse.group9()[2]])) + (Simd32x3::from(self[e45]) * reverse.group5())
                - (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group9()[2], self.group9()[0], self.group9()[1]]))
                + (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[0]]))
                - (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group9()[0], self.group9()[1], self.group9()[2]]))
                - (Simd32x3::from(self.group7()[2]) * Simd32x3::from([reverse.group6()[1], reverse.group1()[0], reverse.group0()[1]]))
                + (Simd32x3::from(self.group7()[2]) * Simd32x3::from([reverse.group1()[1], reverse.group6()[0], reverse.group6()[3]]))
                + (Simd32x3::from(self.group7()[1]) * Simd32x3::from([reverse.group6()[2], reverse.group6()[3], reverse.group1()[0]]))
                - (Simd32x3::from(self.group7()[1]) * Simd32x3::from([reverse.group1()[2], reverse.group0()[1], reverse.group6()[0]]))
                + (Simd32x3::from(self.group7()[0]) * Simd32x3::from([reverse.group6()[3], reverse.group1()[2], reverse.group6()[1]]))
                - (Simd32x3::from(self.group7()[0]) * Simd32x3::from([reverse.group0()[1], reverse.group6()[2], reverse.group1()[1]]))
                - (Simd32x3::from(self.group6()[3]) * reverse.group7())
                - (swizzle!(reverse.group7(), 1, 2, 0) * Simd32x3::from([self.group6()[2], self.group6()[0], self.group6()[1]]))
                + (swizzle!(reverse.group7(), 2, 0, 1) * Simd32x3::from([self.group6()[1], self.group6()[2], self.group6()[0]]))
                - (Simd32x3::from(reverse.group1()[3]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (swizzle!(self.group5(), 2, 1, 2) * Simd32x3::from([reverse.group4()[1], reverse[e45], reverse[e45]]))
                - (swizzle!(self.group5(), 1, 2, 0) * swizzle!(reverse.group4(), 2, 0, 1))
                + (swizzle!(self.group5(), 0, 0, 1) * Simd32x3::from([reverse[e45], reverse.group4()[2], reverse.group4()[0]]))
                + (swizzle!(self.group4(), 2, 1, 2) * Simd32x3::from([reverse.group9()[1], reverse.group3()[3], reverse.group3()[3]]))
                + (swizzle!(self.group4(), 2, 1, 2) * Simd32x3::from([reverse.group5()[1], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([reverse.group9()[2], reverse.group9()[0], reverse.group9()[1]]))
                - (swizzle!(self.group4(), 1, 2, 0) * swizzle!(reverse.group5(), 2, 0, 1))
                + (swizzle!(self.group4(), 0, 0, 1) * Simd32x3::from([reverse.group3()[3], reverse.group9()[2], reverse.group9()[0]]))
                + (swizzle!(self.group4(), 0, 0, 1) * Simd32x3::from([reverse.group0()[0], reverse.group5()[2], reverse.group5()[0]]))
                - (Simd32x3::from(self.group3()[3]) * reverse.group4())
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                - (swizzle!(reverse.group7(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))
                + (swizzle!(reverse.group7(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                - (Simd32x3::from(reverse.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group4())
                - (Simd32x3::from(self.group0()[1]) * reverse.group7())),
            // e23, e31, e12
            ((Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2]]))
                + (Simd32x3::from(self.group9()[3]) * reverse.group4())
                + (Simd32x3::from(reverse.group3()[3]) * Simd32x3::from([self.group9()[0], self.group9()[1], self.group9()[2]]))
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
                + (swizzle!(self.group4(), 2, 1, 2) * Simd32x3::from([reverse.group3()[1], reverse.group9()[3], reverse.group9()[3]]))
                - (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([reverse.group3()[2], reverse.group3()[0], reverse.group3()[1]]))
                + (swizzle!(self.group4(), 0, 0, 1) * Simd32x3::from([reverse.group9()[3], reverse.group3()[2], reverse.group3()[0]]))
                + (Simd32x3::from(self.group3()[3]) * Simd32x3::from([reverse.group9()[0], reverse.group9()[1], reverse.group9()[2]]))
                + (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]))
                - (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]))
                + (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                - (Simd32x3::from(self[e1]) * reverse.group7())
                - (Simd32x3::from(self.group1()[3]) * reverse.group8())
                - (Simd32x3::from(reverse.group6()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group5())
                - (Simd32x3::from(self.group0()[1]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                + Simd32x3::from([
                    ((self.group9()[2] * reverse.group9()[1]) - (self.group9()[1] * reverse.group9()[2]) - (self.group6()[2] * reverse.group6()[1])
                        + (self.group6()[1] * reverse.group6()[2])
                        - (self.group1()[2] * reverse.group1()[1])
                        + (self.group1()[1] * reverse.group1()[2])),
                    (-(self.group9()[2] * reverse.group9()[0]) + (self.group9()[0] * reverse.group9()[2]) + (self.group6()[2] * reverse.group6()[0])
                        - (self.group6()[0] * reverse.group6()[2])
                        + (self.group1()[2] * reverse.group1()[0])
                        - (self.group1()[0] * reverse.group1()[2])),
                    ((self.group9()[1] * reverse.group9()[0]) - (self.group9()[0] * reverse.group9()[1]) - (self.group6()[1] * reverse.group6()[0])
                        + (self.group6()[0] * reverse.group6()[1])
                        - (self.group1()[1] * reverse.group1()[0])
                        + (self.group1()[0] * reverse.group1()[1])),
                ])),
            // e415, e425, e435, e321
            ((Simd32x4::from(self[e45]) * Simd32x4::from([reverse.group8()[0], reverse.group8()[1], reverse.group8()[2], reverse[e1]]))
                + (swizzle!(self.group9(), 3, 3, 3, 2) * Simd32x4::from([reverse.group7()[0], reverse.group7()[1], reverse.group7()[2], reverse.group6()[2]]))
                - (swizzle!(self.group9(), 2, 1, 2, 3) * Simd32x4::from([reverse.group1()[1], reverse.group6()[3], reverse.group6()[3], reverse.group1()[3]]))
                + (swizzle!(self.group9(), 1, 2, 0, 1) * Simd32x4::from([reverse.group1()[2], reverse.group1()[0], reverse.group1()[1], reverse.group6()[1]]))
                - (swizzle!(reverse.group3(), 2, 0, 1, 2) * Simd32x4::from([self.group7()[1], self.group7()[2], self.group7()[0], self.group7()[2]]))
                + (swizzle!(reverse.group9(), 3, 3, 3, 2) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[2]]))
                + (swizzle!(self.group6(), 2, 1, 2, 3) * Simd32x4::from([reverse.group5()[1], reverse.group0()[0], reverse.group0()[0], reverse.group0()[0]]))
                + (swizzle!(self.group6(), 0, 0, 1, 1) * Simd32x4::from([reverse.group0()[0], reverse.group5()[2], reverse.group5()[0], reverse.group9()[1]]))
                + (swizzle!(reverse.group6(), 1, 2, 0, 0) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group9()[0]]))
                - (swizzle!(reverse.group1(), 0, 2, 0, 2) * Simd32x4::from([self.group3()[3], self.group9()[0], self.group9()[1], self.group5()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2, 2) * Simd32x4::from([reverse.group7()[1], reverse.group1()[3], reverse.group1()[3], reverse.group7()[2]]))
                - (swizzle!(self.group3(), 1, 3, 3, 3) * Simd32x4::from([reverse.group7()[2], reverse.group1()[1], reverse.group1()[2], reverse.group0()[1]]))
                + (swizzle!(self.group3(), 0, 0, 1, 1) * Simd32x4::from([reverse.group1()[3], reverse.group7()[2], reverse.group7()[0], reverse.group7()[1]]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2], reverse.group9()[3]]))
                - (swizzle!(self.group1(), 2, 1, 2, 2) * Simd32x4::from([reverse.group9()[1], reverse.group3()[3], reverse.group3()[3], reverse.group5()[2]]))
                + (swizzle!(reverse.group9(), 2, 0, 1, 0) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group6()[0]]))
                - (swizzle!(self.group1(), 0, 0, 1, 1) * Simd32x4::from([reverse.group3()[3], reverse.group9()[2], reverse.group9()[0], reverse.group5()[1]]))
                + (Simd32x4::from(self.group0()[0]) * reverse.group6())
                + Simd32x4::from([
                    (-(self.group9()[0] * reverse.group6()[3]) + (self.group8()[2] * reverse.group4()[1]) - (self.group8()[1] * reverse.group4()[2])
                        + (self.group8()[0] * reverse[e45])
                        + (self.group7()[2] * reverse.group3()[1])
                        - (self.group6()[3] * reverse.group9()[0])
                        - (self.group6()[1] * reverse.group5()[2])
                        - (self.group5()[1] * reverse.group6()[2])
                        + (self.group5()[0] * reverse.group0()[1])
                        + (self.group4()[2] * reverse.group8()[1])
                        - (self.group4()[1] * reverse.group8()[2])
                        + (self.group4()[0] * reverse[e1])
                        + (self[e1] * reverse.group4()[0])
                        + (self.group0()[1] * reverse.group5()[0])),
                    (-(self.group8()[2] * reverse.group4()[0])
                        + (self.group8()[1] * reverse[e45])
                        + (self.group8()[0] * reverse.group4()[2])
                        + (self.group7()[0] * reverse.group3()[2])
                        - (self.group6()[3] * reverse.group9()[1])
                        - (self.group6()[2] * reverse.group5()[0])
                        - (self.group5()[2] * reverse.group6()[0])
                        + (self.group5()[1] * reverse.group0()[1])
                        - (self.group4()[2] * reverse.group8()[0])
                        + (self.group4()[1] * reverse[e1])
                        + (self.group4()[0] * reverse.group8()[2])
                        - (self.group3()[2] * reverse.group7()[0])
                        + (self[e1] * reverse.group4()[1])
                        + (self.group0()[1] * reverse.group5()[1])),
                    ((self.group8()[2] * reverse[e45]) + (self.group8()[1] * reverse.group4()[0]) - (self.group8()[0] * reverse.group4()[1])
                        + (self.group7()[1] * reverse.group3()[0])
                        - (self.group6()[3] * reverse.group9()[2])
                        - (self.group6()[0] * reverse.group5()[1])
                        + (self.group5()[2] * reverse.group0()[1])
                        - (self.group5()[0] * reverse.group6()[1])
                        + (self.group4()[2] * reverse[e1])
                        + (self.group4()[1] * reverse.group8()[0])
                        - (self.group4()[0] * reverse.group8()[1])
                        - (self.group3()[0] * reverse.group7()[1])
                        + (self[e1] * reverse.group4()[2])
                        + (self.group0()[1] * reverse.group5()[2])),
                    ((self.group8()[2] * reverse.group4()[2]) + (self.group8()[1] * reverse.group4()[1]) + (self.group8()[0] * reverse.group4()[0])
                        - (self.group7()[1] * reverse.group3()[1])
                        - (self.group7()[0] * reverse.group3()[0])
                        - (self.group5()[1] * reverse.group1()[1])
                        - (self.group5()[0] * reverse.group1()[0])
                        - (self.group4()[2] * reverse.group8()[2])
                        - (self.group4()[1] * reverse.group8()[1])
                        - (self.group4()[0] * reverse.group8()[0])
                        + (self.group3()[0] * reverse.group7()[0])
                        - (self[e1] * reverse[e45])
                        - (self.group1()[0] * reverse.group5()[0])
                        - (self.group0()[1] * reverse.group3()[3])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                - (Simd32x3::from(self[e45]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                - (swizzle!(reverse.group7(), 1, 2, 0) * Simd32x3::from([self.group9()[2], self.group9()[0], self.group9()[1]]))
                + (swizzle!(reverse.group7(), 2, 0, 1) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[0]]))
                - (Simd32x3::from(reverse.group1()[3]) * Simd32x3::from([self.group9()[0], self.group9()[1], self.group9()[2]]))
                + (swizzle!(self.group7(), 2, 1, 2) * Simd32x3::from([reverse.group9()[1], reverse.group3()[3], reverse.group3()[3]]))
                + (swizzle!(self.group7(), 2, 1, 2) * Simd32x3::from([reverse.group5()[1], reverse.group0()[0], reverse.group0()[0]]))
                - (swizzle!(self.group7(), 1, 2, 0) * Simd32x3::from([reverse.group9()[2], reverse.group9()[0], reverse.group9()[1]]))
                - (swizzle!(self.group7(), 1, 2, 0) * swizzle!(reverse.group5(), 2, 0, 1))
                + (swizzle!(self.group7(), 0, 0, 1) * Simd32x3::from([reverse.group3()[3], reverse.group9()[2], reverse.group9()[0]]))
                + (swizzle!(self.group7(), 0, 0, 1) * Simd32x3::from([reverse.group0()[0], reverse.group5()[2], reverse.group5()[0]]))
                + (Simd32x3::from(self.group6()[3]) * reverse.group4())
                + (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group6()[2], self.group6()[0], self.group6()[1]]))
                - (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group6()[1], self.group6()[2], self.group6()[0]]))
                + (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (swizzle!(self.group5(), 2, 1, 2) * Simd32x3::from([reverse.group7()[1], reverse.group1()[3], reverse.group1()[3]]))
                - (swizzle!(self.group5(), 1, 2, 0) * swizzle!(reverse.group7(), 2, 0, 1))
                + (swizzle!(self.group5(), 0, 0, 1) * Simd32x3::from([reverse.group1()[3], reverse.group7()[2], reverse.group7()[0]]))
                + (Simd32x3::from(self.group4()[2]) * Simd32x3::from([reverse.group6()[1], reverse.group1()[0], reverse.group0()[1]]))
                - (Simd32x3::from(self.group4()[2]) * Simd32x3::from([reverse.group1()[1], reverse.group6()[0], reverse.group6()[3]]))
                - (Simd32x3::from(self.group4()[1]) * Simd32x3::from([reverse.group6()[2], reverse.group6()[3], reverse.group1()[0]]))
                + (Simd32x3::from(self.group4()[1]) * Simd32x3::from([reverse.group1()[2], reverse.group0()[1], reverse.group6()[0]]))
                - (Simd32x3::from(self.group4()[0]) * Simd32x3::from([reverse.group6()[3], reverse.group1()[2], reverse.group6()[1]]))
                + (Simd32x3::from(self.group4()[0]) * Simd32x3::from([reverse.group0()[1], reverse.group6()[2], reverse.group1()[1]]))
                - (Simd32x3::from(self.group3()[3]) * reverse.group7())
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([reverse.group9()[0], reverse.group9()[1], reverse.group9()[2]]))
                + (Simd32x3::from(self.group1()[3]) * reverse.group5())
                + (swizzle!(reverse.group4(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))
                - (swizzle!(reverse.group4(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (Simd32x3::from(reverse[e45]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group7())
                + (Simd32x3::from(self.group0()[1]) * reverse.group4())),
            // e235, e315, e125
            ((Simd32x3::from(self.group9()[3]) * Simd32x3::from([reverse.group6()[0], reverse.group6()[1], reverse.group6()[2]]))
                + (Simd32x3::from(self.group9()[3]) * Simd32x3::from([reverse.group1()[0], reverse.group1()[1], reverse.group1()[2]]))
                + (swizzle!(reverse.group8(), 1, 2, 0) * Simd32x3::from([self.group9()[2], self.group9()[0], self.group9()[1]]))
                - (swizzle!(reverse.group8(), 2, 0, 1) * Simd32x3::from([self.group9()[1], self.group9()[2], self.group9()[0]]))
                + (Simd32x3::from(reverse[e1]) * Simd32x3::from([self.group9()[0], self.group9()[1], self.group9()[2]]))
                - (Simd32x3::from(self.group8()[2]) * Simd32x3::from([reverse.group9()[1], reverse.group5()[0], reverse.group3()[3]]))
                + (Simd32x3::from(self.group8()[2]) * Simd32x3::from([reverse.group5()[1], reverse.group9()[0], reverse.group0()[0]]))
                + (Simd32x3::from(self.group8()[1]) * Simd32x3::from([reverse.group9()[2], reverse.group0()[0], reverse.group5()[0]]))
                - (Simd32x3::from(self.group8()[1]) * Simd32x3::from([reverse.group5()[2], reverse.group3()[3], reverse.group9()[0]]))
                - (Simd32x3::from(self.group8()[0]) * Simd32x3::from([reverse.group3()[3], reverse.group9()[2], reverse.group5()[1]]))
                + (Simd32x3::from(self.group8()[0]) * Simd32x3::from([reverse.group0()[0], reverse.group5()[2], reverse.group9()[1]]))
                - (Simd32x3::from(self.group6()[3]) * Simd32x3::from([reverse.group3()[0], reverse.group3()[1], reverse.group3()[2]]))
                + (Simd32x3::from(reverse.group3()[1]) * Simd32x3::from([self.group6()[2], self.group0()[1], self.group1()[0]]))
                + (Simd32x3::from(reverse.group9()[3]) * Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]))
                + (swizzle!(self.group5(), 2, 1, 2) * Simd32x3::from([reverse.group8()[1], reverse[e1], reverse[e1]]))
                - (swizzle!(self.group5(), 1, 2, 0) * swizzle!(reverse.group8(), 2, 0, 1))
                + (swizzle!(self.group5(), 0, 0, 1) * Simd32x3::from([reverse[e1], reverse.group8()[2], reverse.group8()[0]]))
                + (Simd32x3::from(self.group3()[3]) * reverse.group8())
                + (Simd32x3::from(reverse.group6()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                + (Simd32x3::from(reverse.group0()[1]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                - (Simd32x3::from(self[e1]) * Simd32x3::from([reverse.group9()[0], reverse.group9()[1], reverse.group9()[2]]))
                + (Simd32x3::from(self[e1]) * reverse.group5())
                + (Simd32x3::from(reverse.group3()[2]) * Simd32x3::from([self.group1()[1], self.group6()[0], self.group0()[1]]))
                - (Simd32x3::from(reverse.group9()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * reverse.group8())
                + (Simd32x3::from(reverse.group3()[0]) * Simd32x3::from([self.group0()[1], self.group1()[2], self.group6()[1]]))
                + Simd32x3::from([
                    (-(self.group6()[1] * reverse.group3()[2]) + (self.group3()[2] * reverse.group6()[1]) + (self.group3()[2] * reverse.group1()[1])
                        - (self.group3()[1] * reverse.group6()[2])
                        - (self.group3()[1] * reverse.group1()[2])
                        - (self.group1()[2] * reverse.group3()[1])),
                    (-(self.group6()[2] * reverse.group3()[0]) - (self.group3()[2] * reverse.group6()[0]) - (self.group3()[2] * reverse.group1()[0])
                        + (self.group3()[0] * reverse.group6()[2])
                        + (self.group3()[0] * reverse.group1()[2])
                        - (self.group1()[0] * reverse.group3()[2])),
                    (-(self.group6()[0] * reverse.group3()[1]) + (self.group3()[1] * reverse.group6()[0]) + (self.group3()[1] * reverse.group1()[0])
                        - (self.group3()[0] * reverse.group6()[1])
                        - (self.group3()[0] * reverse.group1()[1])
                        - (self.group1()[1] * reverse.group3()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(reverse.group3() * Simd32x4::from([self[e45], self[e45], self[e45], self.group9()[3]]))
                + (Simd32x4::from(self.group9()[3]) * Simd32x4::from([reverse.group4()[0], reverse.group4()[1], reverse.group4()[2], reverse.group0()[0]]))
                - (swizzle!(self.group9(), 1, 2, 0, 2) * Simd32x4::from([reverse.group5()[2], reverse.group5()[0], reverse.group5()[1], reverse.group3()[2]]))
                + (Simd32x4::from(reverse[e1]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[3]]))
                + (swizzle!(reverse.group6(), 0, 1, 2, 2) * Simd32x4::from([self.group6()[3], self.group6()[3], self.group6()[3], self.group8()[2]]))
                + (swizzle!(self.group6(), 2, 1, 2, 2) * Simd32x4::from([reverse.group1()[1], reverse.group6()[3], reverse.group6()[3], reverse.group8()[2]]))
                + (swizzle!(self.group6(), 0, 0, 1, 1) * Simd32x4::from([reverse.group6()[3], reverse.group1()[2], reverse.group1()[0], reverse.group8()[1]]))
                + (swizzle!(reverse.group9(), 1, 2, 0, 3) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group3()[3]]))
                - (swizzle!(reverse.group3(), 1, 2, 0, 1) * Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group9()[1]]))
                + (swizzle!(self.group3(), 3, 3, 3, 2) * Simd32x4::from([reverse.group5()[0], reverse.group5()[1], reverse.group5()[2], reverse.group9()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2, 1) * Simd32x4::from([reverse.group4()[1], reverse[e45], reverse[e45], reverse.group9()[1]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([reverse.group4()[2], reverse.group4()[0], reverse.group4()[1], reverse.group5()[2]]))
                + (swizzle!(self.group3(), 0, 0, 1, 0) * Simd32x4::from([reverse[e45], reverse.group4()[2], reverse.group4()[0], reverse.group9()[0]]))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([reverse.group7()[0], reverse.group7()[1], reverse.group7()[2], reverse.group6()[3]]))
                + (swizzle!(reverse.group6(), 1, 2, 0, 1) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group8()[1]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([reverse.group6()[2], reverse.group6()[0], reverse.group6()[1], reverse.group8()[2]]))
                + (Simd32x4::from(self.group0()[0]) * reverse.group9())
                + (swizzle!(reverse.group1(), 0, 1, 2, 2) * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group8()[2]]))
                + Simd32x4::from([
                    ((self.group9()[2] * reverse.group5()[1]) + (self.group9()[0] * reverse.group0()[0]) - (self.group8()[2] * reverse.group7()[1])
                        + (self.group8()[1] * reverse.group7()[2])
                        - (self.group8()[0] * reverse.group1()[3])
                        + (self.group7()[2] * reverse.group8()[1])
                        - (self.group7()[1] * reverse.group8()[2])
                        - (self.group6()[1] * reverse.group1()[2])
                        - (self.group5()[1] * reverse.group9()[2])
                        + (self.group5()[0] * reverse.group3()[3])
                        + (self.group4()[1] * reverse.group3()[2])
                        - (self.group4()[0] * reverse.group9()[3])
                        + (self.group1()[3] * reverse.group8()[0])
                        + (self.group1()[0] * reverse.group0()[1])),
                    ((self.group9()[1] * reverse.group0()[0]) + (self.group9()[0] * reverse.group5()[2]) + (self.group8()[2] * reverse.group7()[0])
                        - (self.group8()[1] * reverse.group1()[3])
                        - (self.group8()[0] * reverse.group7()[2])
                        - (self.group7()[2] * reverse.group8()[0])
                        + (self.group7()[0] * reverse.group8()[2])
                        - (self.group6()[2] * reverse.group1()[0])
                        - (self.group5()[2] * reverse.group9()[0])
                        + (self.group5()[1] * reverse.group3()[3])
                        + (self.group4()[2] * reverse.group3()[0])
                        - (self.group4()[1] * reverse.group9()[3])
                        + (self.group1()[3] * reverse.group8()[1])
                        + (self.group1()[1] * reverse.group0()[1])),
                    ((self.group9()[2] * reverse.group0()[0]) + (self.group9()[1] * reverse.group5()[0])
                        - (self.group8()[2] * reverse.group1()[3])
                        - (self.group8()[1] * reverse.group7()[0])
                        + (self.group8()[0] * reverse.group7()[1])
                        + (self.group7()[1] * reverse.group8()[0])
                        - (self.group7()[0] * reverse.group8()[1])
                        - (self.group6()[0] * reverse.group1()[1])
                        + (self.group5()[2] * reverse.group3()[3])
                        - (self.group5()[0] * reverse.group9()[1])
                        - (self.group4()[2] * reverse.group9()[3])
                        + (self.group4()[0] * reverse.group3()[1])
                        + (self.group1()[3] * reverse.group8()[2])
                        + (self.group1()[2] * reverse.group0()[1])),
                    (-(self.group9()[0] * reverse.group3()[0])
                        + (self.group8()[1] * reverse.group1()[1])
                        + (self.group8()[0] * reverse.group6()[0])
                        + (self.group8()[0] * reverse.group1()[0])
                        + (self.group6()[0] * reverse.group8()[0])
                        - (self.group5()[2] * reverse.group3()[2])
                        - (self.group5()[1] * reverse.group3()[1])
                        - (self.group5()[0] * reverse.group3()[0])
                        - (self.group3()[1] * reverse.group5()[1])
                        - (self.group3()[0] * reverse.group5()[0])
                        - (self[e1] * reverse.group0()[1])
                        - (self.group1()[1] * reverse.group8()[1])
                        - (self.group1()[0] * reverse.group8()[0])
                        - (self.group0()[1] * reverse[e1])),
                ])),
            // e1234
            ((self[e45] * reverse.group3()[3])
                + (self[e45] * reverse.group0()[0])
                + (self.group9()[2] * reverse.group4()[2])
                + (self.group9()[1] * reverse.group4()[1])
                + (self.group9()[0] * reverse.group4()[0])
                + (self.group7()[2] * reverse.group6()[2])
                - (self.group7()[2] * reverse.group1()[2])
                + (self.group7()[1] * reverse.group6()[1])
                - (self.group7()[1] * reverse.group1()[1])
                + (self.group7()[0] * reverse.group6()[0])
                - (self.group7()[0] * reverse.group1()[0])
                - (self.group6()[3] * reverse.group1()[3])
                + (self.group6()[2] * reverse.group7()[2])
                + (self.group6()[1] * reverse.group7()[1])
                + (self.group6()[0] * reverse.group7()[0])
                - (self.group5()[2] * reverse.group4()[2])
                - (self.group5()[1] * reverse.group4()[1])
                - (self.group5()[0] * reverse.group4()[0])
                - (self.group4()[2] * reverse.group9()[2])
                - (self.group4()[2] * reverse.group5()[2])
                - (self.group4()[1] * reverse.group9()[1])
                - (self.group4()[1] * reverse.group5()[1])
                - (self.group4()[0] * reverse.group9()[0])
                - (self.group4()[0] * reverse.group5()[0])
                - (self.group3()[3] * reverse[e45])
                + (self.group1()[3] * reverse.group6()[3])
                - (self.group1()[3] * reverse.group0()[1])
                + (self.group1()[2] * reverse.group7()[2])
                + (self.group1()[1] * reverse.group7()[1])
                + (self.group1()[0] * reverse.group7()[0])
                + (self.group0()[0] * reverse[e45])
                - (self.group0()[1] * reverse.group1()[3])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            ((self[e45] * self.group9()[3]) + (self.group9()[3] * self[e45]) - f32::powi(self.group9()[2], 2) - f32::powi(self.group9()[1], 2) - f32::powi(self.group9()[0], 2)
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
            // e15, e25, e35, e45
            geometric_product.group3(),
            // e41, e42, e43
            geometric_product.group4(),
            // e23, e31, e12
            geometric_product.group5(),
            // e415, e425, e435, e321
            geometric_product.group6(),
            // e423, e431, e412
            geometric_product.group7(),
            // e235, e315, e125
            geometric_product.group8(),
            // e4235, e4315, e4125, e3215
            geometric_product.group9(),
            // e1234
            geometric_product[e45],
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
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(self.group0()[1] * self.group0()[2]) + (self.group0()[2] * self.group0()[1])),
                ((self.group0()[0] * self.group0()[2]) - (self.group0()[2] * self.group0()[0])),
                (-(self.group0()[0] * self.group0()[1]) + (self.group0()[1] * self.group0()[0])),
                (-f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                ((self.group0()[0] * self.group0()[3]) - (self.group0()[3] * self.group0()[0])),
                ((self.group0()[1] * self.group0()[3]) - (self.group0()[3] * self.group0()[1])),
                ((self.group0()[2] * self.group0()[3]) - (self.group0()[3] * self.group0()[2])),
                0.0,
            ]),
        );
        let subtraction = AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2]]),
            // e15, e25, e35
            Simd32x3::from([geometric_product.group1()[0], geometric_product.group1()[1], geometric_product.group1()[2]]),
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
impl ConstraintViolation for RoundPoint {
    type Output = AntiCircleRotor;
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
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
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
impl ConstraintViolation for Scalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        let subtraction = Scalar::from_groups(/* scalar */ 0.0);
        return subtraction;
    }
}
impl ConstraintViolation for Sphere {
    type Output = AntiCircleRotor;
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
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
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
impl ConstraintViolation for TripleNum {
    type Output = AntiQuadNum;
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
        let subtraction = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([geometric_product.group0()[0], geometric_product.group0()[1], geometric_product.group0()[2], 0.0]),
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
