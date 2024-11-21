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
impl std::ops::Div<constraint_violation> for DualNum {
    type Output = AntiScalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn constraint_violation(self) -> Self::Output {
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([f32::powi(self.group0()[0], 2), self.group0()[0] * self.group0()[1]]) * Simd32x2::from([1.0, 2.0]),
        );
        return AntiScalar::from_groups(/* e1234 */ geometric_product.group0()[1]);
    }
}
impl std::ops::Div<constraint_violation> for Flector {
    type Output = DualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Flector {
    type Output = DualNum;
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
        let reverse = Flector::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e423, e431, e412, e321 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (reverse.group1()[3] * self.group0()[3])
                    - (reverse.group0()[0] * self.group1()[0])
                    - (reverse.group0()[1] * self.group1()[1])
                    - (reverse.group0()[2] * self.group1()[2]),
            ]) + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([reverse.group0()[0], reverse.group1()[0]]))
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([reverse.group0()[1], reverse.group1()[1]]))
                + (Simd32x2::from(self.group0()[2]) * Simd32x2::from([reverse.group0()[2], reverse.group1()[2]]))
                - (Simd32x2::from(self.group1()[3]) * Simd32x2::from([reverse.group1()[3], reverse.group0()[3]])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            f32::powi(self.group0()[0], 2) + f32::powi(self.group0()[1], 2) + f32::powi(self.group0()[2], 2) - f32::powi(self.group1()[3], 2),
        );
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_product.group0()[0] - scalar_product[scalar], geometric_product.group0()[1]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for Horizon {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Horizon::from_groups(/* e321 */ self[e321] * -1.0);
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - scalar_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for Line {
    type Output = DualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Line {
    type Output = DualNum;
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
        let reverse = Line::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(reverse.group1()[0] * self.group0()[0]) - (reverse.group1()[1] * self.group0()[1]) - (reverse.group1()[2] * self.group0()[2]),
            ]) - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group1()[0], reverse.group0()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group1()[1], reverse.group0()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group1()[2], reverse.group0()[2]])),
        );
        let scalar_product = Scalar::from_groups(/* scalar */ -f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2));
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_product.group0()[0] - scalar_product[scalar], geometric_product.group0()[1]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for Motor {
    type Output = DualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        4        4        0
    // Totals...
    // yes simd       11       14        0
    //  no simd       15       18        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0] * -1.0, self.group0()[1] * -1.0, self.group0()[2] * -1.0, self.group0()[3]]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group1()[0] * -1.0, self.group1()[1] * -1.0, self.group1()[2] * -1.0, self.group1()[3]]),
        );
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (reverse.group1()[3] * self.group0()[3])
                    - (reverse.group1()[0] * self.group0()[0])
                    - (reverse.group1()[1] * self.group0()[1])
                    - (reverse.group1()[2] * self.group0()[2]),
            ]) + (Simd32x2::from(self.group1()[3]) * Simd32x2::from([reverse.group1()[3], reverse.group0()[3]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group1()[0], reverse.group0()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group1()[1], reverse.group0()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group1()[2], reverse.group0()[2]])),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            f32::powi(self.group1()[3], 2) - f32::powi(self.group1()[0], 2) - f32::powi(self.group1()[1], 2) - f32::powi(self.group1()[2], 2),
        );
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_product.group0()[0] - scalar_product[scalar], geometric_product.group0()[1]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for MultiVector {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for MultiVector {
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
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4() * Simd32x4::from(-1.0),
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (reverse.group0()[1] * self.group0()[0]) + (reverse.group4()[3] * self.group1()[3])
                    - (reverse.group3()[0] * self.group2()[0])
                    - (reverse.group3()[1] * self.group2()[1])
                    - (reverse.group3()[2] * self.group2()[2])
                    - (reverse.group1()[0] * self.group4()[0])
                    - (reverse.group1()[1] * self.group4()[1])
                    - (reverse.group1()[2] * self.group4()[2]),
            ]) + (Simd32x2::from(reverse.group0()[0]) * self.group0())
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([reverse.group1()[0], reverse.group4()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([reverse.group1()[1], reverse.group4()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([reverse.group1()[2], reverse.group4()[2]]))
                - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([reverse.group3()[0], reverse.group2()[0]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([reverse.group3()[1], reverse.group2()[1]]))
                - (Simd32x2::from(self.group3()[2]) * Simd32x2::from([reverse.group3()[2], reverse.group2()[2]]))
                - (Simd32x2::from(self.group4()[3]) * Simd32x2::from([reverse.group4()[3], reverse.group1()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse.group3()[1] * self.group1()[2]) + (self.group3()[0] * reverse.group4()[3]) - (self.group3()[1] * reverse.group1()[2]),
                (reverse.group3()[2] * self.group1()[0]) + (self.group3()[1] * reverse.group4()[3]) - (self.group3()[2] * reverse.group1()[0]),
                (reverse.group3()[0] * self.group1()[1]) + (self.group3()[2] * reverse.group4()[3]) - (self.group3()[0] * reverse.group1()[1]),
                (self.group2()[1] * reverse.group1()[1]) + (self.group2()[2] * reverse.group1()[2])
                    - (self.group0()[1] * reverse.group4()[3])
                    - (reverse.group2()[1] * self.group1()[1])
                    - (reverse.group2()[2] * self.group1()[2])
                    - (reverse.group3()[0] * self.group4()[0])
                    - (reverse.group3()[1] * self.group4()[1])
                    - (reverse.group3()[2] * self.group4()[2])
                    - (self.group3()[0] * reverse.group4()[0])
                    - (self.group3()[1] * reverse.group4()[1])
                    - (self.group3()[2] * reverse.group4()[2]),
            ]) + (Simd32x4::from(reverse.group0()[0]) * self.group1())
                + (Simd32x4::from(self.group0()[0]) * reverse.group1())
                + (Simd32x4::from(self.group4()[3]) * crate::swizzle!(reverse.group3(), 0, 1, 2).extend_to_4(reverse.group0()[1]))
                + (crate::swizzle!(self.group3(), 2, 0, 1).extend_to_4(self.group2()[0]) * crate::swizzle!(reverse.group1(), 1, 2, 0, 0))
                - (crate::swizzle!(reverse.group3(), 2, 0, 1).extend_to_4(reverse.group2()[0]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        let scalar_product = Scalar::from_groups(
            // scalar
            f32::powi(self.group0()[0], 2) + f32::powi(self.group1()[0], 2) + f32::powi(self.group1()[1], 2) + f32::powi(self.group1()[2], 2)
                - f32::powi(self.group3()[0], 2)
                - f32::powi(self.group3()[1], 2)
                - f32::powi(self.group3()[2], 2)
                - f32::powi(self.group4()[3], 2),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_product.group0()[0] - scalar_product[scalar], geometric_product.group0()[1]]),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Plane {
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
        let reverse = Plane::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse.group0()[3] * self.group0()[3] * -1.0);
        let scalar_product = Scalar::from_groups(/* scalar */ f32::powi(self.group0()[3], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - scalar_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for Point {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Point {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for Scalar {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for Scalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
