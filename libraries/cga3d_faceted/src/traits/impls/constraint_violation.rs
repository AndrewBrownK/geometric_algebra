// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 59
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        15      21       0
//  Average:        24      30       0
//  Maximum:       200     227       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        17      26       0
//  Average:        33      43       0
//  Maximum:       372     404       0
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        return NullSphereAtOrigin::from_groups(
            // e1234
            -(reverse[e41] * self[e23])
                - (reverse[e42] * self[e31])
                - (reverse[e43] * self[e12])
                - (reverse[e23] * self[e41])
                - (reverse[e31] * self[e42])
                - (reverse[e12] * self[e43]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotor {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       40        0
    //    simd3        0        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       37       45        0
    //  no simd       46       58        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]) + (reverse[e35] * self[e42]),
                (reverse[e43] * self[e15]) + (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]) + (reverse[e15] * self[e43]),
                (reverse[e41] * self[e25]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e25] * self[e41]),
                -(reverse[e12] * self[e35]) - (reverse[e15] * self[e23]) - (reverse[e25] * self[e31]) - (reverse[e35] * self[e12]),
            ]) - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (self.group0().zxy() * reverse.group2().yzx()).with_w(reverse[e31] * self[e25]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e45], 2)
                        - f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - f32::powi(self[scalar], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       33        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       29       38        0
    //  no simd       38       50        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]),
                -(reverse[e12] * self[e35]) - (reverse[e15] * self[e23]) - (reverse[e25] * self[e31]) - (reverse[e35] * self[e12]),
            ]) - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (self.group0().zxy() * reverse.group2().yzx()).with_w(reverse[e31] * self[e25]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - f32::powi(self[scalar], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       15       17        0
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
            Simd32x3::from(0.0).with_w((reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(-f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2) - f32::powi(self[scalar], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd        8       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]),
            (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]),
            (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]),
            -(reverse[e23] * self[e15])
                - (reverse[e31] * self[e25])
                - (reverse[e12] * self[e35])
                - (reverse[e15] * self[e23])
                - (reverse[e25] * self[e31])
                - (reverse[e35] * self[e12]),
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            self.group1() * Simd32x3::from(-1.0),
        );
        return NullSphereAtOrigin::from_groups(
            // e1234
            -(reverse[e41] * self[e23])
                - (reverse[e42] * self[e31])
                - (reverse[e43] * self[e12])
                - (reverse[e23] * self[e41])
                - (reverse[e31] * self[e42])
                - (reverse[e12] * self[e43]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDipoleInversion {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       46        0
    //    simd3        0        2        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       54       61        0
    //  no simd       93      104        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
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
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e4] * self[e235]) - (reverse[e3] * self[e425]),
                -(reverse[e4] * self[e315]) - (reverse[e1] * self[e435]),
                -(reverse[e4] * self[e125]) - (reverse[e2] * self[e415]),
                (reverse[e3] * self[e125]) + (reverse[e5] * self[e321]),
            ]) + (Simd32x4::from([reverse[e315], reverse[e5], reverse[e5], self[e125]]) * self.group0().zyz().with_w(reverse[e435]))
                + (Simd32x4::from([reverse[e5], reverse[e125], reverse[e235], self[e315]]) * self.group0().xxy().with_w(reverse[e425]))
                + (reverse.group3().yzxy() * self.group1().zxy().with_w(self[e315]))
                + (self.group1().xyzz() * reverse.group1().www().with_w(reverse[e125]))
                + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group2().wwwx() * reverse.group2().xyz().with_w(reverse[e1]))
                + (self.group1().ww().with_zw(self[e2], self[e415]) * reverse.group1().xyx().with_w(reverse[e235]))
                + (self.group3().zx().with_zw(self[e321], self[e425]) * reverse.group1().yzz().with_w(reverse[e315]))
                - (self.group3().yzxz() * reverse.group1().zxy().with_w(reverse[e125]))
                - (self.group2().zx().with_zw(self[e5], self[e1]) * reverse.group0().yzz().with_w(reverse[e235]))
                - (self.group3().ww().with_zw(self[e315], self[e5]) * reverse.group0().xyx().with_w(reverse[e321]))
                - (self.group0().yzx() * reverse.group2().zxy()).with_w(reverse[e315] * self[e2]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + 2.0 * (self[e4] * self[e5])
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2)
                        - f32::powi(self[e321], 2)
                        - f32::powi(self[e1], 2)
                        - f32::powi(self[e2], 2)
                        - f32::powi(self[e3], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       34        0
    //  no simd       28       39        0
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (reverse[e415] * self[e321]) + (reverse[e425] * self[e3]) + (reverse[e321] * self[e415]) + (reverse[e2] * self[e435])
                - (reverse[e435] * self[e2])
                - (reverse[e3] * self[e425]),
            (reverse[e425] * self[e321]) + (reverse[e435] * self[e1]) + (reverse[e321] * self[e425]) + (reverse[e3] * self[e415])
                - (reverse[e415] * self[e3])
                - (reverse[e1] * self[e435]),
            (reverse[e415] * self[e2]) + (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]) + (reverse[e1] * self[e425])
                - (reverse[e425] * self[e1])
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
                + (reverse[e5] * self[e321])
                - (reverse[e321] * self[e5])
                - (reverse[e235] * self[e1])
                - (reverse[e315] * self[e2])
                - (reverse[e125] * self[e3]),
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from(-1.0), /* e4, e1, e2, e3 */ self.group1());
        return NullSphereAtOrigin::from_groups(
            // e1234
            (reverse[e423] * self[e1]) + (reverse[e431] * self[e2]) + (reverse[e412] * self[e3]) + (reverse[e321] * self[e4])
                - (reverse[e4] * self[e321])
                - (reverse[e1] * self[e423])
                - (reverse[e2] * self[e431])
                - (reverse[e3] * self[e412]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       40        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       36       47        0
    //  no simd       51       66        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e423] * self[e5]) - (reverse[e431] * self[e125]) - (reverse[e125] * self[e431]) - (reverse[e4] * self[e235]),
                -(reverse[e431] * self[e5]) - (reverse[e412] * self[e235]) - (reverse[e235] * self[e412]) - (reverse[e4] * self[e315]),
                -(reverse[e423] * self[e315]) - (reverse[e412] * self[e5]) - (reverse[e315] * self[e423]) - (reverse[e4] * self[e125]),
                (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]),
            ]) + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group0().zx().with_zw(self[e4], reverse[e235]) * reverse.group2().yzz().with_w(self[e415]))
                + (self.group2().ww().with_zw(self[e431], self[e125]) * reverse.group2().xyx().with_w(reverse[e435]))
                + (reverse.group0().www() * self.group0().xyz()).with_w(reverse[e425] * self[e315]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + 2.0 * (self[e5] * self[e4])
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiDualNum {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiDualNum {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e1234] * self[scalar] * 2.0);
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiFlector {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       17       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiFlector::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3, e5 */ self.group1());
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((reverse[e1] * self[e1]) + (reverse[e2] * self[e2]) + (reverse[e3] * self[e3]) - (reverse[e321] * self[e321])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                (reverse[e1] * self[e235]) + (reverse[e2] * self[e315]) + (reverse[e3] * self[e125]) + (reverse[e5] * self[e321])
                    - (reverse[e235] * self[e1])
                    - (reverse[e315] * self[e2])
                    - (reverse[e125] * self[e3])
                    - (reverse[e321] * self[e5]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(-f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiLine {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd3        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       13       15        0
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
            Simd32x3::from(0.0).with_w(-(reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                -(reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(-f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiMotor {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<ConstraintViolationPrefixOrPostfix> for AntiMotor {
    fn div_assign(&mut self, _rhs: ConstraintViolationPrefixOrPostfix) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       17       20        0
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
            Simd32x3::from(0.0).with_w((reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                (reverse[scalar] * self[e3215]) + (reverse[e3215] * self[scalar])
                    - (reverse[e23] * self[e15])
                    - (reverse[e31] * self[e25])
                    - (reverse[e12] * self[e35])
                    - (reverse[e15] * self[e23])
                    - (reverse[e25] * self[e31])
                    - (reverse[e35] * self[e12]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(-f32::powi(self[e23], 2) - f32::powi(self[e31], 2) - f32::powi(self[e12], 2) - f32::powi(self[scalar], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMysteryCircleRotor {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* scalar */ self[scalar]);
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e45] * self[e45]) + (reverse[scalar] * self[scalar]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]),
                (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]),
                (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiMysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       26        0
    //  no simd       21       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e1, e2, e3 */ self.group1());
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
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
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return NullSphereAtOrigin::from_groups(
            // e1234
            (reverse[scalar] * self[e1234]) + (reverse[e1234] * self[scalar])
                - (reverse[e41] * self[e23])
                - (reverse[e42] * self[e31])
                - (reverse[e43] * self[e12])
                - (reverse[e23] * self[e41])
                - (reverse[e31] * self[e42])
                - (reverse[e12] * self[e43]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Circle {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd3        0        5        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       44       56        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                -(reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                -(reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e425] * self[e315]) + (reverse[e435] * self[e125]),
            ]) + (self.group1().wwwz() * reverse.group1().xyz().with_w(reverse[e125]))
                + (reverse.group0().zxy() * self.group2().yzx()).with_w(reverse[e235] * self[e415])
                + (reverse.group2().yzx() * self.group0().zxy()).with_w(reverse[e315] * self[e425])
                + (reverse.group1().www() * self.group1().xyz()).with_w(reverse[e415] * self[e235]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2)
                        - f32::powi(self[e321], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       27       38        0
    //  no simd       36       48        0
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
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e415] * self[e415])
                    + (reverse[e425] * self[e425])
                    + (reverse[e435] * self[e435])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                -(reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                -(reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e435] * self[e125]) + (reverse[e235] * self[e415]) + (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]),
            ]) + (reverse.group0().zxy() * self.group2().yzx()).with_w(reverse[e415] * self[e235])
                + (reverse.group2().yzx() * self.group0().zxy()).with_w(reverse[e425] * self[e315]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       14        0
    //  no simd        8       19        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
            (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
            (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
            (reverse[e415] * self[e235])
                + (reverse[e425] * self[e315])
                + (reverse[e435] * self[e125])
                + (reverse[e235] * self[e415])
                + (reverse[e315] * self[e425])
                + (reverse[e125] * self[e435]),
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       14       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
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
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        return NullSphereAtOrigin::from_groups(
            // e1234
            (reverse[e423] * self[e415])
                + (reverse[e431] * self[e425])
                + (reverse[e412] * self[e435])
                + (reverse[e415] * self[e423])
                + (reverse[e425] * self[e431])
                + (reverse[e435] * self[e412]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       19        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       26        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
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
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleRotor {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       32        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       46       58        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                -(reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                -(reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]),
            ]) + (reverse.group1().xyzz() * self.group1().www().with_w(self[e125]))
                + (self.group1().xyzx() * reverse.group1().www().with_w(reverse[e235]))
                + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group0().zxy() * reverse.group2().yzx()).with_w(reverse[e425] * self[e315]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2)
                        + f32::powi(self[e12345], 2)
                        - f32::powi(self[e321], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       33        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       29       38        0
    //  no simd       38       50        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e431] * self[e125]) - (reverse[e125] * self[e431]),
                -(reverse[e412] * self[e235]) - (reverse[e235] * self[e412]),
                -(reverse[e423] * self[e315]) - (reverse[e315] * self[e423]),
                (reverse[e435] * self[e125]) + (reverse[e235] * self[e415]) + (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]),
            ]) + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group0().zxy() * reverse.group2().yzx()).with_w(reverse[e425] * self[e315]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2)
                        + f32::powi(self[e12345], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       10        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       12        0
    //  no simd       15       17        0
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
            Simd32x3::from(0.0).with_w((reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e12345] * self[e12345])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) + f32::powi(self[e12345], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd        8       20        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
            (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
            (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
            (reverse[e415] * self[e235])
                + (reverse[e425] * self[e315])
                + (reverse[e435] * self[e125])
                + (reverse[e235] * self[e415])
                + (reverse[e315] * self[e425])
                + (reverse[e125] * self[e435]),
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for CircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
        return NullSphereAtOrigin::from_groups(
            // e1234
            (reverse[e423] * self[e415])
                + (reverse[e431] * self[e425])
                + (reverse[e412] * self[e435])
                + (reverse[e415] * self[e423])
                + (reverse[e425] * self[e431])
                + (reverse[e435] * self[e412]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Dipole {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       35       45        0
    //  no simd       44       56        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]) + (reverse[e35] * self[e42]),
                (reverse[e43] * self[e15]) + (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]) + (reverse[e15] * self[e43]),
                (reverse[e41] * self[e25]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e25] * self[e41]),
                -(reverse[e23] * self[e15]) - (reverse[e31] * self[e25]) - (reverse[e12] * self[e35]) - (reverse[e35] * self[e12]),
            ]) - (reverse.group0().zxy() * self.group2().yzx()).with_w(reverse[e15] * self[e23])
                - (reverse.group2().yzx() * self.group0().zxy()).with_w(reverse[e25] * self[e31]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e45], 2)
                        - f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       19        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       21        0
    //  no simd       15       26        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
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
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       14        0
    //  no simd        8       19        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]),
            (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]),
            (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]),
            -(reverse[e23] * self[e15])
                - (reverse[e31] * self[e25])
                - (reverse[e12] * self[e35])
                - (reverse[e15] * self[e23])
                - (reverse[e25] * self[e31])
                - (reverse[e35] * self[e12]),
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       14       24        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
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
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversion {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       55        0
    //    simd3        0        3        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       60       68        0
    //  no simd       93      104        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
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
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23]) + (reverse[e1234] * self[e15]) + (reverse[e4315] * self[e12]),
                (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31]) + (reverse[e1234] * self[e25]) + (reverse[e4125] * self[e23]),
                (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e1234] * self[e35]) + (reverse[e4235] * self[e31]),
                -(reverse[e25] * self[e31]) - (reverse[e25] * self[e4315]) - (reverse[e35] * self[e12]) - (reverse[e35] * self[e4125]),
            ]) + (self.group1().ww().with_zw(self[e4315], self[e45]) * reverse.group1().xyx().with_w(reverse[e3215]))
                + (self.group2().zx().with_zw(self[e3215], self[e25]) * reverse.group0().yzz().with_w(reverse[e4315]))
                + (self.group3().ww().with_zw(self[e25], self[e15]) * reverse.group0().xyx().with_w(reverse[e4235]))
                + (self.group0().yzx() * reverse.group2().zxy()).with_w(reverse[e4125] * self[e35])
                - (Simd32x4::from([reverse[e25], reverse[e3215], reverse[e3215], self[e35]]) * self.group0().zyz().with_w(reverse[e12]))
                - (Simd32x4::from([reverse[e3215], reverse[e35], reverse[e15], self[e25]]) * self.group0().xxy().with_w(reverse[e31]))
                - (reverse.group1().zxyw() * self.group3().yzxw())
                - (reverse.group2().xyzx() * self.group2().www().with_w(self[e23]))
                - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (reverse.group3().zxy() * self.group1().yzx()).with_w(reverse[e15] * self[e4235]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                        - f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35])
                        - 2.0 * (self[e1234] * self[e3215]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       31        0
    //    simd3        0        3        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       57       68        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e41] * self[e4235]) + (reverse[e42] * self[e4315]) + (reverse[e43] * self[e4125]) + (reverse[e45] * self[e1234])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            (self.group1().zx().with_zw(self[e3215], self[e25]) * reverse.group0().yzz().with_w(reverse[e4315]))
                + (self.group2().ww().with_zw(self[e25], self[e15]) * reverse.group0().xyx().with_w(reverse[e4235]))
                + (reverse.group1().zxy() * self.group0().yzx()).with_w(reverse[e4125] * self[e35])
                + (reverse.group1().www() * self.group1().xyz()).with_w(reverse[e3215] * self[e45])
                - (reverse.group0().zxyw() * self.group1().yzx().with_w(self[e3215]))
                - (reverse.group1().xyxx() * self.group1().ww().with_zw(self[e42], self[e4235]))
                - (reverse.group1().yzzy() * self.group0().zx().with_zw(self[e1234], self[e4315]))
                - (reverse.group2().www() * self.group0().xyz()).with_w(reverse[e35] * self[e4125]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35])
                        - 2.0 * (self[e1234] * self[e3215]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       34        0
    //  no simd       28       39        0
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (reverse[e23] * self[e45]) + (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23]) + (reverse[e4315] * self[e12])
                - (reverse[e12] * self[e4315])
                - (reverse[e4125] * self[e31]),
            (reverse[e31] * self[e45]) + (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31]) + (reverse[e4125] * self[e23])
                - (reverse[e23] * self[e4125])
                - (reverse[e4235] * self[e12]),
            (reverse[e23] * self[e4315]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e4235] * self[e31])
                - (reverse[e31] * self[e4235])
                - (reverse[e4315] * self[e23]),
            (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35]) + (reverse[e3215] * self[e45])
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
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       28       34        0
    //  no simd       28       40        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e3215] * self[e1234]) + (reverse[e1234] * self[e3215])
                    - (reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
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
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0), /* e1234, e4235, e4315, e4125 */ self.group1());
        return NullSphereAtOrigin::from_groups(
            // e1234
            (reverse[e41] * self[e4235]) + (reverse[e42] * self[e4315]) + (reverse[e43] * self[e4125]) + (reverse[e45] * self[e1234])
                - (reverse[e1234] * self[e45])
                - (reverse[e4235] * self[e41])
                - (reverse[e4315] * self[e42])
                - (reverse[e4125] * self[e43]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       40        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       36       47        0
    //  no simd       51       66        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e41] * self[e3215]) + (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15]),
                (reverse[e42] * self[e3215]) + (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25]),
                (reverse[e41] * self[e25]) + (reverse[e43] * self[e3215]) + (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35]),
                -(reverse[e25] * self[e31]) - (reverse[e35] * self[e12]),
            ]) - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (self.group0().zx().with_zw(self[e1234], reverse[e15]) * reverse.group2().yzz().with_w(self[e23]))
                - (self.group2().ww().with_zw(self[e42], self[e35]) * reverse.group2().xyx().with_w(reverse[e12]))
                - (reverse.group0().www() * self.group0().xyz()).with_w(reverse[e31] * self[e25]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35])
                        - 2.0 * (self[e3215] * self[e1234]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DipoleOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       27       38        0
    //  no simd       36       48        0
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
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e15])
                    - (reverse[e42] * self[e25])
                    - (reverse[e43] * self[e35])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e15] * self[e41])
                    - (reverse[e25] * self[e42])
                    - (reverse[e35] * self[e43]),
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                -(reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e42] * self[e35]) + (reverse[e35] * self[e42]),
                (reverse[e43] * self[e15]) + (reverse[e15] * self[e43]),
                (reverse[e41] * self[e25]) + (reverse[e25] * self[e41]),
                -(reverse[e12] * self[e35]) - (reverse[e15] * self[e23]) - (reverse[e25] * self[e31]) - (reverse[e35] * self[e12]),
            ]) - (reverse.group0().zxy() * self.group2().yzx()).with_w(reverse[e23] * self[e15])
                - (reverse.group2().yzx() * self.group0().zxy()).with_w(reverse[e31] * self[e25]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for DualNum {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e4] * self[e12345] * -2.0);
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Flector {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       13        0
    //  no simd       17       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = Flector::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125, e3215 */ self.group1());
        let geometric_product = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w((reverse[e45] * self[e45]) - (reverse[e4235] * self[e4235]) - (reverse[e4315] * self[e4315]) - (reverse[e4125] * self[e4125])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                (reverse[e4235] * self[e15]) + (reverse[e4315] * self[e25]) + (reverse[e4125] * self[e35]) + (reverse[e3215] * self[e45])
                    - (reverse[e15] * self[e4235])
                    - (reverse[e25] * self[e4315])
                    - (reverse[e35] * self[e4125])
                    - (reverse[e45] * self[e3215]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Line {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd3        0        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       11        0
    //  no simd       13       15        0
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
            Simd32x3::from(0.0).with_w((reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       14        0
    //  no simd       17       20        0
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
            Simd32x3::from(0.0).with_w((reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e12345] * self[e12345])),
            // e15, e25, e35, e3215
            Simd32x3::from(0.0).with_w(
                (reverse[e415] * self[e235])
                    + (reverse[e425] * self[e315])
                    + (reverse[e435] * self[e125])
                    + (reverse[e235] * self[e415])
                    + (reverse[e315] * self[e425])
                    + (reverse[e125] * self[e435])
                    - (reverse[e12345] * self[e5])
                    - (reverse[e5] * self[e12345]),
            ),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            geometric_product.group0() + Simd32x3::from(0.0).with_w(f32::powi(self[e415], 2) + f32::powi(self[e425], 2) + f32::powi(self[e435], 2) + f32::powi(self[e12345], 2)),
            // e15, e25, e35, e3215
            geometric_product.group1(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<ConstraintViolationPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: ConstraintViolationPrefixOrPostfix) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      132      153        0
    //    simd2       16       17        0
    //    simd3        0       11        0
    //    simd4       52       46        0
    // Totals...
    // yes simd      200      227        0
    //  no simd      372      404        0
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
                    + (reverse[e5] * self[e1234])
                    + (reverse[e1234] * self[e5])
                    + (reverse[e4235] * self[e1])
                    + (reverse[e4315] * self[e2])
                    + (reverse[e4125] * self[e3])
                    - (reverse[e41] * self[e235])
                    - (reverse[e42] * self[e315])
                    - (reverse[e43] * self[e125])
                    - (reverse[e45] * self[e321])
                    - (reverse[e425] * self[e31])
                    - (reverse[e435] * self[e12])
                    - (reverse[e321] * self[e45])
                    - (reverse[e235] * self[e41])
                    - (reverse[e315] * self[e42])
                    - (reverse[e125] * self[e43]),
            ]) + (Simd32x2::from(reverse[scalar]) * self.group0())
                + (Simd32x2::from(reverse[e1]) * Simd32x2::from([self[e1], self[e4235]]))
                + (Simd32x2::from(reverse[e2]) * Simd32x2::from([self[e2], self[e4315]]))
                + (Simd32x2::from(reverse[e3]) * Simd32x2::from([self[e3], self[e4125]]))
                + (Simd32x2::from(reverse[e3215]) * Simd32x2::from([self[e1234], self[e4]]))
                + (Simd32x2::from(self[e3215]) * Simd32x2::from([reverse[e1234], reverse[e4]]))
                - (Simd32x2::from(reverse[e15]) * Simd32x2::from([self[e41], self[e423]]))
                - (Simd32x2::from(reverse[e25]) * Simd32x2::from([self[e42], self[e431]]))
                - (Simd32x2::from(reverse[e35]) * Simd32x2::from([self[e43], self[e412]]))
                - (Simd32x2::from(reverse[e23]) * Simd32x2::from([self[e23], self[e415]]))
                - (Simd32x2::from(reverse[e31]) * Simd32x2::from([self[e31], self[e425]]))
                - (Simd32x2::from(reverse[e12]) * Simd32x2::from([self[e12], self[e435]]))
                - (Simd32x2::from(self[e15]) * Simd32x2::from([reverse[e41], reverse[e423]]))
                - (Simd32x2::from(self[e25]) * Simd32x2::from([reverse[e42], reverse[e431]]))
                - (Simd32x2::from(self[e35]) * Simd32x2::from([reverse[e43], reverse[e412]]))
                - (Simd32x2::from([self[e321], self[e23]]) * reverse.group6().wx()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse[e5] * self[e41]) + (reverse[e43] * self[e315]) + (reverse[e435] * self[e4315]) + (reverse[e4125] * self[e425]),
                (reverse[e5] * self[e42]) + (reverse[e41] * self[e125]) + (reverse[e415] * self[e4125]) + (reverse[e4235] * self[e435]),
                (reverse[e5] * self[e43]) + (reverse[e42] * self[e235]) + (reverse[e425] * self[e4235]) + (reverse[e4315] * self[e415]),
                -(reverse[e415] * self[e41]) - (reverse[e425] * self[e42]) - (reverse[e435] * self[e43]) - (reverse[e321] * self[e1234]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group1())
                + (Simd32x4::from([reverse[e2], reverse[e321], reverse[e321], self[e4]]) * self.group5().zyz().with_w(reverse[e45]))
                + (Simd32x4::from([reverse[e321], reverse[e3], reverse[e1], self[e43]]) * self.group5().xxy().with_w(reverse[e3]))
                + (self.group0().xx().with_zw(self[scalar], reverse[e12345]) * reverse.group1().xyz().with_w(self[e1234]))
                + (self.group7().zx().with_zw(self[e4], reverse[e1234]) * reverse.group4().yzz().with_w(self[e12345]))
                + (self.group1().zx().with_zw(self[e321], self[e4315]) * reverse.group5().yzz().with_w(reverse[e431]))
                + (self.group1().ww().with_zw(self[e431], reverse[e4]) * reverse.group4().xyx().with_w(self[scalar]))
                + (self.group3().zx().with_zw(self[e1234], self[e42]) * reverse.group8().yzz().with_w(reverse[e2]))
                + (self.group6().ww().with_zw(self[e2], self[e4235]) * reverse.group5().xyx().with_w(reverse[e423]))
                + (self.group9().xx().with_zw(self[e42], self[e41]) * reverse.group8().xyx().with_w(reverse[e1]))
                + (Simd32x3::from(reverse[e3215]) * self.group7()).with_w(reverse[e1234] * self[e321])
                + (reverse.group7().zxy() * self.group4().yzx()).with_w(reverse[e412] * self[e4125])
                - (Simd32x4::from([reverse[e42], reverse[e1234], reverse[e1234], self[e1]]) * self.group8().zyz().with_w(reverse[e41]))
                - (Simd32x4::from([reverse[e1234], reverse[e43], reverse[e41], self[e45]]) * self.group8().xxy().with_w(reverse[e4]))
                - (reverse.group3().xyzx() * Simd32x3::from(self[e5]).with_w(self[e415]))
                - (reverse.group3().wwwy() * self.group6().xyz().with_w(self[e2]))
                - (self.group6().zxyz() * reverse.group9().zwy().with_w(reverse[e43]))
                - (Simd32x2::from(self[e3215]).with_zw(self[e25], self[e31]) * reverse.group7().xyx().with_w(reverse[e431]))
                - (reverse.group0().yy().with_zw(reverse[e12345], reverse[e23]) * self.group9().yzw().with_w(self[e423]))
                - (self.group0().yy().with_zw(self[e12345], reverse[e31]) * reverse.group9().yzw().with_w(self[e431]))
                - (self.group4().zx().with_zw(self[e3215], self[e12]) * reverse.group7().yzz().with_w(reverse[e412]))
                - (self.group3().ww().with_zw(self[e4315], self[e425]) * reverse.group6().xyx().with_w(reverse[e42]))
                - (self.group9().wy().with_zw(self[e45], self[e3]) * reverse.group6().yzz().with_w(reverse[e43]))
                - (self.group4() * reverse.group1().www()).with_w(reverse[e4315] * self[e431])
                - (reverse.group4().zxy() * self.group7().yzx()).with_w(reverse[e12] * self[e412])
                - (reverse.group5().zxy() * self.group1().yzx()).with_w(reverse[e423] * self[e23])
                - (reverse.group8().zxy() * self.group3().yzx()).with_w(reverse[e4235] * self[e423])
                - (self.group5().yzx() * reverse.group1().zxy()).with_w(reverse[e4125] * self[e412]),
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
                (reverse[e321] * self[e4])
                    - (reverse[e3] * self[e412])
                    - (reverse[e4] * self[e321])
                    - (reverse[e43] * self[e12])
                    - (reverse[e1234] * self[e45])
                    - (reverse[e4235] * self[e41])
                    - (reverse[e4315] * self[e42])
                    - (reverse[e4125] * self[e43]),
                (reverse[e1] * self[e12345]) + (reverse[e35] * self[e42]) + (reverse[e23] * self[e45]) + (reverse[e1234] * self[e15]) + (reverse[e4315] * self[e12])
                    - (reverse[e15] * self[e1234])
                    - (reverse[e423] * self[e5])
                    - (reverse[e431] * self[e125]),
                (reverse[e3] * self[e415]) + (reverse[e42] * self[e3215]) + (reverse[e425] * self[e321]) + (reverse[e435] * self[e1]) + (reverse[e321] * self[e425])
                    - (reverse[e1] * self[e435])
                    - (reverse[e415] * self[e3])
                    - (reverse[e3215] * self[e42]),
                (reverse[e1] * self[e425]) + (reverse[e43] * self[e3215]) + (reverse[e415] * self[e2]) + (reverse[e435] * self[e321]) + (reverse[e321] * self[e435])
                    - (reverse[e2] * self[e415])
                    - (reverse[e425] * self[e1])
                    - (reverse[e3215] * self[e43]),
            ]) + (Simd32x4::from(reverse[scalar]) * self.group9())
                + (Simd32x4::from([reverse[e41], reverse[e31], reverse[e1234], reverse[e1234]]) * self.group9().yw().with_zw(self[e25], self[e35]))
                + (Simd32x4::from([reverse[e415], reverse[e315], self[e4], self[e431]]) * self.group7().xz().with_zw(reverse[e315], reverse[e235]))
                + (Simd32x4::from([reverse[e425], reverse[e5], self[e423], self[e4]]) * self.group7().yx().with_zw(reverse[e125], reverse[e125]))
                + (Simd32x4::from([reverse[e423], reverse[e12345], reverse[e4315], reverse[e4125]]) * self.group1().xx().with_zw(self[scalar], self[scalar]))
                + (Simd32x4::from([reverse[e431], reverse[e2], self[e45], self[e4315]]) * self.group6().yz().with_zw(reverse[e31], reverse[e23]))
                + (Simd32x4::from([reverse[e431], reverse[e235], self[e43], self[e41]]) * self.group1().yw().with_zw(reverse[e15], reverse[e25]))
                + (Simd32x4::from([reverse[e412], reverse[e415], self[e125], self[e235]]) * self.group6().zw().with_zw(reverse[e423], reverse[e431]))
                + (Simd32x4::from([reverse[e412], reverse[e425], self[e4235], self[e45]]) * self.group1().zz().with_zw(reverse[e12], reverse[e12]))
                + (Simd32x4::from([self[e415], self[e315], reverse[e2], reverse[e3]]) * reverse.group7().xz().with_zw(self[e12345], self[e12345]))
                + (Simd32x4::from([self[e412], self[e415], reverse[e43], reverse[e41]]) * reverse.group6().zw().with_zw(self[e15], self[e25]))
                + (Simd32x4::from([self[e1234], self[e3215], reverse[e5], reverse[e5]]) * reverse.group3().wx().with_zw(self[e431], self[e412]))
                + (Simd32x4::from([self[e4315], self[e35], reverse[e4125], reverse[e4235]]) * reverse.group3().yy().with_zw(self[e23], self[e31]))
                + (Simd32x4::from([self[e4125], self[e23], reverse[e45], reverse[e45]]) * reverse.group3().zw().with_zw(self[e31], self[e12]))
                + (self.group0().xx() * reverse.group9().xy()).with_zw(reverse[e12345] * self[e2], reverse[e12345] * self[e3])
                - (Simd32x4::from([reverse[e12345], reverse[e435], self[e1234], self[e42]]) * self.group1().wy().with_zw(reverse[e25], reverse[e15]))
                - (Simd32x4::from([reverse[e1], reverse[e125], reverse[e4235], reverse[e4315]]) * self.group7().xy().with_zw(self[e12], self[e23]))
                - (Simd32x4::from([reverse[e42], reverse[e4125], reverse[e41], reverse[e42]]) * self.group5().yy().with_zw(self[e35], self[e15]))
                - (Simd32x4::from([reverse[e23], reverse[e25], self[e4125], self[e4235]]) * self.group3().xz().with_zw(reverse[e23], reverse[e31]))
                - (Simd32x4::from([reverse[e12], reverse[e3215], self[e235], self[e5]]) * self.group3().zx().with_zw(reverse[e412], reverse[e412]))
                - (Simd32x4::from([self[e12345], self[e235], self[e41], self[e1234]]) * reverse.group1().ww().with_zw(reverse[e35], reverse[e35]))
                - (Simd32x4::from([self[e42], self[e4315], self[e5], self[e315]]) * reverse.group5().yz().with_zw(reverse[e431], reverse[e423]))
                - (Simd32x4::from([self[e23], self[e25], self[e412], self[e423]]) * reverse.group3().xz().with_zw(reverse[e235], reverse[e315]))
                - (Simd32x4::from([self[e431], self[e425], reverse[e4], reverse[e4]]) * reverse.group1().yz().with_zw(self[e315], self[e125])),
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, geometric_product[e12345]]),
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
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MysteryCircle {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryCircle {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       14        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0));
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e321] * self[e321]),
                (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
                (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
                (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MysteryCircleRotor {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryCircleRotor {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       15        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from(-1.0), /* e12345 */ self[e12345]);
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e415] * self[e415]) + (reverse[e425] * self[e425]) + (reverse[e435] * self[e435]) - (reverse[e321] * self[e321]) - (reverse[e12345] * self[e12345]),
                (reverse[e415] * self[e321]) + (reverse[e321] * self[e415]),
                (reverse[e425] * self[e321]) + (reverse[e321] * self[e425]),
                (reverse[e435] * self[e321]) + (reverse[e321] * self[e435]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MysteryDipole {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryDipole {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       14        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0));
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e45] * self[e45]) - (reverse[e23] * self[e23]) - (reverse[e31] * self[e31]) - (reverse[e12] * self[e12]),
                (reverse[e23] * self[e45]) + (reverse[e45] * self[e23]),
                (reverse[e31] * self[e45]) + (reverse[e45] * self[e31]),
                (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       26        0
    //  no simd       21       29        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from(-1.0), /* e4235, e4315, e4125 */ self.group1());
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e45] * self[e45])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125]),
                (reverse[e23] * self[e45]) + (reverse[e31] * self[e4125]) + (reverse[e45] * self[e23]) + (reverse[e4315] * self[e12])
                    - (reverse[e12] * self[e4315])
                    - (reverse[e4125] * self[e31]),
                (reverse[e31] * self[e45]) + (reverse[e12] * self[e4235]) + (reverse[e45] * self[e31]) + (reverse[e4125] * self[e23])
                    - (reverse[e23] * self[e4125])
                    - (reverse[e4235] * self[e12]),
                (reverse[e23] * self[e4315]) + (reverse[e12] * self[e45]) + (reverse[e45] * self[e12]) + (reverse[e4235] * self[e31])
                    - (reverse[e31] * self[e4235])
                    - (reverse[e4315] * self[e23]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MysteryVersorEven {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryVersorEven {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       33        0
    //  no simd       28       36        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ self.group0(), /* e415, e425, e435, e321 */ self.group1() * Simd32x4::from(-1.0));
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
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
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for MysteryVersorOdd {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for MysteryVersorOdd {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       33        0
    //  no simd       28       36        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ self.group0(), /* e23, e31, e12, e45 */ self.group1() * Simd32x4::from(-1.0));
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[scalar] * self[scalar]) + (reverse[e45] * self[e45])
                    - (reverse[e4235] * self[e4235])
                    - (reverse[e4315] * self[e4315])
                    - (reverse[e4125] * self[e4125])
                    - (reverse[e23] * self[e23])
                    - (reverse[e31] * self[e31])
                    - (reverse[e12] * self[e12]),
                (reverse[scalar] * self[e4235])
                    + (reverse[e4235] * self[scalar])
                    + (reverse[e4315] * self[e12])
                    + (reverse[e23] * self[e45])
                    + (reverse[e31] * self[e4125])
                    + (reverse[e45] * self[e23])
                    - (reverse[e4125] * self[e31])
                    - (reverse[e12] * self[e4315]),
                (reverse[scalar] * self[e4315])
                    + (reverse[e4315] * self[scalar])
                    + (reverse[e4125] * self[e23])
                    + (reverse[e31] * self[e45])
                    + (reverse[e12] * self[e4235])
                    + (reverse[e45] * self[e31])
                    - (reverse[e4235] * self[e12])
                    - (reverse[e23] * self[e4125]),
                (reverse[scalar] * self[e4125])
                    + (reverse[e4235] * self[e31])
                    + (reverse[e4125] * self[scalar])
                    + (reverse[e23] * self[e4315])
                    + (reverse[e12] * self[e45])
                    + (reverse[e45] * self[e12])
                    - (reverse[e4315] * self[e23])
                    - (reverse[e31] * self[e4235]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       44        0
    //    simd3        0        4        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       57       63        0
    //  no simd      105      116        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
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
            ),
            // e4235, e4315, e4125, e3215
            (reverse.group1().xyxz() * self.group1().ww().with_zw(self[e2], self[e125]))
                + (reverse.group2().xyxz() * self.group3().ww().with_zw(self[e431], self[e435]))
                + (reverse.group2().yzzw() * self.group0().zx().with_zw(self[e4], self[e321]))
                + (reverse.group3().xyxy() * self.group0().ww().with_zw(self[e425], self[e315]))
                + (reverse.group3().yzzz() * self.group1().zx().with_zw(self[e12345], self[e125]))
                + (self.group1().xyzy() * reverse.group1().www().with_w(reverse[e315]))
                + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group3().zx().with_zw(self[e321], self[e415]) * reverse.group1().yzz().with_w(reverse[e235]))
                + (reverse.group0().www() * self.group3().xyz()).with_w(reverse[e425] * self[e315])
                + (reverse.group2().www() * self.group0().xyz()).with_w(reverse[e1] * self[e235])
                - (reverse.group0().xyxw() * self.group2().wwyw())
                - (reverse.group2().zxyy() * self.group0().yzx().with_w(self[e2]))
                - (self.group2().zxww() * reverse.group0().yzz().with_w(reverse[e321]))
                - (self.group3().yzxx() * reverse.group1().zxy().with_w(reverse[e235]))
                - (reverse.group3().zxy() * self.group1().yzx()).with_w(reverse[e125] * self[e3])
                - (reverse.group3().www() * self.group2().xyz()).with_w(reverse[e5] * self[e12345]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + 2.0 * (self[e5] * self[e4])
                        + f32::powi(self[e12345], 2)
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2)
                        - f32::powi(self[e321], 2)
                        - f32::powi(self[e1], 2)
                        - f32::powi(self[e2], 2)
                        - f32::powi(self[e3], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       37        0
    //    simd3        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       36       46        0
    //  no simd       57       72        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e415])
                    + (reverse[e431] * self[e425])
                    + (reverse[e412] * self[e435])
                    + (reverse[e415] * self[e423])
                    + (reverse[e425] * self[e431])
                    + (reverse[e435] * self[e412])
                    - (reverse[e12345] * self[e4])
                    - (reverse[e4] * self[e12345]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(reverse[e4] * self[e235]) - (reverse[e125] * self[e431]),
                -(reverse[e4] * self[e315]) - (reverse[e235] * self[e412]),
                -(reverse[e4] * self[e125]) - (reverse[e315] * self[e423]),
                (reverse[e315] * self[e425]) + (reverse[e125] * self[e435]),
            ]) + (reverse.group2().wwwx() * self.group0().xyz().with_w(self[e415]))
                + (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e415]))
                + (self.group0().zx().with_zw(self[e4], self[e125]) * reverse.group2().yzz().with_w(reverse[e435]))
                + (self.group1().ww().with_zw(self[e431], self[e315]) * reverse.group2().xyx().with_w(reverse[e425]))
                - (reverse.group0().xyxw() * self.group2().wwyw())
                - (reverse.group0().yzz() * self.group2().zxw()).with_w(reverse[e5] * self[e12345]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235])
                        + 2.0 * (self[e431] * self[e315])
                        + 2.0 * (self[e412] * self[e125])
                        + 2.0 * (self[e4] * self[e5])
                        + f32::powi(self[e12345], 2)
                        + f32::powi(self[e415], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e435], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       40        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       36       42        0
    //  no simd       36       48        0
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
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
            (reverse[e1] * self[e235])
                + (reverse[e2] * self[e315])
                + (reverse[e3] * self[e125])
                + (reverse[e415] * self[e235])
                + (reverse[e425] * self[e315])
                + (reverse[e435] * self[e125])
                + (reverse[e235] * self[e415])
                + (reverse[e315] * self[e425])
                + (reverse[e125] * self[e435])
                + (reverse[e5] * self[e321])
                - (reverse[e12345] * self[e5])
                - (reverse[e321] * self[e5])
                - (reverse[e235] * self[e1])
                - (reverse[e315] * self[e2])
                - (reverse[e125] * self[e3])
                - (reverse[e5] * self[e12345]),
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       28       34        0
    //  no simd       28       40        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([
                (reverse[e423] * self[e235])
                    + (reverse[e431] * self[e315])
                    + (reverse[e412] * self[e125])
                    + (reverse[e235] * self[e423])
                    + (reverse[e315] * self[e431])
                    + (reverse[e125] * self[e412])
                    - (reverse[e4] * self[e5])
                    - (reverse[e5] * self[e4]),
                (reverse[e412] * self[e315]) + (reverse[e235] * self[e4]) + (reverse[e315] * self[e412]) + (reverse[e5] * self[e423])
                    - (reverse[e423] * self[e5])
                    - (reverse[e431] * self[e125])
                    - (reverse[e4] * self[e235])
                    - (reverse[e125] * self[e431]),
                (reverse[e423] * self[e125]) + (reverse[e315] * self[e4]) + (reverse[e125] * self[e423]) + (reverse[e5] * self[e431])
                    - (reverse[e431] * self[e5])
                    - (reverse[e412] * self[e235])
                    - (reverse[e4] * self[e315])
                    - (reverse[e235] * self[e412]),
                (reverse[e431] * self[e235]) + (reverse[e235] * self[e431]) + (reverse[e125] * self[e4]) + (reverse[e5] * self[e412])
                    - (reverse[e423] * self[e315])
                    - (reverse[e412] * self[e5])
                    - (reverse[e4] * self[e125])
                    - (reverse[e315] * self[e423]),
            ])
            .yzw(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let reverse = VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return NullSphereAtOrigin::from_groups(
            // e1234
            (reverse[e423] * self[e415])
                + (reverse[e431] * self[e425])
                + (reverse[e412] * self[e435])
                + (reverse[e415] * self[e423])
                + (reverse[e425] * self[e431])
                + (reverse[e435] * self[e412])
                - (reverse[e12345] * self[e4])
                - (reverse[e4] * self[e12345]),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       32        0
    //    simd3        0        4        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       33       42        0
    //  no simd       57       68        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[e423] * self[e1]) + (reverse[e431] * self[e2]) + (reverse[e412] * self[e3]) + (reverse[e321] * self[e4])
                    - (reverse[e1] * self[e423])
                    - (reverse[e2] * self[e431])
                    - (reverse[e3] * self[e412])
                    - (reverse[e4] * self[e321]),
            ),
            // e4235, e4315, e4125, e3215
            (self.group0().zx().with_zw(self[e4], self[e315]) * reverse.group1().yzz().with_w(reverse[e2]))
                + (self.group2().ww().with_zw(self[e431], self[e235]) * reverse.group1().xyx().with_w(reverse[e1]))
                + (reverse.group0().zxy() * self.group1().yzx()).with_w(reverse[e5] * self[e321])
                + (reverse.group1().www() * self.group0().xyz()).with_w(reverse[e3] * self[e125])
                - (reverse.group0().xyxw() * self.group1().wwyw())
                - (reverse.group1().zxyy() * self.group0().yzx().with_w(self[e2]))
                - (reverse.group0().yzz() * self.group1().zxw()).with_w(reverse[e235] * self[e1])
                - (reverse.group2().www() * self.group1().xyz()).with_w(reverse[e125] * self[e3]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e5] * self[e4])
                        - f32::powi(self[e321], 2)
                        - f32::powi(self[e1], 2)
                        - f32::powi(self[e2], 2)
                        - f32::powi(self[e3], 2),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl std::ops::DivAssign<ConstraintViolationPrefixOrPostfix> for VersorOdd {
    fn div_assign(&mut self, _rhs: ConstraintViolationPrefixOrPostfix) {
        *self = self.constraint_violation()
    }
}
impl ConstraintViolation for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       58        0
    //    simd3        0        2        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       66       73        0
    //  no simd      105      116        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
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
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15]) + (reverse[e4235] * self[scalar]) + (reverse[e4315] * self[e12]),
                (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25]) + (reverse[e4315] * self[scalar]) + (reverse[e4125] * self[e23]),
                (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35]) + (reverse[e4235] * self[e31]) + (reverse[e4125] * self[scalar]),
                -(reverse[e25] * self[e31]) - (reverse[e25] * self[e4315]) - (reverse[e35] * self[e12]) - (reverse[e35] * self[e4125]),
            ]) + (self.group1() * reverse.group1().www().with_w(reverse[e3215]))
                + (reverse.group0().xyxw() * self.group3().ww().with_zw(self[e25], self[e3215]))
                + (self.group1().ww().with_zw(self[e4315], self[e35]) * reverse.group1().xyx().with_w(reverse[e4125]))
                + (self.group2().zx().with_zw(self[e3215], self[e15]) * reverse.group0().yzz().with_w(reverse[e4235]))
                + (self.group3().zx().with_zw(self[e45], self[scalar]) * reverse.group1().yzz().with_w(reverse[e3215]))
                + (reverse.group0().www() * self.group3().xyz()).with_w(reverse[e4315] * self[e25])
                - (reverse.group1().zxyy() * self.group3().yzx().with_w(self[e25]))
                - (self.group1().yzxx() * reverse.group3().zxy().with_w(reverse[e15]))
                - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (self.group0().zx().with_zw(self[e1234], self[e3215]) * reverse.group2().yzz().with_w(reverse[e45]))
                - (self.group2().ww().with_zw(self[e42], self[e35]) * reverse.group2().xyx().with_w(reverse[e12]))
                - (reverse.group3().www() * self.group0().xyz()).with_w(reverse[e15] * self[e4235]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)
                        - f32::powi(self[scalar], 2)
                        - f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35])
                        - 2.0 * (self[e1234] * self[e3215]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = Plane;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorOddAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       40        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       36       42        0
    //  no simd       36       48        0
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (reverse[scalar] * self[e4235])
                + (reverse[e23] * self[e45])
                + (reverse[e31] * self[e4125])
                + (reverse[e45] * self[e23])
                + (reverse[e4235] * self[scalar])
                + (reverse[e4315] * self[e12])
                - (reverse[e12] * self[e4315])
                - (reverse[e4125] * self[e31]),
            (reverse[scalar] * self[e4315])
                + (reverse[e31] * self[e45])
                + (reverse[e12] * self[e4235])
                + (reverse[e45] * self[e31])
                + (reverse[e4315] * self[scalar])
                + (reverse[e4125] * self[e23])
                - (reverse[e23] * self[e4125])
                - (reverse[e4235] * self[e12]),
            (reverse[scalar] * self[e4125])
                + (reverse[e23] * self[e4315])
                + (reverse[e12] * self[e45])
                + (reverse[e45] * self[e12])
                + (reverse[e4235] * self[e31])
                + (reverse[e4125] * self[scalar])
                - (reverse[e31] * self[e4235])
                - (reverse[e4315] * self[e23]),
            (reverse[scalar] * self[e3215])
                + (reverse[e4235] * self[e15])
                + (reverse[e4315] * self[e25])
                + (reverse[e4125] * self[e35])
                + (reverse[e3215] * self[scalar])
                + (reverse[e3215] * self[e45])
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
        ]));
    }
}
impl std::ops::Div<ConstraintViolationPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn div(self, _rhs: ConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.constraint_violation()
    }
}
impl ConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       36        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       36       45        0
    //  no simd       57       72        0
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
            Simd32x3::from(0.0).with_w(
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
            ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(
                (reverse[scalar] * self[e1234]) + (reverse[e1234] * self[scalar])
                    - (reverse[e41] * self[e23])
                    - (reverse[e42] * self[e31])
                    - (reverse[e43] * self[e12])
                    - (reverse[e23] * self[e41])
                    - (reverse[e31] * self[e42])
                    - (reverse[e12] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (reverse[e35] * self[e42]) + (reverse[e1234] * self[e15]),
                (reverse[e15] * self[e43]) + (reverse[e1234] * self[e25]),
                (reverse[e25] * self[e41]) + (reverse[e1234] * self[e35]),
                -(reverse[e25] * self[e31]) - (reverse[e35] * self[e12]),
            ]) + (reverse.group0().xyxw() * self.group1().ww().with_zw(self[e25], self[e3215]))
                + (self.group2().zx().with_zw(self[e3215], self[scalar]) * reverse.group0().yzz().with_w(reverse[e3215]))
                - (reverse.group1().wwwy() * self.group0().xyz().with_w(self[e25]))
                - (reverse.group2().yzzx() * self.group0().zx().with_zw(self[e1234], self[e23]))
                - (self.group2().yzxx() * reverse.group0().zxy().with_w(reverse[e23]))
                - (self.group2().ww().with_zw(self[e42], self[e35]) * reverse.group2().xyx().with_w(reverse[e12])),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            geometric_product.group0()
                + Simd32x3::from(0.0).with_w(
                    -f32::powi(self[scalar], 2)
                        - f32::powi(self[e23], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e12], 2)
                        - 2.0 * (self[e41] * self[e15])
                        - 2.0 * (self[e42] * self[e25])
                        - 2.0 * (self[e43] * self[e35])
                        - 2.0 * (self[e3215] * self[e1234]),
                ),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            geometric_product.group2(),
            // e4235, e4315, e4125, e3215
            geometric_product.group3(),
        );
    }
}
