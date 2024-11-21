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
//  Average:        38      41       0
//  Maximum:       347     357       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        14      16       0
//  Average:        41      47       0
//  Maximum:       396     420       0
impl std::ops::Div<anti_constraint_violation> for AntiCircleRotor {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotor {
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
        let anti_reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[scalar]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[scalar] * self[scalar]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]) + (anti_reverse[e35] * self[e42])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]) + (anti_reverse[e15] * self[e43])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]) + (anti_reverse[e25] * self[e41])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[scalar], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDipoleInversion {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       86       92        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       87       95        0
    //  no simd       90      103        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e4]]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e321] * self[e321]) + (anti_reverse[e4] * self[e5]) + (anti_reverse[e5] * self[e4])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125]) + (anti_reverse[e5] * self[e321])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e321] * self[e5])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e435])
                    - (anti_reverse[e125] * self[e3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e423] * self[e5])
                    + (anti_reverse[e412] * self[e315])
                    + (anti_reverse[e415] * self[e321])
                    + (anti_reverse[e435] * self[e2])
                    + (anti_reverse[e315] * self[e412])
                    + (anti_reverse[e4] * self[e235])
                    + (anti_reverse[e3] * self[e425])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e2] * self[e435])
                    - (anti_reverse[e5] * self[e423]),
                (anti_reverse[e423] * self[e125])
                    + (anti_reverse[e431] * self[e5])
                    + (anti_reverse[e415] * self[e3])
                    + (anti_reverse[e425] * self[e321])
                    + (anti_reverse[e125] * self[e423])
                    + (anti_reverse[e4] * self[e315])
                    + (anti_reverse[e1] * self[e435])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e3] * self[e415])
                    - (anti_reverse[e5] * self[e431]),
                (anti_reverse[e431] * self[e235])
                    + (anti_reverse[e412] * self[e5])
                    + (anti_reverse[e425] * self[e1])
                    + (anti_reverse[e435] * self[e321])
                    + (anti_reverse[e235] * self[e431])
                    + (anti_reverse[e4] * self[e125])
                    + (anti_reverse[e2] * self[e415])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e1] * self[e425])
                    - (anti_reverse[e5] * self[e412]),
                (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3])
                    - (anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412])
                    - (anti_reverse[e4] * self[e321])
                    - (anti_reverse[e1] * self[e423])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412]),
            ]) + (Simd32x4::from(anti_reverse[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5]) + f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(
            // e4, e12345
            Simd32x2::from([self[e1234] * self[scalar], f32::powi(self[scalar], 2)]) * Simd32x2::from([-2.0, -1.0]),
        );
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([geometric_anti_product[e4], 0.0]));
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
        let anti_reverse = AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
        );
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e321] * self[e321]);
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_scalar_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiFlector {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
        let anti_reverse = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e5
            self.group1(),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e321] * self[e321]) - (anti_reverse[e1] * self[e1]) - (anti_reverse[e2] * self[e2]) - (anti_reverse[e3] * self[e3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e1] * self[e235]) + (anti_reverse[e2] * self[e315]) + (anti_reverse[e3] * self[e125]) + (anti_reverse[e5] * self[e321])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e3])
                    - (anti_reverse[e321] * self[e5]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiLine {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
            Simd32x4::from([self[e23] * -1.0, self[e31] * -1.0, self[e12] * -1.0, self[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e3215]]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e23] * self[e23]) + (anti_reverse[e31] * self[e31]) + (anti_reverse[e12] * self[e12]) - (anti_reverse[scalar] * self[scalar]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12])
                    - (anti_reverse[scalar] * self[e3215])
                    - (anti_reverse[e3215] * self[scalar]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiPlane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Circle {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e321] * self[e321])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423]),
                -(anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for CircleRotor {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e12345]]),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e321] * self[e321]) + (anti_reverse[e12345] * self[e12345])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e412] * self[e315]) + (anti_reverse[e415] * self[e321]) + (anti_reverse[e321] * self[e415]) + (anti_reverse[e315] * self[e412])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e125] * self[e431]),
                (anti_reverse[e423] * self[e125]) + (anti_reverse[e425] * self[e321]) + (anti_reverse[e321] * self[e425]) + (anti_reverse[e125] * self[e423])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412]),
                (anti_reverse[e431] * self[e235]) + (anti_reverse[e435] * self[e321]) + (anti_reverse[e321] * self[e435]) + (anti_reverse[e235] * self[e431])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423]),
                -(anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e321], 2) + f32::powi(self[e12345], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for Dipole {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Dipole {
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
        let anti_reverse = Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]) * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    - (anti_reverse[e45] * self[e45]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35]) + (anti_reverse[e23] * self[e45]) + (anti_reverse[e45] * self[e23]) + (anti_reverse[e35] * self[e42])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e25] * self[e43]),
                (anti_reverse[e43] * self[e15]) + (anti_reverse[e31] * self[e45]) + (anti_reverse[e45] * self[e31]) + (anti_reverse[e15] * self[e43])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e35] * self[e41]),
                (anti_reverse[e41] * self[e25]) + (anti_reverse[e12] * self[e45]) + (anti_reverse[e45] * self[e12]) + (anti_reverse[e25] * self[e41])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e15] * self[e42]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                - f32::powi(self[e45], 2),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DipoleInversion {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       82       88        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       84       92        0
    //  no simd       90      103        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    + (anti_reverse[e4235] * self[e4235])
                    + (anti_reverse[e4315] * self[e4315])
                    + (anti_reverse[e4125] * self[e4125])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[e1234] * self[e3215])
                    - (anti_reverse[e3215] * self[e1234]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12])
                    + (anti_reverse[e4235] * self[e15])
                    + (anti_reverse[e4315] * self[e25])
                    + (anti_reverse[e4125] * self[e35])
                    + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[e45] * self[e3215])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e15] * self[e1234])
                    + (anti_reverse[e35] * self[e42])
                    + (anti_reverse[e4125] * self[e31])
                    + (anti_reverse[e3215] * self[e41])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[e43] * self[e15])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e15] * self[e43])
                    + (anti_reverse[e25] * self[e1234])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e3215] * self[e42])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[e41] * self[e25])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e25] * self[e41])
                    + (anti_reverse[e35] * self[e1234])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e3215] * self[e43])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e41] * self[e4235])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e42] * self[e4315])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e43] * self[e4125])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43])
                    - (anti_reverse[e4235] * self[e41])
                    - (anti_reverse[e4315] * self[e42])
                    - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from(anti_reverse[e45]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]))
                - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[e45], 2)
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for DualNum {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
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
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([self[e4] * self[e12345], f32::powi(self[e12345], 2)]) * Simd32x2::from([2.0, 1.0]));
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([geometric_anti_product[e4], 0.0]));
    }
}
impl std::ops::Div<anti_constraint_violation> for FlatPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
        let anti_reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0));
        let geometric_anti_product = AntiScalar::from_groups(/* e12345 */ anti_reverse[e45] * self[e45] * -1.0);
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ f32::powi(self[e45], 2) * -1.0);
        return AntiScalar::from_groups(/* e12345 */ geometric_anti_product[e12345] - anti_scalar_product[e12345]);
    }
}
impl std::ops::Div<anti_constraint_violation> for Flector {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e4235] * self[e4235]) + (anti_reverse[e4315] * self[e4315]) + (anti_reverse[e4125] * self[e4125]) - (anti_reverse[e45] * self[e45]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e4235] * self[e15]) + (anti_reverse[e4315] * self[e25]) + (anti_reverse[e4125] * self[e35]) + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125])
                    - (anti_reverse[e45] * self[e3215]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[e45], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for Line {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
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
            Simd32x3::from([self[e415], self[e425], self[e435]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(/* e12345 */ -f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2));
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for Motor {
    type Output = Motor;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for Motor {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
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
            Simd32x4::from([self[e415] * -1.0, self[e425] * -1.0, self[e435] * -1.0, self[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e5]]),
        );
        let geometric_anti_product = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e12345] * self[e12345]) - (anti_reverse[e415] * self[e415]) - (anti_reverse[e425] * self[e425]) - (anti_reverse[e435] * self[e435]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e12345] * self[e5]) + (anti_reverse[e5] * self[e12345])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e125] * self[e435]),
            ]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      320      324        0
    //    simd2       16       16        0
    //    simd3        0        4        0
    //    simd4       11       13        0
    // Totals...
    // yes simd      347      357        0
    //  no simd      396      420        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]) * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group9(),
            // e1234
            self[e1234],
        );
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (anti_reverse[e12345] * self[scalar])
                    + (anti_reverse[e4235] * self[e1])
                    + (anti_reverse[e4315] * self[e2])
                    + (anti_reverse[e4125] * self[e3])
                    + (anti_reverse[e3215] * self[e4])
                    + (anti_reverse[e1234] * self[e5])
                    - (anti_reverse[e415] * self[e23])
                    - (anti_reverse[e425] * self[e31])
                    - (anti_reverse[e435] * self[e12])
                    - (anti_reverse[e321] * self[e45])
                    - (anti_reverse[e423] * self[e15])
                    - (anti_reverse[e431] * self[e25])
                    - (anti_reverse[e412] * self[e35])
                    - (anti_reverse[e235] * self[e41])
                    - (anti_reverse[e315] * self[e42])
                    - (anti_reverse[e125] * self[e43]),
                (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    + (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e321] * self[e321])
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3])
                    - (anti_reverse[e3215] * self[e1234])
                    - (anti_reverse[e1234] * self[e3215]),
            ]) + (Simd32x2::from(anti_reverse[e4]) * Simd32x2::from([self[e3215], self[e5]]))
                + (Simd32x2::from(anti_reverse[e5]) * Simd32x2::from([self[e1234], self[e4]]))
                + (Simd32x2::from(self[e12345]) * Simd32x2::from([anti_reverse[scalar], anti_reverse[e12345]]))
                + (Simd32x2::from(self[e4235]) * Simd32x2::from([anti_reverse[e1], anti_reverse[e4235]]))
                + (Simd32x2::from(self[e4315]) * Simd32x2::from([anti_reverse[e2], anti_reverse[e4315]]))
                + (Simd32x2::from(self[e4125]) * Simd32x2::from([anti_reverse[e3], anti_reverse[e4125]]))
                - (Simd32x2::from(anti_reverse[e45]) * Simd32x2::from([self[e321], self[e45]]))
                - (Simd32x2::from(self[e415]) * Simd32x2::from([anti_reverse[e23], anti_reverse[e415]]))
                - (Simd32x2::from(self[e425]) * Simd32x2::from([anti_reverse[e31], anti_reverse[e425]]))
                - (Simd32x2::from(self[e435]) * Simd32x2::from([anti_reverse[e12], anti_reverse[e435]]))
                - (Simd32x2::from(self[e423]) * Simd32x2::from([anti_reverse[e15], anti_reverse[e235]]))
                - (Simd32x2::from(self[e431]) * Simd32x2::from([anti_reverse[e25], anti_reverse[e315]]))
                - (Simd32x2::from(self[e412]) * Simd32x2::from([anti_reverse[e35], anti_reverse[e125]]))
                - (Simd32x2::from(self[e235]) * Simd32x2::from([anti_reverse[e41], anti_reverse[e423]]))
                - (Simd32x2::from(self[e315]) * Simd32x2::from([anti_reverse[e42], anti_reverse[e431]]))
                - (Simd32x2::from(self[e125]) * Simd32x2::from([anti_reverse[e43], anti_reverse[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[scalar] * self[e4235])
                    + (anti_reverse[e3] * self[e425])
                    + (anti_reverse[e4] * self[e235])
                    + (anti_reverse[e35] * self[e42])
                    + (anti_reverse[e45] * self[e23])
                    + (anti_reverse[e42] * self[e35])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e415] * self[e321])
                    + (anti_reverse[e435] * self[e2])
                    + (anti_reverse[e423] * self[e5])
                    + (anti_reverse[e412] * self[e315])
                    + (anti_reverse[e315] * self[e412])
                    + (anti_reverse[e4235] * self[scalar])
                    + (anti_reverse[e4125] * self[e31])
                    + (anti_reverse[e3215] * self[e41])
                    - (anti_reverse[e2] * self[e435])
                    - (anti_reverse[e5] * self[e423])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[scalar] * self[e4315])
                    + (anti_reverse[e1] * self[e435])
                    + (anti_reverse[e4] * self[e315])
                    + (anti_reverse[e15] * self[e43])
                    + (anti_reverse[e45] * self[e31])
                    + (anti_reverse[e43] * self[e15])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e415] * self[e3])
                    + (anti_reverse[e425] * self[e321])
                    + (anti_reverse[e423] * self[e125])
                    + (anti_reverse[e431] * self[e5])
                    + (anti_reverse[e125] * self[e423])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e4315] * self[scalar])
                    + (anti_reverse[e3215] * self[e42])
                    - (anti_reverse[e3] * self[e415])
                    - (anti_reverse[e5] * self[e431])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[scalar] * self[e4125])
                    + (anti_reverse[e2] * self[e415])
                    + (anti_reverse[e4] * self[e125])
                    + (anti_reverse[e25] * self[e41])
                    + (anti_reverse[e45] * self[e12])
                    + (anti_reverse[e41] * self[e25])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e425] * self[e1])
                    + (anti_reverse[e435] * self[e321])
                    + (anti_reverse[e431] * self[e235])
                    + (anti_reverse[e412] * self[e5])
                    + (anti_reverse[e235] * self[e431])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e4125] * self[scalar])
                    + (anti_reverse[e3215] * self[e43])
                    - (anti_reverse[e1] * self[e425])
                    - (anti_reverse[e5] * self[e412])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e41] * self[e4235])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e42] * self[e4315])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e43] * self[e4125])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43])
                    + (anti_reverse[e423] * self[e1])
                    + (anti_reverse[e431] * self[e2])
                    + (anti_reverse[e412] * self[e3])
                    - (anti_reverse[scalar] * self[e1234])
                    - (anti_reverse[e1] * self[e423])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412])
                    - (anti_reverse[e4] * self[e321])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412])
                    - (anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e4235] * self[e41])
                    - (anti_reverse[e4315] * self[e42])
                    - (anti_reverse[e4125] * self[e43])
                    - (anti_reverse[e1234] * self[e45]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + (Simd32x4::from(anti_reverse[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([anti_reverse[e1], anti_reverse[e2], anti_reverse[e3], anti_reverse[e4]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([anti_reverse[e15], anti_reverse[e25], anti_reverse[e35], anti_reverse[e45]]))
                - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]])),
            // e5
            (anti_reverse[e12345] * self[e5])
                + (anti_reverse[e1] * self[e235])
                + (anti_reverse[e2] * self[e315])
                + (anti_reverse[e3] * self[e125])
                + (anti_reverse[e5] * self[e12345])
                + (anti_reverse[e5] * self[e321])
                + (anti_reverse[e15] * self[e23])
                + (anti_reverse[e25] * self[e31])
                + (anti_reverse[e35] * self[e12])
                + (anti_reverse[e23] * self[e15])
                + (anti_reverse[e31] * self[e25])
                + (anti_reverse[e12] * self[e35])
                + (anti_reverse[e4235] * self[e15])
                + (anti_reverse[e4315] * self[e25])
                + (anti_reverse[e4125] * self[e35])
                + (anti_reverse[e3215] * self[e45])
                - (anti_reverse[scalar] * self[e3215])
                - (anti_reverse[e15] * self[e4235])
                - (anti_reverse[e25] * self[e4315])
                - (anti_reverse[e35] * self[e4125])
                - (anti_reverse[e45] * self[e3215])
                - (anti_reverse[e415] * self[e235])
                - (anti_reverse[e425] * self[e315])
                - (anti_reverse[e435] * self[e125])
                - (anti_reverse[e321] * self[e5])
                - (anti_reverse[e235] * self[e1])
                - (anti_reverse[e235] * self[e415])
                - (anti_reverse[e315] * self[e2])
                - (anti_reverse[e315] * self[e425])
                - (anti_reverse[e125] * self[e3])
                - (anti_reverse[e125] * self[e435])
                - (anti_reverse[e3215] * self[scalar]),
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
            Simd32x4::from([
                (anti_reverse[e2] * self[e12])
                    + (anti_reverse[e15] * self[e4])
                    + (anti_reverse[e35] * self[e431])
                    + (anti_reverse[e42] * self[e125])
                    + (anti_reverse[e31] * self[e3])
                    + (anti_reverse[e415] * self[e45])
                    + (anti_reverse[e435] * self[e4315])
                    + (anti_reverse[e431] * self[e35])
                    + (anti_reverse[e235] * self[e1234])
                    + (anti_reverse[e125] * self[e42])
                    + (anti_reverse[e4125] * self[e425])
                    - (anti_reverse[scalar] * self[e1])
                    - (anti_reverse[e1] * self[scalar])
                    - (anti_reverse[e3] * self[e31])
                    - (anti_reverse[e4] * self[e15])
                    - (anti_reverse[e25] * self[e412])
                    - (anti_reverse[e41] * self[e5])
                    - (anti_reverse[e43] * self[e315])
                    - (anti_reverse[e23] * self[e321])
                    - (anti_reverse[e12] * self[e2])
                    - (anti_reverse[e425] * self[e4125])
                    - (anti_reverse[e423] * self[e3215])
                    - (anti_reverse[e412] * self[e25])
                    - (anti_reverse[e315] * self[e43])
                    - (anti_reverse[e4315] * self[e435])
                    - (anti_reverse[e1234] * self[e235]),
                (anti_reverse[e3] * self[e23])
                    + (anti_reverse[e15] * self[e412])
                    + (anti_reverse[e25] * self[e4])
                    + (anti_reverse[e43] * self[e235])
                    + (anti_reverse[e12] * self[e1])
                    + (anti_reverse[e415] * self[e4125])
                    + (anti_reverse[e425] * self[e45])
                    + (anti_reverse[e412] * self[e15])
                    + (anti_reverse[e235] * self[e43])
                    + (anti_reverse[e315] * self[e1234])
                    + (anti_reverse[e4235] * self[e435])
                    - (anti_reverse[scalar] * self[e2])
                    - (anti_reverse[e1] * self[e12])
                    - (anti_reverse[e2] * self[scalar])
                    - (anti_reverse[e4] * self[e25])
                    - (anti_reverse[e35] * self[e423])
                    - (anti_reverse[e41] * self[e125])
                    - (anti_reverse[e42] * self[e5])
                    - (anti_reverse[e23] * self[e3])
                    - (anti_reverse[e31] * self[e321])
                    - (anti_reverse[e435] * self[e4235])
                    - (anti_reverse[e423] * self[e35])
                    - (anti_reverse[e431] * self[e3215])
                    - (anti_reverse[e125] * self[e41])
                    - (anti_reverse[e4125] * self[e415])
                    - (anti_reverse[e1234] * self[e315]),
                (anti_reverse[e1] * self[e31])
                    + (anti_reverse[e25] * self[e423])
                    + (anti_reverse[e35] * self[e4])
                    + (anti_reverse[e41] * self[e315])
                    + (anti_reverse[e23] * self[e2])
                    + (anti_reverse[e425] * self[e4235])
                    + (anti_reverse[e435] * self[e45])
                    + (anti_reverse[e423] * self[e25])
                    + (anti_reverse[e315] * self[e41])
                    + (anti_reverse[e125] * self[e1234])
                    + (anti_reverse[e4315] * self[e415])
                    - (anti_reverse[scalar] * self[e3])
                    - (anti_reverse[e2] * self[e23])
                    - (anti_reverse[e3] * self[scalar])
                    - (anti_reverse[e4] * self[e35])
                    - (anti_reverse[e15] * self[e431])
                    - (anti_reverse[e42] * self[e235])
                    - (anti_reverse[e43] * self[e5])
                    - (anti_reverse[e31] * self[e1])
                    - (anti_reverse[e12] * self[e321])
                    - (anti_reverse[e415] * self[e4315])
                    - (anti_reverse[e431] * self[e15])
                    - (anti_reverse[e412] * self[e3215])
                    - (anti_reverse[e235] * self[e42])
                    - (anti_reverse[e4235] * self[e425])
                    - (anti_reverse[e1234] * self[e125]),
                (anti_reverse[scalar] * self[e5])
                    + (anti_reverse[e1] * self[e15])
                    + (anti_reverse[e2] * self[e25])
                    + (anti_reverse[e3] * self[e35])
                    + (anti_reverse[e235] * self[e4235])
                    + (anti_reverse[e315] * self[e4315])
                    + (anti_reverse[e125] * self[e4125])
                    - (anti_reverse[e5] * self[e45])
                    - (anti_reverse[e15] * self[e1])
                    - (anti_reverse[e15] * self[e415])
                    - (anti_reverse[e25] * self[e2])
                    - (anti_reverse[e25] * self[e425])
                    - (anti_reverse[e35] * self[e3])
                    - (anti_reverse[e35] * self[e435])
                    - (anti_reverse[e23] * self[e235])
                    - (anti_reverse[e31] * self[e315])
                    - (anti_reverse[e12] * self[e125])
                    - (anti_reverse[e415] * self[e15])
                    - (anti_reverse[e425] * self[e25])
                    - (anti_reverse[e435] * self[e35])
                    - (anti_reverse[e235] * self[e23])
                    - (anti_reverse[e315] * self[e31])
                    - (anti_reverse[e125] * self[e12])
                    - (anti_reverse[e4235] * self[e235])
                    - (anti_reverse[e4315] * self[e315])
                    - (anti_reverse[e4125] * self[e125]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                + (Simd32x4::from(anti_reverse[e5]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]))
                + (Simd32x4::from(anti_reverse[e45]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                + (Simd32x4::from(anti_reverse[e3215]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([anti_reverse[e4235], anti_reverse[e4315], anti_reverse[e4125], anti_reverse[e3215]]))
                - (Simd32x4::from(anti_reverse[e321]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]])),
            // e1234
            (anti_reverse[scalar] * self[e4])
                + (anti_reverse[e12345] * self[e1234])
                + (anti_reverse[e4] * self[scalar])
                + (anti_reverse[e4] * self[e45])
                + (anti_reverse[e41] * self[e1])
                + (anti_reverse[e42] * self[e2])
                + (anti_reverse[e43] * self[e3])
                + (anti_reverse[e321] * self[e1234])
                + (anti_reverse[e4235] * self[e423])
                + (anti_reverse[e4315] * self[e431])
                + (anti_reverse[e4125] * self[e412])
                + (anti_reverse[e1234] * self[e12345])
                - (anti_reverse[e1] * self[e41])
                - (anti_reverse[e2] * self[e42])
                - (anti_reverse[e3] * self[e43])
                - (anti_reverse[e45] * self[e4])
                - (anti_reverse[e41] * self[e415])
                - (anti_reverse[e42] * self[e425])
                - (anti_reverse[e43] * self[e435])
                - (anti_reverse[e23] * self[e423])
                - (anti_reverse[e31] * self[e431])
                - (anti_reverse[e12] * self[e412])
                - (anti_reverse[e415] * self[e41])
                - (anti_reverse[e425] * self[e42])
                - (anti_reverse[e435] * self[e43])
                - (anti_reverse[e423] * self[e23])
                - (anti_reverse[e423] * self[e4235])
                - (anti_reverse[e431] * self[e31])
                - (anti_reverse[e431] * self[e4315])
                - (anti_reverse[e412] * self[e12])
                - (anti_reverse[e412] * self[e4125])
                - (anti_reverse[e1234] * self[e321]),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e4] * self[e5])
                + 2.0 * (self[e15] * self[e41])
                + 2.0 * (self[e25] * self[e42])
                + 2.0 * (self[e35] * self[e43])
                + f32::powi(self[e12345], 2)
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e321], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - f32::powi(self[e45], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125])
                - 2.0 * (self[e3215] * self[e1234]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_anti_product[scalar], geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e1, e2, e3, e4
            geometric_anti_product.group1(),
            // e5
            geometric_anti_product[e5],
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
            geometric_anti_product[e1234],
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Plane {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for RoundPoint {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Scalar {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for Scalar {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Scalar {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Sphere {
    type Output = Scalar;
    fn anti_constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<anti_constraint_violation> for VersorEven {
    fn div_assign(&mut self, _rhs: anti_constraint_violation) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90       98        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       93      102        0
    //  no simd      102      114        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] * -1.0, self[e431] * -1.0, self[e412] * -1.0, self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] * -1.0, self[e315] * -1.0, self[e125] * -1.0, self[e5]]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e12345] * self[e12345]) + (anti_reverse[e321] * self[e321]) + (anti_reverse[e5] * self[e4]) + (anti_reverse[e4] * self[e5])
                    - (anti_reverse[e423] * self[e235])
                    - (anti_reverse[e431] * self[e315])
                    - (anti_reverse[e412] * self[e125])
                    - (anti_reverse[e415] * self[e415])
                    - (anti_reverse[e425] * self[e425])
                    - (anti_reverse[e435] * self[e435])
                    - (anti_reverse[e235] * self[e423])
                    - (anti_reverse[e315] * self[e431])
                    - (anti_reverse[e125] * self[e412])
                    - (anti_reverse[e1] * self[e1])
                    - (anti_reverse[e2] * self[e2])
                    - (anti_reverse[e3] * self[e3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e12345] * self[e5])
                    + (anti_reverse[e5] * self[e12345])
                    + (anti_reverse[e5] * self[e321])
                    + (anti_reverse[e1] * self[e235])
                    + (anti_reverse[e2] * self[e315])
                    + (anti_reverse[e3] * self[e125])
                    - (anti_reverse[e415] * self[e235])
                    - (anti_reverse[e425] * self[e315])
                    - (anti_reverse[e435] * self[e125])
                    - (anti_reverse[e321] * self[e5])
                    - (anti_reverse[e235] * self[e415])
                    - (anti_reverse[e235] * self[e1])
                    - (anti_reverse[e315] * self[e425])
                    - (anti_reverse[e315] * self[e2])
                    - (anti_reverse[e125] * self[e435])
                    - (anti_reverse[e125] * self[e3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e423] * self[e5])
                    + (anti_reverse[e412] * self[e315])
                    + (anti_reverse[e415] * self[e321])
                    + (anti_reverse[e435] * self[e2])
                    + (anti_reverse[e315] * self[e412])
                    + (anti_reverse[e3] * self[e425])
                    + (anti_reverse[e4] * self[e235])
                    - (anti_reverse[e431] * self[e125])
                    - (anti_reverse[e425] * self[e3])
                    - (anti_reverse[e235] * self[e4])
                    - (anti_reverse[e125] * self[e431])
                    - (anti_reverse[e5] * self[e423])
                    - (anti_reverse[e2] * self[e435]),
                (anti_reverse[e423] * self[e125])
                    + (anti_reverse[e431] * self[e5])
                    + (anti_reverse[e415] * self[e3])
                    + (anti_reverse[e425] * self[e321])
                    + (anti_reverse[e125] * self[e423])
                    + (anti_reverse[e1] * self[e435])
                    + (anti_reverse[e4] * self[e315])
                    - (anti_reverse[e412] * self[e235])
                    - (anti_reverse[e435] * self[e1])
                    - (anti_reverse[e235] * self[e412])
                    - (anti_reverse[e315] * self[e4])
                    - (anti_reverse[e5] * self[e431])
                    - (anti_reverse[e3] * self[e415]),
                (anti_reverse[e431] * self[e235])
                    + (anti_reverse[e412] * self[e5])
                    + (anti_reverse[e425] * self[e1])
                    + (anti_reverse[e435] * self[e321])
                    + (anti_reverse[e235] * self[e431])
                    + (anti_reverse[e2] * self[e415])
                    + (anti_reverse[e4] * self[e125])
                    - (anti_reverse[e423] * self[e315])
                    - (anti_reverse[e415] * self[e2])
                    - (anti_reverse[e315] * self[e423])
                    - (anti_reverse[e125] * self[e4])
                    - (anti_reverse[e5] * self[e412])
                    - (anti_reverse[e1] * self[e425]),
                (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3])
                    - (anti_reverse[e423] * self[e415])
                    - (anti_reverse[e431] * self[e425])
                    - (anti_reverse[e412] * self[e435])
                    - (anti_reverse[e415] * self[e423])
                    - (anti_reverse[e425] * self[e431])
                    - (anti_reverse[e435] * self[e412])
                    - (anti_reverse[e1] * self[e423])
                    - (anti_reverse[e2] * self[e431])
                    - (anti_reverse[e3] * self[e412])
                    - (anti_reverse[e4] * self[e321]),
            ]) + (Simd32x4::from(anti_reverse[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + (Simd32x4::from(anti_reverse[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([anti_reverse[e1], anti_reverse[e2], anti_reverse[e3], anti_reverse[e4]])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e5] * self[e4]) + f32::powi(self[e12345], 2) + f32::powi(self[e321], 2)
                - f32::powi(self[e415], 2)
                - f32::powi(self[e425], 2)
                - f32::powi(self[e435], 2)
                - f32::powi(self[e1], 2)
                - f32::powi(self[e2], 2)
                - f32::powi(self[e3], 2)
                - 2.0 * (self[e423] * self[e235])
                - 2.0 * (self[e431] * self[e315])
                - 2.0 * (self[e412] * self[e125]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
impl std::ops::Div<anti_constraint_violation> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: anti_constraint_violation) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       94      102        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       96      105        0
    //  no simd      102      114        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] * -1.0, self[e42] * -1.0, self[e43] * -1.0, self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_anti_product = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e41] * self[e15])
                    + (anti_reverse[e42] * self[e25])
                    + (anti_reverse[e43] * self[e35])
                    + (anti_reverse[e23] * self[e23])
                    + (anti_reverse[e31] * self[e31])
                    + (anti_reverse[e12] * self[e12])
                    + (anti_reverse[e15] * self[e41])
                    + (anti_reverse[e25] * self[e42])
                    + (anti_reverse[e35] * self[e43])
                    + (anti_reverse[e4235] * self[e4235])
                    + (anti_reverse[e4315] * self[e4315])
                    + (anti_reverse[e4125] * self[e4125])
                    - (anti_reverse[scalar] * self[scalar])
                    - (anti_reverse[e45] * self[e45])
                    - (anti_reverse[e1234] * self[e3215])
                    - (anti_reverse[e3215] * self[e1234]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (anti_reverse[e23] * self[e15])
                    + (anti_reverse[e31] * self[e25])
                    + (anti_reverse[e12] * self[e35])
                    + (anti_reverse[e15] * self[e23])
                    + (anti_reverse[e25] * self[e31])
                    + (anti_reverse[e35] * self[e12])
                    + (anti_reverse[e4235] * self[e15])
                    + (anti_reverse[e4315] * self[e25])
                    + (anti_reverse[e4125] * self[e35])
                    + (anti_reverse[e3215] * self[e45])
                    - (anti_reverse[scalar] * self[e3215])
                    - (anti_reverse[e45] * self[e3215])
                    - (anti_reverse[e15] * self[e4235])
                    - (anti_reverse[e25] * self[e4315])
                    - (anti_reverse[e35] * self[e4125])
                    - (anti_reverse[e3215] * self[scalar]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse[e42] * self[e35])
                    + (anti_reverse[scalar] * self[e4235])
                    + (anti_reverse[e23] * self[e45])
                    + (anti_reverse[e12] * self[e4315])
                    + (anti_reverse[e15] * self[e1234])
                    + (anti_reverse[e35] * self[e42])
                    + (anti_reverse[e4235] * self[scalar])
                    + (anti_reverse[e4125] * self[e31])
                    + (anti_reverse[e3215] * self[e41])
                    - (anti_reverse[e41] * self[e3215])
                    - (anti_reverse[e43] * self[e25])
                    - (anti_reverse[e31] * self[e4125])
                    - (anti_reverse[e25] * self[e43])
                    - (anti_reverse[e4315] * self[e12]),
                (anti_reverse[e43] * self[e15])
                    + (anti_reverse[scalar] * self[e4315])
                    + (anti_reverse[e23] * self[e4125])
                    + (anti_reverse[e31] * self[e45])
                    + (anti_reverse[e15] * self[e43])
                    + (anti_reverse[e25] * self[e1234])
                    + (anti_reverse[e4235] * self[e12])
                    + (anti_reverse[e4315] * self[scalar])
                    + (anti_reverse[e3215] * self[e42])
                    - (anti_reverse[e41] * self[e35])
                    - (anti_reverse[e42] * self[e3215])
                    - (anti_reverse[e12] * self[e4235])
                    - (anti_reverse[e35] * self[e41])
                    - (anti_reverse[e4125] * self[e23]),
                (anti_reverse[e41] * self[e25])
                    + (anti_reverse[scalar] * self[e4125])
                    + (anti_reverse[e31] * self[e4235])
                    + (anti_reverse[e12] * self[e45])
                    + (anti_reverse[e25] * self[e41])
                    + (anti_reverse[e35] * self[e1234])
                    + (anti_reverse[e4315] * self[e23])
                    + (anti_reverse[e4125] * self[scalar])
                    + (anti_reverse[e3215] * self[e43])
                    - (anti_reverse[e42] * self[e15])
                    - (anti_reverse[e43] * self[e3215])
                    - (anti_reverse[e23] * self[e4315])
                    - (anti_reverse[e15] * self[e42])
                    - (anti_reverse[e4235] * self[e31]),
                (anti_reverse[e41] * self[e23])
                    + (anti_reverse[e41] * self[e4235])
                    + (anti_reverse[e42] * self[e31])
                    + (anti_reverse[e42] * self[e4315])
                    + (anti_reverse[e43] * self[e12])
                    + (anti_reverse[e43] * self[e4125])
                    + (anti_reverse[e23] * self[e41])
                    + (anti_reverse[e31] * self[e42])
                    + (anti_reverse[e12] * self[e43])
                    - (anti_reverse[scalar] * self[e1234])
                    - (anti_reverse[e1234] * self[e45])
                    - (anti_reverse[e4235] * self[e41])
                    - (anti_reverse[e4315] * self[e42])
                    - (anti_reverse[e4125] * self[e43]),
            ]) + (Simd32x4::from(anti_reverse[e45]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]))
                - (Simd32x4::from(anti_reverse[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]])),
        );
        let anti_scalar_product = AntiScalar::from_groups(
            // e12345
            2.0 * (self[e41] * self[e15])
                + 2.0 * (self[e42] * self[e25])
                + 2.0 * (self[e43] * self[e35])
                + f32::powi(self[e23], 2)
                + f32::powi(self[e31], 2)
                + f32::powi(self[e12], 2)
                + f32::powi(self[e4235], 2)
                + f32::powi(self[e4315], 2)
                + f32::powi(self[e4125], 2)
                - f32::powi(self[scalar], 2)
                - f32::powi(self[e45], 2)
                - 2.0 * (self[e1234] * self[e3215]),
        );
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e12345] - anti_scalar_product[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, geometric_anti_product[e5]]),
            // e1, e2, e3, e4
            geometric_anti_product.group3(),
        );
    }
}
