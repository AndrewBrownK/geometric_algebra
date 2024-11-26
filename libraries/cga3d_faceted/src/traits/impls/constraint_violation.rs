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
//   Median:        12      12       0
//  Average:        24      27       0
//  Maximum:       347     357       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        14      18       0
//  Average:        26      33       0
//  Maximum:       396     420       0
impl std::ops::Div<constraint_violation> for AntiCircleOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
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
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([-(reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43]), 0.0])
                - (Simd32x2::from(self[e23]) * Simd32x2::from([reverse[e41], reverse[e23]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([reverse[e42], reverse[e31]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([reverse[e43], reverse[e12]])),
        );
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for AntiCircleRotor {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       47        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       43       50        0
    //  no simd       43       58        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e45] * self[e45]) + (reverse[scalar] * self[scalar])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]) + (reverse[e35] * self[e42])
                    - (reverse[e43] * self[e25])
                    - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]) + (reverse[e15] * self[e43])
                    - (reverse[e41] * self[e35])
                    - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e25] * self[e41])
                    - (reverse[e42] * self[e15])
                    - (reverse[e15] * self[e42]),
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) + f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       40        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       35       43        0
    //  no simd       35       50        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[scalar] * self[scalar])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) - (reverse[e43] * self[e25]) - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) - (reverse[e41] * self[e35]) - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]) - (reverse[e42] * self[e15]) - (reverse[e15] * self[e42]),
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       12       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiCircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       17       19        0
    //  no simd       17       25        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[e45] * self[e45]) + (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]),
                (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]),
                (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]),
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) + f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiCircleRotorOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        4        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9        9        0
    //  no simd       12       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([-(reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43]), reverse[scalar] * self[scalar]])
                - (Simd32x2::from(self[e23]) * Simd32x2::from([reverse[e41], reverse[e23]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([reverse[e42], reverse[e31]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([reverse[e43], reverse[e12]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for AntiDipoleInversion {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       82       85        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       84       90        0
    //  no simd       90      104        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    + (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    - (reverse[e321] * self[e321])
                    - (reverse[e4] * self[e5])
                    - (reverse[e5] * self[e4]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e423] * self[e1])
                    + (reverse[e431] * self[e425])
                    + (reverse[e431] * self[e2])
                    + (reverse[e412] * self[e435])
                    + (reverse[e412] * self[e3])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412])
                    + (reverse[e321] * self[e4])
                    - (reverse[e4] * self[e321])
                    - (reverse[e1] * self[e423])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315])
                    + (reverse[e425] * self[e3])
                    + (reverse[e321] * self[e415])
                    + (reverse[e235] * self[e4])
                    + (reverse[e315] * self[e412])
                    + (reverse[e2] * self[e435])
                    + (reverse[e5] * self[e423])
                    - (reverse[e431] * self[e125])
                    - (reverse[e435] * self[e2])
                    - (reverse[e125] * self[e431])
                    - (reverse[e4] * self[e235])
                    - (reverse[e3] * self[e425]),
                (reverse[e423] * self[e125])
                    + (reverse[e435] * self[e1])
                    + (reverse[e321] * self[e425])
                    + (reverse[e315] * self[e4])
                    + (reverse[e125] * self[e423])
                    + (reverse[e3] * self[e415])
                    + (reverse[e5] * self[e431])
                    - (reverse[e412] * self[e235])
                    - (reverse[e415] * self[e3])
                    - (reverse[e235] * self[e412])
                    - (reverse[e4] * self[e315])
                    - (reverse[e1] * self[e435]),
                (reverse[e431] * self[e235])
                    + (reverse[e415] * self[e2])
                    + (reverse[e321] * self[e435])
                    + (reverse[e235] * self[e431])
                    + (reverse[e125] * self[e4])
                    + (reverse[e1] * self[e425])
                    + (reverse[e5] * self[e412])
                    - (reverse[e423] * self[e315])
                    - (reverse[e425] * self[e1])
                    - (reverse[e315] * self[e423])
                    - (reverse[e4] * self[e125])
                    - (reverse[e2] * self[e415]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435])
                    + (reverse[e1] * self[e235])
                    + (reverse[e2] * self[e315])
                    + (reverse[e3] * self[e125])
                    - (reverse[e235] * self[e1])
                    - (reverse[e315] * self[e2])
                    - (reverse[e125] * self[e3]),
            ]) + (Simd32x4::from(self[e321]) * Simd32x4::from([reverse[e415], reverse[e425], reverse[e435], reverse[e5]]))
                - (Simd32x4::from(self[e5]) * Simd32x4::from([reverse[e423], reverse[e431], reverse[e412], reverse[e321]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e4] * self[e5]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       35        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       38       38        0
    //  no simd       41       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2(),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    - (reverse[e321] * self[e321]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e425] * self[e3]) + (reverse[e321] * self[e415]) + (reverse[e2] * self[e435]) - (reverse[e435] * self[e2]) - (reverse[e3] * self[e425]),
                (reverse[e435] * self[e1]) + (reverse[e321] * self[e425]) + (reverse[e3] * self[e415]) - (reverse[e415] * self[e3]) - (reverse[e1] * self[e435]),
                (reverse[e415] * self[e2]) + (reverse[e321] * self[e435]) + (reverse[e1] * self[e425]) - (reverse[e425] * self[e1]) - (reverse[e2] * self[e415]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435])
                    + (reverse[e1] * self[e235])
                    + (reverse[e2] * self[e315])
                    + (reverse[e3] * self[e125])
                    - (reverse[e321] * self[e5])
                    - (reverse[e235] * self[e1])
                    - (reverse[e315] * self[e2])
                    - (reverse[e125] * self[e3]),
            ]) + (Simd32x4::from(self[e321]) * Simd32x4::from([reverse[e415], reverse[e425], reverse[e435], reverse[e5]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e321], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiDipoleInversionOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1());
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([(reverse[e321] * self[e4]) - (reverse[e1] * self[e423]) - (reverse[e2] * self[e431]) - (reverse[e3] * self[e412]), 0.0])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([reverse[e423], reverse[e1]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([reverse[e431], reverse[e2]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([reverse[e412], reverse[e3]]))
                - (Simd32x2::from(self[e321]) * Simd32x2::from([reverse[e4], reverse[e321]])),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       55        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       48       58        0
    //  no simd       48       66        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e5] * self[e4])
                    - (reverse[e4] * self[e5]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315]) + (reverse[e5] * self[e423]) + (reverse[e235] * self[e4]) + (reverse[e315] * self[e412])
                    - (reverse[e423] * self[e5])
                    - (reverse[e431] * self[e125])
                    - (reverse[e125] * self[e431])
                    - (reverse[e4] * self[e235]),
                (reverse[e423] * self[e125]) + (reverse[e5] * self[e431]) + (reverse[e315] * self[e4]) + (reverse[e125] * self[e423])
                    - (reverse[e431] * self[e5])
                    - (reverse[e412] * self[e235])
                    - (reverse[e235] * self[e412])
                    - (reverse[e4] * self[e315]),
                (reverse[e431] * self[e235]) + (reverse[e5] * self[e412]) + (reverse[e235] * self[e431]) + (reverse[e125] * self[e4])
                    - (reverse[e423] * self[e315])
                    - (reverse[e412] * self[e5])
                    - (reverse[e315] * self[e423])
                    - (reverse[e4] * self[e125]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - 2.0 * (self[e5] * self[e4]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiDipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for AntiDualNum {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        use crate::elements::*;
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([self[e1234] * self[scalar], f32::powi(self[scalar], 2)]) * Simd32x2::from([2.0, 1.0]),
        );
        return NullSphereAtOrigin::from_groups(/* e1234 */ geometric_product[e1234]);
    }
}
impl std::ops::Div<constraint_violation> for AntiFlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiFlatOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlatOrigin::from_groups(/* e321 */ self[e321] * -1.0);
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for AntiFlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e321] * self[e321] * -1.0);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e321], 2) * -1.0);
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for AntiFlector {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e1] * self[e1]) + (reverse[e2] * self[e2]) + (reverse[e3] * self[e3]) - (reverse[e321] * self[e321]),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e1] * self[e235]) + (reverse[e2] * self[e315]) + (reverse[e3] * self[e125]) + (reverse[e5] * self[e321])
                    - (reverse[e235] * self[e1])
                    - (reverse[e315] * self[e2])
                    - (reverse[e125] * self[e3])
                    - (reverse[e321] * self[e5]),
            ]),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiFlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiFlectorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]));
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[e1] * self[e1]) + (reverse[e2] * self[e2]) + (reverse[e3] * self[e3]) - (reverse[e321] * self[e321]),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) - f32::powi(self[e321], 2));
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for AntiLine {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, -(reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12])]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiLineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0() * Simd32x3::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ -(reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]));
        let dot_product = Scalar::from_groups(/* scalar */ -f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for AntiMotor {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       14       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[scalar] * self[e3215]) + (reverse[e3215] * self[scalar])
                    - (reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiMotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMotorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for AntiMysteryCircleRotor {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar]);
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                reverse[e45] * self[e23],
                reverse[e45] * self[e31],
                reverse[e45] * self[e12],
            ]) + (Simd32x4::from(self[e45]) * crate::swizzle!(reverse.group0(), 3, 0, 1, 2)),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) + f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiMysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       26        0
    //  no simd       28       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1());
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    - (reverse[e321] * self[e321]),
                (reverse[e415] * self[e321]) + (reverse[e425] * self[e3]) + (reverse[e321] * self[e415]) + (reverse[e2] * self[e435])
                    - (reverse[e435] * self[e2])
                    - (reverse[e3] * self[e425]),
                (reverse[e425] * self[e321]) + (reverse[e435] * self[e1]) + (reverse[e321] * self[e425]) + (reverse[e3] * self[e415])
                    - (reverse[e415] * self[e3])
                    - (reverse[e1] * self[e435]),
                (reverse[e415] * self[e2]) + (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]) + (reverse[e1] * self[e425])
                    - (reverse[e425] * self[e1])
                    - (reverse[e2] * self[e415]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2)
                - f32::powi(self[e321], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for AntiPlane {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiPlane {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiPlaneOnOrigin {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiScalar {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for AntiSphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiSphereOnOrigin {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for AntiVersorEvenOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       15       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([
                (reverse[e1234] * self[scalar]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(reverse[scalar]) * Simd32x2::from([self[e1234], self[scalar]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([reverse[e41], reverse[e23]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([reverse[e42], reverse[e31]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([reverse[e43], reverse[e12]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2),
        );
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for Circle {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Circle {
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
        let reverse = Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e321] * self[e321]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315]) + (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]) + (reverse[e315] * self[e412])
                    - (reverse[e431] * self[e125])
                    - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]) + (reverse[e125] * self[e423])
                    - (reverse[e412] * self[e235])
                    - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]) + (reverse[e235] * self[e431])
                    - (reverse[e423] * self[e315])
                    - (reverse[e315] * self[e423]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315]) + (reverse[e315] * self[e412]) - (reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e125] * self[e423]) - (reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) - (reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleAtInfinity {
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
        let reverse = CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e321] * self[e321]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
                (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
                (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e321], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleAtOrigin {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412]),
                (reverse[e412] * self[e315]) + (reverse[e315] * self[e412]) - (reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e125] * self[e423]) - (reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) - (reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(/* scalar */ 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]));
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([(reverse[e415] * self[e423]) + (reverse[e425] * self[e431]) + (reverse[e435] * self[e412]), 0.0])
                + (Simd32x2::from(self[e415]) * Simd32x2::from([reverse[e423], reverse[e415]]))
                + (Simd32x2::from(self[e425]) * Simd32x2::from([reverse[e431], reverse[e425]]))
                + (Simd32x2::from(self[e435]) * Simd32x2::from([reverse[e412], reverse[e435]])),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for CircleOrthogonalOrigin {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleOrthogonalOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       19       32        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e321] * self[e321]),
                (reverse[e412] * self[e315]) + (reverse[e315] * self[e412]) - (reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e125] * self[e423]) - (reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) - (reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - f32::powi(self[e321], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleRotor {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       47        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       43       50        0
    //  no simd       43       58        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e321] * self[e321])
                    - (reverse[e12345] * self[e12345]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315]) + (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]) + (reverse[e315] * self[e412])
                    - (reverse[e431] * self[e125])
                    - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]) + (reverse[e125] * self[e423])
                    - (reverse[e412] * self[e235])
                    - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]) + (reverse[e235] * self[e431])
                    - (reverse[e423] * self[e315])
                    - (reverse[e315] * self[e423]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e12345], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       40        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       35       43        0
    //  no simd       35       50        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e12345] * self[e12345]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315]) + (reverse[e315] * self[e412]) - (reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e125] * self[e423]) - (reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) - (reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       12       17        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e12345] * self[e12345]),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       17       19        0
    //  no simd       17       25        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e321] * self[e321]) - (reverse[e12345] * self[e12345]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
                (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
                (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e321], 2) - f32::powi(self[e12345], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for CircleRotorOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        5        0
    //    simd2        3        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       10        0
    //  no simd       12       18        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([
                (reverse[e415] * self[e423]) + (reverse[e425] * self[e431]) + (reverse[e435] * self[e412]),
                (reverse[e12345] * self[e12345]) * -1.0,
            ]) + (Simd32x2::from(self[e415]) * Simd32x2::from([reverse[e423], reverse[e415]]))
                + (Simd32x2::from(self[e425]) * Simd32x2::from([reverse[e431], reverse[e425]]))
                + (Simd32x2::from(self[e435]) * Simd32x2::from([reverse[e412], reverse[e435]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for Dipole {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e45] * self[e45])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]) + (reverse[e35] * self[e42])
                    - (reverse[e43] * self[e25])
                    - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]) + (reverse[e15] * self[e43])
                    - (reverse[e41] * self[e35])
                    - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e25] * self[e41])
                    - (reverse[e42] * self[e15])
                    - (reverse[e15] * self[e42]),
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleAligningOrigin {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleAligningOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       19       32        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e45] * self[e45])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) - (reverse[e43] * self[e25]) - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) - (reverse[e41] * self[e35]) - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]) - (reverse[e42] * self[e15]) - (reverse[e15] * self[e42]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[e45] * self[e45]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]),
                (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]),
                (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]),
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleAtOrigin {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                -(reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) - (reverse[e43] * self[e25]) - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) - (reverse[e41] * self[e35]) - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]) - (reverse[e42] * self[e15]) - (reverse[e15] * self[e42]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(/* scalar */ -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]));
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleInversion {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       86       89        0
    //    simd3        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       87       93        0
    //  no simd       90      104        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e45] * self[e45]) + (reverse[e1234] * self[e3215]) + (reverse[e3215] * self[e1234])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e41] * self[e4235]) + (reverse[e42] * self[e4315]) + (reverse[e43] * self[e4125]) + (reverse[e45] * self[e1234])
                    - (reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e41] * self[e3215])
                    + (reverse[e42] * self[e35])
                    + (reverse[e31] * self[e4125])
                    + (reverse[e45] * self[e23])
                    + (reverse[e35] * self[e42])
                    + (reverse[e1234] * self[e15])
                    + (reverse[e4315] * self[e12])
                    - (reverse[e43] * self[e25])
                    - (reverse[e12] * self[e4315])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e25] * self[e43])
                    - (reverse[e4125] * self[e31])
                    - (reverse[e3215] * self[e41]),
                (reverse[e42] * self[e3215])
                    + (reverse[e43] * self[e15])
                    + (reverse[e12] * self[e4235])
                    + (reverse[e45] * self[e31])
                    + (reverse[e15] * self[e43])
                    + (reverse[e1234] * self[e25])
                    + (reverse[e4125] * self[e23])
                    - (reverse[e41] * self[e35])
                    - (reverse[e23] * self[e4125])
                    - (reverse[e25] * self[e1234])
                    - (reverse[e35] * self[e41])
                    - (reverse[e4235] * self[e12])
                    - (reverse[e3215] * self[e42]),
                (reverse[e41] * self[e25])
                    + (reverse[e43] * self[e3215])
                    + (reverse[e23] * self[e4315])
                    + (reverse[e45] * self[e12])
                    + (reverse[e25] * self[e41])
                    + (reverse[e1234] * self[e35])
                    + (reverse[e4235] * self[e31])
                    - (reverse[e42] * self[e15])
                    - (reverse[e31] * self[e4235])
                    - (reverse[e15] * self[e42])
                    - (reverse[e35] * self[e1234])
                    - (reverse[e4315] * self[e23])
                    - (reverse[e3215] * self[e43]),
                (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35])
                    - (reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e45] * self[e3215])
                    - (reverse[e15] * self[e23])
                    - (reverse[e15] * self[e4235])
                    - (reverse[e25] * self[e31])
                    - (reverse[e25] * self[e4315])
                    - (reverse[e35] * self[e12])
                    - (reverse[e35] * self[e4125]),
            ]) + (Simd32x4::from(self[e45]) * Simd32x4::from([reverse[e23], reverse[e31], reverse[e12], reverse[e3215]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[e3215]) + f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleInversionAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       60        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       54       62        0
    //  no simd       54       68        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e45] * self[e45]) + (reverse[e1234] * self[e3215]) + (reverse[e3215] * self[e1234])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e41] * self[e4235]) + (reverse[e42] * self[e4315]) + (reverse[e43] * self[e4125]) + (reverse[e45] * self[e1234])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e41] * self[e3215]) + (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15])
                    - (reverse[e43] * self[e25])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e25] * self[e43])
                    - (reverse[e3215] * self[e41]),
                (reverse[e42] * self[e3215]) + (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25])
                    - (reverse[e41] * self[e35])
                    - (reverse[e25] * self[e1234])
                    - (reverse[e35] * self[e41])
                    - (reverse[e3215] * self[e42]),
                (reverse[e41] * self[e25]) + (reverse[e43] * self[e3215]) + (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35])
                    - (reverse[e42] * self[e15])
                    - (reverse[e15] * self[e42])
                    - (reverse[e35] * self[e1234])
                    - (reverse[e3215] * self[e43]),
                (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35]) + (reverse[e3215] * self[e45])
                    - (reverse[e45] * self[e3215])
                    - (reverse[e15] * self[e4235])
                    - (reverse[e25] * self[e4315])
                    - (reverse[e35] * self[e4125]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[e3215]) + f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       35        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       38       38        0
    //  no simd       41       46        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[e45] * self[e45])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23]) + (reverse[e4315] * self[e12]) - (reverse[e12] * self[e4315]) - (reverse[e4125] * self[e31]),
                (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31]) + (reverse[e4125] * self[e23]) - (reverse[e23] * self[e4125]) - (reverse[e4235] * self[e12]),
                (reverse[e23] * self[e4315]) + (reverse[e45] * self[e12]) + (reverse[e4235] * self[e31]) - (reverse[e31] * self[e4235]) - (reverse[e4315] * self[e23]),
                (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35])
                    - (reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e45] * self[e3215])
                    - (reverse[e15] * self[e23])
                    - (reverse[e15] * self[e4235])
                    - (reverse[e25] * self[e31])
                    - (reverse[e25] * self[e4315])
                    - (reverse[e35] * self[e12])
                    - (reverse[e35] * self[e4125]),
            ]) + (Simd32x4::from(self[e45]) * Simd32x4::from([reverse[e23], reverse[e31], reverse[e12], reverse[e3215]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleInversionAtOrigin {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionAtOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       32       48        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e3215] * self[e1234])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15])
                    - (reverse[e43] * self[e25])
                    - (reverse[e3215] * self[e41])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25])
                    - (reverse[e41] * self[e35])
                    - (reverse[e3215] * self[e42])
                    - (reverse[e25] * self[e1234])
                    - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35])
                    - (reverse[e42] * self[e15])
                    - (reverse[e3215] * self[e43])
                    - (reverse[e15] * self[e42])
                    - (reverse[e35] * self[e1234]),
            ]) + (Simd32x4::from(self[e3215]) * Simd32x4::from([reverse[e1234], reverse[e41], reverse[e42], reverse[e43]])),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleInversionOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1());
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([
                (reverse[e41] * self[e4235]) + (reverse[e42] * self[e4315]) + (reverse[e43] * self[e4125]) - (reverse[e1234] * self[e45]),
                0.0,
            ]) + (Simd32x2::from(reverse[e45]) * Simd32x2::from([self[e1234], self[e45]]))
                - (Simd32x2::from(reverse[e4235]) * Simd32x2::from([self[e41], self[e4235]]))
                - (Simd32x2::from(reverse[e4315]) * Simd32x2::from([self[e42], self[e4315]]))
                - (Simd32x2::from(reverse[e4125]) * Simd32x2::from([self[e43], self[e4125]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for DipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       55        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       48       58        0
    //  no simd       48       66        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e3215] * self[e1234]) + (reverse[e1234] * self[e3215])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e41] * self[e3215]) + (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15])
                    - (reverse[e43] * self[e25])
                    - (reverse[e3215] * self[e41])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e25] * self[e43]),
                (reverse[e42] * self[e3215]) + (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25])
                    - (reverse[e41] * self[e35])
                    - (reverse[e3215] * self[e42])
                    - (reverse[e25] * self[e1234])
                    - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e43] * self[e3215]) + (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35])
                    - (reverse[e42] * self[e15])
                    - (reverse[e3215] * self[e43])
                    - (reverse[e15] * self[e42])
                    - (reverse[e35] * self[e1234]),
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234])
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DipoleOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e45] * self[e45]);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2));
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for DipoleOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group2() * Simd32x3::from(-1.0),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) - (reverse[e43] * self[e25]) - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) - (reverse[e41] * self[e35]) - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]) - (reverse[e42] * self[e15]) - (reverse[e15] * self[e42]),
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            -f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for DualNum {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        use crate::elements::*;
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([self[e4] * self[e12345], f32::powi(self[e12345], 2)]) * Simd32x2::from([-2.0, -1.0]),
        );
        return NullSphereAtOrigin::from_groups(/* e1234 */ geometric_product[e1234]);
    }
}
impl std::ops::Div<constraint_violation> for FlatOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for FlatOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = FlatOrigin::from_groups(/* e45 */ self[e45] * -1.0);
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e45] * self[e45]);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2));
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for FlatPoint {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ reverse[e45] * self[e45]);
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2));
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for Flector {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e45] * self[e45]) - (reverse[e4235] * self[e4235]) - (reverse[e4315] * self[e4315]) - (reverse[e4125] * self[e4125]),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35]) + (reverse[e3215] * self[e45])
                    - (reverse[e15] * self[e4235])
                    - (reverse[e25] * self[e4315])
                    - (reverse[e35] * self[e4125])
                    - (reverse[e45] * self[e3215]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for FlectorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for FlectorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]));
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[e45] * self[e45]) - (reverse[e4235] * self[e4235]) - (reverse[e4315] * self[e4315]) - (reverse[e4125] * self[e4125]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4125], 2),
        );
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for Line {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435])]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ]),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for LineOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0));
        let geometric_product = Scalar::from_groups(/* scalar */ (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]));
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2));
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
    }
}
impl std::ops::Div<constraint_violation> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       14       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e12345] * self[e12345]),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435])
                    - (reverse[e12345] * self[e5])
                    - (reverse[e5] * self[e12345]),
            ]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e3215]]),
        );
    }
}
impl std::ops::Div<constraint_violation> for MotorOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MotorOnOrigin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7        8        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        let geometric_product = Scalar::from_groups(
            // scalar
            (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e12345] * self[e12345]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return Scalar::from_groups(/* scalar */ geometric_product[scalar] - dot_product[scalar]);
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
    //      f32      320      324        0
    //    simd2       16       16        0
    //    simd3        0        4        0
    //    simd4       11       13        0
    // Totals...
    // yes simd      347      357        0
    //  no simd      396      420        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
        let geometric_product = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (reverse[e45] * self[e45])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e12345] * self[e12345])
                    - (reverse[e4] * self[e5])
                    - (reverse[e5] * self[e4])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
                (reverse[e12345] * self[scalar])
                    + (reverse[e1234] * self[e5])
                    + (reverse[e4235] * self[e1])
                    + (reverse[e4315] * self[e2])
                    + (reverse[e4125] * self[e3])
                    + (reverse[e3215] * self[e4])
                    - (reverse[e415] * self[e23])
                    - (reverse[e425] * self[e31])
                    - (reverse[e435] * self[e12])
                    - (reverse[e321] * self[e45])
                    - (reverse[e423] * self[e15])
                    - (reverse[e431] * self[e25])
                    - (reverse[e412] * self[e35])
                    - (reverse[e235] * self[e41])
                    - (reverse[e315] * self[e42])
                    - (reverse[e125] * self[e43]),
            ]) + (Simd32x2::from(reverse[scalar]) * self.group0())
                + (Simd32x2::from(reverse[e1]) * Simd32x2::from([self[e1], self[e4235]]))
                + (Simd32x2::from(reverse[e2]) * Simd32x2::from([self[e2], self[e4315]]))
                + (Simd32x2::from(reverse[e3]) * Simd32x2::from([self[e3], self[e4125]]))
                + (Simd32x2::from(self[e1234]) * Simd32x2::from([reverse[e3215], reverse[e5]]))
                + (Simd32x2::from(self[e3215]) * Simd32x2::from([reverse[e1234], reverse[e4]]))
                - (Simd32x2::from(reverse[e41]) * Simd32x2::from([self[e15], self[e235]]))
                - (Simd32x2::from(reverse[e42]) * Simd32x2::from([self[e25], self[e315]]))
                - (Simd32x2::from(reverse[e43]) * Simd32x2::from([self[e35], self[e125]]))
                - (Simd32x2::from(reverse[e15]) * Simd32x2::from([self[e41], self[e423]]))
                - (Simd32x2::from(reverse[e25]) * Simd32x2::from([self[e42], self[e431]]))
                - (Simd32x2::from(reverse[e35]) * Simd32x2::from([self[e43], self[e412]]))
                - (Simd32x2::from(reverse[e23]) * Simd32x2::from([self[e23], self[e415]]))
                - (Simd32x2::from(reverse[e31]) * Simd32x2::from([self[e31], self[e425]]))
                - (Simd32x2::from(reverse[e12]) * Simd32x2::from([self[e12], self[e435]]))
                - (Simd32x2::from(self[e321]) * Simd32x2::from([reverse[e321], reverse[e45]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse[e2] * self[e12])
                    + (reverse[e5] * self[e41])
                    + (reverse[e43] * self[e315])
                    + (reverse[e25] * self[e412])
                    + (reverse[e31] * self[e3])
                    + (reverse[e435] * self[e4315])
                    + (reverse[e321] * self[e23])
                    + (reverse[e412] * self[e25])
                    + (reverse[e315] * self[e43])
                    + (reverse[e4125] * self[e425])
                    + (reverse[e3215] * self[e423])
                    - (reverse[e12345] * self[e4235])
                    - (reverse[e3] * self[e31])
                    - (reverse[e41] * self[e5])
                    - (reverse[e42] * self[e125])
                    - (reverse[e45] * self[e415])
                    - (reverse[e35] * self[e431])
                    - (reverse[e12] * self[e2])
                    - (reverse[e415] * self[e45])
                    - (reverse[e425] * self[e4125])
                    - (reverse[e423] * self[e3215])
                    - (reverse[e431] * self[e35])
                    - (reverse[e125] * self[e42])
                    - (reverse[e1234] * self[e235])
                    - (reverse[e4235] * self[e12345])
                    - (reverse[e4315] * self[e435]),
                (reverse[e3] * self[e23])
                    + (reverse[e5] * self[e42])
                    + (reverse[e41] * self[e125])
                    + (reverse[e35] * self[e423])
                    + (reverse[e12] * self[e1])
                    + (reverse[e415] * self[e4125])
                    + (reverse[e321] * self[e31])
                    + (reverse[e423] * self[e35])
                    + (reverse[e125] * self[e41])
                    + (reverse[e4235] * self[e435])
                    + (reverse[e3215] * self[e431])
                    - (reverse[e12345] * self[e4315])
                    - (reverse[e1] * self[e12])
                    - (reverse[e42] * self[e5])
                    - (reverse[e43] * self[e235])
                    - (reverse[e45] * self[e425])
                    - (reverse[e15] * self[e412])
                    - (reverse[e23] * self[e3])
                    - (reverse[e425] * self[e45])
                    - (reverse[e435] * self[e4235])
                    - (reverse[e431] * self[e3215])
                    - (reverse[e412] * self[e15])
                    - (reverse[e235] * self[e43])
                    - (reverse[e1234] * self[e315])
                    - (reverse[e4315] * self[e12345])
                    - (reverse[e4125] * self[e415]),
                (reverse[e1] * self[e31])
                    + (reverse[e5] * self[e43])
                    + (reverse[e42] * self[e235])
                    + (reverse[e15] * self[e431])
                    + (reverse[e23] * self[e2])
                    + (reverse[e425] * self[e4235])
                    + (reverse[e321] * self[e12])
                    + (reverse[e431] * self[e15])
                    + (reverse[e235] * self[e42])
                    + (reverse[e4315] * self[e415])
                    + (reverse[e3215] * self[e412])
                    - (reverse[e12345] * self[e4125])
                    - (reverse[e2] * self[e23])
                    - (reverse[e41] * self[e315])
                    - (reverse[e43] * self[e5])
                    - (reverse[e45] * self[e435])
                    - (reverse[e25] * self[e423])
                    - (reverse[e31] * self[e1])
                    - (reverse[e415] * self[e4315])
                    - (reverse[e435] * self[e45])
                    - (reverse[e423] * self[e25])
                    - (reverse[e412] * self[e3215])
                    - (reverse[e315] * self[e41])
                    - (reverse[e1234] * self[e125])
                    - (reverse[e4235] * self[e425])
                    - (reverse[e4125] * self[e12345]),
                (reverse[e1] * self[e41])
                    + (reverse[e2] * self[e42])
                    + (reverse[e3] * self[e43])
                    + (reverse[e423] * self[e4235])
                    + (reverse[e431] * self[e4315])
                    + (reverse[e412] * self[e4125])
                    + (reverse[e1234] * self[e12345])
                    - (reverse[e41] * self[e1])
                    - (reverse[e41] * self[e415])
                    - (reverse[e42] * self[e2])
                    - (reverse[e42] * self[e425])
                    - (reverse[e43] * self[e3])
                    - (reverse[e43] * self[e435])
                    - (reverse[e23] * self[e423])
                    - (reverse[e31] * self[e431])
                    - (reverse[e12] * self[e412])
                    - (reverse[e415] * self[e41])
                    - (reverse[e425] * self[e42])
                    - (reverse[e435] * self[e43])
                    - (reverse[e321] * self[e1234])
                    - (reverse[e423] * self[e23])
                    - (reverse[e431] * self[e31])
                    - (reverse[e412] * self[e12])
                    - (reverse[e4235] * self[e423])
                    - (reverse[e4315] * self[e431])
                    - (reverse[e4125] * self[e412]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group1())
                + (Simd32x4::from(self[scalar]) * reverse.group1())
                + (Simd32x4::from(self[e4]) * Simd32x4::from([reverse[e15], reverse[e25], reverse[e35], reverse[e45]]))
                + (Simd32x4::from(self[e321]) * Simd32x4::from([reverse[e23], reverse[e31], reverse[e12], reverse[e1234]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([reverse[e235], reverse[e315], reverse[e125], reverse[e12345]]))
                - (Simd32x4::from(reverse[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]])),
            // e5
            (reverse[scalar] * self[e5])
                + (reverse[e12345] * self[e3215])
                + (reverse[e5] * self[scalar])
                + (reverse[e5] * self[e45])
                + (reverse[e15] * self[e1])
                + (reverse[e25] * self[e2])
                + (reverse[e35] * self[e3])
                + (reverse[e321] * self[e3215])
                + (reverse[e4235] * self[e235])
                + (reverse[e4315] * self[e315])
                + (reverse[e4125] * self[e125])
                + (reverse[e3215] * self[e12345])
                - (reverse[e1] * self[e15])
                - (reverse[e2] * self[e25])
                - (reverse[e3] * self[e35])
                - (reverse[e45] * self[e5])
                - (reverse[e15] * self[e415])
                - (reverse[e25] * self[e425])
                - (reverse[e35] * self[e435])
                - (reverse[e23] * self[e235])
                - (reverse[e31] * self[e315])
                - (reverse[e12] * self[e125])
                - (reverse[e415] * self[e15])
                - (reverse[e425] * self[e25])
                - (reverse[e435] * self[e35])
                - (reverse[e235] * self[e23])
                - (reverse[e235] * self[e4235])
                - (reverse[e315] * self[e31])
                - (reverse[e315] * self[e4315])
                - (reverse[e125] * self[e12])
                - (reverse[e125] * self[e4125])
                - (reverse[e3215] * self[e321]),
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
            Simd32x4::from([
                (reverse[e41] * self[e4235])
                    + (reverse[e42] * self[e4315])
                    + (reverse[e43] * self[e4125])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412])
                    + (reverse[e423] * self[e1])
                    + (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e2])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e3])
                    + (reverse[e412] * self[e435])
                    - (reverse[e12345] * self[e4])
                    - (reverse[e1] * self[e423])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412])
                    - (reverse[e4] * self[e321])
                    - (reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
                (reverse[e12345] * self[e1])
                    + (reverse[e1] * self[e12345])
                    + (reverse[e2] * self[e435])
                    + (reverse[e5] * self[e423])
                    + (reverse[e41] * self[e3215])
                    + (reverse[e42] * self[e35])
                    + (reverse[e35] * self[e42])
                    + (reverse[e23] * self[e45])
                    + (reverse[e31] * self[e4125])
                    + (reverse[e415] * self[e321])
                    + (reverse[e425] * self[e3])
                    + (reverse[e412] * self[e315])
                    + (reverse[e235] * self[e4])
                    + (reverse[e315] * self[e412])
                    + (reverse[e4235] * self[scalar])
                    + (reverse[e4315] * self[e12])
                    - (reverse[e3] * self[e425])
                    - (reverse[e43] * self[e25])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e25] * self[e43])
                    - (reverse[e12] * self[e4315])
                    - (reverse[e435] * self[e2])
                    - (reverse[e423] * self[e5])
                    - (reverse[e431] * self[e125])
                    - (reverse[e125] * self[e431])
                    - (reverse[e4125] * self[e31])
                    - (reverse[e3215] * self[e41]),
                (reverse[e12345] * self[e2])
                    + (reverse[e2] * self[e12345])
                    + (reverse[e3] * self[e415])
                    + (reverse[e5] * self[e431])
                    + (reverse[e42] * self[e3215])
                    + (reverse[e43] * self[e15])
                    + (reverse[e15] * self[e43])
                    + (reverse[e31] * self[e45])
                    + (reverse[e12] * self[e4235])
                    + (reverse[e425] * self[e321])
                    + (reverse[e435] * self[e1])
                    + (reverse[e423] * self[e125])
                    + (reverse[e315] * self[e4])
                    + (reverse[e125] * self[e423])
                    + (reverse[e4315] * self[scalar])
                    + (reverse[e4125] * self[e23])
                    - (reverse[e1] * self[e435])
                    - (reverse[e41] * self[e35])
                    - (reverse[e25] * self[e1234])
                    - (reverse[e35] * self[e41])
                    - (reverse[e23] * self[e4125])
                    - (reverse[e415] * self[e3])
                    - (reverse[e431] * self[e5])
                    - (reverse[e412] * self[e235])
                    - (reverse[e235] * self[e412])
                    - (reverse[e4235] * self[e12])
                    - (reverse[e3215] * self[e42]),
                (reverse[e12345] * self[e3])
                    + (reverse[e1] * self[e425])
                    + (reverse[e3] * self[e12345])
                    + (reverse[e5] * self[e412])
                    + (reverse[e41] * self[e25])
                    + (reverse[e43] * self[e3215])
                    + (reverse[e25] * self[e41])
                    + (reverse[e23] * self[e4315])
                    + (reverse[e12] * self[e45])
                    + (reverse[e415] * self[e2])
                    + (reverse[e435] * self[e321])
                    + (reverse[e431] * self[e235])
                    + (reverse[e235] * self[e431])
                    + (reverse[e125] * self[e4])
                    + (reverse[e4235] * self[e31])
                    + (reverse[e4125] * self[scalar])
                    - (reverse[e2] * self[e415])
                    - (reverse[e42] * self[e15])
                    - (reverse[e15] * self[e42])
                    - (reverse[e35] * self[e1234])
                    - (reverse[e31] * self[e4235])
                    - (reverse[e425] * self[e1])
                    - (reverse[e423] * self[e315])
                    - (reverse[e412] * self[e5])
                    - (reverse[e315] * self[e423])
                    - (reverse[e4315] * self[e23])
                    - (reverse[e3215] * self[e43]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group9())
                + (Simd32x4::from(reverse[e45]) * Simd32x4::from([self[e1234], self[e23], self[e31], self[e12]]))
                + (Simd32x4::from(reverse[e321]) * Simd32x4::from([self[e4], self[e415], self[e425], self[e435]]))
                + (Simd32x4::from(reverse[e1234]) * Simd32x4::from([self[scalar], self[e15], self[e25], self[e35]]))
                - (Simd32x4::from(reverse[e4]) * Simd32x4::from([self[e12345], self[e235], self[e315], self[e125]])),
            // e3215
            (reverse[scalar] * self[e3215])
                + (reverse[e1] * self[e235])
                + (reverse[e2] * self[e315])
                + (reverse[e3] * self[e125])
                + (reverse[e5] * self[e321])
                + (reverse[e415] * self[e235])
                + (reverse[e425] * self[e315])
                + (reverse[e435] * self[e125])
                + (reverse[e235] * self[e415])
                + (reverse[e315] * self[e425])
                + (reverse[e125] * self[e435])
                + (reverse[e4235] * self[e15])
                + (reverse[e4315] * self[e25])
                + (reverse[e4125] * self[e35])
                + (reverse[e3215] * self[scalar])
                + (reverse[e3215] * self[e45])
                - (reverse[e12345] * self[e5])
                - (reverse[e5] * self[e12345])
                - (reverse[e45] * self[e3215])
                - (reverse[e15] * self[e23])
                - (reverse[e15] * self[e4235])
                - (reverse[e25] * self[e31])
                - (reverse[e25] * self[e4315])
                - (reverse[e35] * self[e12])
                - (reverse[e35] * self[e4125])
                - (reverse[e23] * self[e15])
                - (reverse[e31] * self[e25])
                - (reverse[e12] * self[e35])
                - (reverse[e321] * self[e5])
                - (reverse[e235] * self[e1])
                - (reverse[e315] * self[e2])
                - (reverse[e125] * self[e3]),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + 2.0 * (self[e1234] * self[e3215])
                + f32::powi(self[scalar], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                + f32::powi(self[e45], 2)
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e321], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e4] * self[e5])
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([geometric_product[scalar] - dot_product[scalar], geometric_product[e12345]]),
            // e1, e2, e3, e4
            geometric_product.group1(),
            // e5
            geometric_product[e5],
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
            geometric_product[e3215],
        );
    }
}
impl std::ops::Div<constraint_violation> for MysteryCircle {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryCircle {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       10       14        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e321] * self[e321]),
                (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
                (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
                (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e321], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for MysteryCircleRotor {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryCircleRotor {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       12       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345]);
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e321] * self[e321]) - (reverse[e12345] * self[e12345]),
                (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
                (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
                (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e321], 2) - f32::powi(self[e12345], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for MysteryDipole {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
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
        let reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                -(reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                reverse[e45] * self[e23],
                reverse[e45] * self[e31],
                reverse[e45] * self[e12],
            ]) + (Simd32x4::from(self[e45]) * crate::swizzle!(reverse.group0(), 3, 0, 1, 2)),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(/* scalar */ f32::powi(self[e45], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2));
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for MysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryDipoleInversion {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       21        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       25       23        0
    //  no simd       28       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1());
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                -(reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
                (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23]) + (reverse[e4315] * self[e12]) - (reverse[e12] * self[e4315]) - (reverse[e4125] * self[e31]),
                (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31]) + (reverse[e4125] * self[e23]) - (reverse[e23] * self[e4125]) - (reverse[e4235] * self[e12]),
                (reverse[e23] * self[e4315]) + (reverse[e45] * self[e12]) + (reverse[e4235] * self[e31]) - (reverse[e31] * self[e4235]) - (reverse[e4315] * self[e23]),
            ]) + (Simd32x4::from(self[e45]) * crate::swizzle!(reverse.group0(), 3, 0, 1, 2)),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       36       33        0
    //  no simd       36       36        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    - (reverse[e12345] * self[e12345])
                    - (reverse[e321] * self[e321]),
                (reverse[e12345] * self[e1])
                    + (reverse[e1] * self[e12345])
                    + (reverse[e2] * self[e435])
                    + (reverse[e415] * self[e321])
                    + (reverse[e425] * self[e3])
                    + (reverse[e321] * self[e415])
                    - (reverse[e3] * self[e425])
                    - (reverse[e435] * self[e2]),
                (reverse[e12345] * self[e2])
                    + (reverse[e2] * self[e12345])
                    + (reverse[e3] * self[e415])
                    + (reverse[e425] * self[e321])
                    + (reverse[e435] * self[e1])
                    + (reverse[e321] * self[e425])
                    - (reverse[e1] * self[e435])
                    - (reverse[e415] * self[e3]),
                (reverse[e12345] * self[e3])
                    + (reverse[e1] * self[e425])
                    + (reverse[e3] * self[e12345])
                    + (reverse[e415] * self[e2])
                    + (reverse[e435] * self[e321])
                    + (reverse[e321] * self[e435])
                    - (reverse[e2] * self[e415])
                    - (reverse[e425] * self[e1]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e321], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for MysteryVersorOdd {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryVersorOdd {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       24        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       30       27        0
    //  no simd       36       36        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0));
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                -(reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12]),
                (reverse[e4235] * self[scalar]) + (reverse[e4315] * self[e12]) + (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23])
                    - (reverse[e4125] * self[e31])
                    - (reverse[e12] * self[e4315]),
                (reverse[e4315] * self[scalar]) + (reverse[e4125] * self[e23]) + (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31])
                    - (reverse[e4235] * self[e12])
                    - (reverse[e23] * self[e4125]),
                (reverse[e4235] * self[e31]) + (reverse[e4125] * self[scalar]) + (reverse[e23] * self[e4315]) + (reverse[e45] * self[e12])
                    - (reverse[e4315] * self[e23])
                    - (reverse[e31] * self[e4235]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group0())
                + (Simd32x4::from(self[e45]) * crate::swizzle!(reverse.group1(), 3, 0, 1, 2)),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e45], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
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
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for PlaneOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for PlaneOnOrigin {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for RoundPoint {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for RoundPoint {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for RoundPointAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for RoundPointAtOrigin {
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
impl std::ops::Div<constraint_violation> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Sphere {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for SphereAtOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for SphereAtOrigin {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for SphereOnOrigin {
    type Output = Scalar;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for SphereOnOrigin {
    type Output = Scalar;
    fn constraint_violation(self) -> Self::Output {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl std::ops::Div<constraint_violation> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       94       96        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       96      101        0
    //  no simd      102      116        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    + (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    - (reverse[e12345] * self[e12345])
                    - (reverse[e321] * self[e321])
                    - (reverse[e5] * self[e4])
                    - (reverse[e4] * self[e5]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e423] * self[e1])
                    + (reverse[e431] * self[e425])
                    + (reverse[e431] * self[e2])
                    + (reverse[e412] * self[e435])
                    + (reverse[e412] * self[e3])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412])
                    + (reverse[e321] * self[e4])
                    - (reverse[e12345] * self[e4])
                    - (reverse[e1] * self[e423])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412])
                    - (reverse[e4] * self[e12345])
                    - (reverse[e4] * self[e321]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315])
                    + (reverse[e12345] * self[e1])
                    + (reverse[e425] * self[e3])
                    + (reverse[e321] * self[e415])
                    + (reverse[e235] * self[e4])
                    + (reverse[e315] * self[e412])
                    + (reverse[e5] * self[e423])
                    + (reverse[e1] * self[e12345])
                    + (reverse[e2] * self[e435])
                    - (reverse[e431] * self[e125])
                    - (reverse[e435] * self[e2])
                    - (reverse[e125] * self[e431])
                    - (reverse[e3] * self[e425])
                    - (reverse[e4] * self[e235]),
                (reverse[e423] * self[e125])
                    + (reverse[e12345] * self[e2])
                    + (reverse[e435] * self[e1])
                    + (reverse[e321] * self[e425])
                    + (reverse[e315] * self[e4])
                    + (reverse[e125] * self[e423])
                    + (reverse[e5] * self[e431])
                    + (reverse[e2] * self[e12345])
                    + (reverse[e3] * self[e415])
                    - (reverse[e412] * self[e235])
                    - (reverse[e415] * self[e3])
                    - (reverse[e235] * self[e412])
                    - (reverse[e1] * self[e435])
                    - (reverse[e4] * self[e315]),
                (reverse[e431] * self[e235])
                    + (reverse[e12345] * self[e3])
                    + (reverse[e415] * self[e2])
                    + (reverse[e321] * self[e435])
                    + (reverse[e235] * self[e431])
                    + (reverse[e125] * self[e4])
                    + (reverse[e5] * self[e412])
                    + (reverse[e1] * self[e425])
                    + (reverse[e3] * self[e12345])
                    - (reverse[e423] * self[e315])
                    - (reverse[e425] * self[e1])
                    - (reverse[e315] * self[e423])
                    - (reverse[e2] * self[e415])
                    - (reverse[e4] * self[e125]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435])
                    + (reverse[e1] * self[e235])
                    + (reverse[e2] * self[e315])
                    + (reverse[e3] * self[e125])
                    - (reverse[e321] * self[e5])
                    - (reverse[e235] * self[e1])
                    - (reverse[e315] * self[e2])
                    - (reverse[e125] * self[e3])
                    - (reverse[e5] * self[e12345]),
            ]) + (Simd32x4::from(self[e321]) * Simd32x4::from([reverse[e415], reverse[e425], reverse[e435], reverse[e5]]))
                - (Simd32x4::from(self[e5]) * reverse.group0()),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e5] * self[e4]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorEvenAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       56        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       51       60        0
    //  no simd       54       72        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e12345] * self[e12345])
                    - (reverse[e4] * self[e5])
                    - (reverse[e5] * self[e4]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412])
                    - (reverse[e12345] * self[e4])
                    - (reverse[e4] * self[e12345]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315]) + (reverse[e235] * self[e4]) + (reverse[e315] * self[e412]) + (reverse[e5] * self[e423])
                    - (reverse[e431] * self[e125])
                    - (reverse[e4] * self[e235])
                    - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e315] * self[e4]) + (reverse[e125] * self[e423]) + (reverse[e5] * self[e431])
                    - (reverse[e412] * self[e235])
                    - (reverse[e4] * self[e315])
                    - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) + (reverse[e125] * self[e4]) + (reverse[e5] * self[e412])
                    - (reverse[e423] * self[e315])
                    - (reverse[e4] * self[e125])
                    - (reverse[e315] * self[e423]),
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435])
                    - (reverse[e5] * self[e12345]),
            ]) - (Simd32x4::from(self[e5]) * reverse.group0()),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e415], 2)
                + f32::powi(self[e425], 2)
                + f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - 2.0 * (self[e4] * self[e5]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       44        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       48       47        0
    //  no simd       51       56        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    - (reverse[e12345] * self[e12345])
                    - (reverse[e321] * self[e321]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e12345] * self[e1]) + (reverse[e1] * self[e12345]) + (reverse[e2] * self[e435]) + (reverse[e425] * self[e3]) + (reverse[e321] * self[e415])
                    - (reverse[e3] * self[e425])
                    - (reverse[e435] * self[e2]),
                (reverse[e12345] * self[e2]) + (reverse[e2] * self[e12345]) + (reverse[e3] * self[e415]) + (reverse[e435] * self[e1]) + (reverse[e321] * self[e425])
                    - (reverse[e1] * self[e435])
                    - (reverse[e415] * self[e3]),
                (reverse[e12345] * self[e3]) + (reverse[e1] * self[e425]) + (reverse[e3] * self[e12345]) + (reverse[e415] * self[e2]) + (reverse[e321] * self[e435])
                    - (reverse[e2] * self[e415])
                    - (reverse[e425] * self[e1]),
                (reverse[e1] * self[e235])
                    + (reverse[e2] * self[e315])
                    + (reverse[e3] * self[e125])
                    + (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435])
                    - (reverse[e12345] * self[e5])
                    - (reverse[e321] * self[e5])
                    - (reverse[e235] * self[e1])
                    - (reverse[e315] * self[e2])
                    - (reverse[e125] * self[e3])
                    - (reverse[e5] * self[e12345]),
            ]) + (Simd32x4::from(self[e321]) * Simd32x4::from([reverse[e415], reverse[e425], reverse[e435], reverse[e5]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e1], 2) + f32::powi(self[e2], 2) + f32::powi(self[e3], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2)
                - f32::powi(self[e12345], 2)
                - f32::powi(self[e321], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorEvenAtOrigin {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenAtOrigin {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       29       39        0
    //  no simd       32       48        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e5] * self[e4]),
                (reverse[e412] * self[e315]) + (reverse[e235] * self[e4]) + (reverse[e315] * self[e412]) + (reverse[e5] * self[e423])
                    - (reverse[e431] * self[e125])
                    - (reverse[e4] * self[e235])
                    - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e315] * self[e4]) + (reverse[e125] * self[e423]) + (reverse[e5] * self[e431])
                    - (reverse[e412] * self[e235])
                    - (reverse[e4] * self[e315])
                    - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) + (reverse[e125] * self[e4]) + (reverse[e5] * self[e412])
                    - (reverse[e423] * self[e315])
                    - (reverse[e4] * self[e125])
                    - (reverse[e315] * self[e423]),
            ]) - (Simd32x4::from(self[e5]) * crate::swizzle!(reverse.group0(), 3, 0, 1, 2)),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - 2.0 * (self[e4] * self[e5]),
        );
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([
                geometric_product[scalar] - dot_product[scalar],
                geometric_product[e4235],
                geometric_product[e4315],
                geometric_product[e4125],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorEvenOnOrigin {
    type Output = AntiDualNum;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenOnOrigin {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       10        0
    //  no simd       15       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([
                (reverse[e415] * self[e423]) + (reverse[e425] * self[e431]) + (reverse[e435] * self[e412]) - (reverse[e4] * self[e12345]),
                0.0,
            ]) + (Simd32x2::from(self[e415]) * Simd32x2::from([reverse[e423], reverse[e415]]))
                + (Simd32x2::from(self[e425]) * Simd32x2::from([reverse[e431], reverse[e425]]))
                + (Simd32x2::from(self[e435]) * Simd32x2::from([reverse[e412], reverse[e435]]))
                - (Simd32x2::from(reverse[e12345]) * Simd32x2::from([self[e4], self[e12345]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) - f32::powi(self[e12345], 2),
        );
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([geometric_product[e1234], geometric_product[scalar] - dot_product[scalar]]));
    }
}
impl std::ops::Div<constraint_violation> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       52        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       48       56        0
    //  no simd       54       68        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    + (reverse[e1] * self[e1])
                    + (reverse[e2] * self[e2])
                    + (reverse[e3] * self[e3])
                    - (reverse[e321] * self[e321])
                    - (reverse[e5] * self[e4])
                    - (reverse[e4] * self[e5]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e423] * self[e1]) + (reverse[e431] * self[e2]) + (reverse[e412] * self[e3]) + (reverse[e321] * self[e4])
                    - (reverse[e1] * self[e423])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412])
                    - (reverse[e4] * self[e321]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e412] * self[e315]) + (reverse[e235] * self[e4]) + (reverse[e315] * self[e412])
                    - (reverse[e431] * self[e125])
                    - (reverse[e125] * self[e431])
                    - (reverse[e4] * self[e235]),
                (reverse[e423] * self[e125]) + (reverse[e315] * self[e4]) + (reverse[e125] * self[e423])
                    - (reverse[e412] * self[e235])
                    - (reverse[e235] * self[e412])
                    - (reverse[e4] * self[e315]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) + (reverse[e125] * self[e4])
                    - (reverse[e423] * self[e315])
                    - (reverse[e315] * self[e423])
                    - (reverse[e4] * self[e125]),
                (reverse[e1] * self[e235]) + (reverse[e2] * self[e315]) + (reverse[e3] * self[e125])
                    - (reverse[e235] * self[e1])
                    - (reverse[e315] * self[e2])
                    - (reverse[e125] * self[e3]),
            ]) + (Simd32x4::from(reverse[e5]) * self.group0())
                - (Simd32x4::from(self[e5]) * reverse.group0()),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e423] * self[e235])
                + 2.0 * (self[e431] * self[e315])
                + 2.0 * (self[e412] * self[e125])
                + f32::powi(self[e1], 2)
                + f32::powi(self[e2], 2)
                + f32::powi(self[e3], 2)
                - f32::powi(self[e321], 2)
                - 2.0 * (self[e5] * self[e4]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for VersorOdd {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90       92        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       93       98        0
    //  no simd      102      116        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[scalar] * self[scalar]) + (reverse[e45] * self[e45]) + (reverse[e1234] * self[e3215]) + (reverse[e3215] * self[e1234])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[e41] * self[e4235])
                    + (reverse[e42] * self[e4315])
                    + (reverse[e43] * self[e4125])
                    + (reverse[scalar] * self[e1234])
                    + (reverse[e45] * self[e1234])
                    + (reverse[e1234] * self[scalar])
                    - (reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35])
                    + (reverse[scalar] * self[e4235])
                    + (reverse[e31] * self[e4125])
                    + (reverse[e45] * self[e23])
                    + (reverse[e35] * self[e42])
                    + (reverse[e1234] * self[e15])
                    + (reverse[e4315] * self[e12])
                    - (reverse[e43] * self[e25])
                    - (reverse[e12] * self[e4315])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e25] * self[e43])
                    - (reverse[e4125] * self[e31])
                    - (reverse[e3215] * self[e41]),
                (reverse[e43] * self[e15])
                    + (reverse[scalar] * self[e4315])
                    + (reverse[e12] * self[e4235])
                    + (reverse[e45] * self[e31])
                    + (reverse[e15] * self[e43])
                    + (reverse[e1234] * self[e25])
                    + (reverse[e4125] * self[e23])
                    - (reverse[e41] * self[e35])
                    - (reverse[e23] * self[e4125])
                    - (reverse[e25] * self[e1234])
                    - (reverse[e35] * self[e41])
                    - (reverse[e4235] * self[e12])
                    - (reverse[e3215] * self[e42]),
                (reverse[e41] * self[e25])
                    + (reverse[scalar] * self[e4125])
                    + (reverse[e23] * self[e4315])
                    + (reverse[e45] * self[e12])
                    + (reverse[e25] * self[e41])
                    + (reverse[e1234] * self[e35])
                    + (reverse[e4235] * self[e31])
                    - (reverse[e42] * self[e15])
                    - (reverse[e31] * self[e4235])
                    - (reverse[e15] * self[e42])
                    - (reverse[e35] * self[e1234])
                    - (reverse[e4315] * self[e23])
                    - (reverse[e3215] * self[e43]),
                (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35])
                    - (reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e45] * self[e3215])
                    - (reverse[e15] * self[e23])
                    - (reverse[e15] * self[e4235])
                    - (reverse[e25] * self[e31])
                    - (reverse[e25] * self[e4315])
                    - (reverse[e35] * self[e12])
                    - (reverse[e35] * self[e4125]),
            ]) + (Simd32x4::from(self[scalar]) * reverse.group3())
                + (Simd32x4::from(self[e45]) * Simd32x4::from([reverse[e23], reverse[e31], reverse[e12], reverse[e3215]]))
                + (Simd32x4::from(self[e3215]) * reverse.group0()),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e1234] * self[e3215]) + f32::powi(self[scalar], 2) + f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<constraint_violation> for VersorOddAtInfinity {
    fn div_assign(&mut self, _rhs: constraint_violation) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       36        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       42       41        0
    //  no simd       51       56        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        let geometric_product = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                (reverse[scalar] * self[scalar]) + (reverse[e45] * self[e45])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
                0.0,
                0.0,
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23]) + (reverse[e4315] * self[e12]) - (reverse[e12] * self[e4315]) - (reverse[e4125] * self[e31]),
                (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31]) + (reverse[e4125] * self[e23]) - (reverse[e23] * self[e4125]) - (reverse[e4235] * self[e12]),
                (reverse[e23] * self[e4315]) + (reverse[e45] * self[e12]) + (reverse[e4235] * self[e31]) - (reverse[e31] * self[e4235]) - (reverse[e4315] * self[e23]),
                (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e15] * self[e4235])
                    - (reverse[e25] * self[e31])
                    - (reverse[e25] * self[e4315])
                    - (reverse[e35] * self[e12])
                    - (reverse[e35] * self[e4125])
                    - (reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e45] * self[e3215]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group2())
                + (Simd32x4::from(self[scalar]) * reverse.group2())
                + (Simd32x4::from(self[e45]) * Simd32x4::from([reverse[e23], reverse[e31], reverse[e12], reverse[e3215]])),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            f32::powi(self[scalar], 2) + f32::powi(self[e45], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - f32::powi(self[e4235], 2)
                - f32::powi(self[e4315], 2)
                - f32::powi(self[e4125], 2),
        );
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([geometric_product[scalar] - dot_product[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            geometric_product.group2(),
        );
    }
}
impl std::ops::Div<constraint_violation> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: constraint_violation) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       56        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       51       60        0
    //  no simd       54       72        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let geometric_product = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[scalar] * self[scalar]) + (reverse[e3215] * self[e1234]) + (reverse[e1234] * self[e3215])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (reverse[scalar] * self[e1234]) + (reverse[e1234] * self[scalar])
                    - (reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15])
                    - (reverse[e43] * self[e25])
                    - (reverse[e3215] * self[e41])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e25] * self[e43]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25])
                    - (reverse[e41] * self[e35])
                    - (reverse[e3215] * self[e42])
                    - (reverse[e25] * self[e1234])
                    - (reverse[e35] * self[e41]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35])
                    - (reverse[e42] * self[e15])
                    - (reverse[e3215] * self[e43])
                    - (reverse[e15] * self[e42])
                    - (reverse[e35] * self[e1234]),
                (reverse[e3215] * self[scalar])
                    - (reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ]) + (Simd32x4::from(self[e3215]) * reverse.group0()),
        );
        let dot_product = Scalar::from_groups(
            // scalar
            2.0 * (self[e3215] * self[e1234]) + f32::powi(self[scalar], 2)
                - f32::powi(self[e23], 2)
                - f32::powi(self[e31], 2)
                - f32::powi(self[e12], 2)
                - 2.0 * (self[e41] * self[e15])
                - 2.0 * (self[e42] * self[e25])
                - 2.0 * (self[e43] * self[e35]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[scalar] - dot_product[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, geometric_product[e1234]]),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
