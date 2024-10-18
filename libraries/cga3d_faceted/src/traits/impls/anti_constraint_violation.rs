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
//  Average:        20      23       0
//  Maximum:       269     279       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        15      17       0
//  Average:        26      32       0
//  Maximum:       396     420       0
impl AntiConstraintViolation for AntiCircleOnOrigin {
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
        let anti_reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([
                ((anti_reverse.group1()[0] * self.group0()[0]) + (anti_reverse.group1()[1] * self.group0()[1]) + (anti_reverse.group1()[2] * self.group0()[2])),
                0.0,
            ]) + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([anti_reverse.group0()[1], anti_reverse.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([anti_reverse.group0()[2], anti_reverse.group1()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)));
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
    }
}
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       43        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       35       45        0
    //  no simd       35       49        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    + (self.group0()[0] * anti_reverse.group2()[0])
                    + (self.group0()[1] * anti_reverse.group2()[1])
                    + (self.group0()[2] * anti_reverse.group2()[2])
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
                    + (self.group1()[0] * anti_reverse.group2()[0])
                    + (self.group1()[1] * anti_reverse.group2()[1])
                    + (self.group1()[2] * anti_reverse.group2()[2])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       13        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       14        0
    //  no simd       12       16        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])
                    - (anti_reverse.group1()[3] * self.group1()[3])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group1()[0])
                    + (anti_reverse.group0()[1] * self.group1()[1])
                    + (anti_reverse.group0()[2] * self.group1()[2])
                    + (self.group0()[0] * anti_reverse.group1()[0])
                    + (self.group0()[1] * anti_reverse.group1()[1])
                    + (self.group0()[2] * anti_reverse.group1()[2])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group1()[3], 2)),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
    }
}
impl AntiConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       12        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       14       15        0
    //  no simd       20       24        0
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
            Simd32x4::from(0.0),
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
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[3], 2)),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       12        0
    //  no simd       12       17        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([
                ((self.group1()[0] * anti_reverse.group0()[0]) + (self.group1()[1] * anti_reverse.group0()[1]) + (self.group1()[2] * anti_reverse.group0()[2])),
                ((anti_reverse.group0()[3] * self.group0()[3]) * -1.0),
            ]) + (Simd32x2::from(anti_reverse.group1()[0]) * Simd32x2::from([self.group0()[0], self.group1()[0]]))
                + (Simd32x2::from(anti_reverse.group1()[1]) * Simd32x2::from([self.group0()[1], self.group1()[1]]))
                + (Simd32x2::from(anti_reverse.group1()[2]) * Simd32x2::from([self.group0()[2], self.group1()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       27        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       32       32        0
    //  no simd       41       46        0
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
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (self.group1()[0] * anti_reverse.group0()[0])
                    + (self.group1()[0] * anti_reverse.group2()[0])
                    - (self.group1()[1] * anti_reverse.group0()[1])
                    + (self.group1()[1] * anti_reverse.group2()[1])
                    - (self.group1()[2] * anti_reverse.group0()[2])
                    + (self.group1()[2] * anti_reverse.group2()[2])
                    - (anti_reverse.group0()[3] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group0()[3])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)
                - f32::powi(self.group2()[0], 2)
                - f32::powi(self.group2()[1], 2)
                - f32::powi(self.group2()[2], 2)),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOnOrigin {
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
        let anti_reverse =
            AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e4, e1, e2, e3 */ self.group1());
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([
                ((anti_reverse.group0()[0] * self.group1()[1]) + (anti_reverse.group0()[1] * self.group1()[2]) + (anti_reverse.group0()[2] * self.group1()[3])
                    - (anti_reverse.group1()[0] * self.group0()[3])),
                0.0,
            ]) + (Simd32x2::from(anti_reverse.group0()[3]) * Simd32x2::from([self.group1()[0], self.group0()[3]]))
                - (Simd32x2::from(anti_reverse.group1()[1]) * Simd32x2::from([self.group0()[0], self.group1()[1]]))
                - (Simd32x2::from(anti_reverse.group1()[2]) * Simd32x2::from([self.group0()[1], self.group1()[2]]))
                - (Simd32x2::from(anti_reverse.group1()[3]) * Simd32x2::from([self.group0()[2], self.group1()[3]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       39       53        0
    //  no simd       48       64        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    + (anti_reverse.group0()[3] * self.group2()[3])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])
                    + (anti_reverse.group2()[3] * self.group0()[3])),
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
                    - (self.group1()[0] * anti_reverse.group2()[0])
                    - (self.group1()[1] * anti_reverse.group2()[1])
                    - (self.group1()[2] * anti_reverse.group2()[2])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for AntiDipoleOnOrigin {
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
        let anti_reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3]));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self.group0()[3], 2));
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = Origin;
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
        return Origin::from_groups(/* e4 */ geometric_anti_product.group0()[0]);
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
    }
}
impl AntiConstraintViolation for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        5        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            ((anti_reverse.group0()[0] * self.group0()[0]) + (anti_reverse.group0()[1] * self.group0()[1]) + (anti_reverse.group0()[2] * self.group0()[2])),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2)));
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
    }
}
impl AntiConstraintViolation for AntiMysteryCircleRotor {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9        6        0
    //  no simd       15       15        0
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
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2) - f32::powi(self[e31], 2)),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for AntiMysteryDipoleInversion {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       19       17        0
    //  no simd       28       29        0
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
            Simd32x4::from(0.0),
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
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for AntiPlane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for AntiSphereOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for AntiVersorEvenOnOrigin {
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
        let anti_reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = DualNum::from_groups(
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
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for CircleAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       39        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       33       48        0
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
                0.0,
                0.0,
                0.0,
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for CircleAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       12        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       15        0
    //  no simd       15       23        0
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
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
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
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for CircleAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       17       30        0
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
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-2.0 * (self.group0()[0] * self.group1()[0]) - 2.0 * (self.group0()[1] * self.group1()[1]) - 2.0 * (self.group0()[2] * self.group1()[2])),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for CircleOnOrigin {
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
        let anti_reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([
                (-(anti_reverse.group1()[0] * self.group0()[0]) - (anti_reverse.group1()[1] * self.group0()[1]) - (anti_reverse.group1()[2] * self.group0()[2])),
                0.0,
            ]) - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group1()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([anti_reverse.group0()[1], anti_reverse.group1()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([anti_reverse.group0()[2], anti_reverse.group1()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2)));
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
    }
}
impl AntiConstraintViolation for CircleOrthogonalOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       19       32        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    - (self.group1()[1] * anti_reverse.group0()[1])
                    - (self.group1()[2] * anti_reverse.group0()[2])),
                (self.group1()[1] * anti_reverse.group0()[2]),
                (self.group1()[2] * anti_reverse.group0()[0]),
                (self.group1()[0] * anti_reverse.group0()[1]),
            ]) - (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group1()[0], self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 0, 1, 2, 0))
                + (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 3, 2, 0, 1))),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[3], 2)
                - 2.0 * (self.group1()[0] * self.group0()[0])
                - 2.0 * (self.group1()[1] * self.group0()[1])
                - 2.0 * (self.group1()[2] * self.group0()[2])),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       43        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       35       45        0
    //  no simd       35       49        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group2()[0])
                    - (anti_reverse.group0()[1] * self.group2()[1])
                    - (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group1()[0] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group1()[1])
                    - (anti_reverse.group1()[2] * self.group1()[2])
                    - (self.group0()[0] * anti_reverse.group2()[0])
                    - (self.group0()[1] * anti_reverse.group2()[1])
                    - (self.group0()[2] * anti_reverse.group2()[2])
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
                    - (self.group1()[0] * anti_reverse.group2()[0])
                    - (self.group1()[1] * anti_reverse.group2()[1])
                    - (self.group1()[2] * anti_reverse.group2()[2])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       13        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       14        0
    //  no simd       12       16        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group1()[3])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
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
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group1()[3], 2)),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
    }
}
impl AntiConstraintViolation for CircleRotorAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       16        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       18        0
    //  no simd       17       24        0
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
            Simd32x4::from(0.0),
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
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[3], 2)),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for CircleRotorOnOrigin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        7        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       11        0
    //  no simd       12       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([
                (-(self.group1()[0] * anti_reverse.group0()[0]) - (self.group1()[1] * anti_reverse.group0()[1]) - (self.group1()[2] * anti_reverse.group0()[2])),
                (anti_reverse.group0()[3] * self.group0()[3]),
            ]) - (Simd32x2::from(anti_reverse.group1()[0]) * Simd32x2::from([self.group0()[0], self.group1()[0]]))
                - (Simd32x2::from(anti_reverse.group1()[1]) * Simd32x2::from([self.group0()[1], self.group1()[1]]))
                - (Simd32x2::from(anti_reverse.group1()[2]) * Simd32x2::from([self.group0()[2], self.group1()[2]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for DipoleAligningOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       19       35        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    + (self.group1()[1] * anti_reverse.group0()[1])
                    + (self.group1()[2] * anti_reverse.group0()[2])),
                ((self.group1()[1] * anti_reverse.group0()[2]) * -1.0),
                ((self.group1()[2] * anti_reverse.group0()[0]) * -1.0),
                ((self.group1()[0] * anti_reverse.group0()[1]) * -1.0),
            ]) + (Simd32x4::from([anti_reverse.group1()[0], anti_reverse.group1()[2], anti_reverse.group1()[0], anti_reverse.group1()[1]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                + (Simd32x4::from([self.group1()[0], self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(anti_reverse.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([anti_reverse.group0()[3], anti_reverse.group1()[1], anti_reverse.group1()[2], anti_reverse.group1()[0]]) * swizzle!(self.group0(), 3, 2, 0, 1))),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2)
                + 2.0 * (self.group1()[0] * self.group0()[0])
                + 2.0 * (self.group1()[1] * self.group0()[1])
                + 2.0 * (self.group1()[2] * self.group0()[2])),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for DipoleAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        8        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       18       23        0
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
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
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
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for DipoleAtOrigin {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       17       30        0
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
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (2.0 * (self.group0()[0] * self.group1()[0]) + 2.0 * (self.group0()[1] * self.group1()[1]) + 2.0 * (self.group0()[2] * self.group1()[2])),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for DipoleInversionAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       51        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       45       55        0
    //  no simd       54       67        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group1()[0]) + (anti_reverse.group0()[1] * self.group1()[1]) + (anti_reverse.group0()[2] * self.group1()[2])
                    - (anti_reverse.group0()[3] * self.group0()[3])
                    + (anti_reverse.group1()[0] * self.group0()[0])
                    + (anti_reverse.group1()[1] * self.group0()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    - (anti_reverse.group1()[3] * self.group2()[3])
                    + (anti_reverse.group2()[0] * self.group2()[0])
                    + (anti_reverse.group2()[1] * self.group2()[1])
                    + (anti_reverse.group2()[2] * self.group2()[2])
                    - (anti_reverse.group2()[3] * self.group1()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[3] * self.group2()[3])
                    - (anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    + (anti_reverse.group2()[0] * self.group1()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])
                    + (anti_reverse.group2()[3] * self.group0()[3])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       22        0
    //    simd3        0        1        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       26       29        0
    //  no simd       41       49        0
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
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group1()[0] * self.group0()[0]) - (anti_reverse.group1()[0] * self.group2()[0]) + (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    + (anti_reverse.group1()[2] * self.group0()[2])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    + (self.group1()[0] * anti_reverse.group0()[0])
                    + (self.group1()[0] * anti_reverse.group2()[0])
                    + (self.group1()[1] * anti_reverse.group0()[1])
                    + (self.group1()[1] * anti_reverse.group2()[1])
                    + (self.group1()[2] * anti_reverse.group0()[2])
                    + (self.group1()[2] * anti_reverse.group2()[2])
                    - (anti_reverse.group0()[3] * self.group2()[3])
                    + (anti_reverse.group2()[3] * self.group0()[3])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)
                + f32::powi(self.group2()[0], 2)
                + f32::powi(self.group2()[1], 2)
                + f32::powi(self.group2()[2], 2)),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for DipoleInversionAtOrigin {
    type Output = MysteryVersorEven;
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
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                ((anti_reverse.group1()[1] * self.group0()[1]) + (anti_reverse.group1()[2] * self.group0()[2])),
                (-(anti_reverse.group0()[2] * self.group1()[1]) - (anti_reverse.group1()[3] * self.group1()[0])),
                (-(anti_reverse.group1()[2] * self.group0()[0]) - (anti_reverse.group1()[3] * self.group1()[1])),
                (-(anti_reverse.group1()[0] * self.group0()[1]) - (anti_reverse.group1()[3] * self.group1()[2])),
            ]) + (Simd32x4::from([anti_reverse.group0()[2], anti_reverse.group1()[0], anti_reverse.group1()[1], anti_reverse.group1()[2]]) * swizzle!(self.group1(), 2, 3, 3, 3))
                - (Simd32x4::from([anti_reverse.group1()[3], anti_reverse.group1()[1], anti_reverse.group0()[1], anti_reverse.group0()[2]]) * swizzle!(self.group0(), 3, 2, 3, 3))
                + (Simd32x4::from([self.group1()[1], self.group0()[0], self.group0()[1], self.group0()[2]]) * swizzle!(anti_reverse.group0(), 1, 3, 3, 3))
                - (Simd32x4::from([self.group1()[3], self.group0()[3], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group0(), 3, 0, 0, 1))
                + (swizzle!(anti_reverse.group0(), 0, 1, 2, 0) * swizzle!(self.group1(), 0, 2, 0, 1))
                + (swizzle!(anti_reverse.group1(), 0, 2, 0, 1) * swizzle!(self.group0(), 0, 1, 2, 0))),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (2.0 * (self.group0()[0] * self.group1()[0]) + 2.0 * (self.group0()[1] * self.group1()[1]) + 2.0 * (self.group0()[2] * self.group1()[2])
                - 2.0 * (self.group0()[3] * self.group1()[3])),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for DipoleInversionOnOrigin {
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
        let anti_reverse = DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e1234, e4235, e4315, e4125
            self.group1(),
        );
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            (Simd32x2::from([
                ((anti_reverse.group0()[3] * self.group1()[0])
                    - (anti_reverse.group1()[1] * self.group0()[0])
                    - (anti_reverse.group1()[2] * self.group0()[1])
                    - (anti_reverse.group1()[3] * self.group0()[2])),
                0.0,
            ]) - (Simd32x2::from(self.group0()[3]) * Simd32x2::from([anti_reverse.group1()[0], anti_reverse.group0()[3]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([anti_reverse.group0()[0], anti_reverse.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([anti_reverse.group0()[1], anti_reverse.group1()[2]]))
                + (Simd32x2::from(self.group1()[3]) * Simd32x2::from([anti_reverse.group0()[2], anti_reverse.group1()[3]]))),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[3], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) + f32::powi(self.group1()[3], 2)),
        );
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
    }
}
impl AntiConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       39       53        0
    //  no simd       48       64        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group1()[0] * self.group1()[0])
                    + (anti_reverse.group1()[1] * self.group1()[1])
                    + (anti_reverse.group1()[2] * self.group1()[2])
                    + (anti_reverse.group0()[0] * self.group2()[0])
                    + (anti_reverse.group0()[1] * self.group2()[1])
                    + (anti_reverse.group0()[2] * self.group2()[2])
                    - (anti_reverse.group0()[3] * self.group2()[3])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group0()[3])),
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
                    + (self.group1()[0] * anti_reverse.group2()[0])
                    + (self.group1()[1] * anti_reverse.group2()[1])
                    + (self.group1()[2] * anti_reverse.group2()[2])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for DipoleOnOrigin {
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
        let anti_reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ (anti_reverse.group0()[3] * self.group0()[3] * -1.0));
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (f32::powi(self.group0()[3], 2) * -1.0));
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
    }
}
impl AntiConstraintViolation for DipoleOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       39        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       33       48        0
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
                0.0,
                0.0,
                0.0,
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Origin;
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
        return Origin::from_groups(/* e4 */ geometric_anti_product.group0()[0]);
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
    }
}
impl AntiConstraintViolation for LineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        5        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
        let geometric_anti_product = AntiScalar::from_groups(
            // e12345
            (-(anti_reverse.group0()[0] * self.group0()[0]) - (anti_reverse.group0()[1] * self.group0()[1]) - (anti_reverse.group0()[2] * self.group0()[2])),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2)));
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
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
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group1()[3]]),
        );
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
        return AntiScalar::from_groups(/* e12345 */ (-anti_scalar_product[e12345] + geometric_anti_product[e12345]));
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      216      220        0
    //    simd2       16       16        0
    //    simd3        0        4        0
    //    simd4       37       39        0
    // Totals...
    // yes simd      269      279        0
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
            // e1, e2, e3, e4
            geometric_anti_product.group1(),
            // e5
            geometric_anti_product[e1],
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
            geometric_anti_product.group9(),
            // e3215
            geometric_anti_product[e45],
        );
    }
}
impl AntiConstraintViolation for MysteryCircle {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       10       14        0
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
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2)),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for MysteryCircleRotor {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9        9        0
    //  no simd       12       15        0
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
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) - f32::powi(self.group0()[1], 2) - f32::powi(self.group0()[2], 2) + f32::powi(self.group0()[3], 2) + f32::powi(self[e425], 2)),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for MysteryDipole {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        7        5        0
    //  no simd       13       14        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ (self.group0() * Simd32x4::from(-1.0)));
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([((anti_reverse.group0()[2] * self.group0()[2]) - (anti_reverse.group0()[3] * self.group0()[3])), 0.0, 0.0, 0.0])
                + (swizzle!(anti_reverse.group0(), 0, 0, 1, 2) * swizzle!(self.group0(), 0, 3, 3, 3))
                + (swizzle!(anti_reverse.group0(), 1, 3, 3, 3) * swizzle!(self.group0(), 1, 0, 1, 2))),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group0()[3], 2)),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for MysteryDipoleInversion {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12        9        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       16       14        0
    //  no simd       28       29        0
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
            Simd32x4::from(0.0),
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
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for MysteryVersorEven {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       16        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       24       21        0
    //  no simd       36       36        0
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
            Simd32x4::from(0.0),
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
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       15        9        0
    //  no simd       36       36        0
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
            Simd32x4::from(0.0),
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
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for Plane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for PlaneOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for RoundPoint {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for RoundPointAtOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for Scalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for Sphere {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for SphereAtOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl AntiConstraintViolation for SphereOnOrigin {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for VersorEvenAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       45        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       36       51        0
    //  no simd       54       69        0
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
                    + (anti_reverse.group1()[3] * self.group2()[3])
                    - (anti_reverse.group2()[0] * self.group0()[0])
                    - (anti_reverse.group2()[1] * self.group0()[1])
                    - (anti_reverse.group2()[2] * self.group0()[2])
                    + (anti_reverse.group2()[3] * self.group1()[3])),
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
                    - (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    + (anti_reverse.group2()[3] * self.group0()[3])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       35        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       39       40        0
    //  no simd       51       55        0
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
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((anti_reverse.group0()[0] * self.group2()[3])
                    + (anti_reverse.group0()[1] * self.group2()[0])
                    + (anti_reverse.group0()[2] * self.group2()[1])
                    + (anti_reverse.group0()[3] * self.group2()[2])
                    - (anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group1()[3] * self.group2()[3])
                    - (anti_reverse.group2()[0] * self.group0()[1])
                    - (anti_reverse.group2()[0] * self.group1()[0])
                    - (anti_reverse.group2()[1] * self.group0()[2])
                    - (anti_reverse.group2()[1] * self.group1()[1])
                    - (anti_reverse.group2()[2] * self.group0()[3])
                    - (anti_reverse.group2()[2] * self.group1()[2])
                    + (anti_reverse.group2()[3] * self.group0()[0])
                    + (anti_reverse.group2()[3] * self.group1()[3])),
            ]),
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
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for VersorEvenAtOrigin {
    type Output = MysteryVersorEven;
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
        let geometric_anti_product = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            (Simd32x4::from([
                (-(anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2])),
                ((anti_reverse.group0()[2] * self.group1()[1]) + (anti_reverse.group0()[3] * self.group1()[0])),
                ((anti_reverse.group0()[1] * self.group1()[3]) + (anti_reverse.group0()[3] * self.group1()[1])),
                ((anti_reverse.group0()[2] * self.group1()[3]) + (anti_reverse.group0()[3] * self.group1()[2])),
            ]) - (swizzle!(anti_reverse.group0(), 0, 1, 2, 0) * swizzle!(self.group1(), 0, 2, 0, 1))
                + (swizzle!(anti_reverse.group0(), 3, 0, 0, 1) * swizzle!(self.group1(), 3, 3, 2, 0))
                - (swizzle!(anti_reverse.group1(), 0, 0, 0, 1) * swizzle!(self.group0(), 0, 3, 2, 0))
                - (swizzle!(anti_reverse.group1(), 1, 2, 1, 2) * swizzle!(self.group0(), 1, 1, 3, 3))
                - (swizzle!(anti_reverse.group1(), 2, 3, 3, 3) * swizzle!(self.group0(), 2, 0, 1, 2))
                + (swizzle!(anti_reverse.group1(), 3, 1, 2, 0) * swizzle!(self.group0(), 3, 2, 0, 1))),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-2.0 * (self.group0()[0] * self.group1()[0]) - 2.0 * (self.group0()[1] * self.group1()[1]) - 2.0 * (self.group0()[2] * self.group1()[2])
                + 2.0 * (self.group0()[3] * self.group1()[3])),
        );
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiConstraintViolation for VersorEvenOnOrigin {
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
        let anti_reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
        let geometric_anti_product = DualNum::from_groups(
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
        return DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([geometric_anti_product.group0()[0], (geometric_anti_product.group0()[1] - anti_scalar_product[e12345])]),
        );
    }
}
impl AntiConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       47        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       42       52        0
    //  no simd       54       67        0
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group1()[0]) - (anti_reverse.group0()[1] * self.group1()[1]) - (anti_reverse.group0()[2] * self.group1()[2])
                    + (anti_reverse.group0()[3] * self.group0()[3])
                    - (anti_reverse.group1()[0] * self.group0()[0])
                    - (anti_reverse.group1()[1] * self.group0()[1])
                    - (anti_reverse.group1()[2] * self.group0()[2])
                    + (anti_reverse.group1()[3] * self.group2()[3])
                    - (anti_reverse.group2()[0] * self.group2()[0])
                    - (anti_reverse.group2()[1] * self.group2()[1])
                    - (anti_reverse.group2()[2] * self.group2()[2])
                    + (anti_reverse.group2()[3] * self.group1()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[3] * self.group1()[3])
                    - (anti_reverse.group1()[0] * self.group2()[0])
                    - (anti_reverse.group1()[1] * self.group2()[1])
                    - (anti_reverse.group1()[2] * self.group2()[2])
                    + (anti_reverse.group1()[3] * self.group0()[3])
                    + (anti_reverse.group2()[0] * self.group1()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl AntiConstraintViolation for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       27        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       30       35        0
    //  no simd       51       59        0
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
            (Simd32x4::from([
                ((anti_reverse.group0()[0] * self.group0()[0]) * -1.0),
                ((anti_reverse.group2()[1] * self.group1()[2]) * -1.0),
                ((anti_reverse.group2()[2] * self.group1()[0]) * -1.0),
                ((anti_reverse.group2()[0] * self.group1()[1]) * -1.0),
            ]) + (Simd32x4::from([anti_reverse.group2()[0], anti_reverse.group0()[0], anti_reverse.group0()[0], anti_reverse.group0()[0]]) * swizzle!(self.group2(), 0, 0, 1, 2))
                + (Simd32x4::from([self.group1()[0], self.group1()[3], self.group2()[2], self.group2()[0]]) * swizzle!(anti_reverse.group1(), 0, 0, 0, 1))
                + (Simd32x4::from([self.group1()[1], self.group2()[1], self.group1()[3], self.group1()[3]]) * swizzle!(anti_reverse.group1(), 1, 2, 1, 2))
                - (Simd32x4::from([self.group1()[3], self.group2()[2], self.group2()[0], self.group2()[1]]) * swizzle!(anti_reverse.group1(), 3, 1, 2, 0))
                + (Simd32x4::from([self.group2()[1], self.group0()[0], self.group1()[2], self.group1()[0]]) * swizzle!(anti_reverse.group2(), 1, 0, 0, 1))
                + (Simd32x4::from([self.group2()[2], self.group1()[1], self.group0()[0], self.group0()[0]]) * swizzle!(anti_reverse.group2(), 2, 2, 1, 2))
                + (swizzle!(anti_reverse.group1(), 2, 3, 3, 3) * swizzle!(self.group1(), 2, 0, 1, 2))),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[0] * self.group2()[3]) + (anti_reverse.group0()[1] * self.group1()[0]) - (anti_reverse.group0()[1] * self.group2()[0])
                    + (anti_reverse.group0()[2] * self.group1()[1])
                    - (anti_reverse.group0()[2] * self.group2()[1])
                    + (anti_reverse.group0()[3] * self.group1()[2])
                    - (anti_reverse.group0()[3] * self.group2()[2])
                    + (anti_reverse.group1()[0] * self.group0()[1])
                    + (anti_reverse.group1()[1] * self.group0()[2])
                    + (anti_reverse.group1()[2] * self.group0()[3])
                    - (anti_reverse.group1()[3] * self.group2()[3])
                    + (anti_reverse.group2()[0] * self.group0()[1])
                    + (anti_reverse.group2()[1] * self.group0()[2])
                    + (anti_reverse.group2()[2] * self.group0()[3])
                    - (anti_reverse.group2()[3] * self.group0()[0])
                    + (anti_reverse.group2()[3] * self.group1()[3])),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            (-f32::powi(self.group0()[0], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2) - f32::powi(self.group1()[3], 2)
                + f32::powi(self.group2()[0], 2)
                + f32::powi(self.group2()[1], 2)
                + f32::powi(self.group2()[2], 2)),
        );
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (geometric_anti_product.group0()[0] - anti_scalar_product[e12345]),
                geometric_anti_product.group0()[1],
                geometric_anti_product.group0()[2],
                geometric_anti_product.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
        );
    }
}
impl AntiConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       49        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       39       54        0
    //  no simd       54       69        0
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
                    - (anti_reverse.group1()[3] * self.group2()[3])
                    + (anti_reverse.group2()[0] * self.group0()[0])
                    + (anti_reverse.group2()[1] * self.group0()[1])
                    + (anti_reverse.group2()[2] * self.group0()[2])
                    - (anti_reverse.group2()[3] * self.group1()[3])),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(anti_reverse.group0()[3] * self.group1()[3])
                    + (anti_reverse.group1()[0] * self.group2()[0])
                    + (anti_reverse.group1()[1] * self.group2()[1])
                    + (anti_reverse.group1()[2] * self.group2()[2])
                    - (anti_reverse.group1()[3] * self.group0()[3])
                    + (anti_reverse.group2()[0] * self.group1()[0])
                    + (anti_reverse.group2()[1] * self.group1()[1])
                    + (anti_reverse.group2()[2] * self.group1()[2])),
            ]),
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
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (geometric_anti_product.group0()[3] - anti_scalar_product[e12345])]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product.group2()[3]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
